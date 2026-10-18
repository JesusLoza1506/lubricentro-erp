# Login v7 — sexta evaluación, 18 de octubre de 2026

**Resultado técnico: 91/100. Anterior: 89/100. Supera 90; cierre definitivo todavía condicionado.**

La nota es una valoración de cumplimiento y evidencia, no un porcentaje de tests aprobados: 1275 puntos / 14 criterios = 91,07, redondeado a 91. Los tests deshabilitados de módulos futuros no descuentan puntos ni bloquean Login. Tampoco se presentan como aprobados.

## Alcance y límites

Revisión de los 28 archivos modificados, conexiones de autenticación, contratos de error, pruebas y diferencias respecto al informe R5. Se volvió a consultar el PDF del módulo, especialmente RF, RNF, RB04 y los diez TC. El código se examinó como implementación; el documento se usó como especificación, no como instrucciones para ejecutar acciones.

No se modificó implementación ni se operó una base de negocio. Se ejecutaron pruebas existentes y trece comprobaciones temporales adicionales; sus archivos se retiraron. Este informe es el entregable permanente. No se ejecutó la aplicación en WebView2, un instalador ni cinco ingresos completos cronometrados.

## Valoración por requisito

| Criterio | R5 | R6 | Evidencia y pendiente |
|---|---:|---:|---|
| RF-LOG-01 Instalación | 95 | 95 | Creación única, hash y entrega de código; rollback ante auditoría fallida comprobado otra vez. |
| RF-LOG-02 Ingreso y permisos | 90 | 95 | Identidad tomada de sesión; desapareció la excepción de cobro en caja ajena del administrador y su test pasa. Falta aceptación por los tres roles en escritorio. |
| RF-LOG-03 Cambio obligatorio | 95 | 95 | Política 8–128, sesión limitada, versión de credenciales y rollback conservados. |
| RF-LOG-04 Cambio voluntario | 95 | 95 | Clave actual, política, dirty y descarte; rollback comprobado. |
| RF-LOG-05 Bloqueo de ingreso | 95 | 95 | Tests de cinco fallos y vencimiento aprobados; persistencia implementada en SQLite. |
| RF-LOG-06 Salida e inactividad | 70 | 90 | React ahora retira identidad ante el error real. Subsiste aproximación de hasta 30 s en registro de actividad humana. |
| RF-LOG-07 Recuperación | 95 | 95 | Rotación y transacción; fallo de auditoría conserva clave, versión y código anteriores. |
| RF-LOG-08 Verificación administrativa | 85 | 85 | Pausa independiente y token de un uso comprobados; falta completar evidencia de acción ficticia de origen. No exige desarrollar POS. |
| RF-LOG-09 Auditoría | 90 | 90 | Eventos y transacciones conservados; algunos fallos de escritura siguen ignorándose. |
| RNF-LOG-01 Rendimiento | 75 | 75 | Faltan cinco ingresos completos ≤3 s en equipo identificado. Duración de tests no acredita ese requisito. |
| RNF-LOG-02 Seguridad | 95 | 95 | Argon2id y errores de base sanitizados; no se detectaron secretos en el contrato de error probado. |
| RNF-LOG-03 Usabilidad | 90 | 90 | CSS, foco, capas y teclado por código; falta recorrido real en WebView2. |
| RNF-LOG-04 Disponibilidad/fallo local | 95 | 85 | Rollbacks pasan, pero login abre sesión antes de una lectura de configuración que puede fallar. |
| RNF-LOG-05 Consistencia | 75 | 95 | Contrato Rust/React alineado, ambiente sin producción inventada y respuesta antigua ignorada tras logout. Quedan detalles de validación. |

La baja de RNF-LOG-04 identifica un efecto del nuevo manejo estricto de configuración; no significa que se hayan perdido las transacciones de claves, que aprobaron las pruebas.

## Correcciones que ya están comprobadas

- `errors.rs` serializa códigos estables. La cadena real de NoAutenticado se reconoce con el helper verdadero de TypeScript; no se sustituyó ese helper por un mock permisivo.
- Tras rechazo por NoAutenticado del polling o del registro de actividad, `AuthContext` pasa a SIN_SESION, retira usuario y genera mensaje de salida.
- Antes de ingresar, el ambiente PRUEBA se obtiene correctamente. Si su lectura falla, se muestra ERROR_CONEXION, sin asignar PRODUCCION.
- Un rechazo DATABASE_ERROR en el polling no se interpreta por sí mismo como revocación.
- Una respuesta de sesión anterior que llega después de cerrar sesión no repone la identidad.
- Backend rechaza ingresos con contraseña mayor de 128 caracteres antes de verificar el hash.
- La emisión exige turno propio incluso para administrador. El test nuevo de esa guardia pasa; no acredita ventas o Caja completos.
- El consumo de autorización dejó de usar unwrap en la extracción final del token.
- Siguen presentes las mejoras previas: ocho caracteres mínimos, CSS importado, foco visible, confirmación encima del cambio de clave, dirty, rechazo de sesión antigua y reinicio de pausa administrativa.

