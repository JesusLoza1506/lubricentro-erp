# Reevaluación de Login y Sesión v7 — 16 de octubre de 2026

**Resultado: 75/100. Anterior: 60/100. Mejora: 15 puntos. No aprobado aún para cerrar el módulo.**

Esta revisión contrasta el código local actual, incluidos cambios sin commit, con el PDF de Login v7 y el informe anterior del 15 de octubre. El plan adjunto se leyó como propuesta de cambios a verificar, no como autorización para implementarlos ni como evidencia de cumplimiento. La meta +95 no se asume alcanzada por existir el plan o pasar una suite.

La nota es el promedio redondeado de las catorce valoraciones siguientes: 1045/14 = 74,64. Es una valoración técnica, no el porcentaje de pruebas aprobadas. El criterio de aprobación del PDF exige ausencia de acceso indebido, estados parciales y secretos expuestos; un promedio alto no sustituye ese criterio.

## Alcance y evidencia

Se revisaron nuevamente servicio Rust, SessionState, todos los archivos de comandos registrados, DTO, auditoría, errores, inicialización SQL, AuthContext, App, Sidebar, formularios de Login y cambio voluntario, ConfirmModal, tipos/servicios frontend y pruebas. Se inspeccionaron puntos de integración en Caja, comprobantes, Clientes, Inventario y Taller para comprobar identidad y permisos.

No se abrió la aplicación de escritorio ni se modificaron datos reales. Compilar el frontend y probar componentes con JSDOM no certifica la interfaz en WebView2 ni el instalador. No se exige terminar los módulos futuros; sí proteger las rutas ya expuestas y dejar correcto el contrato de autenticación. Los fallos de suites de otros módulos se reportan aparte y no se trasladan automáticamente a la nota de Login.

## Puntuación actual

| Requisito | Anterior | Actual | Evaluación |
|---|---:|---:|---|
| RF-LOG-01: instalación inicial | 90 | 75 | Flujo normal correcto; fallo de auditoría posterior al commit crea cuenta sin entregar código. |
| RF-LOG-02: ingreso/permisos | 35 | 75 | Los comandos ahora exigen sesión; persisten permisos/identidad incompletos en ciertas rutas y reset no revoca. |
| RF-LOG-03: temporal/cambio obligatorio | 65 | 85 | Se cerró la ruta desde sesión normal; falta atomicidad entre cambio, auditoría y habilitación. |
| RF-LOG-04: cambio voluntario | 40 | 90 | Botón y modal integrados; pide clave actual. Pendiente consistencia ante fallo de auditoría y controles UX. |
| RF-LOG-05: bloqueo de ingreso | 85 | 90 | Escrituras ahora propagan errores. Faltan evidencias completas de reinicio y límites temporales con BD de archivo. |
| RF-LOG-06: salida/inactividad | 45 | 70 | Retorno desde temporal corregido; confirmación existe pero su flag no se conecta a capturas. Respuestas tardías fallan. |
| RF-LOG-07: recuperación | 70 | 65 | Espacios normales corregidos; bloqueo vencido sigue fallando y error de auditoría puede perder el código nuevo. |
| RF-LOG-08: comprobación administrativa | 25 | 65 | Cinco fallos realmente pausan; un error tras vencer vuelve a pausar inmediatamente. Tokens aún sin consumidor. |
| RF-LOG-09: auditoría | 55 | 75 | Se agregó salida y se propagan más errores; falta atomicidad y cierre sigue ignorando errores de auditoría. |
| RNF-LOG-01: rendimiento | 70 | 70 | No se volvió a medir el recorrido completo en escritorio/equipo identificado. No se reutiliza la medición previa como evidencia nueva. |
| RNF-LOG-02: hashes/secretos | 90 | 90 | Argon2id y DTO sin hashes; mejor limpieza al cambiar vistas. Persisten excepciones y errores SQL públicos. |
| RNF-LOG-03: usabilidad | 55 | 70 | Mejor acceso y navegación; foco visible ausente y política Unicode frontend/backend diferente. |
| RNF-LOG-04: local/fallo seguro | 65 | 60 | Operación local correcta; fallos de auditoría dejan persistencia o sesión cambiadas mientras se informa error. |
| RNF-LOG-05: consistencia | 50 | 65 | Revoca por rol/activo; falta revocar reset y descartar respuestas anteriores. Actividad conserva aproximación de 30 s. |

Algunas notas bajan porque la modificación de auditoría introdujo un modo de fallo nuevo: antes se ignoraba el error y se entregaba el resultado; ahora se propaga después de confirmar datos, ocultando un código que ya quedó vigente. No significa que no existieran mejoras en esas funciones.

## Estado de los hallazgos anteriores

