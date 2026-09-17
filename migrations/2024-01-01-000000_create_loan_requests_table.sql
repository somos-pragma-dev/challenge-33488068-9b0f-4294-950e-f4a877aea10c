-- Migration: Create loan_requests table
-- Created: 2024-01-01
-- Purpose: Tabla principal para gestionar solicitudes de préstamos en la fintech
-- Pattern: Este script crea la estructura base. Migraciones futuras añadirán índices y constraints.

-- Drop table if exists (only for development/reset scenarios, commented by default)
-- DROP TABLE IF EXISTS loan_requests CASCADE;

CREATE TABLE loan_requests (
    -- Primary key with auto-increment
    id BIGSERIAL PRIMARY KEY,
    
    -- Identificador único de la operación proporcionado por el cliente externo
    -- Útil para trazabilidad y debugging
    operation_number VARCHAR(100) NOT NULL,
    
    -- Canal por el cual se recibió la solicitud (web, mobile, api, branch)
    -- Necesario para la clave de idempotencia compuesta
    channel VARCHAR(50) NOT NULL DEFAULT 'api',
    
    -- Clave de idempotencia: combinación única de operation_number + channel
    -- Garantiza que la misma solicitud no se procese dos veces
    idempotency_key VARCHAR(200) NOT NULL UNIQUE,
    
    -- Identificador único del cliente en el sistema
    customer_id VARCHAR(100) NOT NULL,
    
    -- Nombre completo del solicitante
    customer_name VARCHAR(255) NOT NULL,
    
    -- Documento de identidad del cliente (DNI, RFC, CURP, etc.)
    -- Se almacenará con enmascaramiento parcial para privacidad
    customer_document VARCHAR(50) NOT NULL,
    
    -- Correo electrónico del cliente
    -- Se almacenará con enmascaramiento parcial para privacidad
    customer_email VARCHAR(255) NOT NULL,
    
    -- Teléfono de contacto del cliente
    -- Se almacenará con enmascaramiento parcial para privacidad
    customer_phone VARCHAR(30) NOT NULL,
    
    -- Monto solicitado del préstamo
    -- Tipo NUMERIC para evitar problemas de precisión con dinero
    amount NUMERIC(15, 2) NOT NULL CHECK (amount > 0),
    
    -- Plazo del préstamo en meses
    term_months INTEGER NOT NULL CHECK (term_months > 0 AND term_months <= 360),
    
    -- Tasa de interés anual expresada como decimal (ej: 0.25 para 25%)
    annual_interest_rate NUMERIC(8, 6) NOT NULL CHECK (annual_interest_rate >= 0 AND annual_interest_rate <= 2),
    
    -- Monto de la cuota mensual calculada
    -- Se calcula automáticamente al aprobar la solicitud
    monthly_payment NUMERIC(15, 2),
    
    -- Estado actual de la solicitud
    -- Estados: pending, under_review, approved, rejected, cancelled
    status VARCHAR(30) NOT NULL DEFAULT 'pending',
    
    -- Puntuación de riesgo retornada por el motor de evaluación
    -- Rango típico: 0-100 (mayor score = mayor riesgo)
    risk_score INTEGER,
    
    -- Decisión del motor de riesgos: approved, rejected, review_required
    risk_decision VARCHAR(30),
    
    -- Razón detallada del rechazo o requerimiento de revisión
    risk_reason TEXT,
    
    -- Timestamp de creación del registro
    -- Se mantiene como UTC para consistencia entre servidores
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    -- Timestamp de última modificación
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    -- Timestamp cuando la solicitud fue revisada/aprobada/rechazada
    reviewed_at TIMESTAMP WITH TIME ZONE,
    
    -- Metadatos adicionales en formato JSON
    -- Incluye información como: ip_address, user_agent, device_info, etc.
    metadata JSONB DEFAULT '{}'::jsonb,
    
    -- Constraints adicionales
    CONSTRAINT chk_amount_positive CHECK (amount > 0),
    CONSTRAINT chk_term_months_valid CHECK (term_months > 0 AND term_months <= 360),
    CONSTRAINT chk_annual_rate_valid CHECK (annual_interest_rate >= 0 AND annual_interest_rate <= 2),
    CONSTRAINT chk_status_valid CHECK (status IN ('pending', 'under_review', 'approved', 'rejected', 'cancelled'))
);

-- Índices para optimizar consultas frecuentes

-- Índice para búsqueda por clave de idempotencia (búsqueda exacta, caso crítico para evitar duplicados)
CREATE UNIQUE INDEX idx_loan_requests_idempotency_key ON loan_requests(idempotency_key);

