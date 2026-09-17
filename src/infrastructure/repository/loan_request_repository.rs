use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::domain::models::loan_request::{LoanRequest, LoanRequestStatus};
use crate::domain::error::DomainError;

pub struct LoanRequestRepository {
    pool: Arc<RwLock<diesel_async::AsyncPgConnection>>,
}

impl LoanRequestRepository {
    pub fn new(pool: Arc<RwLock<diesel_async::AsyncPgConnection>>) -> Self {
        Self { pool }
    }

    pub async fn create(&self, loan_request: &LoanRequest) -> Result<LoanRequest, DomainError> {
        use crate::schema::loan_requests::dsl;
        
        let pool = self.pool.read().await;
        
        let result = diesel::insert_into(dsl::loan_requests)
            .values((
                dsl::idempotency_key.eq(loan_request.idempotency_key()),
                dsl::operation_number.eq(loan_request.operation_number()),
                dsl::channel.eq(loan_request.channel()),
                dsl::document_number.eq(loan_request.document_number()),
                dsl::full_name.eq(loan_request.full_name()),
                dsl::email.eq(loan_request.email()),
                dsl::phone.eq(loan_request.phone()),
                dsl::requested_amount.eq(loan_request.requested_amount()),
                dsl::term_months.eq(loan_request.term_months()),
                dsl::annual_interest_rate.eq(loan_request.annual_interest_rate()),
                dsl::monthly_payment.eq(loan_request.monthly_payment()),
                dsl::status.eq(loan_request.status().as_str()),
                dsl::risk_score.eq(loan_request.risk_score()),
                dsl::decision.eq(loan_request.decision()),
                dsl::rejection_reason.eq(loan_request.rejection_reason()),
                dsl::created_at.eq(loan_request.created_at()),
                dsl::updated_at.eq(loan_request.updated_at()),
            ))
            .get_result::<LoanRequest>(&mut pool.get().await
                .map_err(|e| DomainError::persistence(
                    format!("Failed to get connection from pool: {}", e),
                    "create"
                ))?)
            .await
            .map_err(|e| DomainError::persistence(
                format!("Failed to create loan request: {}", e),
                "create"
            ))?;

        Ok(result)
    }

    pub async fn find_by_idempotency_key(
        &self,
        idempotency_key: &str,
    ) -> Result<Option<LoanRequest>, DomainError> {
        use crate::schema::loan_requests::dsl;
        
        let pool = self.pool.read().await;
        
        let result = dsl::loan_requests
            .filter(dsl::idempotency_key.eq(idempotency_key))
            .first::<LoanRequest>(&mut pool.get().await
                .map_err(|e| DomainError::persistence(
                    format!("Failed to get connection from pool: {}", e),
                    "find_by_idempotency_key"
                ))?)
            .await;

        match result {
            Ok(loan_request) => Ok(Some(loan_request)),
            Err(diesel::NotFound) => Ok(None),
            Err(e) => Err(DomainError::persistence(
                format!("Failed to find loan request: {}", e),
                "find_by_idempotency_key"
            )),
        }
    }

    pub async fn find_by_id(&self, id: i64) -> Result<Option<LoanRequest>, DomainError> {
        use crate::schema::loan_requests::dsl;
        
        let pool = self.pool.read().await;
        
        let result = dsl::loan_requests
            .filter(dsl::id.eq(id))
            .first::<LoanRequest>(&mut pool.get().await
                .map_err(|e| DomainError::persistence(
                    format!("Failed to get connection from pool: {}", e),
                    "find_by_id"
                ))?)
            .await;

        match result {
            Ok(loan_request) => Ok(Some(loan_request)),
            Err(diesel::NotFound) => Ok(None),
            Err(e) => Err(DomainError::persistence(
                format!("Failed to find loan request: {}", e),
                "find_by_id"
            )),
        }
    }

    pub async fn update(&self, loan_request: &LoanRequest) -> Result<LoanRequest, DomainError> {
        use crate::schema::loan_requests::dsl;
        
        let pool = self.pool.read().await;
        
        let result = diesel::update(dsl::loan_requests)
            .filter(dsl::id.eq(loan_request.id()))
            .set((
                dsl::status.eq(loan_request.status().as_str()),
                dsl::risk_score.eq(loan_request.risk_score()),
                dsl::decision.eq(loan_request.decision()),
                dsl::rejection_reason.eq(loan_request.rejection_reason()),
                dsl::updated_at.eq(loan_request.updated_at()),
            ))
            .get_result::<LoanRequest>(&mut pool.get().await
                .map_err(|e| DomainError::persistence(
                    format!("Failed to get connection from pool: {}", e),
                    "update"
                ))?)
            .await
            .map_err(|e| DomainError::persistence(
                format!("Failed to update loan request: {}", e),
                "update"
            ))?;

        Ok(result)
    }

    pub async fn find_all_paginated(
        &self,
        page: i32,
        per_page: i32,
    ) -> Result<(Vec<LoanRequest>, i64), DomainError> {
        use crate::schema::loan_requests::dsl;
        
        let pool = self.pool.read().await;
        let offset = (page - 1) * per_page;
        
        let total: i64 = dsl::loan_requests
            .count()
            .get_result(&mut pool.get().await
                .map_err(|e| DomainError::persistence(
                    format!("Failed to get connection from pool: {}", e),
                    "find_all_paginated count"
                ))?)
            .await
            .map_err(|e| DomainError::persistence(
                format!("Failed to count loan requests: {}", e),
                "find_all_paginated"
            ))?;

        let results = dsl::loan_requests
            .order(dsl::created_at.desc())
            .limit(per_page as i64)
            .offset(offset as i64)
            .load::<LoanRequest>(&mut pool.get().await
                .map_err(|e| DomainError::persistence(
                    format!("Failed to get connection from pool: {}", e),
                    "find_all_paginated"
                ))?)
            .await
            .map_err(|e| DomainError::persistence(
                format!("Failed to fetch loan requests: {}", e),
                "find_all_paginated"
            ))?;

        Ok((results, total))
    }

    pub async fn exists_by_idempotency_key(&self, idempotency_key: &str) -> Result<bool, DomainError> {
        use crate::schema::loan_requests::dsl;
        
        let pool = self.pool.read().await;
        
        let count: i64 = dsl::loan_requests
            .filter(dsl::idempotency_key.eq(idempotency_key))
            .count()
            .get_result(&mut pool.get().await
                .map_err(|e| DomainError::persistence(
                    format!("Failed to get connection from pool: {}", e),
                    "exists_by_idempotency_key"
                ))?)
            .await
            .map_err(|e| DomainError::persistence(
                format!("Failed to check idempotency key: {}", e),
                "exists_by_idempotency_key"
            ))?;

        Ok(count > 0)
    }
}