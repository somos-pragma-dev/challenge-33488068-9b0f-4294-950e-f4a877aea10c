# Prompt para Mejorar el Codigo Base

Copia y pega el contenido del bloque de abajo en un asistente de IA (Claude, ChatGPT)
para obtener un ZIP con el proyecto completo y arrancable.

Si preferis trabajar en tu editor con un agente local (Claude Code, Cursor, Copilot), usa `AGENTS.md` en vez de este archivo: dice lo mismo pero para que escriba los archivos en disco.

## Las dos reglas que no se negocian

1. **Completa el boilerplate.** Todo lo que el proyecto necesita para compilar y arrancar: manifiesto de dependencias, punto de entrada, configuracion, capa de interfaz, y las capas del patron arquitectonico declarado. Eso es andamiaje y es tu trabajo.
2. **NO resuelvas el reto.** Los entregables de las fases son el trabajo de la persona. El hueco pedagogico se deja como esta: el proyecto arranca, pero lo que el reto pide implementar NO esta implementado.

Dicho de otra forma: si algo impide compilar, arreglalo. Si algo es logica de negocio incompleta, validaciones ausentes, un secreto hardcodeado o un patron mejorable, dejalo exactamente como esta — es lo que la persona tiene que encontrar.

## Lo que le falta a este proyecto

Esto NO lo tenes que adivinar: salio de comparar el proyecto contra la arquitectura declarada del reto y de un analisis estatico del codigo. Completalo TODO.

### Boilerplate del stack que falta

Sin esto no compila ni arranca. Es andamiaje, no toca nada de lo pedagogico:

- **Punto de entrada del stack elegido** — Sin un punto de entrada reconocible, el runtime no tiene por donde arrancar la aplicacion.
- **Capa de interfaz (controller/handler)** — Sin una capa de interfaz explicita, no hay forma de invocar la logica de negocio desde afuera del proceso.

## Como saber que terminaste

```bash
el comando de build o arranque canonico del stack elegido
```

Ese comando corriendo sin errores es la definicion de "listo".

---

