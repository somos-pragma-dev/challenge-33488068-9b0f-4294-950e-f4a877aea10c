use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::domain::dtos::loan_request_dto::{LoanRequestDTO, LoanRequestResponseDTO};
use crate::domain::dtos::pagination_dto::PaginatedResponse;
use crate::domain::error::DomainError;
use crate::application::services::loan_request_service::LoanRequestService;
use crate::infrastructure::repository::loan_request_repository::LoanRequestRepository;
use crate::infrastructure::risk_engine::risk_engine_client::RiskEngineClient;
use crate::infrastructure::audit::audit_event::AuditEventProducer;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateLoanRequestPayload {
    pub operation_number: String,
    pub channel: String,
    pub document: String,
    pub phone: String,
    pub email: String,
    pub amount: f64,
    pub term_months: i32,
    pub annual_rate: f64,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateLoanRequestPayload {
    pub amount: Option<f64>,
    pub term_months: Option<i32>,
    pub annual_rate: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoanRequestPathParams {
    pub id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryParams {
    pub page: Option<i32>,
    pub per_page: Option<i32>,
    pub status: Option<String>,
}

pub struct LoanRequestController {
    service: LoanRequestService,
}

impl LoanRequestController {
    pub fn new(
        repository: LoanRequestRepository,
        risk_client: RiskEngineClient,
        audit_producer: AuditEventProducer,
    ) -> Self {
        let service = LoanRequestService::new(repository, risk_client, audit_producer);
        Self { service }
    }

    pub async fn create(&self, payload: CreateLoanRequestPayload) -> Result<HttpResponse, DomainError> {
        let dto = LoanRequestDTO {
            operation_number: payload.operation_number,
            channel: payload.channel,
            document: payload.document,
            phone: payload.phone,
            email: payload.email,
            amount: payload.amount,
            term_months: payload.term_months,
            annual_rate: payload.annual_rate,
            first_name: payload.first_name,
            last_name: payload.last_name,
        };

        dto.validate()?;
        
        let idempotency_key = dto.generate_idempotency_key();
        let entity = dto.to_entity(idempotency_key);
        entity.validate()?;

        let result = self.service.create_loan_request(entity).await?;
        let response = LoanRequestResponseDTO::from(result);
        
        Ok(HttpResponse::Created().json(response))
    }

    pub async fn get_by_id(&self, id: i64) -> Result<HttpResponse, DomainError> {
        let result = self.service.get_loan_request_by_id(id).await?;
        let response = LoanRequestResponseDTO::from(result);
        
        Ok(HttpResponse::Ok().json(response))
    }

    pub async fn list(&self, page: i32, per_page: i32, status: Option<String>) -> Result<HttpResponse, DomainError> {
        let result = self.service.list_loan_requests(page, per_page, status).await?;
        let response = PaginatedResponse::from_entities(
            result.records,
            result.total,
            result.page,
            result.per_page,
        );
        
        Ok(HttpResponse::Ok().json(response))
    }

    pub async fn update(&self, id: i64, payload: UpdateLoanRequestPayload) -> Result<HttpResponse, DomainError> {
        let result = self.service.update_loan_request(id, payload.amount, payload.term_months, payload.annual_rate).await?;
        let response = LoanRequestResponseDTO::from(result);
        
        Ok(HttpResponse::Ok().json(response))
    }

    pub async fn delete(&self, id: i64) -> Result<HttpResponse, DomainError> {
        self.service.delete_loan_request(id).await?;
        
        Ok(HttpResponse::NoContent().finish())
    }

    pub async fn approve(&self, id: i64, risk_score: i32) -> Result<HttpResponse, DomainError> {
        let result = self.service.approve_loan_request(id, risk_score).await?;
        let response = LoanRequestResponseDTO::from(result);
        
        Ok(HttpResponse::Ok().json(response))
    }

    pub async fn reject(&self, id: i64, risk_score: i32, reason: String) -> Result<HttpResponse, DomainError> {
        let result = self.service.reject_loan_request(id, risk_score, reason).await?;
        let response = LoanRequestResponseDTO::from(result);
        
        Ok(HttpResponse::Ok().json(response))
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    let repository = LoanRequestRepository::new();
    let risk_client = RiskEngineClient::new(
        std::env::var("RISK_ENGINE_URL").unwrap_or_else(|_| "http://risk-engine.internal:8080".to_string()),
    );
    let audit_producer = AuditEventProducer::new(
        std::env::var("AUDIT_QUEUE").unwrap_or_else(|_| "audit_events".to_string()),
    );
    
    let controller = LoanRequestController::new(repository, risk_client, audit_producer);
    
    cfg.app_data(web::Data::new(controller))
        .service(
            web::resource("/api/v1/loan-requests")
                .route(web::post().to(create_loan_request))
                .route(web::get().to(list_loan_requests))
        )
        .service(
            web::resource("/api/v1/loan-requests/{id}")
                .route(web::get().to(get_loan_request))
                .route(web::put().to(update_loan_request))
                .route(web::delete().to(delete_loan_request))
        )
        .service(
            web::resource("/api/v1/loan-requests/{id}/approve")
                .route(web::post().to(approve_loan_request))
        )
        .service(
            web::resource("/api/v1/loan-requests/{id}/reject")
                .route(web::post().to(reject_loan_request))
        );
}

#[utoipa::path(
    post,
    path = "/api/v1/loan-requests",
    request_body = CreateLoanRequestPayload,
    responses(
        (status = 201, description = "Solicitud de préstamo creada exitosamente", body = LoanRequestResponseDTO),
        (status = 400, description = "Error de validación", body = ApiError),
        (status = 409, description = "Conflicto de idempotencia", body = ApiError),
    )
)]
async fn create_loan_request(
    controller: web::Data<LoanRequestController>,
    payload: web::Json<CreateLoanRequestPayload>,
) -> Result<HttpResponse, DomainError> {
    controller.create(payload.into_inner()).await
}

#[utoipa::path(
    get,
    path = "/api/v1/loan-requests/{id}",
    responses(
        (status = 200, description = "Solicitud de préstamo encontrada", body = LoanRequestResponseDTO),
        (status = 404, description = "Solicitud no encontrada", body = ApiError),
    )
)]
async fn get_loan_request(
    controller: web::Data<LoanRequestController>,
    path: web::Path<i64>,
) -> Result<HttpResponse, DomainError> {
    controller.get_by_id(path.into_inner()).await
}

