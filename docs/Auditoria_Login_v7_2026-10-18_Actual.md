# Auditoría actual de Login y Sesión v7 — 18 de octubre de 2026

**Valoración técnica: 93/100. Decisión: todavía no cerrar el módulo 01 como aprobado.**

El núcleo está implementado y sus suites permanentes pasan. Esta revisión encontró cuatro pendientes concretos adicionales y falta completar la aceptación en el escritorio objetivo. No hace falta terminar POS, Caja, Administración o Backup para resolver estos pendientes de Login.

## Alcance y método

Se contrastó el árbol de trabajo actual, incluidos sus cambios sin commit, con el módulo Login y Sesión v7 simplificado (11 páginas) y el Informe General v7 alineado (15 páginas), ambos del 15 de octubre de 2026. Las instrucciones de esos documentos se trataron como requisitos del producto, no como autorización para modificar la implementación.

Se revisaron comandos registrados en Tauri, servicio de autenticación, gestor de sesión, DTO, esquema SQLite, errores públicos, contexto React, formularios, modal de cambio voluntario, navegación y pruebas. Los informes anteriores sirven de antecedentes; su nota y sus conclusiones no sustituyen esta verificación.

La revisión de otros módulos se limitó a sus conexiones de identidad, sesión y permisos. No se descontó que todavía falten las pantallas/operaciones completas de módulos posteriores, la gestión de cuentas o el cambio/restauración de ambiente. Sus futuras implementaciones deberán respetar los contratos de Login.

No se modificó implementación, no se abrió una base de negocio y no se ejecutaron ventas, recuperaciones ni cambios de contraseña sobre cuentas reales. Las pruebas usaron SQLite en memoria o archivos temporales y APIs simuladas en React. Se añadió este informe y se conservaron los diagnósticos como archivos de texto fuera de las suites permanentes.

## Ejecuciones realizadas

| Comprobación | Resultado observado | Alcance de la evidencia |
|---|---|---|
| `cargo test --test auth_tests --test auth_regression_tests -- --nocapture` | 30 aprobadas: 18 + 12 | Servicios, núcleo de ingreso, estado, validaciones, expiración, revocación, autorización ficticia y rollbacks. |
| `cargo test --lib --offline` | 4 aprobadas | Incluye inicialización real del esquema sobre directorio temporal; las otras tres no certifican Login. |
| `npm test -- --reporter=verbose` | 27 aprobadas | 13 de AuthContext, 4 de autorización y 10 de pantallas de otros módulos. No son 27 casos de Login. |
| `npm run build` | TypeScript y Vite aprobados | Compilación frontend; no equivale a instalador ni recorrido Tauri/WebView2. |
| Diagnóstico Rust adicional | 3 aprobados y 1 fallido | Persistencia de bloqueo, recuperación independiente, alcance por gestor y rechazo de recuperación no contado. |
| Diagnóstico React adicional | 4 fallidos respecto de las expectativas evaluadas | Fallo de logout, dos formularios sin Mostrar/Ocultar y prefijo técnico en recuperación. |
| Escritorio Tauri/WebView2, tres roles y teclado | No ejecutado en esta auditoría | Pendiente de aceptación real. |
| Cinco ingresos completos de hasta 3 s | No ejecutado | No se deduce rendimiento de la duración de tests o del build. |

Vitest y Vite necesitaron ejecución fuera del sandbox porque el primer intento no podía crear procesos auxiliares (`spawn EPERM`). Tras permitir esa ejecución, ambas comprobaciones pasaron. No se calificó ese bloqueo del entorno como defecto del producto.

Los diagnósticos están archivados en `docs/diagnosticos-login-2026-10-18/`. Sus fallos documentan pendientes reales; no se integraron como pruebas permanentes fallidas ni se corrigió código para hacerlos pasar. Los ejemplos de credenciales de esos archivos pertenecen exclusivamente a fixtures ficticios.

## Hallazgos que corregir antes del cierre

### H1. El frontend oculta que el cierre de sesión falló

**Prioridad media. RF-LOG-06; RNF-LOG-04.**

Ubicación: `src/context/AuthContext.tsx:233`, especialmente el `catch` de `apiCerrarSesion` y la limpieza posterior.

El contexto ignora cualquier rechazo del comando y presenta `SIN_SESION` aunque no haya confirmado el cierre del backend. Se comprobó con el AuthProvider real y una API simulada: sesión normal, fallo `DB_LOCK` al cerrar, interfaz en `SIN_SESION`; una consulta posterior que conserva la sesión del backend vuelve a poner el contexto en `NORMAL` sin nuevas credenciales.