```
## Briefing del reto (autoridad)
Este bloque manda sobre los archivos adjuntos. El stack y el rol salen de AQUÍ, no de un topic genérico ni de markdown placeholder.

### Contexto técnico original
Build a REST API with Rust, Actix Web and Diesel ORM

### Reto
- Tema: rust-actix-web
- Seniority: junior-l2
- Tipo: practical
- Título: Diseño y construcción de una API REST en Rust con Actix Web y Diesel ORM
- Tiempo estimado: 8 horas

### Fases (trabajo del HUMANO — PROHIBIDO completarlas)
No implementes estos entregables. Dejalos como hueco pedagógico. El asistente solo materializa el proyecto arrancable para que el participante pueda trabajar.
- Fase 1: Registro de solicitudes — objetivo: Implementar la funcionalidad para registrar solicitudes de préstamos en la API. — entregable (NO resolver): API que acepta y registra solicitudes de préstamos con validación y garantía de idempotencia.
- Fase 2: Interacción con el motor de evaluación de riesgos — objetivo: Implementar la funcionalidad para interactuar con el motor de evaluación de riesgos y devolver la respuesta al cliente. — entregable (NO resolver): API que interactúa con el 'motor de evaluación de riesgos' y devuelve la respuesta al cliente, con manejo de errores y reintentos.
- Fase 3: Emisión de eventos al sistema de auditoría — objetivo: Implementar la funcionalidad para emitir eventos al 'sistema de auditoría' en caso de falla del motor de evaluación. — entregable (NO resolver): API que emite eventos al'sistema de auditoría' en caso de falla del motor de evaluación, con información detallada sobre la solicitud y el error.
- Fase 4: Optimización y refactorización — objetivo: Optimizar y refactorizar el código de la API para mejorar su rendimiento y mantenibilidad. — entregable (NO resolver): Documento de propuestas para optimizar y refactorizar el código de la API, con justificación de las decisiones tomadas.

Eres un asistente experto en análisis, corrección y generación de archivos de cualquier tipo:
código fuente, documentación, hojas de cálculo, documentos Word, configuraciones, entre otros.
Voy a enviarte una cadena de texto que contiene uno o más archivos. Cada archivo está delimitado por un marcador con el siguiente formato:
// === ARCHIVO: ruta/del/archivo.extension ===
o también puede aparecer como:
## === ARCHIVO: ruta/del/archivo.extension ===
Lo que sigue al marcador puede ser:

El contenido real del archivo (código, texto, YAML, etc.)
Una descripción en lenguaje natural de lo que debe contener el archivo


TU TAREA
PASO 0 — ¿Esto es un proyecto o una carcasa?
Antes de extraer archivos, leé el Briefing (si está) y diagnosticá el adjunto.

Es CARCASA si ocurre CUALQUIERA de estas:
- No hay manifiesto de dependencias del stack del briefing (manifest.json de VTEX IO / package.json / pom.xml / build.gradle / requirements.txt / go.mod / *.tf / *.csproj, según corresponda)
- Hay un "binario" que en realidad es un comentario ("no puede ser mostrado como texto plano", placeholder .fig/.docx vacío)
- Los markdowns ya completan entregables de fases posteriores ("se implementó fade-in", lista de áreas ya resuelta)

Si es CARCASA:
- MATERIALIZÁ un proyecto que arranca en el stack del briefing (VTEX IO Store Framework, Angular, Terraform, pytest, Nest, etc.). Incluí manifiesto, punto de entrada y capa de interfaz reales.
- NO copies los markdowns de "solución" como si fueran el producto. Son ruido de generación.
- NO resuelvas las fases del briefing (están marcadas PROHIBIDO). Dejá el hueco pedagógico: el flujo existe, las microinteracciones/calidad/infra que el reto pide NO están hechas.
- Después seguí al PASO 5 (ZIP).

Si es un proyecto REAL (manifiesto + código que compila o arranca):
- Seguí PASO 1 en adelante. 🔴 compilación sí. 🟡 pedagógico no.

PASO 1 — Detección y extracción
Identifica todos los archivos presentes en la cadena. Para cada archivo extrae:

Su ruta completa (ej: src/main/java/com/pragma/Service.java)
Su contenido o descripción

PASO 2 — Clasificación por tipo
Clasifica cada archivo en una de estas categorías:
A) Código fuente (Java, Python, TypeScript, JavaScript, Kotlin, etc.)
B) Configuración / documentación (YAML, properties, Markdown, JSON, txt, etc.)
C) Excel (.xlsx, .xls, .csv)
D) Word (.docx, .doc)
E) Otro tipo de archivo binario o especial
PASO 3 — Clasificación de errores en código fuente

Objetivo prioritario: que el proyecto compile. No corrijas flujo de negocio ni lógica funcional.

Antes de modificar cualquier archivo de código fuente, clasifica cada problema encontrado en una de estas dos categorías:
🔴 ERROR DE COMPILACIÓN — corregir siempre
Son errores que impiden que el proyecto arranque, sin valor pedagógico:

Import faltante o incorrecto
Clase, método o variable referenciada que no existe en ningún archivo del proyecto
Error de sintaxis
Anotación con atributos inválidos
Dependencia ausente en pom.xml, package.json, etc.
Archivo referenciado que no existe y debe ser creado con implementación mínima

→ CORREGIR estos errores.
🟡 PROBLEMA FUNCIONAL O DE CALIDAD — preservar siempre
Son problemas que no impiden compilar. Pueden ser intencionales para el aprendizaje:

Clave secreta hardcodeada ("secret", "password123")
API deprecada que funciona pero tiene reemplazo moderno
Lógica de negocio incorrecta o incompleta
Código redundante o de baja legibilidad
Falta de validaciones en flujo de negocio
Patrones de diseño incorrectos pero funcionales
Concurrencia no segura
Configuración funcional pero no óptima

→ PRESERVAR tal cual. No corregir, no mejorar, no comentar.
PASO 4 — Procesamiento según tipo de archivo
Tipo A — Código fuente
Aplica únicamente las correcciones clasificadas como 🔴 ERROR DE COMPILACIÓN.
No alteres ningún elemento clasificado como 🟡 PROBLEMA FUNCIONAL O DE CALIDAD.
Si falta un archivo referenciado, créalo con la implementación mínima necesaria para compilar.
Tipo B — Configuración / documentación
Extrae el contenido tal cual, sin modificaciones salvo errores evidentes de sintaxis
(ej: YAML mal indentado).
Tipo C — Excel (.xlsx)
Si viene con contenido real, genera el archivo respetando ese contenido.
Si viene con descripción en lenguaje natural, genera un archivo Excel funcional con:

Fila de encabezados en negrita con color de fondo distintivo
Columnas con ancho ajustado al contenido
Tipos de dato correctos por columna
Validaciones si la descripción lo indica
Hojas nombradas descriptivamente si hay más de una
Filas de ejemplo si no hay datos reales

Tipo D — Word (.docx)
Si viene con contenido real, genera el archivo respetando ese contenido.
Si viene con descripción en lenguaje natural, genera un documento Word funcional con:

Estilos de título (Título 1, Título 2) para jerarquía de secciones
Fuente legible (Calibri o equivalente), tamaño 11-12pt para cuerpo
Márgenes estándar
Tabla de contenido si tiene múltiples secciones
Tablas con encabezados en negrita si aplica

Tipo E — Otro
Genera el archivo con el contenido o estructura más apropiada según la descripción.
PASO 5 — Exportación en ZIP
Empaqueta todos los archivos en un único archivo ZIP descargable respetando exactamente
la estructura de rutas indicada por los marcadores.
El ZIP debe incluir:

Archivos de código con únicamente los errores de compilación corregidos
Archivos de configuración y documentación sin cambios
Archivos nuevos creados para resolver dependencias de compilación faltantes
Archivos Excel y Word generados desde descripción

IMPORTANTE: El ZIP debe estar listo para descargar al finalizar. No preguntes si el usuario
quiere generarlo. Simplemente genera el archivo y proporciona el enlace de descarga; No debes desplegar en el chat el resumen de lo que arreglaste al Zip, solo entregalo.

REGLAS IMPORTANTES

No omitas ningún archivo aunque no tenga errores ni modificaciones
Respeta los nombres y rutas exactas indicadas por los marcadores
Si un archivo no tiene marcador claro, infiere el nombre desde su contenido
Si la cadena contiene solo documentación, placeholders o binarios fake, NO la reproduzcas:
aplicá PASO 0 (materializar el proyecto del briefing). Reproducir la carcasa es un fallo.
No agregues texto después del enlace de descarga del ZIP
No preguntes si el usuario quiere el ZIP: simplemente generalo siempre
Si detectas que falta un archivo de configuración necesario para compilar
(pom.xml, package.json, requirements.txt, build.gradle, etc.), créalo e inclúyelo
inferiendo su contenido desde los imports y frameworks detectados en el código
Nunca corrijas problemas 🟡 aunque parezcan obvios o fáciles de mejorar.
El participante que recibirá este proyecto los debe encontrar y resolver él mismo.


INPUT
Aquí está la cadena con los archivos:

// === ARCHIVO: package.json ===
{
  "name": "loan-request-api",
  "version": "1.0.0",
  "description": "API REST para gestión de solicitudes de préstamos en fintech - Rust/Actix Web",
  "private": true,
  "workspaces": [
    "."
  ],
  "scripts": {
    "build": "cargo build --release",
    "dev": "cargo run",
    "test": "cargo test",
    "test:integration": "cargo test --test loan_request_integration_test",
    "migrate": "diesel migration run",
    "migrate:redo": "diesel migration redo",
    "lint": "cargo clippy",
    "fmt": "cargo fmt",
    "db:setup": "echo 'Configurando base de datos...' && diesel setup",
    "clean": "cargo clean"
  },
  "keywords": [
    "actix-web",
    "rust",
    "fintech",
    "loan-management",
    "rest-api",
    "diesel",
    "async"
  ],
  "author": "Fintech Development Team",
  "license": "MIT",
  "repository": {
    "type": "git",
    "url": "https://github.com/company/loan-request-api"
  },
  "engines": {
    "rust": ">=1.75.0",
    "cargo": ">=1.75.0"
  },
  "devDependencies": {
    "dotenv": "0.15.0"
  },
  "dependencies": {
    "actix-web": "4.9.0",
    "diesel": "2.2.0",
    "diesel-async": "0.4.0",
    "serde": "1.0.203",
    "tokio": "1.38.0",
    "thiserror": "1.0.59",
    "actix-web-validator": "5.0.0",
    "backoff": "0.4.0"
  },
  "build": {
    "rust": {
      "profile": "release",
      "lto": true,
      "opt-level": 3
    }
  },
  "config": {
    "database": {
      "url": "postgresql://user:password@localhost/loan_db",
      "pool_size": 20,
      "timeout_seconds": 30
    },
    "risk_engine": {
      "base_url": "http://risk-engine.internal:8080",
      "timeout_seconds": 10,
      "max_retries": 3
    },
    "audit": {
      "enabled": true,
      "queue_name": "audit_events"
    }
  }
}

// === ARCHIVO: src/domain/models/loan_request.rs ===
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
// === ARCHIVO: src/domain/dtos/loan_request_dto.rs ===
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
// === ARCHIVO: src/domain/error/mod.rs ===
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


// === ARCHIVO: Cargo.toml ===
[package]
name = "loan-request-api"
version = "1.0.0"
description = "API REST para gestión de solicitudes de préstamos en fintech - Rust/Actix Web"
authors = ["Fintech Development Team"]
license = "MIT"
edition = "2021"
rust-version = "1.75"

[lib]
path = "src/lib.rs"

[[bin]]
name = "loan-request-api"
path = "src/main.rs"

[dependencies]
actix-web = "4.9.0"
actix-web-validator = "5.0.0"
actix-rt = "2.10.0"
actix-service = "2.0.0"
actix-multipart = "0.6.1"
serde = { version = "1.0.203", features = ["derive"] }
serde_json = "1.0.117"
diesel = { version = "2.2.0", features = ["postgres", "uuid", "chrono", "r2d2"] }
diesel-async = { version = "0.4.0", features = ["postgres", "bb8"] }
tokio = { version = "1.38.0", features = ["full"] }
thiserror = "1.0.59"
backoff = "0.4.0"
uuid = { version = "1.8.0", features = ["v4", "serde"] }
chrono = { version = "0.4.38", features = ["serde"] }
bb8 = "0.8.5"
r2d2 = "0.8.10"
dotenv = "0.15.0"
log = "0.4"
env_logger = "0.11.0"
futures = "0.3.30"
parking_lot = "0.12.1"
async-trait = "0.1.80"

[dev-dependencies]
mockall = "0.12.1"
tokio-test = "0.4.4"

[features]
default = ["production"]
production = []
development = []

[[test]]
name = "loan_request_integration_test"
path = "tests/loan_request_integration_test.rs"

[profile.release]
lto = true
opt-level = 3
codegen-units = 1
strip = true

[profile.dev]
opt-level = 0
debug = true

// === ARCHIVO: src/main.rs ===
use actix_web::{web, App, HttpServer, middleware};
use log::info;
use std::io;

mod interfaces;
mod application;
mod domain;
mod infrastructure;
mod config;

use interfaces::controllers::loan_request_controller;
use config::database;

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    
    info!("Iniciando API de Solicitudes de Préstamos - Fintech v1.0.0");
    info!("Configurando pool de conexiones a base de datos...");
    
    let db_pool = database::create_pool().await.expect("Failed to create database pool");
    
    info!("Iniciando servidor HTTP en 0.0.0.0:8080...");
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .app_data(web::Data::new(config::risk_engine::create_risk_engine_client()))
            .app_data(web::Data::new(config::audit::create_audit_producer()))
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .configure(loan_request_controller::configure)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

// === ARCHIVO: src/interfaces/controllers/loan_request_controller.rs ===
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


// === ARCHIVO: src/infrastructure/repository/loan_request_repository.rs ===
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

// === ARCHIVO: src/infrastructure/audit/audit_event.rs ===
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

// === ARCHIVO: src/infrastructure/risk_engine/risk_engine_client.rs ===
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

// === ARCHIVO: src/application/services/loan_request_service.rs ===
use crate::domain::models::loan_request::{LoanRequest, LoanRequestStatus};
use crate::domain::dtos::loan_request_dto::{LoanRequestDto, LoanRequestResponseDto, PaginatedLoanRequestResponse};
use crate::domain::error::DomainError;
use crate::infrastructure::repository::loan_request_repository::LoanRequestRepository;
use crate::infrastructure::risk_engine::risk_engine_client::RiskEngineClient;
use crate::infrastructure::audit::audit_event::AuditEvent;
use actix_web::web;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use backoff::{ExponentialBackoff, future::retry};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct LoanRequestService {
    repository: Arc<RwLock<dyn LoanRequestRepository>>,
    risk_client: Arc<RiskEngineClient>,
    config: web::Data<AppConfig>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AppConfig {
    pub database_url: String,
    pub database_pool_size: u32,
    pub risk_engine_base_url: String,
    pub risk_engine_timeout_secs: u64,
    pub risk_engine_max_retries: u32,
    pub audit_enabled: bool,
    pub audit_queue_name: String,
    pub retry_initial_delay_ms: u64,
    pub retry_max_delay_ms: u64,
    pub retry_multiplier: f64,
    pub retry_max_retries: u32,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            database_url: String::from("postgresql://user:password@localhost/loan_db"),
            database_pool_size: 20,
            risk_engine_base_url: String::from("http://risk-engine.internal:8080"),
            risk_engine_timeout_secs: 10,
            risk_engine_max_retries: 3,
            audit_enabled: true,
            audit_queue_name: String::from("audit_events"),
            retry_initial_delay_ms: 100,
            retry_max_delay_ms: 30000,
            retry_multiplier: 2.0,
            retry_max_retries: 5,
        }
    }
}

impl LoanRequestService {
    pub fn new(
        repository: Arc<RwLock<dyn LoanRequestRepository>>,
        risk_client: Arc<RiskEngineClient>,
        config: web::Data<AppConfig>,
    ) -> Self {
        Self {
            repository,
            risk_client,
            config,
        }
    }

    pub async fn create_loan_request(
        &self,
        dto: LoanRequestDto,
    ) -> Result<LoanRequestResponseDto, DomainError> {
        dto.validate()?;
        
        let idempotency_key = dto.generate_idempotency_key();
        
        let repo = self.repository.read().await;
        if let Some(existing) = repo.find_by_idempotency_key(&idempotency_key).await? {
            return Err(DomainError::idempotency_conflict(
                "Ya existe una solicitud con este número de operación y canal",
                existing.id,
            ));
        }
        drop(repo);

        let entity = dto.to_entity(idempotency_key.clone());
        entity.validate()?;

        let risk_result = self.evaluate_risk_with_retry(&entity).await;
        
        let mut final_entity = entity;
        match risk_result {
            Ok(risk_response) => {
                if risk_response.approved {
                    final_entity.approve(risk_response.risk_score, risk_response.decision.clone());
                } else {
                    final_entity.reject(
                        risk_response.risk_score,
                        risk_response.decision.clone(),
                        risk_response.rejection_reason.clone().unwrap_or_default(),
                    );
                }
            }
            Err(e) => {
                self.emit_audit_event(&final_entity, &e).await;
                final_entity.mark_under_review();
            }
        }

        let repo = self.repository.write().await;
        let saved = repo.save(final_entity).await?;
        
        Ok(LoanRequestResponseDto::from(saved))
    }

    async fn evaluate_risk_with_retry(
        &self,
        entity: &LoanRequest,
    ) -> Result<RiskEvaluationResponse, DomainError> {
        let config = self.config.clone();
        
        let backoff = ExponentialBackoff {
            initial_interval: std::time::Duration::from_millis(config.retry_initial_delay_ms),
            max_interval: std::time::Duration::from_millis(config.retry_max_delay_ms),
            multiplier: config.retry_multiplier,
            max_elapsed_time: Some(std::time::Duration::from_secs(
                config.risk_engine_timeout_secs * 2,
            )),
            ..Default::default()
        };

        let entity_clone = entity.clone();
        let risk_client = self.risk_client.clone();
        
        retry(backoff, || {
            let entity = entity_clone.clone();
            let client = risk_client.clone();
            async move {
                client.evaluate_risk(&entity).await
            }
        }).await.map_err(|e| {
            DomainError::external_service(
                format!("Error al evaluar riesgo después de reintentos: {}", e),
                "risk_engine",
            )
        })
    }

    async fn emit_audit_event(&self, entity: &LoanRequest, error: &DomainError) {
        if !self.config.audit_enabled {
            return;
        }

        let event = AuditEvent::new(
            "loan_request_risk_evaluation_failed".to_string(),
            entity.id,
            serde_json::json!({
                "error_code": error.error_code(),
                "retryable": error.is_retryable(),
                "timestamp": Utc::now().to_rfc3339(),
            }),
        );

        log::warn!("Audit event emitted: {:?}", event);
    }

    pub async fn get_loan_request(&self, id: i64) -> Result<LoanRequestResponseDto, DomainError> {
        let repo = self.repository.read().await;
        let entity = repo.find_by_id(id).await?;
        Ok(LoanRequestResponseDto::from(entity))
    }

    pub async fn list_loan_requests(
        &self,
        page: i32,
        per_page: i32,
    ) -> Result<PaginatedLoanRequestResponse, DomainError> {
        let repo = self.repository.read().await;
        let (entities, total) = repo.find_all(page, per_page).await?;
        Ok(PaginatedLoanRequestResponse::from_entities(entities, total, page, per_page))
    }

    pub async fn cancel_loan_request(&self, id: i64) -> Result<LoanRequestResponseDto, DomainError> {
        let repo = self.repository.write().await;
        let mut entity = repo.find_by_id(id).await?;
        
        if !entity.can_be_modified() {
            return Err(DomainError::business(
                "La solicitud no puede ser cancelada en su estado actual",
                "INVALID_STATE_TRANSITION",
            ));
        }
        
        entity.cancel();
        let updated = repo.update(entity).await?;
        Ok(LoanRequestResponseDto::from(updated))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskEvaluationResponse {
    pub approved: bool,
    pub risk_score: i32,
    pub decision: String,
    pub rejection_reason: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::models::loan_request::LoanRequestStatus;
    
    #[test]
    fn test_app_config_default() {
        let config = AppConfig::default();
        assert_eq!(config.database_pool_size, 20);
        assert_eq!(config.risk_engine_max_retries, 3);
        assert_eq!(config.retry_max_retries, 5);
        assert!(config.audit_enabled);
    }
}

// === ARCHIVO: src/config/mod.rs ===
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

// === ARCHIVO: tests/loan_request_integration_test.rs ===
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


// === ARCHIVO: migrations/2024-01-01-000000_create_loan_requests_table.sql ===
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

```
