# AGENTS.md

Instrucciones para el agente de IA que abra este repositorio (Claude Code, Cursor, Codex, Copilot, Gemini). Se cargan solas: no hay que pegar nada en ningun chat.

## Que es este repositorio

Es el codigo base de un reto de aprendizaje de Pragma: **Diseño y construcción de una API REST en Rust con Actix Web y Diesel ORM**.

| | |
|---|---|
| Tema | rust-actix-web |
| Nivel | junior-l2 |
| Chapter | Generico |
| Especialidad | Inferido del contexto |
| Stack | Rust / Actix Web 4.9 |
| Patron arquitectonico | capas estándar (controlador, servicio, repositorio, dominio) |
| Tiempo estimado | 8 horas |

## Tu tarea

Dejar este proyecto en estado **verificable**: que el comando de verificacion corra sin errores. Escribi los archivos en disco, en este repositorio. No generes ZIPs ni archivos adjuntos.

En orden:

1. Corre `el comando de build o arranque canonico del stack elegido` y mira que falla.
2. Completa lo que falte de la lista de abajo: manifiesto de dependencias, punto de entrada, capa de interfaz y las capas del patron declarado.
3. Arregla SOLO los errores que impiden compilar o arrancar.
4. Volve a correr `el comando de build o arranque canonico del stack elegido` hasta que pase.
5. Pará ahí.

## Regla dura: las fases son trabajo del humano

**PROHIBIDO implementar los entregables de las fases.** El valor del reto esta en que la persona los resuelva. Tu trabajo es que tenga un proyecto que arranca; el hueco pedagogico se queda como esta.

No resuelvas nada de esto:

- **Fase 1 — Registro de solicitudes**: API que acepta y registra solicitudes de préstamos con validación y garantía de idempotencia.
- **Fase 2 — Interacción con el motor de evaluación de riesgos**: API que interactúa con el 'motor de evaluación de riesgos' y devuelve la respuesta al cliente, con manejo de errores y reintentos.
- **Fase 3 — Emisión de eventos al sistema de auditoría**: API que emite eventos al'sistema de auditoría' en caso de falla del motor de evaluación, con información detallada sobre la solicitud y el error.
- **Fase 4 — Optimización y refactorización**: Documento de propuestas para optimizar y refactorizar el código de la API, con justificación de las decisiones tomadas.

Distincion operativa:

- **Arreglar** (si): import faltante, tipo que no existe, dependencia sin declarar, error de sintaxis, archivo referenciado que no existe.
- **No tocar** (no): logica de negocio incompleta, validaciones ausentes, secretos hardcodeados, APIs deprecadas que funcionan, concurrencia insegura, patrones mejorables. Eso es lo que la persona tiene que encontrar.

## Lo que falta y tenes que completar

### 1. Boilerplate del stack (2)

Sin esto el proyecto no compila ni arranca. **Es tu trabajo crearlo**, y no toca nada de lo pedagogico: es andamiaje del stack.

- [ ] **Punto de entrada del stack elegido** — Sin un punto de entrada reconocible, el runtime no tiene por donde arrancar la aplicacion.
- [ ] **Capa de interfaz (controller/handler)** — Sin una capa de interfaz explicita, no hay forma de invocar la logica de negocio desde afuera del proceso.

### Presentes (14)

- `package.json`
- `src/domain/models/loan_request.rs`
- `src/domain/dtos/loan_request_dto.rs`
- `src/domain/error/mod.rs`
- `Cargo.toml`
- `src/main.rs`
- `src/interfaces/controllers/loan_request_controller.rs`
- `src/infrastructure/repository/loan_request_repository.rs`
- `src/infrastructure/audit/audit_event.rs`
- `src/infrastructure/risk_engine/risk_engine_client.rs`
- `src/application/services/loan_request_service.rs`
- `src/config/mod.rs`
- `tests/loan_request_integration_test.rs`
- `migrations/2024-01-01-000000_create_loan_requests_table.sql`

### Capas del patron declarado

Cada una tiene que existir como directorio real con al menos un archivo. Codigo plano en la raiz no satisface el patron.

- `src`
- `src/domain`
- `src/domain/models`
- `src/domain/dtos`
- `src/domain/error`
- `src/infrastructure`
- `src/infrastructure/repository`
- `src/infrastructure/audit`
- `src/infrastructure/risk_engine`
- `src/application`
- `src/application/services`
- `src/interfaces`
- `src/interfaces/controllers`
- `src/config`
- `tests`

## Verificacion

```bash
el comando de build o arranque canonico del stack elegido
```

Ese comando pasando es la definicion de "terminado" para vos.

## Convenciones que tenes que respetar

- Un solo ecosistema: no declares librerias de otro lenguaje ni mezcles gestores de paquetes.
- Toda libreria que uses tiene que estar declarada en el manifiesto de dependencias.
- Todo import declarado tiene que usarse; todo tipo usado tiene que existir o venir de una dependencia declarada.
- El patron es **capas estándar (controlador, servicio, repositorio, dominio)**: los contratos (interfaces, puertos) los define la capa interna y los implementa la externa, nunca al revés.
- Los archivos que crees llevan implementacion real, no stubs: sin `TODO`, sin cuerpos vacios, sin `// getters y setters`.

## Contexto del candidato

Sirve para calibrar el nivel del codigo, no para resolver las fases.

- Brecha que el reto ataca: Build a REST API with Rust, Actix Web and Diesel ORM

---

*Generado por Challenge Generator — Pragma. `README.md` tiene el enunciado completo del reto para la persona. `PROMPT_MEJORA.md` es la variante para pegar en un chat, si se prefiere ese flujo.*
