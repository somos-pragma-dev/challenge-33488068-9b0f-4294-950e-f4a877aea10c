# Diseño y construcción de una API REST en Rust con Actix Web y Diesel ORM

La empresa necesita una API REST que gestione solicitudes de préstamos en una fintech. La API debe registrar solicitudes, validar la información, interactuar con el motor de evaluación de riesgos y devolver respuestas al cliente. Los actores involucrados son el 'originador de créditos', el'motor de evaluación de riesgos' y el 'cliente'. La API debe manejar un volumen de 1 500 solicitudes por segundo en hora pico y garantizar la idempotencia del registro de solicitudes por 'número de operación + canal'. En caso de falla del motor de evaluación, la API debe emitir un evento al 'sistema de auditoría' y reintentar la operación.

## Informacion General

| Campo | Valor |
|-------|-------|
| **Tema** | rust-actix-web |
| **Nivel** | junior-l2 |
| **Tipo** | practical |
| **Tiempo estimado** | 8 horas |

## Fases del Reto

### Fase 0: Configuración del Proyecto

**Objetivo:** Obtener el proyecto base funcional enviando el Código Base a un asistente de IA, que lo analizará, corregirá errores y generará un ZIP listo para usar.

**Tiempo estimado:** 15-30 minutos

**Instrucciones:**

- Asegúrate de tener instalado para ejecutar el proyecto: Node.js 18+, npm, VS Code o similar.
- Copia todo el contenido del campo **Código Base** de este reto — incluyendo el texto de instrucciones que aparece al inicio.
- Abre un asistente de IA (Claude en claude.ai, ChatGPT o Gemini — se recomienda Claude), pega el contenido copiado en el chat y envíalo.
- El asistente analizará los archivos, corregirá errores y generará un archivo ZIP descargable. Descárgalo y extráelo en la carpeta donde quieras trabajar.
- Ejecuta `npm install && npm run build` (o `npm start`). Si no hay errores, estás listo.

**Entregable:** El proyecto compila/arranca sin errores.

<details>
<summary>Pistas de conocimiento</summary>

- Copia el Código Base completo incluyendo el texto de instrucciones al inicio — esas instrucciones le indican al asistente exactamente qué hacer con los archivos.
- Si el asistente no genera el ZIP automáticamente al terminar el análisis, escríbele: "genera el ZIP ahora".
- Si el proyecto tiene errores al arrancar, comparte el mensaje de error con el mismo asistente para que lo corrija.

</details>

### Fase 1: Registro de solicitudes

**Objetivo:** Implementar la funcionalidad para registrar solicitudes de préstamos en la API.

**Tiempo estimado:** 2 horas

**Instrucciones:**

- La API debe aceptar solicitudes de préstamos con los campos 'número de operación','monto', 'plazo' y 'canal'. Validar que el 'número de operación' sea único y que el 'monto' y 'plazo' sean positivos. Garantizar la idempotencia del registro por 'número de operación + canal'.

**Entregable:** API que acepta y registra solicitudes de préstamos con validación y garantía de idempotencia.

<details>
<summary>Pistas de conocimiento</summary>

- Considera cómo modelar la entidad 'solicitud de préstamo' y cómo validar sus atributos.
- Piensa en cómo implementar la idempotencia en el registro de solicitudes.

</details>

### Fase 2: Interacción con el motor de evaluación de riesgos

**Objetivo:** Implementar la funcionalidad para interactuar con el motor de evaluación de riesgos y devolver la respuesta al cliente.

**Tiempo estimado:** 3 horas

**Instrucciones:**

- La API debe enviar la solicitud al 'motor de evaluación de riesgos' y esperar la respuesta. Si el motor falla, la API debe emitir un evento al'sistema de auditoría' y reintentar la operación después de un tiempo configurado.

**Entregable:** API que interactúa con el 'motor de evaluación de riesgos' y devuelve la respuesta al cliente, con manejo de errores y reintentos.