La reproducción demuestra la respuesta del contexto ante un cierre no confirmado. No demuestra que el logout normal falle en producción ni que un Mutex envenenado pueda recuperarse; el error se inyectó en la frontera API para comprobar ese camino. Un IPC que no hubiera ejecutado el cierre también puede dejar el backend abierto: se debe manejar explícitamente la incertidumbre.

Corrección esperada: retirar inmediatamente los datos visibles por privacidad y, además, distinguir cierre confirmado de cierre pendiente/fallido. Resolver o comunicar el fallo y evitar readmitir silenciosamente la identidad anterior. No basta con conservar el `catch` vacío. Probar cierre normal, fallo del comando y consulta/reapertura posterior.

### H2. No todos los errores de credenciales de recuperación incrementan el contador

**Prioridad media. RF-LOG-07; RB07.**

Ubicación: `src-tauri/src/services/auth_service.rs:455` y `:483`.

El contador aumenta si el código es incorrecto. Con código vigente pero usuario inexistente, inactivo o no administrativo, se devuelve rechazo desde la consulta del usuario sin aumentar los fallos ni registrar ese rechazo de la misma manera.

Reproducción ejecutada con el servicio real: instalar un administrador ficticio; conservar su código vigente; efectuar cinco recuperaciones con ese código y un usuario inexistente de formato válido. Las cinco se rechazan, pero `recuperacion_fallos` queda en **0**, no en 5, y no se activa la pausa de ese flujo.

El PDF indica cinco fallos de recuperación, sin limitar el contador exclusivamente a códigos incorrectos. Este caso requiere poseer el código; no permite recuperar una cuenta sin él ni constituye por sí solo un acceso indebido. Aun así, la combinación de credenciales se está tratando de forma incompleta.

Corrección esperada: contar de manera uniforme los rechazos de las credenciales de recuperación —código y cuenta elegible—, conservar el mensaje público genérico y mantener ese límite independiente del Login. Los errores de formato y política de contraseña deben distinguirse de un rechazo de credenciales. El diagnóstico de cinco códigos erróneos sí pasó y comprobó que el Login del administrador seguía disponible.

### H3. Mostrar/Ocultar está incompleto en los formularios del módulo

**Prioridad baja. RNF-LOG-03.**

Ubicaciones: `src/pages/auth/components/LoginPasswordChange.tsx:56`, `src/components/CambioClaveModal.tsx:276`, `src/pages/auth/components/LoginInstallForm.tsx` y `src/pages/auth/components/LoginRecoverForm.tsx:103`.

Login normal tiene Mostrar/Ocultar y conserva el valor. Los cambios obligatorio y voluntario tienen inputs de contraseña fijos y carecen del control; se comprobó renderizando los componentes reales. La inspección también muestra esa ausencia en instalación y recuperación.

Corrección esperada: completar el control en los campos de contraseña de los formularios del módulo, con nombre accesible, navegación por teclado y sin cambiar el valor. El control no debe convertirse en envío accidental del formulario.

### H4. Recuperación muestra el prefijo técnico del error

**Prioridad baja. RNF-LOG-03; presentación de errores §4.4.**

Ubicación: `src/pages/auth/LoginPage.tsx:301`.

El `catch` de recuperación presenta directamente la cadena serializada, a diferencia del ingreso y del cambio de clave, que utilizan `mensajeErrorAuth`.

Reproducción React ejecutada: rechazar la API con la cadena pública realista `AUTH_ERROR: No se pudo recuperar el acceso con los datos ingresados.`. La pantalla muestra también `AUTH_ERROR:`. No se observó exposición de SQL, rutas o secretos; es una inconsistencia de presentación.

Corrección esperada: aplicar el mismo mapeo público de errores que en los otros formularios y conservar un texto comprensible para fallos locales.

## Aspectos que ya están correctamente encaminados y comprobados

