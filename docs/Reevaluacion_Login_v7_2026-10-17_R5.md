# Login v7 — quinta evaluación, 17 de octubre de 2026

**Resultado: 89/100. Previa: 88/100. No aprobado aún para cerrar Login.**

Promedio redondeado de catorce valoraciones: 1240/14 = 88,57. No es porcentaje de tests aprobados. La nota reconoce correcciones comprobadas; la regresión del tratamiento de sesión vencida impide la aprobación.

## Alcance

Se revisaron los 25 archivos modificados mostrados al comienzo, sus conexiones con los módulos existentes, los contratos de error y ambiente y las pruebas. Se ejecutó la suite Rust completa habilitada, compilación y pruebas frontend, y diagnósticos adicionales. No se abrió WebView2 ni se operó sobre bases de negocio. No se certifica instalador ni cinco ingresos completos en escritorio. No se modificó implementación; las pruebas temporales se retiraron y este informe es el único entregable permanente de la revisión.

Se mantiene el alcance módulo por módulo: no se exige terminar Caja, POS o Taller. Sí deben protegerse sus rutas ya expuestas o deshabilitarse mientras se desarrollan. Deshabilitar sus suites puede servir para aislar alcance, pero no acredita que pasaron o que sus incompatibilidades quedaron resueltas.

## Puntuación

| Requisito | R4 | Actual | Fundamentación |
|---|---:|---:|---|
| RF-LOG-01 Instalación | 90 | 95 | Mínimo ocho restaurado y rollback conservado. |
| RF-LOG-02 Ingreso/permisos | 85 | 90 | Mejor comprobación de titularidad; queda excepción administrativa incorrecta para cobros ajenos y validación de integración parcial. |
| RF-LOG-03 Cambio obligatorio | 90 | 95 | Sesión antigua rechazada, mínimo correcto y transacción conservada. |
| RF-LOG-04 Cambio voluntario | 90 | 95 | Capa del descarte corregida, dirty y foco programático mejorados. |
| RF-LOG-05 Bloqueo Login | 95 | 95 | Implementación y casos habilitados conservados. |
| RF-LOG-06 Salida/inactividad | 85 | 70 | Regresión: backend invalida pero React no limpia la sesión con el mensaje real. |
| RF-LOG-07 Recuperación | 90 | 95 | Mínimo correcto y rollback comprobado; quedan detalles menores de contrato/UX. |
| RF-LOG-08 Verificación administrativa | 80 | 85 | Handler ficticio de consumo añadido; no demuestra aún una acción con datos definitivos elegidos por backend. |
| RF-LOG-09 Auditoría | 90 | 90 | Se conserva expiración/atomicidad; fallos de registro de salida siguen ignorados. |
| RNF-LOG-01 Rendimiento | 75 | 75 | Faltan cinco tiempos de ingreso completo en equipo identificado. |
| RNF-LOG-02 Hashes/secretos | 95 | 95 | Argon2id y purgado conservados. |
| RNF-LOG-03 Usabilidad | 80 | 90 | CSS cargado, capas y comportamiento de foco mejorados; falta WebView2. |
| RNF-LOG-04 Local/fallos | 95 | 95 | Cuatro rollbacks adicionales aprobados. |
| RNF-LOG-05 Consistencia | 85 | 75 | Error público Rust no coincide con detección frontend; fallo de ambiente aún se convierte en Producción. |

## Correcciones comprobadas

1. Política Rust y handlers vuelven a ocho caracteres. Diagnósticos de 7, 8, 128 y 129 aprobados; el frontend rechaza siete y admite ocho con acentos.
2. `main.tsx` importa App.css. La compilación genera ahora un asset CSS, además del HTML/JS.
3. ConfirmModal usa z-index 10000 y CambioClaveModal 9000. La prueba confirma que descarte queda por encima. Se añadieron foco inicial, restauración de foco, Tab y Escape por código; la experiencia real requiere WebView2.
4. La captura de cambio de contraseña activa dirty y se mantiene la confirmación.
5. `obtener_ambiente_inicial` permite consultar configuración sin sesión y está registrado. En un arranque sin sesión con lectura exitosa, el contexto muestra PRUEBA. Ya no deriva el ambiente de Vite.
6. El guard de credenciales invalida sesiones antiguas incluso en cambio obligatorio. Prueba permanente y diagnóstico adicional aprobados.
7. Expiración de la pausa administrativa reinicia su contador. Diagnóstico aprobado.
8. Instalación, recuperación y ambos cambios de clave revierten ante auditoría fallida. Cuatro diagnósticos aprobados.
9. Cierre de Caja ahora rechaza al usuario que no es titular ni administrador. Emisión comprueba caja abierta y titularidad para no administradores. La excepción administrativa de emisión aún contradice el turno propio obligatorio.
10. Se añadió un handler con sesión que consume un token ficticio y devuelve autorizante. Es avance frente a consumir solo un mapa en tests, pero no hay prueba de ese handler ni un efecto de origen completo.

