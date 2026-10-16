# Evaluación de Login y Sesión v7

Fecha de revisión: 15 de octubre de 2026. Código local del proyecto, incluidos cambios sin commit. Aplicación declarada 0.1.0, especificación funcional 7.0.

**Resultado: 60/100. No aprobado todavía para cerrar el primer módulo.**

La puntuación es el promedio simple de las catorce valoraciones RF/RNF de este informe: 840/14 = 60. Es una valoración técnica del cumplimiento observado, no un porcentaje estadístico de pruebas aprobadas. Un fallo que permita omitir autenticación o cambiar una contraseña por una ruta incorrecta impide aprobar aunque el promedio aumente.

## Alcance y límites

Referencia principal: `01_Modulo_Login_y_Sesion_v7_Simplificado.pdf`, once páginas. El Informe General v7, quince páginas, se utiliza para concordancia de reglas comunes. Ambos se leyeron completos.

Se revisaron interfaz React, contexto de autenticación, servicios TypeScript, DTO, comandos Tauri registrados, servicio Rust, gestor de sesión, auditoría, esquema SQL y pruebas. Se ejecutó compilación frontend, suite existente de autenticación y pruebas frontend existentes. Se añadieron y retiraron comprobaciones temporales para reproducir defectos; no se corrigió la implementación.

No se abrió la aplicación de escritorio ni se operó sobre su base de negocio. No se certifican el instalador, la experiencia visual real en WebView2, la base actualmente usada por una instalación, ni un recorrido completo con teclado en Windows. Las pruebas adicionales de backend utilizaron el esquema v7 real en SQLite en memoria.

La revisión no exige terminar POS, Caja, Backup o Administración para aprobar Login. Sí exige que el contrato de sesión sea seguro y que los comandos ya expuestos no permitan evadirlo. La creación de cuentas y la emisión de temporales pertenecen a Administración; para probar Login pueden prepararse cuentas ficticias. Una operación ficticia basta para probar la verificación administrativa.

## Puntuación por requisito

| Requisito | Nota | Resultado observado |
|---|---:|---|
| RF-LOG-01: primer administrador | 90 | Alta transaccional, sin credenciales por defecto, clave definitiva y código entregado. Validaciones y manejo de secretos en interfaz requieren ajustes. |
| RF-LOG-02: ingreso y permisos | 35 | Credenciales, estado activo y bloqueo se comprueban en Login. Existen comandos registrados que aceptan identidad del frontend o carecen de sesión. |
| RF-LOG-03: cambio obligatorio | 65 | Dos campos nuevos y transición correcta en flujo normal. El comando también acepta sesiones normales; la restricción de negocio no cubre todos los comandos. |
| RF-LOG-04: cambio voluntario | 40 | Servicio y modal existen. El modal no está importado ni montado por la aplicación; falta acceso visible. Además, otra ruta permite omitir la clave actual. |
| RF-LOG-05: bloqueo de ingreso | 85 | Cinco errores, quince minutos y estado persistido; intentos durante bloqueo no amplían plazo. Se ignoran errores de escritura; no se ejecutó reapertura de archivo para probar reinicio. |
| RF-LOG-06: salida e inactividad | 45 | Sesión en memoria, salida directa y control de 900 s. Falta confirmación por captura modificada y falla el retorno desde cambio obligatorio. |
| RF-LOG-07: recuperación | 70 | Código y administrador activo; cambio y rotación transaccionales. No normaliza espacios interiores y el contador no se limpia al vencer su bloqueo. |
| RF-LOG-08: verificación administrativa | 25 | Comprueba administrador y conserva sesión. El límite independiente no está conectado; hay aprobaciones almacenadas en Login y falta integración en origen. |
| RF-LOG-09: auditoría | 55 | Registra accesos, bloqueos y cambios sin claves/hashes en DTO ordinarios. No registra salida; errores de auditoría se silencian. |
| RNF-LOG-01: rendimiento | 70 | Cinco ingresos consecutivos de servicio entre 664 y 795 ms en una ejecución secuencial. Falta medir ingreso completo en escritorio e identificar hardware de referencia. |
| RNF-LOG-02: hash y secretos | 90 | Argon2id con sal aleatoria para contraseña y código; respuestas ordinarias sin hashes. Persisten secretos de formularios al cambiar ciertas vistas. Esta fila evalúa el requisito criptográfico, no toda la seguridad del módulo. |
| RNF-LOG-03: usabilidad | 55 | Formularios, foco inicial, Enter, botones de carga y Mostrar en Login. Foco visual deshabilitado en estilos y cambio voluntario inaccesible. |
| RNF-LOG-04: operación local y fallos | 65 | Autenticación sin HTTP; creación/recuperación transaccionales. Login puede informar éxito aunque falle una escritura necesaria. |
| RNF-LOG-05: consistencia | 50 | Sesión única en Rust, bloqueo en BD, fechas UTC−05:00. Revocación incompleta, respuestas tardías sin generación y actividad humana registrada con desfase de hasta casi 30 s. |

