# Reevaluación de Login y Sesión — 17 de octubre de 2026

**Nota: 85/100. Evolución: 60 → 75 → 85. No aprobado todavía para cerrar el módulo.**

Se contrastó el código local actual con Login v7 y las revisiones previas. La nota es el promedio redondeado de catorce valoraciones: 1185/14 = 84,64. Es una estimación técnica de cumplimiento, no porcentaje de tests aprobados. Los defectos de revocación/permisos impiden aprobación aunque el promedio suba.

## Alcance

Se revisaron los dieciséis archivos con cambios que mostraba Git al comenzar, los componentes extraídos de Login, sus dependencias y los límites con comandos/servicios existentes. No se exige terminar módulos posteriores. Sí se exige que los comandos ya expuestos respeten autenticación, identidad y permisos, o permanezcan deshabilitados.

No se abrió WebView2, no se operó sobre datos reales ni se validó instalador. Las pruebas adicionales usaron SQLite ficticio, una base temporal en archivo y componentes React con API simulada. Los tests temporales se retiraron; no se cambió la implementación. El único entregable nuevo permanente es este informe.

## Puntuación

| Requisito | 16 de octubre | Actual | Evidencia y límite |
|---|---:|---:|---|
| RF-LOG-01 Instalación | 75 | 95 | Fallo de auditoría revierte cuenta y código. Falta recorrido real de entrega/teclado. |
| RF-LOG-02 Ingreso/permisos | 75 | 80 | Sesión y versión verificadas en negocio; roles/propiedad incompletos en ciertas rutas expuestas. |
| RF-LOG-03 Cambio obligatorio | 85 | 85 | Atomicidad corregida; una sesión limitada obsoleta todavía puede modificar clave tras reset. |
| RF-LOG-04 Cambio voluntario | 90 | 95 | Transacción y versión sincronizada; éxito conserva sesión. Pendientes UX y validación general de sesión en ruta auth. |
| RF-LOG-05 Bloqueo Login | 90 | 95 | Persistencia tras reapertura y no ampliación de plazo comprobadas. |
| RF-LOG-06 Salida/inactividad | 70 | 80 | Polling antiguo descartado y limpieza de App. Flag de capturas sigue sin activarse. |
| RF-LOG-07 Recuperación | 65 | 90 | Rotación/auditoría atómicas y contador vencido corregido. Falta cerrar detalles de UI/código. |
| RF-LOG-08 Verificación administrativa | 65 | 65 | El ciclo después de expirar sigue mal; tokens siguen sin integración en origen. |
| RF-LOG-09 Auditoría | 75 | 85 | Mejor atomicidad; salida todavía ignora errores y no hay evento de vencimiento conectado. |
| RNF-LOG-01 Rendimiento | 70 | 75 | Cinco ingresos de servicio medidos; no equivalen a ingreso completo del escritorio. |
| RNF-LOG-02 Hashes/secretos | 90 | 90 | Argon2id y mejor error central; temporal sigue en estado React al pasar a cambio obligatorio. |
| RNF-LOG-03 Usabilidad | 70 | 75 | Refactor de formularios y foco en botones Sidebar; inputs Login sin foco visible, políticas diferentes. |
| RNF-LOG-04 Local/fallos | 60 | 95 | Rollback probado en instalación, recuperación y ambos cambios de clave. |
| RNF-LOG-05 Consistencia | 65 | 80 | Versión, reapertura e inactividad verificadas; falta control de versión en cambio obligatorio y ambiente real. |

## Correcciones confirmadas mediante ejecución

1. Fallo al insertar auditoría durante instalación: no queda administrador creado.
2. Fallo de auditoría durante recuperación: se conserva código/hash anterior y contraseña anterior.
3. Fallo de auditoría en cambio obligatorio: conserva bandera pendiente y sesión limitada.
4. Fallo de auditoría en cambio voluntario: conserva contraseña previa y sesión válida.
5. Cambio voluntario exitoso: actualiza versión de sesión y permite continuar operando.
6. Incrementar versión al restablecer: el siguiente comando operativo invalida sesión normal.
7. Bloqueo de recuperación vencido: primer error nuevo deja contador 1.
8. Bloqueo Login: persiste al cerrar y reabrir una BD SQLite temporal; insistir no cambia su fecha.
9. Inactividad: permite a 899 segundos y rechaza a 900.
10. Error SQLite de columna inexistente: el mapeo central devuelve mensaje sin SQL.
11. Consulta periódica antigua rechazada después de cambiar usuario: ya no cierra la sesión nueva.
12. Salida de cambio obligatorio, dos campos y botón de cambio voluntario: comprobaciones React aprobadas.

