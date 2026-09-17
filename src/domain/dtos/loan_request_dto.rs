use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::domain::models::loan_request::{LoanRequest, LoanRequestStatus};

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateLoanRequestDto {
    #[validate(length(min = 1, max = 50, message = "El número de operación debe tener entre 1 y 50 caracteres"))]
    pub operation_number: String,

    #[validate(length(min = 1, max = 30, message = "El canal debe tener entre 1 y 30 caracteres"))]
    pub channel: String,

    #[validate(length(min = 1, max = 100, message = "El nombre debe tener entre 1 y 100 caracteres"))]
    pub applicant_name: String,

    #[validate(length(min = 1, max = 20, message = "El documento debe tener entre 1 y 20 caracteres"))]
    pub applicant_document: String,

    #[validate(email(message = "El correo electrónico debe tener formato válido"))]
    pub applicant_email: String,

    #[validate(length(min = 1, max = 20, message = "El teléfono debe tener entre 1 y 20 caracteres"))]
    pub applicant_phone: String,

    #[validate(range(min = 100.0, max = 1_000_000.0, message = "El monto debe estar entre 100 y 1,000,000"))]
    pub amount: f64,

    #[validate(range(min = 1, max = 360, message = "El plazo debe estar entre 1 y 360 meses"))]
    pub term_months: i32,

    #[validate(range(min = 0.0, max = 100.0, message = "La tasa de interés debe estar entre 0 y 100%"))]
    pub interest_rate: f64,

    #[validate(length(min = 1, max = 200, message = "El propósito debe tener entre 1 y 200 caracteres"))]
    pub purpose: String,
}

impl CreateLoanRequestDto {
    pub fn to_entity(&self, idempotency_key: String) -> LoanRequest {
        LoanRequest::new(
            self.operation_number.clone(),
            self.channel.clone(),
            self.applicant_name.clone(),
            self.applicant_document.clone(),
            self.applicant_email.clone(),
            self.applicant_phone.clone(),
            self.amount,
            self.term_months,
            self.interest_rate,
            self.purpose.clone(),
            idempotency_key,
        )
    }

    pub fn generate_idempotency_key(&self) -> String {
        format!("{}:{}", self.operation_number, self.channel)
    }

    pub fn validate(&self) -> Result<(), String> {
        self.validate()
            .map_err(|e| e.to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseLoanRequestDto {
    pub id: i64,
    pub operation_number: String,
    pub channel: String,
    pub applicant_name: String,
    pub applicant_document: String,
    pub applicant_email: String,
    pub applicant_phone: String,
    pub amount: f64,
    pub term_months: i32,
    pub interest_rate: f64,
    pub monthly_payment: f64,
    pub purpose: String,
    pub status: String,
    pub risk_score: Option<i32>,
    pub risk_decision: Option<String>,
    pub rejection_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<LoanRequest> for ResponseLoanRequestDto {
    fn from(entity: LoanRequest) -> Self {
        Self {
            id: entity.id,
            operation_number: entity.operation_number,
            channel: entity.channel,
            applicant_name: entity.applicant_name,
            applicant_document: mask_document(&entity.applicant_document),
            applicant_email: mask_email(&entity.applicant_email),
            applicant_phone: mask_phone(&entity.applicant_phone),
            amount: entity.amount,
            term_months: entity.term_months,
            interest_rate: entity.interest_rate,
            monthly_payment: entity.monthly_payment,
            purpose: entity.purpose,
            status: entity.status.as_str().to_string(),
            risk_score: entity.risk_score,
            risk_decision: entity.risk_decision,
            rejection_reason: entity.rejection_reason,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }
}

fn mask_document(document: &str) -> String {
    if document.len() <= 4 {
        return "****".to_string();
    }
    let visible = &document[document.len() - 4..];
    format!("****{}", visible)
}

fn mask_email(email: &str) -> String {
    if let Some(at_pos) = email.find('@') {
        let local_part = &email[..at_pos];
        if local_part.len() <= 2 {
            return format!("**{}@{}", &local_part[0..1], &email[at_pos + 1..]);
        }
        let masked = format!("{}**", &local_part[0..2]);
        return format!("{}@{}", masked, &email[at_pos + 1..]);
    }
    "****".to_string()
}

fn mask_phone(phone: &str) -> String {
    if phone.len() < 4 {
        return "****".to_string();
    }
    let visible = &phone[phone.len() - 4..];
    format!("****{}", visible)
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateLoanRequestStatusDto {
    #[validate(range(min = 1, message = "El ID de la solicitud es requerido"))]
    pub id: i64,

    #[validate(length(min = 1, message = "El estado es requerido"))]
    pub status: String,

    #[validate(range(min = 0, max = 100, message = "El puntaje de riesgo debe estar entre 0 y 100"))]
    pub risk_score: Option<i32>,

    #[validate(length(max = 500, message = "La decisión de riesgo no puede exceder 500 caracteres"))]
    pub risk_decision: Option<String>,

    #[validate(length(max = 1000, message = "La razón de rechazo no puede exceder 1000 caracteres"))]
    pub rejection_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoanRequestListResponseDto {
    pub data: Vec<ResponseLoanRequestDto>,
    pub total: i64,
    pub page: i32,
    pub per_page: i32,
    pub total_pages: i32,
}

impl LoanRequestListResponseDto {
    pub fn from_entities(entities: Vec<LoanRequest>, total: i64, page: i32, per_page: i32) -> Self {
        let total_pages = (total as f64 / per_page as f64).ceil() as i32;
        Self {
            data: entities.into_iter().map(ResponseLoanRequestDto::from).collect(),
            total,
            page,
            per_page,
            total_pages,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoanRequestErrorDto {
    pub code: String,
    pub message: String,
    pub details: Option<Vec<String>>,
    pub timestamp: DateTime<Utc>,
}

impl LoanRequestErrorDto {
    pub fn new(code: &str, message: &str) -> Self {
        Self {
            code: code.to_string(),
            message: message.to_string(),
            details: None,
            timestamp: Utc::now(),
        }
    }

    pub fn with_details(code: &str, message: &str, details: Vec<String>) -> Self {
        Self {
            code: code.to_string(),
            message: message.to_string(),
            details: Some(details),
            timestamp: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoanRequestSearchCriteriaDto {
    pub operation_number: Option<String>,
    pub channel: Option<String>,
    pub status: Option<String>,
    pub min_amount: Option<f64>,
    pub max_amount: Option<f64>,
    pub applicant_document: Option<String>,
    pub from_date: Option<DateTime<Utc>>,
    pub to_date: Option<DateTime<Utc>>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}

impl Default for LoanRequestSearchCriteriaDto {
    fn default() -> Self {
        Self {
            operation_number: None,
            channel: None,
            status: None,
            min_amount: None,
            max_amount: None,
            applicant_document: None,
            from_date: None,
            to_date: None,
            page: Some(1),
            per_page: Some(20),
        }
    }
}

impl LoanRequestSearchCriteriaDto {
    pub fn page(&self) -> i32 {
        self.page.unwrap_or(1).max(1)
    }

    pub fn per_page(&self) -> i32 {
        let per_page = self.per_page.unwrap_or(20).max(1);
        per_page.min(100)
    }
}