<details>
<summary>Pistas de conocimiento</summary>

- Considera cómo modelar la interacción con el'motor de evaluación de riesgos' y cómo manejar los posibles errores.
- Piensa en cómo implementar el reintento de la operación en caso de falla del motor.

</details>

### Fase 3: Emisión de eventos al sistema de auditoría

**Objetivo:** Implementar la funcionalidad para emitir eventos al 'sistema de auditoría' en caso de falla del motor de evaluación.

**Tiempo estimado:** 2 horas

**Instrucciones:**

- La API debe emitir un evento al'sistema de auditoría' cuando el 'motor de evaluación de riesgos' falla. El evento debe contener información sobre la solicitud y el error ocurrido.

**Entregable:** API que emite eventos al'sistema de auditoría' en caso de falla del motor de evaluación, con información detallada sobre la solicitud y el error.

<details>
<summary>Pistas de conocimiento</summary>

- Considera cómo modelar el evento que se emite al'sistema de auditoría' y qué información debe contener.
- Piensa en cómo implementar la emisión del evento en caso de falla del motor.

</details>

### Fase 4: Optimización y refactorización

**Objetivo:** Optimizar y refactorizar el código de la API para mejorar su rendimiento y mantenibilidad.

**Tiempo estimado:** 1 hora

**Instrucciones:**

- Analiza el código de la API y propone mejoras para optimizar su rendimiento y refactorizarlo para mejorar su mantenibilidad. Considera aspectos como la estructura del código, el uso de patrones de diseño y la eficiencia de las operaciones.

**Entregable:** Documento de propuestas para optimizar y refactorizar el código de la API, con justificación de las decisiones tomadas.

<details>
<summary>Pistas de conocimiento</summary>

- Considera cómo mejorar la estructura del código y el uso de patrones de diseño.
- Piensa en cómo optimizar la eficiencia de las operaciones en la API.

</details>

## Dimensiones Evaluadas

- **queEs**: ¿Qué es una solicitud de préstamo y cuáles son sus atributos?
- **paraQueSirve**: ¿Para qué sirve la interacción con el motor de evaluación de riesgos?
- **comoSeUsa**: ¿Cómo se usa la idempotencia en el registro de solicitudes?
- **erroresComunes**: ¿Cuáles son los errores comunes en la interacción con el motor de evaluación de riesgos?
- **queDecisionesImplica**: ¿Qué decisiones implica la emisión de eventos al sistema de auditoría en caso de falla del motor de evaluación?

## Criterios de Evaluacion

- Implementar la funcionalidad para registrar solicitudes de préstamos con validación y garantía de idempotencia.
- Implementar la funcionalidad para interactuar con el motor de evaluación de riesgos y devolver la respuesta al cliente, con manejo de errores y reintentos.
- Implementar la funcionalidad para emitir eventos al sistema de auditoría en caso de falla del motor de evaluación, con información detallada sobre la solicitud y el error.
- Proponer mejoras para optimizar y refactorizar el código de la API, con justificación de las decisiones tomadas.

## Como trabajar con un asistente de IA

Hay dos caminos, elegi uno:

- **AGENTS.md** (recomendado) — instrucciones nativas del repo. Abri esta carpeta con tu agente local (Claude Code, Cursor, Codex, Copilot, Gemini) y las carga solo. Sabe que archivos faltan y con que comando se verifica, y completa el scaffold escribiendo en disco.
- **PROMPT_MEJORA.md** — para copiar y pegar en un chat (claude.ai, ChatGPT). Devuelve un ZIP con el proyecto. Sirve si no tenes un agente en el IDE.

Ninguno de los dos resuelve las fases del reto: eso es tu trabajo.

## Verificacion

El proyecto esta listo para trabajar cuando este comando corre sin errores:

```bash
el comando de build o arranque canonico del stack elegido
```

---

*Reto generado automaticamente por Challenge Generator - Pragma*