## Hallazgos que impiden cerrar Login

### H01. Comandos existentes permiten evadir la sesión — crítico

Evidencia por lectura de código:

- `src-tauri/src/commands/inventario_commands.rs:12`: `verificar_permiso_rol` recibe `id_usuario` desde el llamador. No consulta `SessionState`. Por ejemplo, `listar_productos_cmd`, línea 56, usa ese ID para decidir si devolver datos administrativos.
- `src-tauri/src/commands/cliente_commands.rs:12`: aplica el mismo patrón.
- `src-tauri/src/commands/dashboard_commands.rs:10`: recibe el ID desde React. Alertas y backup tampoco exigen sesión en sus comandos.
- `src-tauri/src/commands/comprobante_commands.rs:9`: emisión sin argumento de estado de sesión.
- Todos están expuestos en `src-tauri/src/lib.rs` mediante `generate_handler!`.

Un ID existente y activo demuestra que existe una cuenta, pero no que el llamador haya iniciado sesión con ella. La omisión también impide exigir inactividad y cambio obligatorio en esas rutas. No se ejecutaron escrituras comerciales para explotar estas rutas; el defecto está verificado en su control de entrada.

Contrato incumplido: sección 3.3, RB01, RB04 y 7.4. Aunque los módulos posteriores estén en desarrollo, sus comandos expuestos deben quedar protegidos o deshabilitados antes de considerar seguro Login.

### H02. Cambio de clave sin acreditar la actual en sesión normal — crítico

`src-tauri/src/commands/auth_commands.rs:114` comprueba existencia/inactividad, pero no que el estado sea CAMBIO_OBLIGATORIO. `src-tauri/src/services/auth_service.rs:513` tampoco comprueba ese estado ni la bandera de cambio pendiente antes de actualizar el hash.

Reproducción ejecutada: abrir sesión NORMAL, llamar al servicio de cambio obligatorio con una clave nueva válida. Esperado: rechazo. Observado: cambio exitoso sin contraseña actual.

El comando expuesto contiene la misma omisión. La ruta destinada a reemplazar una temporal evita las condiciones del cambio voluntario. La solución requiere validar el estado y la cuenta en backend.

### H03. El límite administrativo no se aplica — alto

`src-tauri/src/services/session_manager.rs:159` contiene métodos de pausa/contador. `AuthService::autorizar_descuento`, línea 610, recibe `_session` y no los usa. La prueba existente solo manipula el contador directamente.

Reproducción ejecutada: cinco verificaciones administrativas erróneas y una correcta inmediatamente después. Esperado: pausa de quince minutos. Observado: autorización aceptada. El contador de Login permanece separado, pero la protección independiente requerida por RB03 no opera.

### H04. Cambiar rol no revoca la sesión — alto

`SessionState` conserva identidad y rol, pero no versión de cuenta ni una generación de credenciales comparada contra BD. `AuthService::obtener_usuario_sesion`, línea 334, consulta activo y cambio obligatorio; devuelve una sesión válida con el rol nuevo después de un cambio de rol.

Reproducción ejecutada sobre esquema real: preparar dos administradores, cambiar el rol del primero a CAJERO y consultar su sesión anterior. Esperado por RB09: acceso invalidado y nuevo ingreso. Observado: sesión válida. Se prepararon dos administradores para respetar la protección SQL del último administrador.

Además, `validar_sesion_operativa`, línea 139, solo consulta memoria. Los comandos de OT consultan activo/rol, pero no la bandera de cambio obligatorio de BD. El polling React llama a `obtener_sesion` y descarta los cambios de una respuesta exitosa. No basta con mantener un método `invalidar_sesion_usuario` sin conectarlo al ciclo real de cambios y a la validación de cada comando.

