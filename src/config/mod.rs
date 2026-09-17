use serde::{Deserialize, Serialize};
use std::env;
use std::sync::OnceLock;

pub static CONFIG: OnceLock<AppConfig> = OnceLock::new();

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub database: DatabaseConfig,
    pub risk_engine: RiskEngineConfig,
    pub audit: AuditConfig,
    pub server: ServerConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub pool_size: u32,
    pub timeout_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskEngineConfig {
    pub base_url: String,
    pub timeout_seconds: u64,
    pub max_retries: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditConfig {
    pub enabled: bool,
    pub queue_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: usize,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        Ok(Self {
            database: DatabaseConfig {
                url: env::var("DATABASE_URL")
                    .unwrap_or_else(|_| "postgresql://user:password@localhost/loan_db".to_string()),
                pool_size: env::var("DATABASE_POOL_SIZE")
                    .unwrap_or_else(|_| "20".to_string())
                    .parse()
                    .unwrap_or(20),
                timeout_seconds: env::var("DATABASE_TIMEOUT_SECONDS")
                    .unwrap_or_else(|_| "30".to_string())
                    .parse()
                    .unwrap_or(30),
            },
            risk_engine: RiskEngineConfig {
                base_url: env::var("RISK_ENGINE_BASE_URL")
                    .unwrap_or_else(|_| "http://risk-engine.internal:8080".to_string()),
                timeout_seconds: env::var("RISK_ENGINE_TIMEOUT_SECONDS")
                    .unwrap_or_else(|_| "10".to_string())
                    .parse()
                    .unwrap_or(10),
                max_retries: env::var("RISK_ENGINE_MAX_RETRIES")
                    .unwrap_or_else(|_| "3".to_string())
                    .parse()
                    .unwrap_or(3),
            },
            audit: AuditConfig {
                enabled: env::var("AUDIT_ENABLED")
                    .unwrap_or_else(|_| "true".to_string())
                    .parse()
                    .unwrap_or(true),
                queue_name: env::var("AUDIT_QUEUE_NAME")
                    .unwrap_or_else(|_| "audit_events".to_string()),
            },
            server: ServerConfig {
                host: env::var("SERVER_HOST")
                    .unwrap_or_else(|_| "0.0.0.0".to_string()),
                port: env::var("SERVER_PORT")
                    .unwrap_or_else(|_| "8080".to_string())
                    .parse()
                    .unwrap_or(8080),
                workers: env::var("SERVER_WORKERS")
                    .unwrap_or_else(|_| "4".to_string())
                    .parse()
                    .unwrap_or(4),
            },
        })
    }

    pub fn from_defaults() -> Self {
        Self {
            database: DatabaseConfig {
                url: "postgresql://user:password@localhost/loan_db".to_string(),
                pool_size: 20,
                timeout_seconds: 30,
            },
            risk_engine: RiskEngineConfig {
                base_url: "http://risk-engine.internal:8080".to_string(),
                timeout_seconds: 10,
                max_retries: 3,
            },
            audit: AuditConfig {
                enabled: true,
                queue_name: "audit_events".to_string(),
            },
            server: ServerConfig {
                host: "0.0.0.0".to_string(),
                port: 8080,
                workers: 4,
            },
        }
    }

    pub fn init() -> &'static AppConfig {
        CONFIG.get_or_init(|| {
            Self::from_env().unwrap_or_else(|_| Self::from_defaults())
        })
    }

    pub fn get() -> Option<&'static AppConfig> {
        CONFIG.get()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    pub initial_delay_ms: u64,
    pub max_delay_ms: u64,
    pub multiplier: f64,
    pub max_retries: u32,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            initial_delay_ms: 100,
            max_delay_ms: 30000,
            multiplier: 2.0,
            max_retries: 5,
        }
    }
}

impl RetryConfig {
    pub fn from_env() -> Self {
        Self {
            initial_delay_ms: env::var("RETRY_INITIAL_DELAY_MS")
                .unwrap_or_else(|_| "100".to_string())
                .parse()
                .unwrap_or(100),
            max_delay_ms: env::var("RETRY_MAX_DELAY_MS")
                .unwrap_or_else(|_| "30000".to_string())
                .parse()
                .unwrap_or(30000),
            multiplier: env::var("RETRY_MULTIPLIER")
                .unwrap_or_else(|_| "2.0".to_string())
                .parse()
                .unwrap_or(2.0),
            max_retries: env::var("RETRY_MAX_RETRIES")
                .unwrap_or_else(|_| "5".to_string())
                .parse()
                .unwrap_or(5),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationConfig {
    pub min_loan_amount: f64,
    pub max_loan_amount: f64,
    pub min_term_months: i32,
    pub max_term_months: i32,
    pub min_annual_rate: f64,
    pub max_annual_rate: f64,
}

impl Default for ValidationConfig {
    fn default() -> Self {
        Self {
            min_loan_amount: 1000.0,
            max_loan_amount: 500000.0,
            min_term_months: 3,
            max_term_months: 360,
            min_annual_rate: 1.0,
            max_annual_rate: 100.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConfigError {
    MissingVariable(String),
    InvalidValue(String),
    ParseError(String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::MissingVariable(name) => write!(f, "Missing required variable: {}", name),
            ConfigError::InvalidValue(msg) => write!(f, "Invalid configuration value: {}", msg),
            ConfigError::ParseError(msg) => write!(f, "Failed to parse configuration: {}", msg),
        }
    }
}

impl std::error::Error for ConfigError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_from_defaults() {
        let config = AppConfig::from_defaults();
        assert_eq!(config.database.pool_size, 20);
        assert_eq!(config.risk_engine.max_retries, 3);
        assert!(config.audit.enabled);
        assert_eq!(config.server.port, 8080);
    }

    #[test]
    fn test_retry_config_default() {
        let retry = RetryConfig::default();
        assert_eq!(retry.initial_delay_ms, 100);
        assert_eq!(retry.max_delay_ms, 30000);
        assert_eq!(retry.multiplier, 2.0);
        assert_eq!(retry.max_retries, 5);
    }

    #[test]
    fn test_validation_config_default() {
        let validation = ValidationConfig::default();
        assert_eq!(validation.min_loan_amount, 1000.0);
        assert_eq!(validation.max_loan_amount, 500000.0);
        assert_eq!(validation.min_term_months, 3);
        assert_eq!(validation.max_term_months, 360);
    }
}