Estas correcciones no deben seguir reportándose como ausentes. La nota sube poco por una regresión concreta de sesión, no por desconocer las mejoras.

## Bloqueante actual: React no reconoce el rechazo real de Rust

Fuentes: `src/context/AuthContext.tsx:130–183`, `src-tauri/src/errors.rs:53,89`, `src/services/authService.ts`.

Rust serializa `AppError::NoAutenticado` como la cadena pública:

> Debes iniciar sesión para realizar esta acción.

El polling busca `NO_AUTENTICADO`, `NoAutenticado` o `inactividad`. Ninguna está contenida en esa cadena. El registro de actividad usa también códigos que no se envían. El servicio TypeScript pasa directamente el rechazo de invoke, sin traducirlo.

Diagnóstico React ejecutado:

1. Iniciar el contexto con una sesión válida y nombre ficticio visible.
2. Hacer que el siguiente polling reciba exactamente el mensaje serializado por Rust.
3. Avanzar diez segundos con reloj simulado.

Esperado: SIN_SESION e identidad retirada. Observado: sigue NORMAL y conserva nombre. El diagnóstico simula la API con el contrato real; no se invocó IPC en WebView2.

El backend sigue rechazando operaciones tras el vencimiento; no se afirma que permite operar sin autenticación. El incumplimiento es que la pantalla/identidad no se limpian ni se vuelve a Login. El mismo tipo de rechazo puede representar revocación por cambio de cuenta.

Corrección recomendada: un contrato de error con código estable separado del mensaje público, aplicado a obtener sesión, actividad y comandos protegidos. Alternativamente una adaptación central coherente del contrato existente. No basar la seguridad en buscar palabras traducidas. Mantener el comportamiento correcto de no tratar un error local de almacenamiento como revocación.

Pruebas mínimas: expiración, reset/desactivación y fallo local. Los dos primeros retiran identidad; el tercero muestra un error controlado sin inventar inactividad. Conservar la protección contra respuestas antiguas ya corregida.

## Ambiente: éxito corregido, error todavía inventa Producción

Fuentes: `AuthContext.tsx:111,238`, `auth_commands.rs:23–29`.

La consulta pública ya propaga errores SQL. Sin embargo, el catch frontend asigna PRODUCCION cuando falla; el helper privado de sesión sigue devolviendo Producción si falla SQLite. El arranque no debe transformar un fallo en un ambiente confirmado.

Diagnóstico ejecutado: sin sesión, consulta de ambiente rechazada. Esperado: desconocido/error de lectura. Observado: PRODUCCION. El caso exitoso con PRUEBA pasa.

Corregir ambos fallbacks, mostrar estado desconocido y ofrecer reintento. No se necesita implementar el cambio completo de ambiente para resolverlo.

## Emisión: administrador todavía puede superar la guardia de turno propio ajeno

Fuente: `src-tauri/src/services/comprobante_service.rs:44`.

La nueva condición rechaza propietario distinto solo si el rol no es ADMINISTRADOR. El Informe General exige turno propio también para administrador al cobrar/devolver. Su permiso para consultar o cerrar por ausencia no lo convierte en titular de una caja ajena para vender.

Corregir guardia o deshabilitar la ruta pendiente. No se ejecutó una venta real ni se afirma una emisión completada desde ese caso; la excepción está verificada por lectura antes de los efectos comerciales.

En Caja, la nueva excepción para cierre administrativo es razonable, pero el servicio conserva tipo NORMAL y no recibe motivo de ausencia. Ese flujo completo se desarrollará al llegar a Caja; no debe confundirse con una aprobación actual de todo el módulo.

## Autorización ficticia: qué demuestra y qué falta

`probar_consumo_autorizacion_cmd` está registrado, valida sesión y consume el token del solicitante. Sin embargo, recibe `accion_esperada` y `referencia_esperada` del propio frontend y solo devuelve identidad; no decide los datos definitivos de una operación ni registra su efecto.