#[utoipa::path(
    get,
    path = "/api/v1/loan-requests",
    responses(
        (status = 200, description = "Lista de solicitudes de préstamos", body = PaginatedResponse<LoanRequestResponseDTO>),
    )
)]
async fn list_loan_requests(
    controller: web::Data<LoanRequestController>,
    query: web::Query<QueryParams>,
) -> Result<HttpResponse, DomainError> {
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(20);
    controller.list(page, per_page, query.status.clone()).await
}

#[utoipa::path(
    put,
    path = "/api/v1/loan-requests/{id}",
    request_body = UpdateLoanRequestPayload,
    responses(
        (status = 200, description = "Solicitud de préstamo actualizada", body = LoanRequestResponseDTO),
        (status = 400, description = "Error de validación", body = ApiError),
        (status = 404, description = "Solicitud no encontrada", body = ApiError),
    )
)]
async fn update_loan_request(
    controller: web::Data<LoanRequestController>,
    path: web::Path<i64>,
    payload: web::Json<UpdateLoanRequestPayload>,
) -> Result<HttpResponse, DomainError> {
    controller.update(path.into_inner(), payload.into_inner()).await
}

#[utoipa::path(
    delete,
    path = "/api/v1/loan-requests/{id}",
    responses(
        (status = 204, description = "Solicitud de préstamo eliminada"),
        (status = 404, description = "Solicitud no encontrada", body = ApiError),
    )
)]
async fn delete_loan_request(
    controller: web::Data<LoanRequestController>,
    path: web::Path<i64>,
) -> Result<HttpResponse, DomainError> {
    controller.delete(path.into_inner()).await
}

#[utoipa::path(
    post,
    path = "/api/v1/loan-requests/{id}/approve",
    responses(
        (status = 200, description = "Solicitud de préstamo aprobada", body = LoanRequestResponseDTO),
        (status = 404, description = "Solicitud no encontrada", body = ApiError),
    )
)]
async fn approve_loan_request(
    controller: web::Data<LoanRequestController>,
    path: web::Path<i64>,
    body: web::Json<ApprovePayload>,
) -> Result<HttpResponse, DomainError> {
    controller.approve(path.into_inner(), body.risk_score).await
}

#[utoipa::path(
    post,
    path = "/api/v1/loan-requests/{id}/reject",
    responses(
        (status = 200, description = "Solicitud de préstamo rechazada", body = LoanRequestResponseDTO),
        (status = 404, description = "Solicitud no encontrada", body = ApiError),
    )
)]
async fn reject_loan_request(
    controller: web::Data<LoanRequestController>,
    path: web::Path<i64>,
    body: web::Json<RejectPayload>,
) -> Result<HttpResponse, DomainError> {
    controller.reject(path.into_inner(), body.risk_score, body.reason.clone()).await
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApprovePayload {
    pub risk_score: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RejectPayload {
    pub risk_score: i32,
    pub reason: String,
}