### H05. Cambio voluntario no disponible para el trabajador — alto

Existe `src/components/CambioClaveModal.tsx:24`, con validación y servicio. La búsqueda de referencias solo encuentra su definición: no se importa ni monta. `Sidebar.tsx` ofrece cerrar sesión y muestra nombre/rol, pero no permite cambiar contraseña. `App.tsx` tampoco lo integra.

Comprobación React ejecutada: renderizar navegación de sesión normal y buscar el botón para cambiar contraseña. Observado: ausente. La existencia de un modal aislado no satisface RF-LOG-04.

### H06. Salir de cambio obligatorio deja el formulario anterior — alto

`src/pages/auth/LoginPage.tsx:76`: el efecto cambia a CAMBIO_OBLIGATORIO, pero al pasar a SIN_SESION solo comprueba instalación. Si ya existen cuentas no vuelve a poner `modo = LOGIN`.

`MainApp` devuelve el mismo tipo de componente `LoginPage` tanto durante cambio obligatorio como sin sesión. React conserva la instancia y su estado local en esa transición.

Comprobación React ejecutada: pasar de CAMBIO_OBLIGATORIO a SIN_SESION con instalación existente. Esperado: formulario de ingreso. Observado: sigue el cambio obligatorio. Sus claves permanecen en el estado del componente.

### H07. Cierre sin protección de capturas pendientes — alto

`src/components/Sidebar.tsx` llama directamente a `cerrarSesion`. `AuthContext.tsx:151` limpia usuario/estado sin consultar capturas modificadas. No se encontró un mecanismo compartido de captura pendiente.

La salida directa con caja abierta es correcta. Falta la única excepción exigida: una confirmación cuando existen datos sin guardar. No es necesario implementar Caja para probarla; basta una pantalla ficticia con captura modificada.

### H08. Fallos de persistencia se ignoran — alto

En `AuthService::login`, varias actualizaciones de contador/bloqueo y limpieza usan `let _ = conn.execute(...)`, por ejemplo línea 287. También se ignoran errores de auditoría.

Reproducción ejecutada sobre esquema real: crear un trigger de diagnóstico que rechace UPDATE de usuarios e ingresar con credenciales válidas. Esperado: fallo local controlado. Observado: Login devuelve éxito aunque no se guardó el estado. La actualización fallida no impide al comando abrir sesión.

Las transacciones de instalación y recuperación sí protegen sus escrituras principales. Debe extenderse el manejo correcto de fallos a Login y definir la consistencia con auditoría.

## Otros incumplimientos y puntos pendientes