| Hallazgo | Estado actual |
|---|---|
| H01: rutas sin sesión/identidad desde React | Corregido el ingreso sin sesión en los handlers revisados. Parcial para permisos, actor y propiedad: comprobar sesión no basta por sí solo. |
| H02: cambio obligatorio desde NORMAL | Corregido: guardia en comando y bandera de cuenta en servicio. Prueba adicional aprobada. |
| H03: límite administrativo desconectado | Conectado y probado; falta reiniciar contador tras vencimiento. |
| H04: revocación de cuentas | Corregido cambio de rol/activo. Reset o cambio pendiente no revoca la sesión operativa anterior. |
| H05: cambio voluntario inaccesible | Corregido: Sidebar llama callback y App monta modal. Prueba React aprobada. |
| H06: salida de cambio obligatorio | Corregido: vuelve a LOGIN y limpia los dos campos. Prueba React aprobada. |
| H07: salida con captura pendiente | Parcial: modal funciona con flag true simulado; ninguna captura activa ese flag. |
| H08: errores de escritura ignorados | Corregida propagación en Login. Incompleto: auditoría fuera de transacción y cierre con errores ignorados. |

También se corrigieron los espacios ordinarios del código, el cómputo de caracteres Unicode en la política Rust y la carga de la migración real en `auth_tests.rs`.

## Defectos reproducidos que bloquean aprobación

### R01. Instalación/recuperación se guardan pero no entregan el código si falla auditoría — alto

Archivo: `src-tauri/src/services/auth_service.rs`, instalación líneas 171–181 y recuperación desde 510.

En ambas operaciones `tx.commit()` ocurre antes de `registrar_evento_login(...)?`. Si el INSERT de auditoría falla, se retorna Err después de confirmar los datos. El frontend no recibe la respuesta que contiene el nuevo código.

Pruebas ejecutadas con esquema real y trigger de diagnóstico que rechaza INSERT en auditoría:

- Instalación: retorna error, pero `COUNT(usuarios)` queda en 1 y existe hash de recuperación cuyo código no se entregó. Reintentar instalación encuentra una cuenta existente.
- Recuperación: retorna error, pero cambió el hash del código y se invalidó el anterior; el código nuevo no se entrega.
- Cambio obligatorio: retorna error, pero la sesión queda NORMAL y la clave ya cambió. La interfaz no recibe éxito aunque el backend ya habilitó negocio.

El cambio voluntario también actualiza antes de auditar por lectura de código; no se añadió una prueba independiente para él. La solución debe preservar una unidad consistente entre datos, auditoría, respuesta y transición de sesión. Añadir `?` después del commit no revierte una transacción ya confirmada.

### R02. Restablecer contraseña no revoca la sesión anterior — alto

Archivo: `src-tauri/src/services/session_manager.rs:141`.

La validación operativa consulta `activo, rol`, pero no `debe_cambiar_clave`, hash o una generación de credenciales. No guarda versión de autenticación en memoria. `obtener_usuario_sesion` devuelve un resumen con CAMBIO_OBLIGATORIO si la bandera cambió, pero no modifica la sesión Rust; el polling frontend descarta respuestas exitosas.

Prueba ejecutada: abrir sesión NORMAL y simular reset actualizando `debe_cambiar_clave=1` y `version=version+1`. Esperado: siguiente operación rechazada y acceso anterior invalidado. Observado: `validar_sesion_operativa` devuelve el usuario válido.

Esto incumple RB09. No exige desarrollar Administración ahora: el cambio puede simularse en una base ficticia, como en la prueba.

### R03. Recuperación vuelve a bloquear tras un único error después de vencer — medio/alto

Archivo: `src-tauri/src/services/auth_service.rs:390–437`.

Se lee `fallos_rec` antes de limpiar el estado vencido. El UPDATE deja cero en BD, pero la variable sigue siendo 5. Al fallar el código se calcula 5+1 y se guarda otro bloqueo de quince minutos.

Prueba ejecutada: cinco fallos previos y vencimiento pasado; un código erróneo. Esperado: contador 1. Observado: contador 5 y nuevo bloqueo. La intención del cambio está presente; su efecto completo no se cumple.

### R04. La pausa administrativa tiene el mismo problema de ciclo — medio

Archivo: `src-tauri/src/services/session_manager.rs:193–204`.

Al expirar `pausado_hasta`, `esta_verificacion_admin_pausada` deja pasar, pero `fallos` conserva 5. Un nuevo error vuelve a pausar en lugar de iniciar un ciclo nuevo.

Prueba ejecutada: cinco verificaciones reales fallidas, vencimiento simulado, un error nuevo. Observado: queda pausado otra vez. Una clave correcta tras vencer sí limpia el estado; el problema afecta al primer error del ciclo nuevo.

### R05. Capturas sin guardar no activan la confirmación — medio

Archivos: `AuthContext.tsx`, `Sidebar.tsx` y `CambioClaveModal.tsx`.