Por lectura se confirmó el reinicio de navegación/placa/modal al salir, la identidad de emisión sobrescrita desde sesión y la limpieza de ramas técnicas del mapeo central. El refactor de Login en componentes mantiene los flujos y pasa compilación. Estos cambios no deben seguir figurando como fallos sin corregir de la revisión anterior.

## Fallos reproducidos restantes

### A. Sesión limitada anterior a un reset puede cambiar la clave vigente — alto

Fuentes: `src-tauri/src/commands/auth_commands.rs:116` y `src-tauri/src/services/auth_service.rs:536`.

El comando comprueba inactividad y CAMBIO_OBLIGATORIO, pero no compara `usuario_version` con BD. El servicio comprueba activo y bandera, pero tampoco la versión. Después del cambio actualiza la versión de la sesión, legitimando la sesión antigua.

Reproducción sobre esquema real:

1. Crear cuenta y abrir sesión limitada con su versión actual.
2. Simular otro restablecimiento: reemplazar hash por una temporal nueva e incrementar versión, conservando cambio obligatorio.
3. Usar la sesión limitada anterior para cambiar la clave sin acreditar la temporal nueva.

Esperado: rechazo/revocación y nuevo ingreso. Observado: éxito, cambio de clave y habilitación. El diagnóstico ejecutó el servicio; la revisión del comando confirmó que tampoco cierra esta condición. No se hizo una invocación IPC real ni se modificó una cuenta real.

Corrección: introducir validación de identidad/versiones que permita el modo limitado, pero rechace una generación obsoleta, antes de cambiar clave. Reutilizar directamente el guard de negocio no sirve sin adaptarlo: ese guard rechaza todas las sesiones limitadas. Revisar también el contrato del cambio voluntario, que actualmente solo valida inactividad en su comando.

La revocación de operaciones NORMAL sí está corregida; este es el hueco restante en la ruta de cambio de temporal.

### B. Expiración administrativa conserva cinco fallos — medio

Fuente: `src-tauri/src/services/session_manager.rs:186–207` y verificación real en `auth_service.rs:664`.

Reproducción: cinco verificaciones incorrectas, simular vencimiento, otro intento incorrecto. Esperado: primer fallo de un ciclo nuevo. Observado: otra pausa completa. `esta_verificacion_admin_pausada` solo compara hora y `registrar_fallo_verificacion_admin` incrementa el contador viejo.

Corregir el reinicio efectivo de contador y pausa al vencer. Recuperación y Login ya realizan el reinicio correctamente; no hay que volver a modificar esas funciones por este hallazgo.

### C. Capturas pendientes no conectadas — medio

La búsqueda de `setHayCambiosSinGuardar` no encuentra formularios que lo pongan en true; Sidebar solo lo limpia. Editar el modal real de cambio de contraseña no activa el indicador. La confirmación sí funciona si una prueba suministra true.

El nuevo control al cambiar de módulo tampoco resuelve este punto mientras las capturas no registren modificaciones. Conectar modificación/guardar/cancelar/desmontaje y comprobar una captura propia o ficticia; no hace falta desarrollar todos los formularios futuros.

### D. Política de contraseña distinta entre React y Rust — medio

Prueba React: nueva contraseña ficticia compuesta por siete letras acentuadas y un dígito. Rust la admite por letra+número y ocho caracteres; el formulario obligatorio la rechaza por regex ASCII y no llama al backend. La longitud frontend también sigue usando unidades UTF-16 frente a caracteres Rust.

Unificar política y longitud en ingreso, instalación, recuperación y cambios de clave. El servicio Login aún no limita explícitamente la longitud antes de verificar hash. No se solicita restringir más la política que el PDF.

## Integraciones incompletas verificadas por lectura

- **Ambiente:** `LoginPage.tsx:66,313` y `Sidebar.tsx:46,207` derivan DEV/PROD de `import.meta.env.DEV`. Esto indica modo de compilación, no `configuracion.ambiente_actual`. El esquema actualmente inserta PRODUCCION y la apertura usa una ruta fija: una ejecución dev puede estar sobre esa base. Mostrar Prueba/Producción desde el backend. La separación/cambio de bases puede integrarse con Administración, pero no debe confundirse con Vite.
- **Caja:** `cerrar_caja_cmd` sigue descartando `_user_id`; el servicio no recibe al actor, registra al titular como cerrador y no establece el permiso/propiedad desde sesión.
- **OT:** creación sigue descartando `_user_id`; el INSERT del servicio mantiene `id_creador=1` literal. Añadir un comentario no cambió esta identidad.
- **Comprobantes:** corregido `id_cajero` desde sesión; aún falta restricción explícita de rol POS y verificación de turno propio en la frontera correspondiente. No se afirma que un mecánico completó una venta: no se ejecutó esa explotación.
- **Autorizaciones:** siguen almacenándose tokens y no hay consumidor de negocio de `consumir_autorizacion`. Falta operación ficticia que pruebe ejecutor, autorizante y datos definitivos; no se exige POS terminado.
- **Clientes:** permanecen rutas habilitadas a mecánico que devuelven contactos. Completar restricción de datos antes de integrar esos accesos, o deshabilitarlos.
- **Errores:** el mapeo Database está corregido. Todavía hay servicios/comandos que convierten error técnico con `Validation(e.to_string())`; el mapeo no protege esas rutas. Su eliminación corresponde a la integración segura, sin rehacer aquí módulos completos.