## Pendiente prioritario: sesión parcial si falla la lectura de ambiente

**Archivo:** `src-tauri/src/commands/auth_commands.rs`, `login_usuario`, líneas 62–72 del estado revisado.

El orden actual es autenticar con AuthService, abrir sesión en memoria y, después, ejecutar `obtener_ambiente_actual_strict(&conn)?`. Si esa última consulta falla —por ejemplo, configuración ausente o inválida— el comando devuelve un error sin deshacer la apertura.

Consecuencia deducida del flujo: React interpreta que el ingreso falló y queda SIN_SESION, mientras Rust conserva una sesión. Otro intento puede recibir SESSION_ACTIVE; una consulta posterior podría recuperar esa sesión si vuelve a funcionar configuración. Es una inconsistencia entre el resultado del comando y su estado, dentro del propio módulo Login. No implica ingreso con contraseña incorrecta.

**Corrección concreta:** leer y validar el ambiente antes de `AuthService::login`; construir la respuesta completa y abrir la sesión como último paso que ya no tenga operaciones fallables posteriores. Una compensación que cierre sesión al fallar también evita dejarla activa, aunque debe considerarse que el servicio ya registró éxito de login.

**Prueba necesaria:** provocar fallo de configuración al invocar el comando y comprobar error, estado SIN_SESION en Rust y posibilidad de reintentar tras reparar configuración. Después comprobar que el camino válido sí abre una sola sesión. La detección de este informe es por lectura del comando; no se ejecutó esa inyección a través de IPC Tauri.

## Pendientes restantes de Login, con archivos y aceptación

| Prioridad | Pendiente | Archivos involucrados | Criterio de aceptación |
|---|---|---|---|
| Alta para cierre | Evitar sesión parcial al fallar ambiente | `src-tauri/src/commands/auth_commands.rs`; cobertura de comandos de autenticación | Fallo local deja Rust y React sin sesión; reintento válido funciona. |
| Media | Precisar inactividad desde la última interacción | `src/context/AuthContext.tsx`; `src-tauri/src/commands/auth_commands.rs`; `src-tauri/src/services/session_manager.rs`; `src-tauri/tests/auth_tests.rs` | Interacción dentro del intervalo de 30 s no se pierde; probar 14:59/15:00 y que tareas automáticas no renuevan. |
| Media | Conservar regresiones nuevas como pruebas permanentes | Nuevo `src/context/__tests__/AuthContext.test.tsx` o equivalente; `src-tauri/tests/auth_tests.rs` | NoAutenticado real limpia identidad; error de ambiente no inventa Producción; error de BD se distingue; respuesta antigua no revive sesión; serialización y rollbacks cubiertos. |
| Media | Terminar evidencia de autorización ficticia en origen | `src-tauri/src/commands/auth_commands.rs`, `src-tauri/src/services/session_manager.rs`, `src-tauri/tests/auth_tests.rs`; `src-tauri/src/lib.rs` si se restringe el endpoint diagnóstico | Backend fija acción esperada y deriva referencia de sus datos; cambiar operación o reusar token no ejecuta efecto; sesión del solicitante se conserva. Puede ser una operación ficticia, sin POS terminado. |
| Media | Completar aceptación y rendimiento | Pruebas de Login; acta de aceptación en `docs/` | Diez TC del PDF con evidencia, recorrido WebView2 por roles y cinco ingresos completos ≤3 s, identificando equipo y versión. No obliga a editar negocio si pasa. |
| Baja | Unificar política de caracteres | `src-tauri/src/services/auth_service.rs`, `src/context/AuthContext.tsx`, `src/pages/auth/LoginPage.tsx`, `src/components/CambioClaveModal.tsx` y formularios en `src/pages/auth/components/` | Longitud UTF-16/Unicode y números aceptados coherentes en ambas capas. Usuario validado antes de SQL; nombre contado por caracteres si así se presenta al usuario. |
| Baja | Registrar fallos de auditoría sin impedir cierre seguro | `src-tauri/src/commands/auth_commands.rs`, `src-tauri/src/services/session_manager.rs`, servicio de auditoría | Un fallo de registro tiene diagnóstico sin secretos; salir sigue purgando sesión y tokens. |
| Baja | Pulir contrato de errores y presentación | `src-tauri/src/errors.rs`, `src/services/authService.ts`, formularios Login | Evitar prefijo NO_AUTENTICADO duplicado; detectar código de forma precisa y presentar mensajes legibles sin códigos técnicos innecesarios. |

