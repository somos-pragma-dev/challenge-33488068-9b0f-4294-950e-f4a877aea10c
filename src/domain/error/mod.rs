use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum DomainError {
    #[error("Error de validación: {message}")]
    ValidationError {
        message: String,
        field: Option<String>,
    },

    #[error("Recurso no encontrado: {resource} con identificador {id}")]
    NotFound {
        resource: String,
        id: String,
    },

    #[error("Conflicto de idempotencia: {message}")]
    IdempotencyConflict {
        message: String,
        existing_id: i64,
    },

    #[error("Error de persistencia: {message}")]
    PersistenceError {
        message: String,
        operation: String,
    },

    #[error("Error de integración externa: {service} - {message}")]
    ExternalServiceError {
        service: String,
        message: String,
        retryable: bool,
    },

    #[error("Error de configuración: {message}")]
    ConfigurationError {
        message: String,
    },

    #[error("Error de negocio: {message}")]
    BusinessError {
        message: String,
        code: String,
    },

    #[error("Error interno del servidor: {message}")]
    InternalError {
        message: String,
        code: String,
    },
}

impl DomainError {
    pub fn validation(message: impl Into<String>, field: Option<String>) -> Self {
        DomainError::ValidationError {
            message: message.into(),
            field,
        }
    }

    pub fn not_found(resource: impl Into<String>, id: impl Into<String>) -> Self {
        DomainError::NotFound {
            resource: resource.into(),
            id: id.into(),
        }
    }

    pub fn idempotency_conflict(message: impl Into<String>, existing_id: i64) -> Self {
        DomainError::IdempotencyConflict {
            message: message.into(),
            existing_id,
        }
    }

    pub fn persistence(message: impl Into<String>, operation: impl Into<String>) -> Self {
        DomainError::PersistenceError {
            message: message.into(),
            operation: operation.into(),
        }
    }

    pub fn external_service(
        service: impl Into<String>,
        message: impl Into<String>,
        retryable: bool,
    ) -> Self {
        DomainError::ExternalServiceError {
            service: service.into(),
            message: message.into(),
            retryable,
        }
    }

    pub fn configuration(message: impl Into<String>) -> Self {
        DomainError::ConfigurationError {
            message: message.into(),
        }
    }

    pub fn business(message: impl Into<String>, code: impl Into<String>) -> Self {
        DomainError::BusinessError {
            message: message.into(),
            code: code.into(),
        }
    }

    pub fn internal(message: impl Into<String>, code: impl Into<String>) -> Self {
        DomainError::InternalError {
            message: message.into(),
            code: code.into(),
        }
    }

    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            DomainError::ExternalServiceError { retryable: true, .. }
        )
    }

    pub fn error_code(&self) -> String {
        match self {
            DomainError::ValidationError { .. } => "VALIDATION_ERROR".to_string(),
            DomainError::NotFound { .. } => "NOT_FOUND".to_string(),
            DomainError::IdempotencyConflict { .. } => "IDEMPOTENCY_CONFLICT".to_string(),
            DomainError::PersistenceError { .. } => "PERSISTENCE_ERROR".to_string(),
            DomainError::ExternalServiceError { .. } => "EXTERNAL_SERVICE_ERROR".to_string(),
            DomainError::ConfigurationError { .. } => "CONFIGURATION_ERROR".to_string(),
            DomainError::BusinessError { code, .. } => code.clone(),
            DomainError::InternalError { code, .. } => code.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    pub code: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub details: Option<Vec<String>>,
}

impl ErrorResponse {
    pub fn from_domain_error(error: &DomainError) -> Self {
        let code = error.error_code();
        let message = error.to_string();

        let details = match error {
            DomainError::ValidationError { message, .. } => Some(vec![message.clone()]),
            DomainError::BusinessError { message, .. } => Some(vec![message.clone()]),
            _ => None,
        };

        Self {
            error: error.to_string(),
            message,
            code,
            timestamp: chrono::Utc::now(),
            details,
        }
    }

    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            error: message.to_string(),
            message: message.into(),
            code: code.into(),
            timestamp: chrono::Utc::now(),
            details: None,
        }
    }
}

pub type DomainResult<T> = Result<T, DomainError>;