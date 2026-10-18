# Login v7 — octava evaluación, 18 de octubre de 2026

**Resultado técnico: 92/100. R7: 92/100. Cierre definitivo pendiente.**

Hay correcciones verificadas y más cobertura permanente. La nota se mantiene porque el nuevo contrato de actividad introduce un error de cálculo, además de conservar la pérdida de interacciones de la revisión anterior. No se penalizan módulos futuros ni sus tests deshabilitados.

## Alcance y estado revisado

Se revisaron el código actual, las diferencias inicialmente visibles de 29 archivos, la nueva suite de AuthContext, las conexiones de identidad/permisos y el README. Se contrastaron con R7 y con el PDF de Login, especialmente RB04, TC-LOG-07 y RNF-LOG-01. No se certifican funcionalidades de módulos posteriores.

Durante la revisión apareció el commit `5018250` (`feat(auth): modulo login terminado parcialmente al 95%`), que incluye los cambios examinados y los dos archivos de diagnóstico creados por esta revisión. Se conservaron esos archivos al comprobar que ya estaban versionados. No se modificó implementación ni se ejecutaron operaciones sobre bases de negocio.

No se abrió WebView2 ni se midieron cinco ingresos completos. Las conclusiones de ejecución separan tests existentes, diagnósticos adicionales y deducciones del flujo; no se presenta simulación de API como prueba de IPC real.

## Correcciones verificadas

1. **Orden del ingreso y auditoría:** `auth_commands.rs:62` consulta ambiente antes de llamar a AuthService::login. El fallo de esa consulta ya no abre sesión ni alcanza el registro LOGIN_OK del servicio.
2. **Polling funcional:** `AuthContext.tsx:170` aplica nombre, rol, permisos, estado y ambiente del resumen exitoso. El diagnóstico avanzó diez segundos y comprobó datos cambiados, sin registrar actividad humana.
3. **Respuesta tardía:** una respuesta exitosa de polling recibida después del logout no recupera identidad ni estado NORMAL.
4. **Error público:** NoAutenticado se serializa con un solo prefijo. La cadena exacta se comprobó en Rust y se utilizó con el helper real de TypeScript.
5. **Trazabilidad:** salida, expiración y autorización registran errores mediante tracing en lugar de ignorar directamente el fallo de auditoría. Esto mejora el diagnóstico; no demuestra por sí solo entrega/persistencia de esos logs en producción.
6. **Unicode parcial:** nombre y usuario se cuentan por caracteres en Rust. El ingreso en contexto y página usa code points. Siguen diferencias en maxLength y números.
7. **Pruebas nuevas:** tres casos permanentes de AuthContext, fronteras 7/8/128/129 y un caso adicional de vínculos de token. Pasaron las suites existentes.

## Hallazgo 1 — el delta representa conceptos distintos en React y Rust

**Severidad: media; pendiente funcional para cerrar Login.**

Archivos:

- `src/context/AuthContext.tsx:134`: calcula delta como tiempo desde el último envío.
- `src/services/authService.ts:86`: transmite ese delta.
- `src-tauri/src/commands/auth_commands.rs:186`: lo recibe y entrega al gestor.
- `src-tauri/src/services/session_manager.rs:130`: lo resta del instante actual, como si fuera la antigüedad de la última interacción.

Son magnitudes distintas. La captura ocurre en el propio evento humano; que hayan transcurrido muchos minutos desde el envío anterior no hace antigua la interacción nueva.

Reproducción ejecutada en dos capas, sin IPC real:

1. AuthProvider carga sesión a las 08:00.
2. El reloj simulado avanza 870 segundos, sin eventos humanos; los pollings no registran actividad.
3. Un keydown a las 08:14:30 llama a apiRegistrarActividad con **870000 ms**. El diagnóstico frontend confirma ese argumento.
4. Se llama al método Rust real `renovar_actividad_con_delta(870000)` en una sesión activa.
5. Se esperaba última actividad reciente; queda aproximadamente **870 segundos en el pasado**. El diagnóstico Rust falla.

Consecuencia deducida del vencimiento de 900 segundos: el backend puede cerrar a las 08:15, unos treinta segundos después de la nueva interacción, en vez de esperar hasta alrededor de las 08:29:30 si no hay más actividad.