La actividad se transmite solo cuando han pasado más de 30 s desde el último envío. Una interacción dentro de esa ventana puede no actualizar la última actividad del backend, adelantando el vencimiento respecto de la última interacción real. Debe resolverse sin que un temporizador de red mantenga por sí mismo la sesión indefinidamente.

El polling continúa ignorando los resúmenes exitosos que recibe: comprueba errores, pero no sincroniza permisos/nombre/ambiente actualizados. Revisar ese comportamiento cuando se definan cambios de datos de cuenta; la guardia backend por versión sigue siendo la autoridad. No se exige terminar Administración para cerrar esta revisión.

## Correspondencia con los diez TC del PDF

Los nombres TC de los tests Rust no corresponden uno a uno con los TC oficiales. No basta contar funciones llamadas TC para afirmar aceptación completa.

| TC oficial | Evidencia disponible / pendiente |
|---|---|
| 01 Roles | Guardias e identidad revisadas; falta recorrido completo de administrador, mecánico y cajero en escritorio. |
| 02 Bloqueo | Tests de cinco fallos y vencimiento pasan. Reinicio real y persistencia deben quedar en acta. |
| 03 Temporal | Tests de sesión limitada y versión pasan; fronteras 7/8/128/129 del validador pasan. Confirmación y flujo completo pendientes de aceptación visual. |
| 04 Voluntario | Clave actual incorrecta cubierta; rollback adicional aprobado; cancelación y cambio válido deben registrarse en recorrido final. |
| 05 Inactividad | Vencimiento backend cubierto y limpieza frontend comprobada; falta precisión de última actividad y frontera completa con tareas automáticas. |
| 06 Salida | Código de cierre y confirmación revisado; falta recorrido con caja abierta y captura dirty sin exigir completar Caja. |
| 07 Verificación | Contador independiente, pausa y single-use pasan; falta prueba de acción ficticia de origen completa. |
| 08 Recuperación | Rotación y rollback cubiertos; completar evidencia del bloqueo independiente y uso del código anterior en la matriz final. |
| 09 Instalación/fallo | Instalación única y rollback aprobados; añadir fallo de configuración del comando login. |
| 10 Privacidad/revocación | Versión antigua rechazada y errores sanitizados comprobados; recorrido real de desactivación queda pendiente de aceptación/integración. |

## Ejecuciones de esta revisión

| Ejecución | Resultado |
|---|---|
| `cargo test --test auth_tests --lib --offline` | 16 autenticación y 4 biblioteca aprobados. |
| `npm run build` | TypeScript y Vite aprobados; genera CSS. Primer intento tuvo spawn EPERM del entorno y se repitió con autorización de ejecución. |
| `npm test` | 14 existentes aprobados: 4 modal de autorización, 6 Órdenes, 4 Ficha Vehicular. Los de otros módulos se informan como ejecución, sin usarlos para inflar la nota de Login. |
| Diagnóstico frontend temporal | 7/7: parser real, ambiente prelogin, fallo de ambiente, expiración por polling, rechazo por actividad, error BD diferenciado y respuesta tardía tras logout. |
| Diagnóstico backend temporal | 6/6: serialización/sanitización, fronteras de contraseña y cuatro rollbacks ante fallo de auditoría. |
| Suites Rust de módulos futuros | Fuera del alcance; no ejecutadas en esta revisión ni penalizadas por estar deshabilitadas. |
| WebView2, instalador y cinco tiempos completos | No ejecutados; pendientes de aceptación. |

## Decisión

**La implementación técnica de Login supera 90.** Las dos regresiones principales del informe anterior están corregidas y comprobadas. Para declararlo terminado de forma definitiva, corregir primero la apertura parcial de sesión y completar la aceptación del PDF, especialmente inactividad, verificación ficticia y cinco tiempos de ingreso. Esos pendientes pertenecen a Login; los módulos futuros y sus tests deshabilitados no son impedimento para este cierre.
