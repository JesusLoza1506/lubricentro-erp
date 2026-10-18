# Login v7 — séptima evaluación, 18 de octubre de 2026

**Resultado técnico: 92/100. R6: 91/100. Cierre definitivo pendiente.**

Se conserva la escala del informe anterior: promedio de catorce criterios. La corrección de la sesión parcial mejora RNF-LOG-04 de 85 a 95; total 1285/14 = 91,79, redondeado a 92. La nota no equivale a porcentaje de tests aprobados. Los módulos posteriores y sus tests deshabilitados quedan fuera de la calificación.

## Alcance

Se revisaron los cambios de los 28 archivos modificados que muestra Git, el código actual de autenticación, sus conexiones y el informe R6. Se volvió a consultar el PDF de Login, especialmente RB04, RF-LOG-08, RNF-LOG-01 y aceptación. Los cambios de otros módulos se examinaron como conexiones de identidad/permisos, sin certificar su funcionalidad completa.

Se ejecutaron compilación, pruebas permanentes y diagnósticos temporales. No se modificó implementación ni bases de negocio. Los archivos temporales se retiraron y este informe es el entregable permanente. No se abrió WebView2 ni se midieron cinco ingresos completos de escritorio.

## Estado de los pendientes de R6

| Pendiente R6 | Estado actual |
|---|---|
| Sesión abierta aunque falla ambiente | **Corregido por orden del código.** Lectura de ambiente antes de abrir sesión. No queda operación fallable posterior a la apertura. No se probó por IPC. |
| Precisión de última actividad humana | **Pendiente comprobado.** Sigue el filtro de 30 segundos sin conservación/envío posterior de la última interacción omitida. |
| Pruebas permanentes de AuthContext y fallos locales | **Pendiente.** Permanecen tres archivos de tests frontend, sin suite propia de AuthContext. Los casos adicionales de esta revisión son diagnósticos temporales. |
| Autorización desde una acción ficticia | **Pendiente de integración y evidencia.** El handler sigue recibiendo acción/referencia esperadas del frontend y devuelve autorizante; no demuestra un efecto ficticio de origen. |
| Aceptación en escritorio y rendimiento | **Sin evidencia nueva en esta revisión.** Falta recorrido real y cinco tiempos de ingreso completo. |
| Coherencia Unicode y trazabilidad de auditoría | **Pendientes menores conservados.** No desaparecen por compilar o pasar las suites actuales. |

## Corrección principal comprobada por lectura

En `src-tauri/src/commands/auth_commands.rs:62–77`, `login_usuario` ahora:

1. Valida credenciales.
2. Obtiene el ambiente con propagación de error.
3. Completa el DTO.
4. Abre sesión y retorna éxito.

El fallo de lectura ya no deja una sesión nueva abierta. No debe seguir reportándose ese defecto como presente.

Queda un ajuste de trazabilidad: `AuthService::login` escribe LOGIN_OK antes de que el comando consulte el ambiente. Por tanto, una lectura de ambiente fallida todavía puede dejar un evento de ingreso exitoso aunque el comando no abra sesión. Mover la consulta de ambiente antes de llamar al servicio evita también ese resultado de auditoría. Este detalle no equivale a conservar acceso parcial.

En AuthContext se inicializa ahora `ultimoRegistroRef` al obtener una sesión y el helper TypeScript reconoce más textos de rechazo. Las comprobaciones conservan el helper real de la aplicación, sin reemplazar su lógica por un mock.

## Hallazgo principal restante: interacción humana omitida

**Archivo:** `src/context/AuthContext.tsx:128`. **Requisito:** RB04, quince minutos desde la última interacción humana.

La condición `ahora - ultimoRegistroRef.current > 30000` solo envía actividad si han pasado más de treinta segundos desde el registro anterior. Una interacción dentro de esa ventana no actualiza el backend ni queda pendiente para envío posterior.

Diagnóstico ejecutado sobre AuthProvider con APIs simuladas y reloj controlado:

1. Cargar sesión válida a las 08:00:00.
2. Avanzar veinte segundos y emitir un evento keydown.
3. Avanzar otros doce segundos sin actividad.
4. Comprobar si se llamó a `apiRegistrarActividad`.