**Corrección:** definir una sola semántica. Para un evento recién ocurrido y enviado inmediatamente, su antigüedad es cercana a cero; no es el tiempo desde el envío anterior. Si se envían eventos agrupados, transmitir la antigüedad de la última interacción conservada y aplicar límites y orden para que un reporte viejo no haga retroceder una actividad más reciente. Verificar también el tratamiento de un reporte pendiente cercano al límite: el guard actual comprueba vencimiento antes de aplicar el delta.

No basta con cambiar el nombre del parámetro. Revisar juntos los cuatro archivos anteriores y probar el contrato entre ambas capas.

## Hallazgo 2 — continúa la interacción perdida dentro de treinta segundos

**Severidad: media; pendiente ya señalado en R7.**

`ultimaInteraccionHumanaRef` ahora almacena el instante, pero no se lee para recuperar un evento descartado. La condición `>30000` sigue enviando únicamente en el siguiente evento que supere esa ventana, sin envío posterior de la interacción pendiente.

Diagnóstico ejecutado: sesión válida; avanzar veinte segundos, emitir keydown y avanzar otros doce. **Cero llamadas a apiRegistrarActividad.** El caso de conservación de la interacción falla.

Corregir en `AuthContext.tsx` la conservación y entrega de la última interacción, coordinándolo con el contrato del hallazgo 1. Limpiar cualquier trabajo pendiente al salir/cambiar de sesión. No usar un temporizador que renueve la sesión aunque no haya actividad humana pendiente.

Aceptación conjunta: interacción a 20 s conservada; interacción a 14:30 vuelve reciente el registro; 14:59/15:00 se cuentan desde la última interacción real; tareas automáticas no renuevan; respuestas/envíos anteriores no afectan una sesión nueva.

## Nuevas pruebas permanentes: qué prueban y qué falta

En `src/context/__tests__/AuthContext.test.tsx` hay tres casos aprobados, pero sus nombres abarcan más que su ejecución:

- “Expiración y revocación” arranca sin identidad y rechaza la consulta inicial. No establece primero una sesión NORMAL ni demuestra retirada posterior de un usuario existente.
- “Polling exitoso” comprueba la carga inicial. No avanza diez segundos ni cambia la respuesta para probar el polling.
- El helper `esErrorNoAutenticado` se reimplementa dentro del mock. Ese test no detectaría por sí mismo una regresión del helper de producción.

Mejorar la suite conservando el helper real mediante mock parcial, establecer la sesión antes de simular revocación y avanzar el reloj para probar un tick real. Añadir actividad, respuesta tardía, login fallido y fallos locales. Los diagnósticos de esta revisión sí comprobaron varios de esos recorridos con el helper real.

## Autorización ficticia: avance parcial

El nuevo `test_operacion_ficticia_autorizada_con_tokens`, en `auth_tests.rs:374`, comprueba referencia alterada, autorizante devuelto y rechazo de reuso. Es cobertura útil del gestor de tokens.

Todavía no existe en ese caso una operación de origen con efecto verificable, ni se autentica al autorizante dentro del recorrido. El handler `probar_consumo_autorizacion_cmd` sigue recibiendo acción/referencia esperadas desde el frontend y devolviendo un identificador.

Pendiente: demostrar una operación ficticia con acción fijada por backend, referencia derivada de sus datos y efecto único; credenciales erróneas sin efecto ni cambio de sesión; quinto fallo pausa esa verificación sin bloquear Login. Archivos: `auth_commands.rs`, `session_manager.rs` y `auth_tests.rs`; `lib.rs` si se restringe el comando diagnóstico. No requiere POS terminado.

## Pruebas ejecutadas

| Ejecución | Resultado |
|---|---|
| `cargo test --test auth_tests --lib --offline` | **18 autenticación y 4 biblioteca aprobados.** |
| `npm run build` | **Aprobado**, TypeScript y Vite. |
| `npm test`, antes de agregar diagnósticos | **17/17 aprobados**, incluidos los tres nuevos de contexto. |
| `npm test -- src/context/AuthContext.audit-r8.test.tsx` | **7 aprobados, 1 fallido:** interacción perdida a los veinte segundos. |
| `cargo test --test audit_login_r8 --offline` | **1 aprobado, 1 fallido:** contrato de error correcto; cálculo de actividad incorrecto. |
| WebView2 y rendimiento completo | No ejecutados. |
| Tests deshabilitados de módulos futuros | Fuera de alcance, sin penalización. |

