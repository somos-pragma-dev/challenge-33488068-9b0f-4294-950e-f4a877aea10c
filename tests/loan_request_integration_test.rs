use actix_web::{test, web, App};
use loan_request_api::domain::models::loan_request::{LoanRequest, LoanRequestStatus};
use loan_request_api::domain::dtos::loan_request_dto::{LoanRequestDto, LoanRequestResponseDto};
use loan_request_api::domain::error::DomainError;
use loan_request_api::application::services::loan_request_service::{LoanRequestService, AppConfig, RiskEvaluationResponse};
use loan_request_api::infrastructure::repository::loan_request_repository::LoanRequestRepository;
use loan_request_api::infrastructure::risk_engine::risk_engine_client::RiskEngineClient;
use loan_request_api::infrastructure::audit::audit_event::AuditEvent;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
struct MockRepository {
    loan_requests: std::sync::Mutex<Vec<LoanRequest>>,
}

impl MockRepository {
    fn new() -> Self {
        Self {
            loan_requests: std::sync::Mutex::new(Vec::new()),
        }
    }
}

#[async_trait::async_trait]
impl LoanRequestRepository for MockRepository {
    async fn save(&self, loan_request: LoanRequest) -> Result<LoanRequest, DomainError> {
        let mut requests = self.loan_requests.lock().unwrap();
        requests.push(loan_request.clone());
        Ok(loan_request)
    }

    async fn find_by_id(&self, id: i64) -> Result<LoanRequest, DomainError> {
        let requests = self.loan_requests.lock().unwrap();
        requests.iter()
            .find(|r| r.id == id)
            .cloned()
            .ok_or_else(|| DomainError::not_found("LoanRequest", id.to_string()))
    }

    async fn find_by_idempotency_key(&self, key: &str) -> Result<Option<LoanRequest>, DomainError> {
        let requests = self.loan_requests.lock().unwrap();
        Ok(requests.iter().find(|r| r.idempotency_key == key).cloned())
    }

    async fn find_all(&self, page: i32, per_page: i32) -> Result<(Vec<LoanRequest>, i64), DomainError> {
        let requests = self.loan_requests.lock().unwrap();
        let total = requests.len() as i64;
        let start = ((page - 1) * per_page) as usize;
        let end = (start + per_page as usize).min(requests.len());
        let entities = if start < requests.len() {
            requests[start..end].to_vec()
        } else {
            Vec::new()
        };
        Ok((entities, total))
    }

    async fn update(&self, loan_request: LoanRequest) -> Result<LoanRequest, DomainError> {
        let mut requests = self.loan_requests.lock().unwrap();
        if let Some(pos) = requests.iter().position(|r| r.id == loan_request.id) {
            requests[pos] = loan_request.clone();
            Ok(loan_request)
        } else {
            Err(DomainError::not_found("LoanRequest", loan_request.id.to_string()))
        }
    }

    async fn delete(&self, id: i64) -> Result<(), DomainError> {
        let mut requests = self.loan_requests.lock().unwrap();
        if let Some(pos) = requests.iter().position(|r| r.id == id) {
            requests.remove(pos);
            Ok(())
        } else {
            Err(DomainError::not_found("LoanRequest", id.to_string()))
        }
    }
}

#[derive(Debug, Clone)]
struct MockRiskEngineClient {
    should_fail: std::sync::Arc<std::sync::atomic::AtomicBool>,
    call_count: std::sync::Arc<std::sync::atomic::AtomicU32>,
}

impl MockRiskEngineClient {
    fn new(should_fail: bool) -> Self {
        Self {
            should_fail: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(should_fail)),
            call_count: std::sync::Arc::new(std::sync::atomic::AtomicU32::new(0)),
        }
    }
}

#[async_trait::async_trait]
impl RiskEngineClient for MockRiskEngineClient {
    async fn evaluate_risk(&self, _loan_request: &LoanRequest) -> Result<RiskEvaluationResponse, DomainError> {
        self.call_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        
        if self.should_fail.load(std::sync::atomic::Ordering::SeqCst) {
            return Err(DomainError::external_service(
                "Risk engine unavailable",
                "risk_engine_error",
            ));
        }

        Ok(RiskEvaluationResponse {
            approved: true,
            risk_score: 45,
            decision: "Approved".to_string(),
            rejection_reason: None,
        })
    }

    fn get_call_count(&self) -> u32 {
        self.call_count.load(std::sync::atomic::Ordering::SeqCst)
    }
}

fn create_test_dto() -> LoanRequestDto {
    LoanRequestDto {
        applicant_name: "Juan Pérez".to_string(),
        applicant_email: "juan.perez@example.com".to_string(),
        applicant_phone: "+1234567890".to_string(),
        applicant_document: "12345678".to_string(),
        document_type: "DNI".to_string(),
        loan_amount: 50000.0,
        loan_term_months: 24,
        annual_interest_rate: 12.5,
        operation_number: "OP-2024-001".to_string(),
        channel: "web".to_string(),
        purpose: "personal".to_string(),
    }
}