**Resultado: cero llamadas.** El caso que exige conservar esa última interacción falla. Los ocho casos restantes del diagnóstico React pasan.

Consecuencia por el contrato actual: el backend podría vencer a los quince minutos del último registro recibido, aunque la interacción humana ocurrió veinte segundos después. El comentario nuevo de “manejo preciso” no cambia esta condición. Inicializar la referencia resuelve su valor inicial, pero no conserva la interacción omitida.

**Corrección requerida:** preservar la última actividad humana y transmitirla con un contrato que mantenga su instante real. Si se agrupan eventos, enviar también la última interacción pendiente y considerar el retraso de transporte; no renovar indefinidamente mediante tareas automáticas.

Archivos involucrados: `src/context/AuthContext.tsx`, `src/services/authService.ts` y `src-tauri/src/commands/auth_commands.rs` si cambia el payload; `src-tauri/src/services/session_manager.rs` si se adapta el registro temporal. Añadir cobertura en una suite de AuthContext y `src-tauri/tests/auth_tests.rs`.

Aceptación: 14:59 desde la última interacción mantiene sesión; 15:00 vence; una interacción dentro del intervalo de agrupación no se pierde; polling y tareas de fondo no mantienen la sesión. El backend sí aprobó la frontera 899/900 segundos desde su registro: el problema restante está en cómo recibe la última interacción real.

## Verificación administrativa: alcance exacto de lo demostrado

Se mantienen pruebas aprobadas de pausa independiente, solicitante correcto y consumo único. El diagnóstico adicional comprobó además rechazo por acción incorrecta, referencia distinta y token vencido, sin cambiar la identidad del cajero.

Eso demuestra el comportamiento de SessionState. Todavía no demuestra una operación de origen completa: `probar_consumo_autorizacion_cmd`, en `src-tauri/src/commands/auth_commands.rs:341`, sigue recibiendo `accion_esperada` y `referencia_esperada` del frontend y retorna un identificador.

Para cerrar la evidencia basta una acción ficticia, con acción esperada fijada por backend y referencia derivada de sus datos; comprobar que credenciales erróneas no generan efecto, que cinco fallos pausan esa verificación sin bloquear Login, y que la autorización correcta ejecuta una sola vez conservando solicitante y autorizante. No se exige terminar POS, Caja ni Administración.

Archivos: `auth_commands.rs`, `session_manager.rs`, `auth_tests.rs` y, si se restringe el endpoint diagnóstico a desarrollo, su registro en `src-tauri/src/lib.rs`.

## Matriz de valoración

| Criterio | R6 | R7 | Motivo |
|---|---:|---:|---|
| RF-LOG-01 Instalación | 95 | 95 | Creación inicial y rollback conservados. |
| RF-LOG-02 Ingreso/permisos | 95 | 95 | Sesión como fuente de identidad y turno propio conservados; aceptación de tres roles pendiente. |
| RF-LOG-03 Cambio obligatorio | 95 | 95 | Política, sesión limitada, versión y rollback conservados. |
| RF-LOG-04 Cambio voluntario | 95 | 95 | Clave actual, descarte, dirty y rollback conservados. |
| RF-LOG-05 Bloqueo | 95 | 95 | Pruebas de cinco fallos y vencimiento pasan. |
| RF-LOG-06 Salida/inactividad | 90 | 90 | Identidad se retira al vencer; última interacción todavía puede omitirse. |
| RF-LOG-07 Recuperación | 95 | 95 | Rotación y rollback pasan. |
| RF-LOG-08 Verificación administrativa | 85 | 85 | Núcleo probado; acción ficticia de origen pendiente. |
| RF-LOG-09 Auditoría | 90 | 90 | Expiración registrada una vez con usuario válido; fallos ignorados y orden de LOGIN_OK por pulir. |
| RNF-LOG-01 Rendimiento | 75 | 75 | Faltan cinco ingresos completos hasta 3 s en equipo identificado. |
| RNF-LOG-02 Seguridad | 95 | 95 | Hashes conservados y serialización sanitizada comprobada. |
| RNF-LOG-03 Usabilidad | 90 | 90 | CSS/foco/capas presentes; falta recorrido real WebView2. |
| RNF-LOG-04 Fallos locales | 85 | 95 | No queda nueva sesión abierta tras fallo de ambiente; cuatro rollbacks pasan. |
| RNF-LOG-05 Consistencia | 95 | 95 | Error real reconocido, ambiente sin producción inventada y respuesta antigua ignorada. |