- Instalación condicionada a cero cuentas, comprobada dentro de la transacción; cuenta inicial sin temporal ni credenciales por defecto; código aleatorio entregado una vez y persistido como hash.
- Argon2id con sal aleatoria; normalización de usuario; contraseña sin recorte ni cambio de mayúsculas; política de 8–128 caracteres, letra, número ASCII y diferencia respecto de usuario/clave actual.
- Contraseña temporal autenticada una sola vez: el cambio obligatorio pide nueva y confirmación. El backend rechaza negocio en el estado limitado; cambio válido habilita la sesión normal.
- Cambio voluntario exige la actual, conserva sesión/rol y no suma errores al contador de ingreso. Los cambios guardan auditoría y contraseña de forma transaccional.
- Bloqueo de ingreso de cinco fallos durante quince minutos; contador y vencimiento persistentes. La prueba adicional cerró y reabrió una conexión a un archivo SQLite: el bloqueo se conservó, insistir no extendió la fecha, el primer fallo tras vencer quedó en uno y un ingreso válido lo reinició.
- Expiración backend a 900 segundos mediante reloj monotónico; actividad real diferenciada del polling. Se cubren 899/900 segundos, interacción a 20 segundos y a 14:30, revocación y actividad de una sesión anterior.
- Las regresiones históricas de ventana de treinta segundos y `deltaMs` ya no corresponden al código actual; sus correcciones tienen pruebas permanentes aprobadas.
- Revocación mediante estado/rol/versión; el próximo comando protegido consulta la cuenta. El rol enviado por React no se utiliza como autorización.
- Recuperación válida cambia contraseña, limpia bloqueo, rota código y guarda auditoría en una transacción; el anterior deja de servir. La sesión se invalida desde el comando después del guardado válido.
- Verificación administrativa tiene límite independiente en memoria por administrador. La prueba ficticia actual autentica de verdad mediante el servicio, conserva al cajero, rechaza referencia alterada y aplica un efecto único con ejecutor/autorizante.
- El comando diagnóstico genérico anterior ya no está expuesto en el registro Tauri. La ejecución ficticia de autorización es suficiente para probar Login sin exigir POS terminado.
- Salida normal no contiene instrucciones que cierren caja ni cancelen OT; Sidebar decide la confirmación a partir de capturas pendientes, sin consultar caja.
- Errores de almacenamiento se serializan sin SQL; DTO de sesión no entrega hashes. Consultar sesión y registrar actividad no insertan auditoría repetitiva.
- Instalación, recuperación y ambos cambios de clave tienen pruebas de rollback por fallo de auditoría. La instalación también comprueba que exista la fila de configuración que se actualiza.
- Las fechas de autenticación utilizan el criterio UTC−05 de Lima en SQLite, sin depender de la zona configurada en Windows.

## Matriz de valoración de 0 a 100

La nota es un juicio técnico de implementación y evidencia, no un porcentaje medido de cobertura ni una aprobación de escritorio. Se promedian los nueve RF y cinco RNF con el mismo peso. No se presenta un requisito no medido como si hubiese fallado en ejecución: la nota de rendimiento refleja la falta de evidencia.

| Requisito | Nota | Evaluación actual |
|---|---:|---|
| RF-LOG-01 Instalación inicial | 98 | Transacción, exclusividad de creación, hashes y fallos locales comprobados; recorrido de entrega en escritorio pendiente. |
| RF-LOG-02 Ingreso y menú por rol | 95 | Credenciales/estado y permisos derivados del backend; falta aceptación completa con los tres roles en escritorio. |
| RF-LOG-03 Cambio obligatorio | 97 | Dos campos, restricciones y guardado comprobados; completar Mostrar/Ocultar y aceptación visual. |
| RF-LOG-04 Cambio voluntario | 96 | Actual/nueva, reglas, cancelación y conservación de sesión; completar usabilidad y recorrido real. |
| RF-LOG-05 Bloqueo de ingreso | 99 | Cinco fallos, persistencia entre conexiones, no extensión, vencimiento y limpieza comprobados; falta reinicio del ejecutable como evidencia final. |
| RF-LOG-06 Salida e inactividad | 92 | Correcciones de actividad verificadas; pendiente H1 y aceptación de salida con captura/caja abierta. |
| RF-LOG-07 Recuperación administrativa | 92 | Atomicidad y rotación correctas; pendiente H2 y entrega confirmada en escritorio. |
| RF-LOG-08 Verificación administrativa | 96 | Credenciales, pausa independiente, identidad y efecto ficticio único comprobados; contrato final con cada origen se valida al desarrollar ese módulo. |
| RF-LOG-09 Auditoría | 96 | Eventos principales y rollbacks comprobados; revisar camino de rechazos de H2 y evidencias operativas finales. |
| RNF-LOG-01 Rendimiento | 70 | Hay implementación local, pero faltan los cinco tiempos completos en equipo identificado. No es una medición de lentitud. |
| RNF-LOG-02 Seguridad | 95 | Argon2id, permisos backend y secretos protegidos en contratos examinados; H1/H2 pendientes y aceptación final. |
| RNF-LOG-03 Usabilidad | 85 | Tab/Enter/formularios/foco definidos; H3/H4 y recorrido WebView2 pendientes. |
| RNF-LOG-04 Operación local y fallos | 95 | No depende de red y hay rollbacks reales; cerrar el tratamiento del fallo de logout. |
| RNF-LOG-05 Consistencia | 95 | Estado en memoria, versiones, tiempo Lima y protección de actividad tardía; pendiente aceptación del ejecutable y límites entre instancias. |