Los tests permanentes siguen consumiendo el mapa directamente. Falta probar una operación ficticia con acción esperada establecida por backend, referencia derivada de sus datos, cambio de datos rechazado, efecto único y ejecutor/autorizante conservados. No se exige POS terminado. Evitar presentar el endpoint de diagnóstico como integración comercial terminada y considerar limitarlo a pruebas/desarrollo.

## Suite Rust: termina en verde porque cuatro archivos están deshabilitados

Comando ejecutado: `cargo test --offline --no-fail-fast`, exit code 0.

Inventario, Taller, Vehículos y benchmark contienen ahora en la primera línea:

```rust
#![cfg(feature = "tests_deshabilitados_temporalmente")]
```

Por ello cada binario anuncia `running 0 tests`. En el Cargo.toml revisado no hay una sección que declare esa feature. Algunos llamadores se actualizaron, pero no se verificaron sus escenarios activos. Integración NubeFact ya estaba comentada y también ejecuta cero.

Es aceptable aislar módulos futuros durante el desarrollo si queda declarado como NO EJECUTADO. No es equivalente a corregir y aprobar la integración ni debe contarse como mayor cobertura de Login. Este informe no los penaliza como decenas de defectos nuevos del primer módulo; impide afirmar que todo el ERP pasó.

Para verificar esas integraciones, reactivar/adaptar cada suite cuando corresponda y ejecutar sus casos reales. Conservar por ahora los resultados de Login separados y no reportar el benchmark como aprobado en esta revisión: no se ejecutó.

## Pruebas de esta revisión

| Verificación | Resultado |
|---|---|
| Compilación frontend | Aprobada; genera CSS. El primer intento tuvo spawn EPERM del entorno y la repetición aprobada pasó. |
| Rust habilitado | 15 tests Auth y 4 de biblioteca aprobados; doctests/binarios sin casos. |
| Otras suites Rust | Inventario, Taller, Vehículos y benchmark: 0 casos por cfg; NubeFact: 0 por código comentado. |
| Frontend permanente | 14/14 aprobados, incluidos 4 del modal de autorización. |
| Backend diagnóstico | 7/7 aprobados: fronteras de clave, sesión antigua, pausa y cuatro rollbacks. |
| Frontend diagnóstico | 5/7 aprobados, 2 fallidos: contrato de sesión vencida y ambiente ante fallo. |

Frontend diagnóstico aprobado: siete rechazado, ocho con acentos aceptado, dirty/capas, ambiente Prueba al inicio y fallo local sin revocación inventada. Fallido: limpiar sesión con mensaje real de Rust y no inventar Producción si falla configuración.

No se midieron cinco ingresos completos de escritorio ni se ejecutó una prueba de extremo a extremo en Tauri. Los tests de API simulada y de servicios no sustituyen ese cierre de aceptación.

## Pendientes secundarios conservados

- Actividad humana todavía se envía con intervalo de hasta treinta segundos; falta precisión y prueba desde última interacción real.
- Polling ignora respuestas exitosas que cambien el resumen. Separar revocación, inactividad y fallo local mediante código de error estable.
- Longitud de ingreso/contexto y atributos maxLength usan unidades UTF-16; política Rust usa caracteres. Regex de números todavía difiere. No ampliar la política fuera del PDF; hacerla coherente.
- Máximo de ingreso en backend, formato completo del usuario antes de SQL y longitud de nombre por caracteres siguen pendientes menores.
- Auditoría de salida/expiración conserva errores ignorados; definir trazabilidad del fallo sin impedir salida segura.
- Mostrar/Ocultar, captura/entrega de recuperación y recorrido de teclado deben verificarse en escritorio; no afirmar aprobación visual solo por DOM.

## Qué falta para cerrar Login

1. Corregir el contrato de errores Rust/React y probar expiración/revocación con el mensaje/código real.
2. Eliminar los fallbacks que inventan Producción al fallar configuración.
3. Completar o deshabilitar la excepción de emisión desde turno ajeno y dejar claro el estado pendiente de integraciones comerciales.
4. Probar la autorización ficticia en origen con datos elegidos por backend; no hace falta desarrollar POS.
5. Completar matriz de los diez TC y RNF, recorrido de teclado en WebView2 y cinco tiempos completos en equipo identificado.

Los cuatro arreglos principales de R4 están mayormente resueltos. La regresión del rechazo de sesión es el motivo principal de no aprobar aún ni asignar más de noventa con la evidencia actual.