1. **Código con espacios:** `normalizar_codigo`, línea 66, elimina guiones y espacios extremos; conserva espacios interiores. Sustituir los guiones de un código válido por espacios hace fallar recuperación. Reproducido.
2. **Bloqueo de recuperación vencido:** al vencer no se limpia `recuperacion_fallos`. Con cinco fallos previos, el siguiente fallo vuelve a bloquear inmediatamente quince minutos. Reproducido. La especificación fija cinco fallos por bloqueo; conviene hacer explícito el reinicio del contador de este flujo.
3. **Pausa administrativa en memoria:** además de no usarse, el método que suma fallos conserva el contador de una pausa vencida. Al conectarlo debe corregirse su ciclo de expiración.
4. **Longitudes Unicode:** política Rust usa `String::len()` en bytes. Una contraseña válida de 128 caracteres con letras acentuadas puede rechazarse. Reproducido. La interfaz cuenta unidades UTF-16 y el backend bytes; deben acordar caracteres. SQL sí valida el formato permitido de usuario; el normalizador Rust no lo valida completamente y deja llegar errores de restricciones a la interfaz.
5. **Errores técnicos visibles:** `src-tauri/src/errors.rs:19` serializa el error SQLite completo. Los formularios muestran la cadena recibida. Incumple el mensaje local sin SQL/rutas/trazas requerido en 4.4.
6. **Auditoría de salida:** `cerrar_sesion` solo limpia memoria; no recibe conexión ni registra salida. Tampoco se encontró evento de vencimiento. La ausencia de salida incumple explícitamente 7.4; conviene distinguir salida manual y vencimiento sin generar eventos repetitivos.
7. **Respuestas tardías:** existe `session_id` en Rust, pero no se transmite ni se contrasta en React. Operaciones asíncronas antiguas pueden alterar el estado después de salir o cambiar de usuario. Riesgo comprobado por lectura; no se ejecutó una prueba de concurrencia para esta condición.
8. **Actividad aproximada:** React envía actividad como máximo una vez cada treinta segundos. Si la última interacción ocurre entre envíos, el backend puede vencer antes de quince minutos desde esa última interacción. El polling no renueva actividad, lo cual sí es correcto. No se ejecutó una simulación completa de este desfase.
9. **Contexto de navegación:** `App.tsx:12` conserva módulo y placa seleccionada en `MainApp` al pasar por Login. Un nuevo ingreso no garantiza abrir Inicio ni retirar todo contexto del usuario anterior. Las páginas internas sí se desmontan al salir.
10. **Secretos en vistas:** pasar de Login a Recuperación conserva `passwordInput`; volver desde recuperación sin completar conserva campos; finalizar instalación oculta el código pero no limpia las contraseñas del formulario. No se usa almacenamiento persistente de esos campos, pero deben limpiarse al abandonar/cerrar su formulario.
11. **Foco:** estilos `inputBox` e `inputWithIcon` eliminan `outline`, sin una alternativa de foco visible encontrada. Muchos labels no están asociados al input con `htmlFor`. Mostrar/ocultar está implementado en ingreso, pero no en las otras vistas de Login. Pendiente verificar Tab/Enter y foco en WebView2.
12. **Ambiente:** la base se abre con una única ruta fija en `db.rs` y el esquema inicial crea PRUEBA. Login no muestra el ambiente. No se exige implementar ahora toda Administración, pero la identificación del ambiente y el contrato para invalidar sesión/cambiar base deben quedar definidos y comprobables antes de integrar ese módulo.
13. **Autorizaciones:** Login guarda tokens temporales con acción/referencia. `descuento_bp` llega en la solicitud, pero no se incorpora a la autorización; no se encontró consumo de esos tokens desde comandos de negocio. El PDF exige validación en origen sin aprobaciones conservadas para otras acciones. No se exige un POS terminado: debe probarse el contrato con una operación ficticia, conservando ejecutor/autorizante y datos exactos.
14. **Navegación/versiones:** Sidebar indica v5.0 y carece de entradas de Administración/Reportes, aunque el servicio genera permisos para ellas. Es una integración pendiente de navegación; no se penaliza como si esos módulos completos debieran estar terminados ahora. El número 0.1.0 de la aplicación puede diferir legítimamente del 7.0 del documento; la inconsistencia visible v5.0 sí conviene corregirla.

## Pruebas ejecutadas y significado

| Verificación | Resultado |
|---|---|
| `cargo test --test auth_tests --offline` | 8/8 aprobadas. |
| `npm run build` | Aprobada: TypeScript y compilación Vite. |
| `npm test` antes de añadir diagnósticos | 10/10 aprobadas, en dos archivos de Taller/Ficha. Ninguna prueba frontend existente cubría Login. |
| Diagnósticos temporales Rust sobre esquema v7 real | 1 aprobado y 7 fallidos por incumplimientos reproducidos. |
| Diagnósticos temporales React | 2 fallidos: retorno desde cambio obligatorio y acceso a cambio voluntario. |

Los procesos de Vite/Vitest inicialmente fallaron con `spawn EPERM` dentro del entorno restringido. Se repitieron mediante el mecanismo de aprobación fuera de ese entorno y se obtuvieron los resultados anteriores. Ese primer fallo fue del entorno, no un defecto de la aplicación.

Los siete fallos definitivos de backend fueron: cambio obligatorio desde sesión normal, sesión válida después de cambio de rol, ausencia de pausa administrativa, clave Unicode rechazada, Login exitoso ante UPDATE fallido, código con espacios rechazado y recuperación que vuelve a bloquear tras un solo fallo después del vencimiento. El caso aprobado verifica instalación, cinco ingresos y rotación de recuperación con el esquema real. Una primera preparación del caso de cambio de rol tropezó con la protección del último administrador; se corrigió el escenario creando otro administrador y se confirmó el defecto con esa condición válida.