**1301 / 14 = 92,93 → 93/100.** Los criterios del PDF son condiciones de aprobación; una media alta no sustituye cumplirlos.

## Observaciones y dependencias que no se convierten en bloqueos de módulos futuros

1. **Una sesión por proceso frente a una sesión por PC.** El estado único es un `Mutex<SessionState>` por proceso. No se encontró mecanismo de instancia única en `lib.rs` o Cargo. El diagnóstico creó dos gestores contra la misma base y ambos autenticaron. Esto caracteriza la limitación de la capa, no reproduce dos ejecutables Tauri abiertos. Antes de certificar la regla de una sesión, probar el segundo lanzamiento y restringirlo si permite otra instancia. No se afirma haber observado dos ventanas reales.
2. **Capturas sin guardar.** El mecanismo global existe y CambioClaveModal lo integra. Los formularios actuales de negocio no llaman a `setHayCambiosSinGuardar`; al revisar cada módulo habrá que conectarlos. No se exige completar todos esos formularios para cerrar Login, pero sí demostrar el contrato con una captura ficticia o la de cambio de clave.
3. **Gestión de cuentas y ambientes.** El backend de Login ya comprueba versiones y estado. Administración tendrá que incrementar versión/invalidate cuando corresponda y separar físicamente Prueba/Producción; hoy `db.rs` abre una ruta fija. Son dependencias futuras, no una solicitud de terminar Administración ahora.
4. **Autorización en origen.** Los tokens actuales están ligados a acción, referencia y solicitante y se consumen una vez. Los módulos futuros deben derivar datos definitivos y auditar allí el resultado, incluyendo los importes exactos cuando corresponda. No se certifica POS con la prueba ficticia.
5. **Discrepancia documental.** Login §3.3 contempla contraseña propia si la acción administrativa la exige; el informe general §2.2/§6.1 evita repetir contraseña en una sesión administrativa vigente. El comando `reautenticar` existe, pero no está conectado a una pantalla de gestión terminada. Alinear los documentos antes de imponer su uso en módulos posteriores; su mera existencia no demuestra un incumplimiento actual de un flujo de gestión.
6. **Copia del código.** `LoginPage.tsx:251` marca Copiado sin esperar `navigator.clipboard.writeText`. Por inspección puede anunciar éxito antes de confirmarlo. Conviene esperar la promesa y manejar su fallo; no se reprodujo un fallo real del portapapeles en Windows. En recuperación también conviene que la salida de la entrega exprese claramente que se guardó el nuevo código.
7. **Carga de actividad.** Cada `mousemove` envía IPC y comprueba sesión/SQLite. No se demostró degradación; observar movimiento continuo en escritorio. Una optimización futura no debe reintroducir pérdida de la última interacción.
8. **Cifrado de base.** La implementación usa SQLite `bundled`; un `PRAGMA key` no certifica SQLCipher. Login sí protege contraseñas/código mediante hashes. El PDF de Login no exige cifrar todo SQLite; no se descontó este punto ni se certifica cifrado integral del archivo.

## Evidencia mínima para decidir el pase al módulo 02

1. Corregir H1–H4 y convertir sus reproducciones pertinentes en regresiones permanentes. Confirmar que el cierre fallido no readmite la sesión anterior y que cinco rechazos de credenciales de recuperación activan su pausa independiente.
2. Completar en Windows/Tauri los diez TC del PDF, registrando responsable, fecha, versión/commit, preparación, esperado, observado y evidencia sin secretos. El acta de `Cierre_Tecnico_Login_v7_2026-10-18.md` sigue marcando los recorridos manuales como pendientes.
3. Medir cinco ingresos válidos consecutivos desde confirmar hasta menú utilizable, cada uno con objetivo ≤3 s, en equipo identificado. Registrar tiempos reales; la compilación debug o release utilizada debe quedar indicada.
4. Incluir reinicio real, Tab/Shift+Tab/Enter, Mostrar/Ocultar, botón bloqueado mientras procesa, salida directa con caja abierta, una confirmación con captura ficticia, límites 14:59/15:00 y segundo lanzamiento de la aplicación.
5. Si aparece un defecto, corregir y repetir solo el alcance afectado y sus regresiones. El pase requiere que los criterios aplicables queden aprobados, sin condicionar este cierre a terminar módulos posteriores.

**Decisión vigente: Login tiene una base sólida y está cerca del cierre, pero todavía no está aprobado como módulo terminado.**
