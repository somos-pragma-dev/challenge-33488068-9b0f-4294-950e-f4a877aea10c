use crate::domain::models::loan_request::{LoanRequest, LoanRequestStatus};
use crate::domain::dtos::loan_request_dto::{LoanRequestDto, LoanRequestResponseDto, PaginatedLoanRequestResponse};
use crate::domain::error::DomainError;
use crate::infrastructure::repository::loan_request_repository::LoanRequestRepository;
use crate::infrastructure::risk_engine::risk_engine_client::RiskEngineClient;
use crate::infrastructure::audit::audit_event::AuditEvent;
use actix_web::web;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use backoff::{ExponentialBackoff, future::retry};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct LoanRequestService {
    repository: Arc<RwLock<dyn LoanRequestRepository>>,
    risk_client: Arc<RiskEngineClient>,
    config: web::Data<AppConfig>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AppConfig {
    pub database_url: String,
    pub database_pool_size: u32,
    pub risk_engine_base_url: String,
    pub risk_engine_timeout_secs: u64,
    pub risk_engine_max_retries: u32,
    pub audit_enabled: bool,
    pub audit_queue_name: String,
    pub retry_initial_delay_ms: u64,
    pub retry_max_delay_ms: u64,
    pub retry_multiplier: f64,
    pub retry_max_retries: u32,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            database_url: String::from("postgresql://user:password@localhost/loan_db"),
            database_pool_size: 20,
            risk_engine_base_url: String::from("http://risk-engine.internal:8080"),
            risk_engine_timeout_secs: 10,
            risk_engine_max_retries: 3,
            audit_enabled: true,
            audit_queue_name: String::from("audit_events"),
            retry_initial_delay_ms: 100,
            retry_max_delay_ms: 30000,
            retry_multiplier: 2.0,
            retry_max_retries: 5,
        }
    }
}

impl LoanRequestService {
    pub fn new(
        repository: Arc<RwLock<dyn LoanRequestRepository>>,
        risk_client: Arc<RiskEngineClient>,
        config: web::Data<AppConfig>,
    ) -> Self {
        Self {
            repository,
            risk_client,
            config,
        }
    }

    pub async fn create_loan_request(
        &self,
        dto: LoanRequestDto,
    ) -> Result<LoanRequestResponseDto, DomainError> {
        dto.validate()?;
        
        let idempotency_key = dto.generate_idempotency_key();
        
        let repo = self.repository.read().await;
        if let Some(existing) = repo.find_by_idempotency_key(&idempotency_key).await? {
            return Err(DomainError::idempotency_conflict(
                "Ya existe una solicitud con este número de operación y canal",
                existing.id,
            ));
        }
        drop(repo);

        let entity = dto.to_entity(idempotency_key.clone());
        entity.validate()?;

        let risk_result = self.evaluate_risk_with_retry(&entity).await;
        
        let mut final_entity = entity;
        match risk_result {
            Ok(risk_response) => {
                if risk_response.approved {
                    final_entity.approve(risk_response.risk_score, risk_response.decision.clone());
                } else {
                    final_entity.reject(
                        risk_response.risk_score,
                        risk_response.decision.clone(),
                        risk_response.rejection_reason.clone().unwrap_or_default(),
                    );
                }
            }
            Err(e) => {
                self.emit_audit_event(&final_entity, &e).await;
                final_entity.mark_under_review();
            }
        }

        let repo = self.repository.write().await;
        let saved = repo.save(final_entity).await?;
        
        Ok(LoanRequestResponseDto::from(saved))
    }

    async fn evaluate_risk_with_retry(
        &self,
        entity: &LoanRequest,
    ) -> Result<RiskEvaluationResponse, DomainError> {
        let config = self.config.clone();
        
        let backoff = ExponentialBackoff {
            initial_interval: std::time::Duration::from_millis(config.retry_initial_delay_ms),
            max_interval: std::time::Duration::from_millis(config.retry_max_delay_ms),
            multiplier: config.retry_multiplier,
            max_elapsed_time: Some(std::time::Duration::from_secs(
                config.risk_engine_timeout_secs * 2,
            )),
            ..Default::default()
        };

        let entity_clone = entity.clone();
        let risk_client = self.risk_client.clone();
        
        retry(backoff, || {
            let entity = entity_clone.clone();
            let client = risk_client.clone();
            async move {
                client.evaluate_risk(&entity).await
            }
        }).await.map_err(|e| {
            DomainError::external_service(
                format!("Error al evaluar riesgo después de reintentos: {}", e),
                "risk_engine",
            )
        })
    }

    async fn emit_audit_event(&self, entity: &LoanRequest, error: &DomainError) {
        if !self.config.audit_enabled {
            return;
        }

        let event = AuditEvent::new(
            "loan_request_risk_evaluation_failed".to_string(),
            entity.id,
            serde_json::json!({
                "error_code": error.error_code(),
                "retryable": error.is_retryable(),
                "timestamp": Utc::now().to_rfc3339(),
            }),
        );

        log::warn!("Audit event emitted: {:?}", event);
    }

    pub async fn get_loan_request(&self, id: i64) -> Result<LoanRequestResponseDto, DomainError> {
        let repo = self.repository.read().await;
        let entity = repo.find_by_id(id).await?;
        Ok(LoanRequestResponseDto::from(entity))
    }

    pub async fn list_loan_requests(
        &self,
        page: i32,
        per_page: i32,
    ) -> Result<PaginatedLoanRequestResponse, DomainError> {
        let repo = self.repository.read().await;
        let (entities, total) = repo.find_all(page, per_page).await?;
        Ok(PaginatedLoanRequestResponse::from_entities(entities, total, page, per_page))
    }

    pub async fn cancel_loan_request(&self, id: i64) -> Result<LoanRequestResponseDto, DomainError> {
        let repo = self.repository.write().await;
        let mut entity = repo.find_by_id(id).await?;
        
        if !entity.can_be_modified() {
            return Err(DomainError::business(
                "La solicitud no puede ser cancelada en su estado actual",
                "INVALID_STATE_TRANSITION",
            ));
        }
        
        entity.cancel();
        let updated = repo.update(entity).await?;
        Ok(LoanRequestResponseDto::from(updated))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskEvaluationResponse {
    pub approved: bool,
    pub risk_score: i32,
    pub decision: String,
    pub rejection_reason: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::models::loan_request::LoanRequestStatus;
    
    #[test]
    fn test_app_config_default() {
        let config = AppConfig::default();
        assert_eq!(config.database_pool_size, 20);
        assert_eq!(config.risk_engine_max_retries, 3);
        assert_eq!(config.retry_max_retries, 5);
        assert!(config.audit_enabled);
    }
}