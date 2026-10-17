# Login v7 — cuarta evaluación, 17 de octubre de 2026

**Resultado: 88/100. Evolución: 60 → 75 → 85 → 88. No aprobado todavía para cerrar el módulo.**

Promedio redondeado de las catorce valoraciones: 1225/14 = 87,5. La nota es una estimación técnica de cumplimiento, no el porcentaje de pruebas aprobadas. Las correcciones son reales, pero existen incumplimientos del contrato y pruebas de integración desactualizadas.

## Alcance y límites

Se revisaron el estado local de Git, los 31 archivos con cambios, la nueva suite de autorización y los límites de Login con comandos/servicios. Se volvió a leer la página 4 del PDF original: exige contraseña nueva de **8 a 128 caracteres**. No se recibió una modificación de ese contrato que autorice siete.

Se ejecutaron compilación frontend, pruebas existentes y diagnósticos adicionales. Estos últimos usaron SQLite ficticio y componentes React con API simulada. No se abrió la aplicación Windows/WebView2, no se probó instalador y no se operó sobre datos reales. Se retiraron las comprobaciones temporales; no se corrigió implementación. Este informe es el único archivo permanente añadido por la revisión.

No se exige terminar módulos posteriores. Las rutas ya expuestas deben aplicar permisos y propiedad o quedar deshabilitadas. Los defectos de esos módulos se separan de la nota de Login; la suite general no puede declararse aprobada si no compila.

## Puntuación por requisito

| Requisito | Previa | Actual | Fundamentación |
|---|---:|---:|---|
| RF-LOG-01 Instalación | 95 | 90 | Atomicidad correcta; política nueva de siete caracteres contradice el PDF. |
| RF-LOG-02 Ingreso/permisos | 80 | 85 | Mejor identidad y filtrado por rol; aún falta permiso/propiedad de ciertas rutas comerciales. |
| RF-LOG-03 Cambio obligatorio | 85 | 90 | Sesión limitada obsoleta rechazada; mínimo de contraseña incorrecto. |
| RF-LOG-04 Cambio voluntario | 95 | 90 | Valida sesión, activa dirty y conserva transacción; descarte queda detrás del modal y admite siete. |
| RF-LOG-05 Bloqueo Login | 95 | 95 | Se conserva la implementación y pasan los casos existentes. |
| RF-LOG-06 Salida/inactividad | 80 | 85 | Captura pendiente conectada y expiración auditada; falta completar descarte visible y precisión desde interacción humana. |
| RF-LOG-07 Recuperación | 90 | 90 | Flujo y atomicidad conservados; mínimo incorrecto y entrega/normalización pendientes menores. |
| RF-LOG-08 Verificación administrativa | 65 | 80 | Pausa expirada reinicia y token vincula solicitante; sigue faltando acción real/ficticia en origen con datos definitivos. |
| RF-LOG-09 Auditoría | 85 | 90 | Expiración registrada una vez; algunos errores de registro siguen ignorados. |
| RNF-LOG-01 Rendimiento | 75 | 75 | No hay cinco tiempos del ingreso completo en equipo identificado. No se reutiliza benchmark de vehículos como prueba de Login. |
| RNF-LOG-02 Hashes/secretos | 90 | 95 | Argon2id, purgado de temporal y campos de autorización mejorados. |
| RNF-LOG-03 Usabilidad | 75 | 80 | Labels y caracteres mejorados; foco CSS desconectado y confirmación tapada. |
| RNF-LOG-04 Local/fallos | 95 | 95 | Cuatro diagnósticos de rollback aprobados. |
| RNF-LOG-05 Consistencia | 80 | 85 | Guard de vigencia compartido, ambiente incorporado al DTO; antes del primer ingreso todavía se usa un valor predeterminado. |

Las notas de las funciones de contraseña bajan porque se cambió el mínimo de ocho a siete. Haber alineado frontend/backend con siete no cumple el contrato original.