El diagnóstico frontend aprobado que observa delta=870000 caracteriza el envío actual y ayuda a reproducir el error; **no significa que ese valor cumpla el requisito**. Los archivos `AuthContext.audit-r8.test.tsx` y `audit_login_r8.rs` son diagnósticos de revisión; incluyen fallos conocidos y quedaron versionados durante la sesión. Por tanto, no debe afirmarse que la suite completa actual está toda en verde basándose en los 17 casos ejecutados antes de agregarlos.

## Puntuación con la misma escala de R7

| Criterio | R7 | R8 | Motivo |
|---|---:|---:|---|
| RF-LOG-01 Instalación | 95 | 95 | Se conservan creación única y transacción; no se repitieron los cuatro diagnósticos de rollback de R7. |
| RF-LOG-02 Ingreso/permisos | 95 | 95 | Orden de ingreso y guardias conservados; aceptación por roles pendiente. |
| RF-LOG-03 Cambio obligatorio | 95 | 95 | Pruebas pasan y fronteras ahora permanentes. |
| RF-LOG-04 Cambio voluntario | 95 | 95 | Validaciones, dirty y descarte conservados; aceptación visual pendiente. |
| RF-LOG-05 Bloqueo | 95 | 95 | Cinco fallos y vencimiento pasan. |
| RF-LOG-06 Salida/inactividad | 90 | 80 | Pérdida anterior más nuevo error de delta reproducido. |
| RF-LOG-07 Recuperación | 95 | 95 | Rotación y rechazo de código anterior pasan. |
| RF-LOG-08 Verificación administrativa | 85 | 90 | Más cobertura permanente de vínculo/reuso; falta efecto ficticio de origen. |
| RF-LOG-09 Auditoría | 90 | 95 | Ambiente antes de LOGIN_OK y diagnóstico de fallos de escritura. |
| RNF-LOG-01 Rendimiento | 75 | 75 | Faltan cinco ingresos completos hasta tres segundos en equipo identificado. |
| RNF-LOG-02 Seguridad | 95 | 95 | Hashes y contrato público conservados; no se certifican logs de producción. |
| RNF-LOG-03 Usabilidad | 90 | 90 | CSS/foco presentes; falta recorrido WebView2. |
| RNF-LOG-04 Fallos locales | 95 | 95 | Se conserva apertura final sin pasos fallables posteriores. |
| RNF-LOG-05 Consistencia | 95 | 95 | Polling sincroniza y mantiene protección contra respuestas antiguas; quedan detalles menores. |

**1285/14 = 91,79 → 92/100.** Las mejoras de verificación/auditoría compensan la regresión de inactividad. El nombre del commit con “95%” es una declaración del proyecto, no un resultado independiente de aceptación.

## Pendientes menores conservados

- `maxLength={128}` sigue contando unidades UTF-16 en los campos, aunque el validador use code points. React usa números ASCII y Rust is_numeric. Alinear en formularios, `CambioClaveModal.tsx` y `auth_service.rs`.
- El normalizador de usuario valida longitud y espacio ordinario; falta validar completamente el formato antes de SQL.
- `authService.ts` conserva coincidencias amplias de texto. Añadir startsWith junto a includes no vuelve estricta la detección por código.
- Incorporar rollback ante fallos locales y auditoría única con usuario válido a pruebas permanentes. La prueba existente de expiración no demuestra una inserción exitosa de auditoría.
- Confirmar destino y sanitización de trazas operativas. tracing mejora observabilidad, pero la configuración de salida efectiva y protección de logs requieren verificación.
- README reconoce Login pendiente y enlaza R4; actualizar referencias a la evaluación vigente cuando se cierre. Es documentación, no un bloqueo funcional ni un requisito de terminar otros módulos.

## Decisión

**Se mantiene sobre 90 técnicamente. No cerrar todavía Login.** Resolver primero los dos problemas de actividad y repetir sus regresiones; completar la acción ficticia y la aceptación de los diez TC en escritorio. Registrar responsable, fecha, resultado real y cinco tiempos de ingreso completo. Los módulos futuros y sus suites deshabilitadas no son impedimento para este cierre.