Tiempos informativos del servicio en ejecución secuencial debug: **795, 692, 709, 664 y 731 ms**. No incluyen React, IPC ni apertura de Inicio; no constituyen la aprobación completa de RNF-LOG-01. No se obtuvo inventario de hardware por restricciones de acceso, por lo que falta identificar el equipo de referencia.

Las pruebas existentes de autenticación crean un esquema reducido a mano. Su comentario dice esquema oficial, pero no cargan la migración completa. Sus nombres TC no equivalen a los diez casos del PDF: por ejemplo, el primer test prueba hash, mientras TC-LOG-01 del documento exige roles y un comando prohibido. El test del hash señuelo solo prueba que no autentica; no verifica igualdad de tiempos. El test de bloqueo no reabre archivo ni comprueba vencimiento, pese a su nombre. El de pausa administra el contador manualmente, sin llamar a la verificación real.

## Estado frente a los diez casos de aceptación del PDF

| Caso documental | Dictamen de esta revisión |
|---|---|
| TC-LOG-01 Roles | No aprobable: comandos con identidad del frontend/sin sesión. |
| TC-LOG-02 Bloqueo | Parcial: cinco errores y rechazo durante bloqueo cubiertos; falta reapertura y límites temporales completos. |
| TC-LOG-03 Temporal | Parcial: flujo de dos campos funciona; falta validar todos los comandos y fronteras 7/8/128/129 con criterios coherentes. |
| TC-LOG-04 Voluntario | No aprobable: formulario inaccesible y ruta alternativa sin clave actual. |
| TC-LOG-05 Inactividad | Parcial: gestor prueba 901 s; falta 14:59/15:00, actividad completa y comandos existentes. |
| TC-LOG-06 Salida | Fallo reproducido desde cambio obligatorio; falta confirmación por datos sin guardar y evidencia de caja intacta. |
| TC-LOG-07 Verificación administrativa | Fallo reproducido: no aplica pausa real después de cinco fallos. |
| TC-LOG-08 Recuperación | Parcial: rotación funciona; fallan espacios y reinicio del bloqueo. |
| TC-LOG-09 Instalación/fallo local | Parcial: alta/duplicación cubiertas; falta fallo transaccional de cambio/recuperación y manejo completo de persistencia. |
| TC-LOG-10 Privacidad/revocación | No aprobable: cambio de rol no revoca; falta cobertura de desactivación/reset y respuestas tardías. |

## Concordancia documental

Login 3.3, página 3, contempla pedir contraseña al administrador en su propia sesión cuando una acción lo exige. El Informe General, páginas 3 y 12, establece gestión administrativa sin repetir contraseña. Debe resolverse la redacción antes de fijar ese comportamiento. No se descontaron puntos por elegir una de esas dos interpretaciones.

Esa discrepancia no afecta la necesidad de acreditar la contraseña actual en el cambio voluntario, de limitar intentos ni de proteger comandos. Tampoco justifica tokens genéricos de aprobación sin integración con una acción concreta.

## Condiciones para pasar al módulo 02

1. Cerrar H01 y H02: ninguna ruta expuesta puede suplir sesión/identidad con datos de React; cambio obligatorio solo para cuenta y sesión elegibles.
2. Conectar el límite real de verificación administrativa y probarlo mediante una operación ficticia; no requiere desarrollar POS.
3. Verificar estado/versiones de cuenta por comando y revocar ante reset, desactivación o cambio de rol.
4. Habilitar cambio voluntario, corregir salida desde cambio obligatorio, confirmar capturas pendientes y limpiar secretos/contexto/respuestas tardías.
5. Propagar fallos locales, presentar errores públicos seguros y completar auditoría de salida y consistencia de eventos.
6. Corregir normalización de código, ciclo de bloqueos, política de caracteres, foco y tratamiento de última actividad humana.
7. Ejecutar los diez TC documentales y RNF aplicables con evidencia real de interfaz/backend/BD. Medir cinco ingresos completos en un equipo identificado. Mantener como integración futura solo lo que depende de módulos todavía no desarrollados, dejando sus contratos seguros.

La base funcional es aprovechable: Argon2id, sesión Rust, alta inicial y recuperación transaccionales y formularios principales ya existen. La recomendación es corregir estos defectos del primer módulo antes de extender el sistema sobre su contrato de autenticación.