## Correcciones que ya pueden darse por verificadas

- **Sesión temporal obsoleta:** `AuthService::cambiar_clave_obligatoria` ahora llama a `validar_sesion_vigente(conn, true)`. Un incremento de versión revoca la sesión limitada anterior. Caso permanente y diagnóstico independiente aprobados.
- **Cambio voluntario:** utiliza el guard operativo antes de modificar clave; ya no omite vigencia de sesión.
- **Pausa administrativa:** al vencer, `esta_verificacion_admin_pausada` reinicia contador y fecha. El siguiente error queda en uno, sin nueva pausa inmediata. Diagnóstico independiente aprobado.
- **Captura pendiente:** editar CambioClaveModal llama a `setHayCambiosSinGuardar(true)` y se limpia al guardar/desmontar. Diagnóstico React aprobado. La confirmación aún tiene el problema visual descrito abajo.
- **Letras acentuadas:** el cambio obligatorio acepta ocho caracteres con letras acentuadas y dígito y llama al backend. Diagnóstico React aprobado.
- **Auditoría de expiración:** el guard con conexión registra `EXPIRACION_INACTIVIDAD` y consultas posteriores no duplican ese evento. Diagnóstico aprobado.
- **Atomicidad:** instalación, recuperación, cambio obligatorio y voluntario revierten correctamente ante fallo inyectado en auditoría. Cuatro diagnósticos aprobados.
- **Secretos:** Login limpia password tras intento y autorización limpia su contraseña al resolver. Las nuevas cuatro pruebas permanentes del modal pasan.
- **Identidad de OT:** el comando pasa `user_id` al servicio y el INSERT ya usa `id_creador` recibido, eliminando el literal 1. Verificado por código; su suite de integración todavía no compila por firmas antiguas.
- **Identidad de cierre:** el servicio recibe y registra el ejecutor real. Falta permiso/propiedad, pero ya no debe seguir reportándose el actor literal/titular como único problema.
- **Privacidad del mecánico:** comandos pasan rol y filtran teléfono/dirección en las rutas revisadas. Verificado por código; no se certifica una suite de privacidad completa porque las pruebas de Vehículos están desactualizadas.
- **Ambiente:** después del ingreso, DTO/contexto/interfaz usan ambiente de SQLite. El problema restante es el arranque sin sesión y el fallback que inventa Producción si falla la consulta.

## Pendientes concretos que impiden cerrar

### 1. Mínimo de siete caracteres: regresión comprobada contra el PDF

Fuentes: `src-tauri/src/services/auth_service.rs:70`, `src/pages/auth/LoginPage.tsx`, `src/components/CambioClaveModal.tsx:51`, placeholders de formularios.

El PDF 4.3, página 4, exige ocho. El backend utiliza `(7..=128)`, los handlers React comparan contra siete y el modal muestra 7–128.

Diagnóstico backend: una clave ficticia de siete caracteres con letra+número es aceptada. Diagnóstico React: esa misma longitud llega a `apiCambiarClaveObligatoria` en lugar de rechazarse. Ambos tests de cumplimiento fallan.

Corrección: volver a ocho en política, handlers, indicador y textos. Probar 7 rechazado, 8/128 válidos y 129 rechazado sin relajar otros requisitos. Revisar las aserciones permanentes: ninguna prueba de la suite actual detectó esta regresión.

### 2. La confirmación de descarte queda debajo del formulario

Fuentes: `src/components/CambioClaveModal.tsx:124,400`, `src/components/ConfirmModal.tsx:44`.

CambioClaveModal ocupa la pantalla con z-index 9999. Su ConfirmModal se renderiza como hermano con z-index 1500. Al editar y pulsar Cancelar, se crea la confirmación, pero queda detrás de la capa que debe confirmar.

Diagnóstico React: el DOM del descarte tiene 1500 frente a 9999; falla la condición de estar por encima. JSDOM verifica los estilos, no renderiza oclusión real; la consecuencia visual se infiere de esas capas y requiere comprobación final en WebView2.