-- Índice para búsqueda por operation_number + channel (consulta de estado por número de operación)
CREATE INDEX idx_loan_requests_operation_channel ON loan_requests(operation_number, channel);

-- Índice para búsqueda por customer_id (historial de solicitudes del cliente)
CREATE INDEX idx_loan_requests_customer_id ON loan_requests(customer_id);

-- Índice para búsqueda por estado (dashboard, reportes, colas de procesamiento)
CREATE INDEX idx_loan_requests_status ON loan_requests(status);

-- Índice para búsqueda por fecha de creación (reportes, auditorías, analytics)
CREATE INDEX idx_loan_requests_created_at ON loan_requests(created_at DESC);

-- Índice compuesto para búsquedas frecuentes: cliente + estado
CREATE INDEX idx_loan_requests_customer_status ON loan_requests(customer_id, status);

-- Índice para búsquedas por fecha de revisión (métricas de tiempo de respuesta)
CREATE INDEX idx_loan_requests_reviewed_at ON loan_requests(reviewed_at) WHERE reviewed_at IS NOT NULL;

-- Índice Partial: solo solicitudes pendientes (cola de procesamiento)
CREATE INDEX idx_loan_requests_pending ON loan_requests(id) WHERE status = 'pending';

-- Índice para búsquedas en el campo JSON de metadata (información adicional)
CREATE INDEX idx_loan_requests_metadata_gin ON loan_requests USING gin(metadata);

-- Comentarios para documentación de la estructura
COMMENT ON TABLE loan_requests IS 'Tabla principal para gestionar solicitudes de préstamos en la plataforma fintech';
COMMENT ON COLUMN loan_requests.id IS 'Identificador único autoincremental de la solicitud';
COMMENT ON COLUMN loan_requests.operation_number IS 'Número de operación proporcionado por el cliente externo para trazabilidad';
COMMENT ON COLUMN loan_requests.channel IS 'Canal de origen de la solicitud (web, mobile, api, branch)';
COMMENT ON COLUMN loan_requests.idempotency_key IS 'Clave compuesta única para garantizar idempotencia: operation_number + channel';
COMMENT ON COLUMN loan_requests.customer_id IS 'Identificador único del cliente en el sistema';
COMMENT ON COLUMN loan_requests.customer_name IS 'Nombre completo del solicitante';
COMMENT ON COLUMN loan_requests.customer_document IS 'Documento de identidad (DNI, RFC, CURP) - almacenado con enmascaramiento parcial';
COMMENT ON COLUMN loan_requests.customer_email IS 'Correo electrónico del cliente - almacenado con enmascaramiento parcial';
COMMENT ON COLUMN loan_requests.customer_phone IS 'Teléfono de contacto - almacenado con enmascaramiento parcial';
COMMENT ON COLUMN loan_requests.amount IS 'Monto solicitado del préstamo en la moneda base';
COMMENT ON COLUMN loan_requests.term_months IS 'Plazo del préstamo expresado en meses';
COMMENT ON COLUMN loan_requests.annual_interest_rate IS 'Tasa de interés anual expresada como decimal (0.25 = 25%)';
COMMENT ON COLUMN loan_requests.monthly_payment IS 'Monto de la cuota mensual calculada';
COMMENT ON COLUMN loan_requests.status IS 'Estado de la solicitud: pending, under_review, approved, rejected, cancelled';
COMMENT ON COLUMN loan_requests.risk_score IS 'Puntuación de riesgo retornada por el motor de evaluación (0-100)';
COMMENT ON COLUMN loan_requests.risk_decision IS 'Decisión del motor de riesgos: approved, rejected, review_required';
COMMENT ON COLUMN loan_requests.risk_reason IS 'Razón detallada del rechazo o requerimiento de revisión';
COMMENT ON COLUMN loan_requests.created_at IS 'Timestamp UTC de creación del registro';
COMMENT ON COLUMN loan_requests.updated_at IS 'Timestamp UTC de última modificación';
COMMENT ON COLUMN loan_requests.reviewed_at IS 'Timestamp UTC cuando la solicitud fue revisada/aprobada/rechazada';
COMMENT ON COLUMN loan_requests.metadata IS 'Metadatos adicionales en formato JSON (ip, user_agent, device_info, etc.)';

-- Trigger para actualizar automáticamente updated_at en cada UPDATE
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_loan_requests_updated_at
    BEFORE UPDATE ON loan_requests
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();