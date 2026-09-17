use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LoanRequestStatus {
    Pending,
    Approved,
    Rejected,
    UnderReview,
    Cancelled,
}

impl LoanRequestStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            LoanRequestStatus::Pending => "pending",
            LoanRequestStatus::Approved => "approved",
            LoanRequestStatus::Rejected => "rejected",
            LoanRequestStatus::UnderReview => "under_review",
            LoanRequestStatus::Cancelled => "cancelled",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "pending" => Some(LoanRequestStatus::Pending),
            "approved" => Some(LoanRequestStatus::Approved),
            "rejected" => Some(LoanRequestStatus::Rejected),
            "under_review" => Some(LoanRequestStatus::UnderReview),
            "cancelled" => Some(LoanRequestStatus::Cancelled),
            _ => None,
        }
    }

    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            LoanRequestStatus::Approved | LoanRequestStatus::Rejected | LoanRequestStatus::Cancelled
        )
    }
}

impl Default for LoanRequestStatus {
    fn default() -> Self {
        LoanRequestStatus::Pending
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoanRequest {
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
    pub status: LoanRequestStatus,
    pub risk_score: Option<i32>,
    pub risk_decision: Option<String>,
    pub rejection_reason: Option<String>,
    pub idempotency_key: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl LoanRequest {
    pub fn new(
        operation_number: String,
        channel: String,
        applicant_name: String,
        applicant_document: String,
        applicant_email: String,
        applicant_phone: String,
        amount: f64,
        term_months: i32,
        interest_rate: f64,
        purpose: String,
        idempotency_key: String,
    ) -> Self {
        let monthly_payment = Self::calculate_monthly_payment(amount, term_months, interest_rate);
        let now = Utc::now();

        Self {
            id: 0,
            operation_number,
            channel,
            applicant_name,
            applicant_document,
            applicant_email,
            applicant_phone,
            amount,
            term_months,
            interest_rate,
            monthly_payment,
            purpose,
            status: LoanRequestStatus::default(),
            risk_score: None,
            risk_decision: None,
            rejection_reason: None,
            idempotency_key,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn calculate_monthly_payment(amount: f64, term_months: i32, annual_rate: f64) -> f64 {
        if term_months <= 0 || amount <= 0.0 || annual_rate <= 0.0 {
            return amount / term_months.max(1) as f64;
        }

        let monthly_rate = annual_rate / 12.0 / 100.0;
        let n = term_months as f64;

        if monthly_rate.abs() < f64::EPSILON {
            return amount / n;
        }

        let payment = amount * (monthly_rate * (1.0 + monthly_rate).powf(n))
            / ((1.0 + monthly_rate).powf(n) - 1.0);

        (payment * 100.0).round() / 100.0
    }

    pub fn approve(&mut self, risk_score: i32, decision: String) {
        self.status = LoanRequestStatus::Approved;
        self.risk_score = Some(risk_score);
        self.risk_decision = Some(decision);
        self.updated_at = Utc::now();
    }

    pub fn reject(&mut self, risk_score: i32, decision: String, reason: String) {
        self.status = LoanRequestStatus::Rejected;
        self.risk_score = Some(risk_score);
        self.risk_decision = Some(decision);
        self.rejection_reason = Some(reason);
        self.updated_at = Utc::now();
    }

    pub fn mark_under_review(&mut self) {
        self.status = LoanRequestStatus::UnderReview;
        self.updated_at = Utc::now();
    }

    pub fn cancel(&mut self) {
        if !self.status.is_terminal() {
            self.status = LoanRequestStatus::Cancelled;
            self.updated_at = Utc::now();
        }
    }

    pub fn is_pending(&self) -> bool {
        self.status == LoanRequestStatus::Pending
    }

    pub fn can_be_modified(&self) -> bool {
        matches!(
            self.status,
            LoanRequestStatus::Pending | LoanRequestStatus::UnderReview
        )
    }

    pub fn generate_idempotency_key(operation_number: &str, channel: &str) -> String {
        format!("{}:{}:{}", operation_number, channel, Utc::now().timestamp())
    }

    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.operation_number.is_empty() {
            errors.push("El número de operación es requerido".to_string());
        }

        if self.channel.is_empty() {
            errors.push("El canal de origen es requerido".to_string());
        }

        if self.applicant_name.is_empty() {
            errors.push("El nombre del solicitante es requerido".to_string());
        }

        if self.applicant_document.is_empty() {
            errors.push("El documento de identidad es requerido".to_string());
        }

        if self.applicant_email.is_empty() {
            errors.push("El correo electrónico es requerido".to_string());
        } else if !self.applicant_email.contains('@') {
            errors.push("El correo electrónico tiene formato inválido".to_string());
        }

        if self.amount <= 0.0 {
            errors.push("El monto del préstamo debe ser mayor a cero".to_string());
        } else if self.amount > 1_000_000.0 {
            errors.push("El monto del préstamo excede el límite permitido".to_string());
        }

        if self.term_months <= 0 {
            errors.push("El plazo en meses debe ser mayor a cero".to_string());
        } else if self.term_months > 360 {
            errors.push("El plazo excede el máximo permitido de 360 meses".to_string());
        }

        if self.interest_rate < 0.0 {
            errors.push("La tasa de interés no puede ser negativa".to_string());
        }

        if self.purpose.is_empty() {
            errors.push("El propósito del préstamo es requerido".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_monthly_payment_standard() {
        let payment = LoanRequest::calculate_monthly_payment(10_000.0, 12, 24.0);
        assert!((payment - 941.48).abs() < 1.0);
    }

    #[test]
    fn test_loan_request_creation() {
        let loan = LoanRequest::new(
            "OP001".to_string(),
            "web".to_string(),
            "Juan Pérez".to_string(),
            "12345678".to_string(),
            "juan@email.com".to_string(),
            "+5491112345678".to_string(),
            10_000.0,
            12,
            24.0,
            "consumo".to_string(),
            "idem_key_001".to_string(),
        );

        assert_eq!(loan.operation_number, "OP001");
        assert_eq!(loan.channel, "web");
        assert!(loan.is_pending());
    }

    #[test]
    fn test_loan_request_validation_errors() {
        let mut loan = LoanRequest::new(
            "OP001".to_string(),
            "web".to_string(),
            "".to_string(),
            "".to_string(),
            "invalid-email".to_string(),
            "".to_string(),
            -100.0,
            -5,
            -10.0,
            "".to_string(),
            "key".to_string(),
        );

        let result = loan.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.len() > 5);
    }

    #[test]
    fn test_approve_loan_request() {
        let mut loan = LoanRequest::new(
            "OP001".to_string(),
            "web".to_string(),
            "Juan Pérez".to_string(),
            "12345678".to_string(),
            "juan@email.com".to_string(),
            "+5491112345678".to_string(),
            10_000.0,
            12,
            24.0,
            "consumo".to_string(),
            "idem_key_001".to_string(),
        );

        loan.approve(75, "approved".to_string());

        assert!(matches!(loan.status, LoanRequestStatus::Approved));
        assert_eq!(loan.risk_score, Some(75));
    }

    #[test]
    fn test_reject_loan_request() {
        let mut loan = LoanRequest::new(
            "OP001".to_string(),
            "web".to_string(),
            "Juan Pérez".to_string(),
            "12345678".to_string(),
            "juan@email.com".to_string(),
            "+5491112345678".to_string(),
            10_000.0,
            12,
            24.0,
            "consumo".to_string(),
            "idem_key_001".to_string(),
        );

        loan.reject(25, "rejected".to_string(), "Riesgo demasiado alto".to_string());

        assert!(matches!(loan.status, LoanRequestStatus::Rejected));
        assert_eq!(loan.rejection_reason, Some("Riesgo demasiado alto".to_string()));
    }
}