## Pendientes para completar aceptación

1. Foco visible en inputs de Login/cambio voluntario: persiste `outline: none` sin alternativa. Los handlers de foco añadidos a botones Sidebar no cubren esos campos.
2. Labels asociados a inputs y Mostrar/Ocultar coherente en formularios. Verificar Tab y Enter en WebView2.
3. Limpiar contraseña temporal al pasar de ingreso a cambio obligatorio; se sigue usando `setModo` sin purgar ese estado.
4. El polling descarta cambios de una respuesta exitosa; solo maneja errores. Cualquier rechazo se presenta como inactividad, aunque tenga otra causa.
5. El registro de interacción humana conserva la aproximación de treinta segundos. La prueba del límite del gestor no certifica el recorrido completo desde la última interacción real.
6. Cierre manual sigue ignorando fallo de auditoría. El helper dice voluntario o expiración, pero el gestor no lo invoca al vencer. Definir trazabilidad de error sin impedir salida segura.
7. Normalización del código elimina guiones y espacios ordinarios, no whitespace interior completo. Confirmar almacenamiento del nuevo código en recuperación con el comportamiento previsto por el manual.
8. Mantener versión de autenticación coherente: se usa la columna genérica `usuarios.version`; evitar que futuras ediciones solo de nombre provoquen revocación si no es el contrato previsto.
9. La discrepancia documental de reautenticación en sesión administrativa propia permanece sin resolver; no se descuentan puntos por elegir una de esas interpretaciones.
10. Completar matriz de los diez TC y RNF con observado/evidencia real. Los tests permanentes no cubren aún todos los escenarios, aunque ahora haya trece.

## Resultados de ejecución

| Comprobación | Resultado |
|---|---|
| Auth Rust | 13/13 aprobadas. |
| Biblioteca Rust | 6/6 aprobadas. |
| Frontend existente | 10/10 aprobadas, de Taller/Ficha. |
| Compilación TypeScript/Vite | Aprobada. Fallo inicial `spawn EPERM` del entorno resuelto mediante ejecución aprobada fuera del entorno restringido. |
| Diagnósticos backend nuevos | 14 casos: 12 aprobados, 2 fallidos: pausa administrativa y temporal obsoleta. |
| Diagnósticos React nuevos | 7 casos: 5 aprobados, 2 fallidos: dirty y letras acentuadas. |
| Benchmark de 50 000 vehículos | 1/1 aprobado. No mide Login. |
| Inventario Rust | 4 aprobadas, 3 fallidas. |
| Taller Rust | 19 fallidas en preparación por `revision_cuentas_pendiente` inexistente. |
| Vehículos Rust | 6 fallidas en preparación por vehículo sin cliente identificado. |

Los fallos de Inventario/Taller/Vehículos ya estaban presentes en la revisión anterior y se mantienen; no se convierten en 28 defectos distintos de Login. No se certifica todo el ERP por pasar las pruebas de biblioteca. El cambio de benchmark sí produjo una ejecución aprobada; su resultado no acredita permisos ni aceptación de Login.

Cinco tiempos del servicio de ingreso: **954, 1128, 1112, 1178 y 1014 ms**, en debug y SQLite ficticio. La ejecución coincidió con otras pruebas; son evidencia informativa, no comparación controlada con ayer. Faltan cinco mediciones del recorrido completo en escritorio/equipo identificado para aprobar RNF-LOG-01.

El test permanente de instalación que se llama atomicidad comprueba presencia de auditoría y duplicación, pero no inyecta un fallo; esa condición se verificó con los diagnósticos de esta revisión. El test sin sesión prueba el gestor, no un handler IPC. No confundir nombre del test con alcance demostrado.

## Condición para pasar al siguiente módulo

Cerrar A y B; conectar captura pendiente; unificar validaciones y ambiente; corregir o deshabilitar las rutas expuestas con identidad/permisos incompletos. Después repetir solo las pruebas afectadas y completar aceptación de Login en escritorio. Las correcciones de atomicidad, reset normal y polling antiguo ya están comprobadas y no requieren rehacerse.

Se justifica subir a 85. Aún no hay evidencia suficiente para más de 90 ni para declarar Login aprobado, principalmente por el cambio de temporal desde una sesión obsoleta y los controles pendientes de integración.