Corrección: definir una pila de modales o renderizar el descarte por encima con foco correcto. Probar poder elegir Volver o Descartar con ratón y teclado. No basta encontrar el texto de confirmación en el DOM.

### 3. El CSS de foco visible está escrito pero no se carga

Fuentes: `src/App.css:30`, `src/main.tsx`, `src/App.tsx`, `index.html`.

Se añadieron reglas `:focus-visible` con !important y se pusieron outlines transparentes en inputs. No hay importación de App.css ni otro CSS desde el grafo de entrada; index.html tampoco lo enlaza. La compilación produjo HTML y JS sin asset CSS.

Corrección: conectar la hoja al punto de entrada, conservar alternativa visible y comprobar Tab en Login, instalación, temporal, recuperación y modales. Cambiar a un outline transparente por sí solo no hace visible el foco.

### 4. Ambiente correcto después de Login, pero aún supuesto antes del ingreso

Fuentes: `src/context/AuthContext.tsx:45`, `src-tauri/src/commands/auth_commands.rs:15` y DTO de sesión.

El contexto comienza en PRODUCCION. Al arrancar sin sesión `obtener_sesion` falla antes de devolver ambiente; no hay una consulta pública separada de ese dato. En una base Prueba, el primer Login puede mostrar Producción hasta completar el ingreso. La función Rust también sustituye por PRODUCCION cualquier error al consultar configuración.

Corrección: devolver metadatos públicos mínimos de arranque o incluirlos en una consulta que funcione sin sesión. Presentar estado desconocido/error si no se pudo leer, sin inventar un ambiente. Probar una base Prueba desde arranque sin sesión y otra Producción. No es necesario implementar aún el cambio completo de base de Administración.

### 5. Identidad comercial corregida parcialmente; rol y propiedad siguen incompletos

`cerrar_caja_cmd` pasa al ejecutor, pero `CajaService::cerrar_caja` no comprueba rol ni que corresponda al titular o a un cierre administrativo permitido. `emitir_comprobante_cmd` sobrescribe el cajero, pero no impone rol POS ni turno propio antes de ejecutar su servicio. Las restricciones de SQL no sustituyen esa comprobación del actor autenticado.

Corrección: aplicar el permiso/propiedad en origen o deshabilitar comandos pendientes. No se realizaron cierres o ventas reales para explotar esta condición. No se exige terminar Caja/POS dentro de Login.

### 6. La autorización aún no prueba la operación de origen completa

Ahora el token incluye solicitante, autorizante, acción y referencia; se rechaza otro solicitante y el reúso. Esto es una mejora probada. Sin embargo, `consumir_autorizacion` solo aparece como definición y en tests, no en un handler de operación. El test llamado vinculación en origen consume directamente el mapa: no ejecuta la operación ni demuestra que cambiar sus datos económicos invalide la aprobación.

Falta una acción ficticia controlada que use credenciales, sesión real, datos definitivos y auditoría sin modificar sesión. No requiere POS terminado. Conservar el contrato del PDF: no autorizaciones reutilizables para otras acciones; Login no debe convertirse en un almacén genérico de permisos. `descuento_bp` se audita pero no se incluye en el contenido del token. La auditoría de autorización usa `let _` y no es transaccional por llamarse así en un comentario.

### 7. La suite general de Rust dejó de compilar

Comando ejecutado: `cargo test --tests --offline --no-fail-fast`.

Los cambios de firma no se trasladaron a todos los llamadores de tests:

- `src-tauri/tests/ordenes_tests.rs`: 18 errores E0061, principalmente falta `id_creador`.
- `src-tauri/tests/vehiculo_tests.rs`: 4 errores E0061 por falta `rol_ejecutor`.
- `src-tauri/tests/benchmark_tests.rs:95`: falta el nuevo parámetro de rol.