La búsqueda de `setHayCambiosSinGuardar` encuentra la declaración, el reset false y la exposición del contexto, sin llamadas de formularios que lo pongan en true. Por ello el modal implementado no se dispara en el uso ordinario.

Pruebas React: con flag true simulado aparece la confirmación y Cancelar conserva sesión; al editar un input del modal real de cambio de contraseña no se registra dirty. El mecanismo existe, pero falta conectarlo a cambios y limpieza/desmontaje de capturas. Su integración en cada módulo futuro podrá completarse al desarrollarlo; Login debe dejar probado el contrato con al menos una captura real o ficticia conectada.

### R06. Una respuesta antigua cierra la sesión nueva — medio/alto

Archivo: `src/context/AuthContext.tsx:119` y gestores asíncronos relacionados.

Prueba React con API simulada: dejar pendiente una consulta periódica de la sesión A, salir, ingresar como B y finalmente rechazar la consulta antigua. Esperado: B continúa autenticado. Observado: React cambia a SIN_SESION. Rust puede seguir teniendo la sesión B abierta.

No hay comparación de generación/usuario antes de aplicar el resultado tardío. El `session_id` de Rust sigue sin intervenir en el contrato frontend. Se confirmó ahora por ejecución lo que antes se había identificado por lectura.

### R07. La respuesta pública aún expone SQL — medio

Archivo: `src-tauri/src/errors.rs:30,39`.

Se mapearon foreign key, unique y locked, pero las ramas restantes interpolan mensaje/error técnico. Prueba ejecutada: error SQLite por columna inexistente; la cadena pública contiene el identificador de columna y la consulta. El mensaje seguro debe cubrir también ramas por defecto. Algunos comandos de Clientes convierten errores a Validation mediante `e.to_string()`, evitando además el mapeo central.

## Integración de identidad y permisos todavía parcial

Son hallazgos por lectura de código; no se realizaron operaciones comerciales sobre la instalación real:

- `comprobante_commands.rs:23`: valida sesión pero descarta `_id_usuario`. El payload conserva `id_cajero` del frontend y `ComprobanteService` lo utiliza como actor del movimiento. Tampoco se ve restricción de rol POS en esa ruta; un mecánico con sesión supera esta guardia. Que otras validaciones SQL puedan rechazar un payload concreto no convierte la entrada en una comprobación de identidad correcta.
- `caja_commands.rs:43`: descarta el usuario validado al cerrar. `CajaService::cerrar_caja` no recibe al actor y registra como cerrador al titular. Falta comprobar permiso/propiedad y distinguir ejecutor cuando corresponda.
- `ordenes_commands.rs:67`: obtiene `_user_id` y lo descarta al crear OT. `ordenes_service.rs`, INSERT de creación, conserva `id_creador = 1` literal. Ya no entra un usuario anónimo, pero la identidad del responsable no queda derivada de la sesión.
- Rutas de Clientes permiten mecánico y devuelven campos de teléfono/dirección. La restricción de datos por rol debe completarse al integrar Ficha; no basta ocultar contactos en React.
- `generar_backup_cmd` ahora exige ADMINISTRADOR, mejora confirmada por lectura. Esto no certifica el flujo de Backup, todavía ajeno a esta aprobación.

Estas interfaces deben corregirse o quedar deshabilitadas mientras se desarrollan. No se propone completar ahora los módulos comerciales para resolverlo.

## Pendientes adicionales conservados

1. Tokens de aprobación todavía se almacenan en SessionState. No se encontró uso productivo de `consumir_autorizacion`; `descuento_bp` no queda ligado a datos definitivos. Falta una operación ficticia que pruebe ejecutor/autorizante, datos exactos y rechazo sin efectos.
2. Polling de sesión no aplica cambios exitosos del resumen; cualquier error se etiqueta como inactividad aunque sea otro fallo.
3. Última interacción humana se transmite como máximo cada treinta segundos, por lo que sigue siendo aproximada. No se repitió una simulación temporal completa de este desfase.
4. App conserva módulo, placa seleccionada y visibilidad del modal al salir. El siguiente ingreso no garantiza Inicio limpio; un modal abierto antes de vencer puede reaparecer.
5. Foco visible continúa sin alternativa a `outline: none`; labels y Mostrar/ocultar no son consistentes entre formularios. Falta prueba real Tab/Enter en escritorio.
6. Backend cuenta caracteres Unicode, frontend usa `.length` y regex ASCII. La política no es idéntica para acentos, números Unicode y caracteres fuera del plano básico. El ingreso Rust no impone por sí mismo el máximo de 128 antes de verificar hash.
7. Normalizador del código elimina espacios U+0020 y guiones, pero no tabuladores/saltos interiores. La prueba aprobada cubre espacios ordinarios, no todo whitespace.
8. El salto a CAMBIO_OBLIGATORIO usa `setModo` y no el limpiador común, conservando la temporal en estado React hasta desmontaje/salida.
9. Cierre manual ahora intenta auditar, pero sigue ignorando fallos de lock/INSERT; no se encontró evento explícito de vencimiento. No se pretende bloquear el cierre por auditoría: debe definirse una política que preserve salida segura y trazabilidad del fallo.
10. Login aún no muestra ambiente, DB mantiene ruta única y Sidebar dice v5.0. Separación/cambio de ambiente pertenecen a Administración, pero el contrato visible y la invalidación de sesión al cambiarlo siguen pendientes.
11. Persisten la discrepancia documental sobre reautenticación administrativa en sesión propia y la falta de medición completa de cinco ingresos en escritorio. No se penaliza elegir una interpretación de esa contradicción; debe resolverse antes de fijarla como contrato.

