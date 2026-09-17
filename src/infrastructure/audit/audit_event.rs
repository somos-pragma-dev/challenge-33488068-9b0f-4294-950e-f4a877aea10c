use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::mpsc;
use crate::domain::models::loan_request::LoanRequest;
use crate::domain::error::DomainError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    pub event_id: String,
    pub event_type: AuditEventType,
    pub loan_request_id: i64,
    pub idempotency_key: String,
    pub document_number: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub payload: AuditPayload,
    pub status: AuditEventStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AuditEventType {
    LoanRequestCreated,
    LoanRequestApproved,
    LoanRequestRejected,
    RiskEngineFailure,
    RiskEngineRetry,
    RiskEngineSuccess,
    SystemError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditPayload {
    pub requested_amount: f64,
    pub term_months: i32,
    pub risk_score: Option<i32>,
    pub error_message: Option<String>,
    pub retry_count: Option<u32>,
    pub additional_info: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AuditEventStatus {
    Pending,
    Sent,
    Failed,
}

impl AuditEvent {
    pub fn new(
        event_type: AuditEventType,
        loan_request: &LoanRequest,
        payload: AuditPayload,
    ) -> Self {
        Self {
            event_id: uuid::Uuid::new_v4().to_string(),
            event_type,
            loan_request_id: loan_request.id(),
            idempotency_key: loan_request.idempotency_key(),
            document_number: loan_request.document_number().to_string(),
            timestamp: chrono::Utc::now(),
            payload,
            status: AuditEventStatus::Pending,
        }
    }

    pub fn for_risk_engine_failure(
        loan_request: &LoanRequest,
        error_message: &str,
        retry_count: u32,
    ) -> Self {
        let payload = AuditPayload {
            requested_amount: loan_request.requested_amount(),
            term_months: loan_request.term_months(),
            risk_score: None,
            error_message: Some(error_message.to_string()),
            retry_count: Some(retry_count),
            additional_info: std::collections::HashMap::new(),
        };

        Self::new(AuditEventType::RiskEngineFailure, loan_request, payload)
    }

    pub fn for_risk_engine_retry(
        loan_request: &LoanRequest,
        retry_count: u32,
    ) -> Self {
        let payload = AuditPayload {
            requested_amount: loan_request.requested_amount(),
            term_months: loan_request.term_months(),
            risk_score: None,
            error_message: None,
            retry_count: Some(retry_count),
            additional_info: std::collections::HashMap::new(),
        };

        Self::new(AuditEventType::RiskEngineRetry, loan_request, payload)
    }

    pub fn mark_as_sent(&mut self) {
        self.status = AuditEventStatus::Sent;
    }

    pub fn mark_as_failed(&mut self) {
        self.status = AuditEventStatus::Failed;
    }
}

pub struct AuditEventEmitter {
    sender: mpsc::Sender<AuditEvent>,
}

impl AuditEventEmitter {
    pub fn new(sender: mpsc::Sender<AuditEvent>) -> Self {
        Self { sender }
    }

    pub async fn emit(&self, event: AuditEvent) -> Result<(), DomainError> {
        self.sender.send(event).await.map_err(|e| {
            DomainError::external_service(
                format!("Failed to emit audit event: {}", e),
                "audit_emitter",
            )
        })
    }

    pub async fn emit_risk_engine_failure(
        &self,
        loan_request: &LoanRequest,
        error_message: &str,
        retry_count: u32,
    ) -> Result<(), DomainError> {
        let event = AuditEvent::for_risk_engine_failure(loan_request, error_message, retry_count);
        self.emit(event).await
    }

    pub async fn emit_risk_engine_retry(
        &self,
        loan_request: &LoanRequest,
        retry_count: u32,
    ) -> Result<(), DomainError> {
        let event = AuditEvent::for_risk_engine_retry(loan_request, retry_count);
        self.emit(event).await
    }

    pub async fn emit_loan_created(&self, loan_request: &LoanRequest) -> Result<(), DomainError> {
        let payload = AuditPayload {
            requested_amount: loan_request.requested_amount(),
            term_months: loan_request.term_months(),
            risk_score: loan_request.risk_score(),
            error_message: None,
            retry_count: None,
            additional_info: std::collections::HashMap::new(),
        };

        let event = AuditEvent::new(AuditEventType::LoanRequestCreated, loan_request, payload);
        self.emit(event).await
    }

    pub async fn emit_loan_approved(&self, loan_request: &LoanRequest) -> Result<(), DomainError> {
        let payload = AuditPayload {
            requested_amount: loan_request.requested_amount(),
            term_months: loan_request.term_months(),
            risk_score: loan_request.risk_score(),
            error_message: None,
            retry_count: None,
            additional_info: std::collections::HashMap::new(),
        };

        let event = AuditEvent::new(AuditEventType::LoanRequestApproved, loan_request, payload);
        self.emit(event).await
    }

    pub async fn emit_loan_rejected(&self, loan_request: &LoanRequest) -> Result<(), DomainError> {
        let payload = AuditPayload {
            requested_amount: loan_request.requested_amount(),
            term_months: loan_request.term_months(),
            risk_score: loan_request.risk_score(),
            error_message: loan_request.rejection_reason(),
            retry_count: None,
            additional_info: std::collections::HashMap::new(),
        };

        let event = AuditEvent::new(AuditEventType::LoanRequestRejected, loan_request, payload);
        self.emit(event).await
    }
}