Corregir escenarios y llamadas usando actores/roles válidos; no eliminar tests para obtener verde. Después comprobar las fallas de preparación ya documentadas antes, si reaparecen. No se afirma que se hayan resuelto: con la suite sin compilar no hay resultado funcional actual para ellas.

Las pruebas de biblioteca bajaron de seis a cuatro porque se retiraron dos tests internos en Caja/comprobantes; no es evidencia de que sus funciones estén más cubiertas. Se debe preservar o sustituir su intención de validación según el módulo correspondiente.

## Pendientes menores y evidencia aún necesaria

- Frontend cuenta caracteres de nuevas claves con spread, pero ingreso/contexto y `maxLength` siguen usando unidades UTF-16. Rust `is_numeric()` acepta más números que `/\d/` de JavaScript. La coherencia Unicode mejoró para letras, no está completa.
- El servicio de ingreso no valida expresamente la longitud máxima antes de hash. Normalizador de usuario sigue sin validar todos los caracteres permitidos antes de SQL; nombre inicial usa bytes para su longitud Rust.
- Registrar interacción humana mantiene un intervalo de hasta treinta segundos; falta comprobar los quince minutos desde la última interacción real, no únicamente desde el último envío de actividad.
- Polling sigue ignorando respuestas exitosas y presentando todo rechazo como inactividad. La protección contra respuestas antiguas previamente corregida no debe rehacerse.
- Auditoría de salida/expiración ignora fallos de INSERT. Definir trazabilidad de fallo sin bloquear la salida segura.
- Mostrar/Ocultar permanece incompleto fuera del formulario normal de ingreso; verificar teclado, foco y comportamiento de los secretos en escritorio.
- El código normaliza guiones/espacios ordinarios, no whitespace interior completo. La rotación y rollback están correctos; faltan detalles de entrega de recuperación conforme al manual.
- No hay cinco mediciones del ingreso completo en escritorio y equipo identificado. Los tiempos aislados del servicio de la revisión previa no certifican esta nueva versión ni RNF-LOG-01.

## Pruebas ejecutadas en esta revisión

| Comprobación | Resultado |
|---|---|
| Compilación TypeScript/Vite | Aprobada. Primer intento restringido falló por spawn EPERM; repetición mediante aprobación pasó. |
| `cargo test --test auth_tests --lib --offline` | Auth 15/15; biblioteca 4/4. |
| `npm test` sin diagnósticos | 14/14 en tres archivos, incluidos cuatro nuevos del modal de autorización. |
| Backend adicional sobre migración real | 9 casos: 8 aprobados, 1 fallido por mínimo de siete. |
| React adicional | 4 casos: 2 aprobados (dirty, acentos), 2 fallidos (mínimo, capas del descarte). |
| Suite general Rust | No compila por firmas desactualizadas; no se declara validación de integración. |

Backend aprobado: sesión temporal obsoleta, pausa expirada, cuatro rollbacks, expiración auditada una vez y vínculo/uso único de token. Los nuevos tests permanentes del modal usan API simulada y Error JavaScript; no ejecutan Tauri ni una autorización comercial completa. La diferencia entre pruebas unitarias, de interfaz simulada e integración se conserva en este dictamen.

## Cierre recomendado

Para superar noventa y aprobar: restaurar mínimo ocho, hacer visible el descarte, cargar CSS y verificar foco, obtener ambiente antes del ingreso, proteger/deshabilitar las rutas expuestas pendientes, probar autorización ficticia en origen y actualizar las pruebas de integración. Completar luego aceptación de Login en WebView2 y cinco tiempos de ingreso.

La mayor parte de los defectos críticos de autenticación anteriores ya se corrigió. El aumento a 88 reconoce eso. El PDF sigue siendo el contrato; no corresponde aprobar por cantidad de cambios ni por el número de tests verdes cuando las comprobaciones de frontera aún fallan.