## Pruebas ejecutadas

| Verificación | Resultado |
|---|---|
| `cargo test --test auth_tests --offline` | 12/12 aprobadas. Antes eran 8. Ahora usan el esquema v7 real. |
| `cargo test --lib --offline` | 6/6 aprobadas. |
| `npm run build` | Aprobada: TypeScript y Vite. |
| `npm test` sin diagnósticos | 10/10 aprobadas, de Taller/Ficha. No hay suite permanente de Login frontend. |
| Diagnósticos temporales backend, esquema real | 13 casos: 6 aprobados y 7 fallidos. |
| Diagnósticos temporales frontend | 5 casos: 3 aprobados y 2 fallidos. |

Backend aprobado: cambio obligatorio NORMAL rechazado, cambio de rol revocado, pausa real al quinto error, código con espacios aceptado, política Unicode Rust y rechazo de Login si falla UPDATE. Backend fallido: tres casos de auditoría parcial, reset sin revocación, expiración administrativa, expiración de recuperación y SQL público. Frontend aprobado: volver a Login, botón de cambio voluntario y confirmación/cancelación con flag simulado. Fallido: formulario que no activa dirty y polling anterior que cierra sesión nueva.

Vite falló primero por `spawn EPERM` del entorno restringido y luego compiló al repetirse con el mecanismo de aprobación. No se contabiliza como defecto de la aplicación.

El nuevo test llamado invocación sin sesión solo llama al gestor de sesión; no ejecuta un comando Tauri. Es evidencia útil del guard central, pero falta probar la frontera IPC/handler y los roles. Pasar doce tests no equivale a completar los diez TC documentales: siguen faltando fronteras temporales 14:59/15:00, reapertura de BD para bloqueo, errores transaccionales y pruebas de interfaz/backend/BD por caso.

### Comprobación ampliada de otros módulos, sin convertirla en nota de Login

| Suite | Resultado actual |
|---|---|
| Inventario Rust | 4 aprobadas, 3 fallidas por movimientos/ajuste incompatibles con restricciones actuales. |
| Taller Rust | 0 aprobadas, 19 fallidas en preparación por columna `revision_cuentas_pendiente` inexistente. |
| Vehículos Rust | 0 aprobadas, 6 fallidas en preparación por `Vehiculo requiere cliente identificado`. |

No se afirma que estos fallos sean nuevos respecto a ayer: no se ejecutaron esas suites en la revisión previa. Tampoco significan que cada comportamiento de Taller/Vehículos fallara en ejecución; sus escenarios no llegan a probarlo porque falla la preparación. No se ejecutó benchmark de 50 000 vehículos ni integración fiscal real. El ERP completo no puede declararse validado con esta evidencia.

## Orden de cierre recomendado para aprobar Login

1. Hacer atómicos datos y auditoría, moviendo cualquier transición de sesión después del guardado completo. Cubrir con fallos inyectados instalación, recuperación y ambos cambios de clave; no perder entrega de secretos confirmados.
2. Revocar sesión por restablecimiento, estado/rol y una generación adecuada de credenciales; no usar cualquier modificación de perfil como revocación indiscriminada.
3. Reiniciar contadores efectivos al expirar ambos flujos de pausa. No reutilizar valores leídos antes del reinicio.
4. Completar identidad/roles en las rutas expuestas o deshabilitarlas mientras se desarrollan sus módulos.
5. Conectar capturas pendientes, descartar resultados antiguos y limpiar navegación/modal al cambiar sesión.
6. Completar errores públicos seguros, foco y validaciones coherentes. Ejecutar los diez TC y RNF con evidencia real, incluyendo cinco tiempos de ingreso de escritorio en equipo identificado.

Las correcciones reales justifican subir de 60 a 75. Los fallos reproducidos de consistencia y revocación impiden aprobar o asignar más de 95 con la evidencia actual. No se necesita reiniciar el módulo: falta una ronda de correcciones concretas y verificación de sus límites.
