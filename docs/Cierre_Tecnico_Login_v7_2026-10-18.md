# Correcciones de Login v7 — 18 de octubre de 2026

## Resultado de implementación

Se corrigieron los dos defectos de actividad de R8 y se incorporaron pruebas permanentes. El cierre técnico automatizado pasa; la aceptación de escritorio queda pendiente de ejecución real. Este documento complementa las evaluaciones históricas, no modifica retroactivamente sus resultados.

### Actividad y sesión

- Cada keydown, mousemove, mousedown y touchstart comunica actividad inmediatamente; se eliminó la ventana de descarte de treinta segundos y el contrato ambiguo deltaMs.
- Rust registra la recepción con Instant y comprueba vigencia antes de renovar. Polling y temporizadores no renuevan actividad.
- Login y resumen incorporan session_id en memoria. Registrar actividad requiere el identificador de la sesión que originó el evento. Una petición antigua no renueva una sesión posterior, aunque sea del mismo usuario.
- Una sesión vencida o revocada no se reactiva por actividad. El estado limitado permite interacción sin habilitar operaciones de negocio.
- Respuestas antiguas no recuperan identidad. Logout limpia indicadores de carga incluso si queda una consulta pendiente; su respuesta de ambiente tampoco sobrescribe una sesión posterior.

No se agrupan eventos de ratón: se priorizó eliminar pérdidas y retrasos intencionales. La latencia/carga efectiva de IPC y la respuesta con movimiento continuo deben observarse en WebView2; las pruebas de API simulada no miden ese comportamiento físico.

### Autorización

- Se retiró de invoke el comando diagnóstico que recibía acción y referencia esperadas del frontend.
- El gestor ofrece ejecución autorizada con acción constante del origen, referencia derivada y efecto posterior a las comprobaciones de sesión y token.
- La prueba ficticia usa SQLite en memoria y verificación administrativa real del servicio: cinco fallos pausan la verificación, Login del administrador no queda bloqueado, una referencia alterada no ejecuta nada, el efecto correcto ocurre una vez y registra ejecutor/autorizante conservando sesión del cajero.
- Si el efecto falla, el token queda consumido y hace falta nueva autorización. Cada futuro módulo sigue siendo responsable de sus permisos, transacción y reglas de negocio; no se implementó POS.

### Validación y fallos locales

- Usuario conforme al formato del esquema antes de SQL: a-z, 0-9, punto, guion y guion bajo; normalización y límites conservados.
- Longitud por code points en validadores; los campos admiten hasta 256 unidades UTF-16 para no truncar una contraseña válida de 128 code points. El validador sigue imponiendo 128, no se amplió la política a 256 caracteres.
- Números ASCII en Rust y React; se conservan letras Unicode y contraseñas de 8–128 caracteres.
- Detección precisa del código NO_AUTENTICADO; los formularios de Login/cambio presentan el mensaje sin prefijos técnicos.
- Tracing usa el Display sanitizado del error de aplicación en salida, expiración y autorización.
- Recuperación falla antes de escribir si no puede bloquear el estado de sesión; no ignora ese fallo y continúa con la escritura.
- Instalación exige actualizar una fila de configuración; si la escritura se ignora, revierte el administrador y no entrega éxito/código de una instalación incompleta.
- El núcleo del comando login se prueba directamente: fallo de lectura de ambiente sin nueva sesión ni LOGIN_OK adicional y reintento exitoso tras restaurarla.

## Pruebas permanentes y resultados

| Verificación | Resultado |
|---|---|
| `npm test` | 27 aprobadas: 13 de contexto y 14 existentes. |
| `cargo test --test auth_tests --test auth_regression_tests --lib --offline` desde src-tauri | 18 Auth + 12 regresiones Auth + 4 biblioteca aprobadas. |
| `npm run build` | TypeScript y Vite aprobados. |
| `git diff --check` | Sin errores de espacios. |

Las pruebas históricas audit-r8 se integraron y corrigieron en `src/context/__tests__/AuthContext.test.tsx` y `src-tauri/tests/auth_regression_tests.rs`; no se eliminaron sus regresiones para ocultar fallos.

Se cubren eventos a 20 segundos y 14:30, frontera backend 899/900 segundos, auditoría única con usuario válido, actividad de sesión anterior, cuenta revocada, fallo de ambiente, reintento, error público real, polling real con reloj simulado, respuesta tardía, fallo de login y cuatro rollbacks: instalación, recuperación y ambos cambios de clave.

Las suites deshabilitadas de módulos posteriores permanecen fuera del alcance. No se afirma que estén aprobadas.

## Acta de aceptación en escritorio — pendiente

Completar en el Windows objetivo con una base de prueba. No registrar contraseñas, códigos de recuperación ni identificadores de sesión en evidencias.

Responsable: __________. Fecha: __________. Commit/versión: __________.

Equipo/CPU/RAM/Windows: __________. Ambiente de BD: __________.

| TC del PDF | Recorrido que ejecutar | Estado manual / evidencia |
|---|---|---|
| 01 Roles | Administrador, mecánico y cajero; menú correcto y comando prohibido rechazado. | Pendiente |
| 02 Bloqueo | Cinco errores, reinicio y comprobación antes/después del vencimiento; éxito limpia fallos. | Pendiente |
| 03 Temporal | Nueva y confirmación; 7/8/128/129; no habilitar negocio antes de completar. | Pendiente |
| 04 Voluntario | Actual incorrecta, cancelar y cambiar correctamente; sin sumar fallos de Login. | Pendiente |
| 05 Inactividad | Interacción a 20 s y 14:30; 14:59/15:00 desde último evento; tareas de fondo no renuevan. Observar respuesta con ratón continuo. | Pendiente |
| 06 Salida | Salida directa sin captura; una confirmación con dirty; no cerrar caja al salir. | Pendiente |
| 07 Verificación | La operación ficticia está automatizada. Recorrido visual de la solicitud administrativa, errores y cancelación en escritorio. | Pendiente |
| 08 Recuperación | Código válido, anterior inválido y cinco códigos erróneos; contador independiente. | Pendiente |
| 09 Instalación/fallo | Una cuenta inicial; duplicación rechazada y fallo local sin éxito parcial. | Pendiente |
| 10 Privacidad/revocación | Evidencias sin secretos y siguiente acción rechazada tras revocación. | Pendiente |

En los recorridos de formularios comprobar Tab, Shift+Tab, Enter, foco visible, Mostrar/Ocultar y bloqueo del botón mientras procesa.

### RNF de rendimiento

Medir desde confirmar un ingreso válido hasta que el menú autorizado esté utilizable, en el equipo identificado. Cinco ingresos consecutivos; objetivo hasta 3 s cada uno. No sustituirlos por duración de tests o de compilación.

| Ingreso | Tiempo real | Cumple ≤3 s |
|---|---|---|
| 1 | Pendiente | Pendiente |
| 2 | Pendiente | Pendiente |
| 3 | Pendiente | Pendiente |
| 4 | Pendiente | Pendiente |
| 5 | Pendiente | Pendiente |

Decisión de aceptación: pendiente. Si se detecta un fallo, anotar pasos, corregirlo y repetir el caso. No es necesario terminar módulos futuros para completar esta acta de Login.