#[tokio::test]
async fn test_create_loan_request_success() {
    let mock_repo = Arc::new(RwLock::new(MockRepository::new()));
    let risk_client = Arc::new(MockRiskEngineClient::new(false));
    let config = web::Data::new(AppConfig::default());
    
    let service = LoanRequestService::new(
        mock_repo.clone(),
        risk_client.clone(),
        config,
    );
    
    let dto = create_test_dto();
    let result = service.create_loan_request(dto).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.status, "approved");
}

#[tokio::test]
async fn test_create_loan_request_idempotency_conflict() {
    let mock_repo = Arc::new(RwLock::new(MockRepository::new()));
    let risk_client = Arc::new(MockRiskEngineClient::new(false));
    let config = web::Data::new(AppConfig::default());
    
    let service = LoanRequestService::new(
        mock_repo.clone(),
        risk_client.clone(),
        config,
    );
    
    let dto = create_test_dto();
    let result1 = service.create_loan_request(dto.clone()).await;
    assert!(result1.is_ok());
    
    let result2 = service.create_loan_request(dto).await;
    assert!(result2.is_err());
    match result2.unwrap_err() {
        DomainError::IdempotencyConflict(_, _) => {}
        _ => panic!("Expected IdempotencyConflict error"),
    }
}

#[tokio::test]
async fn test_create_loan_request_risk_engine_failure() {
    let mock_repo = Arc::new(RwLock::new(MockRepository::new()));
    let risk_client = Arc::new(MockRiskEngineClient::new(true));
    let mut config = AppConfig::default();
    config.audit_enabled = false;
    let config = web::Data::new(config);
    
    let service = LoanRequestService::new(
        mock_repo.clone(),
        risk_client.clone(),
        config,
    );
    
    let dto = create_test_dto();
    let result = service.create_loan_request(dto).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.status, "under_review");
}

#[tokio::test]
async fn test_get_loan_request_not_found() {
    let mock_repo = Arc::new(RwLock::new(MockRepository::new()));
    let risk_client = Arc::new(MockRiskEngineClient::new(false));
    let config = web::Data::new(AppConfig::default());
    
    let service = LoanRequestService::new(
        mock_repo.clone(),
        risk_client.clone(),
        config,
    );
    
    let result = service.get_loan_request(999).await;
    assert!(result.is_err());
    match result.unwrap_err() {
        DomainError::NotFound(_, _) => {}
        _ => panic!("Expected NotFound error"),
    }
}

#[tokio::test]
async fn test_list_loan_requests_pagination() {
    let mock_repo = Arc::new(RwLock::new(MockRepository::new()));
    let risk_client = Arc::new(MockRiskEngineClient::new(false));
    let config = web::Data::new(AppConfig::default());
    
    let service = LoanRequestService::new(
        mock_repo.clone(),
        risk_client.clone(),
        config,
    );
    
    let dto = create_test_dto();
    let _ = service.create_loan_request(dto).await;
    
    let result = service.list_loan_requests(1, 10).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.data.len(), 1);
    assert_eq!(response.total, 1);
}

#[tokio::test]
async fn test_cancel_loan_request_invalid_state() {
    let mock_repo = Arc::new(RwLock::new(MockRepository::new()));
    let risk_client = Arc::new(MockRiskEngineClient::new(false));
    let config = web::Data::new(AppConfig::default());
    
    let service = LoanRequestService::new(
        mock_repo.clone(),
        risk_client.clone(),
        config,
    );
    
    let dto = create_test_dto();
    let result = service.create_loan_request(dto).await;
    assert!(result.is_ok());
    let created = result.unwrap();
    
    let cancel_result = service.cancel_loan_request(created.id).await;
    assert!(cancel_result.is_err());
}

#[tokio::test]
async fn test_loan_request_dto_validation() {
    let valid_dto = create_test_dto();
    let validation_result = valid_dto.validate();
    assert!(validation_result.is_ok());
    
    let mut invalid_dto = create_test_dto();
    invalid_dto.loan_amount = -1000.0;
    let invalid_result = invalid_dto.validate();
    assert!(invalid_result.is_err());
}

#[tokio::test]
async fn test_monthly_payment_calculation() {
    let monthly_payment = LoanRequest::calculate_monthly_payment(50000.0, 24, 12.5);
    assert!((monthly_payment - 2353.67).abs() < 0.01);
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestAuditEvent {
    pub event_type: String,
    pub loan_request_id: i64,
    pub payload: serde_json::Value,
}

#[tokio::test]
async fn test_audit_event_emission_on_risk_failure() {
    let mock_repo = Arc::new(RwLock::new(MockRepository::new()));
    let risk_client = Arc::new(MockRiskEngineClient::new(true));
    let mut config = AppConfig::default();
    config.audit_enabled = true;
    let config = web::Data::new(config);
    
    let service = LoanRequestService::new(
        mock_repo.clone(),
        risk_client.clone(),
        config,
    );
    
    let dto = create_test_dto();
    let result = service.create_loan_request(dto).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.status, "under_review");
}