## Pruebas ejecutadas ahora

| Comando/capa | Resultado |
|---|---|
| `cargo test --test auth_tests --lib --offline` | **16 autenticación + 4 biblioteca aprobados.** |
| `npm run build` | **Aprobado**, TypeScript y Vite; genera asset CSS. |
| `npm test` | **14/14 aprobados**: 4 autorización, 6 Órdenes, 4 Ficha Vehicular. Las suites de otros módulos no inflan la nota de Login. |
| Diagnóstico React temporal | **8 aprobados, 1 fallido.** Falló conservación de interacción dentro de treinta segundos. |
| Diagnóstico Rust temporal | **9/9 aprobados.** |
| Suites Rust deshabilitadas de módulos futuros | Fuera de alcance, sin descuento y sin calificarlas como aprobadas. |
| IPC real/WebView2, instalador y rendimiento de ingreso completo | No ejecutados. |

Los ocho diagnósticos React aprobados cubren: helper real con rechazo de Rust y diferenciación de error de BD, ambiente PRUEBA prelogin, fallo de lectura sin producción inventada, expiración por polling, rechazo de actividad, error de almacenamiento en polling sin revocación inventada, respuesta tardía tras logout y ausencia de actividad humana inventada por polling.

Los nueve diagnósticos Rust cubren: contrato de errores/sanitización, política 7/8/128/129, ingreso de 129 rechazado antes de consultar BD, frontera 14:59/15:00 con una sola auditoría, vínculos/expiración/reuso del token y cuatro rollbacks por fallo de auditoría: instalación, recuperación, cambio obligatorio y voluntario.

La prueba permanente de inactividad ahora llama al método con auditoría, lo cual es avance. Sin embargo, no comprueba el registro insertado y usa una identidad no creada en su BD de prueba. Por sí sola no acredita auditoría exitosa; el diagnóstico adicional de esta revisión sí creó un usuario válido y comprobó un único evento.

## Ajustes menores conservados y archivos

- **Coherencia de caracteres:** `auth_service.rs` usa chars para contraseña, pero bytes para longitud de nombre/usuario e is_numeric para números; React mezcla code points, longitud UTF-16, maxLength y regex ASCII de números. Alinear política en `auth_service.rs`, `AuthContext.tsx`, `LoginPage.tsx`, `CambioClaveModal.tsx` y formularios de autenticación.
- **Errores públicos:** `errors.rs` sigue duplicando NO_AUTENTICADO entre Display y Serialize. `authService.ts` usa coincidencias amplias de texto. Mantener código estable, detección precisa y mensaje legible en formularios.
- **Fallos de auditoría:** `auth_commands.rs` y `session_manager.rs` conservan resultados ignorados al salir/expirar. Añadir diagnóstico sin secretos manteniendo el cierre seguro.
- **Polling exitoso:** `AuthContext.tsx` comprueba errores pero no aplica el resumen exitoso recibido. Revisar sincronización si cambian datos de usuario/ambiente; la validación backend sigue siendo la autoridad.
- **Pruebas permanentes:** incorporar los casos de contexto y fallos locales a la suite del proyecto. La revisión los ejecutó temporalmente, pero no quedan protegidos automáticamente contra futuras regresiones.

## Decisión de cierre

**Login supera 90 técnicamente y la corrección principal de R6 está implementada.** No asigno todavía cierre definitivo porque queda un incumplimiento reproducido de última actividad y evidencia de aceptación incompleta.

Para cerrarlo: corregir la conservación de actividad, demostrar la acción ficticia de verificación y completar el acta de los diez TC del PDF en escritorio, incluyendo Tab/Enter/foco, cambio temporal, cancelación/dirty, recuperación y revocación. Medir cinco ingresos válidos completos en el equipo identificado, con objetivo de hasta tres segundos cada uno. Si esas verificaciones pasan, no hace falta desarrollar módulos posteriores para aprobar Login.
