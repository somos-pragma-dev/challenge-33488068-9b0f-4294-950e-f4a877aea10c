use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use backoff::ExponentialBackoff;
use crate::domain::error::DomainError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessmentRequest {
    pub document_number: String,
    pub requested_amount: f64,
    pub term_months: i32,
    pub annual_interest_rate: f64,
    pub monthly_payment: f64,
    pub channel: String,
    pub operation_number: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessmentResponse {
    pub risk_score: i32,
    pub recommendation: RiskRecommendation,
    pub details: RiskDetails,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RiskRecommendation {
    Approve,
    Reject,
    Review,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskDetails {
    pub credit_score_factor: f64,
    pub debt_to_income_ratio: f64,
    pub requested_amount_vs_income: f64,
    pub previous_loan_history: i32,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct RiskEngineClientConfig {
    pub base_url: String,
    pub timeout_seconds: u64,
    pub max_retries: u32,
    pub initial_interval_ms: u64,
    pub max_interval_ms: u64,
    pub multiplier: f64,
}

impl Default for RiskEngineClientConfig {
    fn default() -> Self {
        Self {
            base_url: "http://risk-engine.internal:8080".to_string(),
            timeout_seconds: 10,
            max_retries: 3,
            initial_interval_ms: 500,
            max_interval_ms: 5000,
            multiplier: 2.0,
        }
    }
}

pub struct RiskEngineClient {
    http_client: reqwest::Client,
    config: Arc<RwLock<RiskEngineClientConfig>>,
}

impl RiskEngineClient {
    pub fn new(config: RiskEngineClientConfig) -> Self {
        let http_client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_seconds))
            .build()
            .expect("Failed to build HTTP client for risk engine");

        Self {
            http_client,
            config: Arc::new(RwLock::new(config)),
        }
    }

    pub async fn assess_risk(
        &self,
        request: RiskAssessmentRequest,
    ) -> Result<RiskAssessmentResponse, DomainError> {
        let config = self.config.read().await.clone();
        
        let backoff = ExponentialBackoff {
            initial_interval: std::time::Duration::from_millis(config.initial_interval_ms),
            max_interval: std::time::Duration::from_millis(config.max_interval_ms),
            multiplier: config.multiplier,
            ..Default::default()
        };

        let mut retry_count = 0u32;
        let mut last_error: Option<DomainError> = None;

        let result = backoff::future::retry(backoff, || {
            let http_client = self.http_client.clone();
            let config = self.config.clone();
            let request = request.clone();
            
            async move {
                let config = config.read().await;
                let url = format!("{}/api/v1/risk/assess", config.base_url);
                
                let response = http_client
                    .post(&url)
                    .json(&request)
                    .send()
                    .await
                    .map_err(|e| {
                        backoff::Error::transient(DomainError::external_service(
                            format!("Risk engine request failed: {}", e),
                            "risk_engine_http"
                        ))
                    })?;

                if response.status().is_success() {
                    response
                        .json::<RiskAssessmentResponse>()
                        .await
                        .map_err(|e| {
                            backoff::Error::transient(DomainError::external_service(
                                format!("Failed to parse risk engine response: {}", e),
                                "risk_engine_parse"
                            ))
                        })
                } else if response.status().is_server_error() {
                    let status = response.status();
                    let error_text = response.text().await.unwrap_or_default();
                    retry_count += 1;
                    last_error = Some(DomainError::external_service(
                        format!("Risk engine server error {}: {}", status, error_text),
                        "risk_engine_server"
                    ));
                    
                    backoff::Error::transient(last_error.clone().unwrap())
                } else {
                    let status = response.status();
                    let error_text = response.text().await.unwrap_or_default();
                    
                    backoff::Error::permanent(DomainError::external_service(
                        format!("Risk engine client error {}: {}", status, error_text),
                        "risk_engine_client"
                    ))
                }
            }
        }).await;

        match result {
            Ok(response) => Ok(response),
            Err(e) => {
                if retry_count >= config.max_retries {
                    Err(DomainError::external_service(
                        format!("Risk engine failed after {} retries: {}", retry_count, e),
                        "risk_engine_max_retries"
                    ))
                } else {
                    Err(e)
                }
            }
        }
    }

    pub async fn get_retry_count(&self) -> u32 {
        let config = self.config.read().await;
        config.max_retries
    }

    pub async fn update_config(&self, new_config: RiskEngineClientConfig) {
        let mut config = self.config.write().await;
        *config = new_config;
    }
}

impl RiskEngineClient {
    pub fn with_default_config() -> Self {
        Self::new(RiskEngineClientConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_risk_assessment_request_serialization() {
        let request = RiskAssessmentRequest {
            document_number: "12345678".to_string(),
            requested_amount: 10000.0,
            term_months: 12,
            annual_interest_rate: 0.15,
            monthly_payment: 902.50,
            channel: "web".to_string(),
            operation_number: "OP-2024-001".to_string(),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("12345678"));
        assert!(json.contains("10000"));
    }

    #[test]
    fn test_risk_assessment_response_deserialization() {
        let json = r#"{
            "risk_score": 75,
            "recommendation": "approve",
            "details": {
                "credit_score_factor": 0.8,
                "debt_to_income_ratio": 0.3,
                "requested_amount_vs_income": 0.5,
                "previous_loan_history": 5,
                "warnings": []
            }
        }"#;

        let response: RiskAssessmentResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.risk_score, 75);
        assert_eq!(response.recommendation, RiskRecommendation::Approve);
    }

    #[tokio::test]
    async fn test_client_config_defaults() {
        let config = RiskEngineClientConfig::default();
        assert_eq!(config.base_url, "http://risk-engine.internal:8080");
        assert_eq!(config.timeout_seconds, 10);
        assert_eq!(config.max_retries, 3);
    }
}