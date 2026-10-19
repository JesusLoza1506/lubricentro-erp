# Reevaluación de Ficha Vehicular e Historial — segunda revisión

Fecha: 18 de octubre de 2026, America/Lima.

**Resultado: 48/100. Anterior: 40/100. El módulo todavía no cumple las condiciones de cierre.**

## Alcance

Se revisó nuevamente el código actual, incluidos archivos nuevos sin commit, contra los dos PDF versión 7 del 15/10/2026 y la auditoría anterior. Se inspeccionaron página principal, componentes, tipos, servicios de invocación, registro de comandos Tauri, comandos y servicios Rust, auditoría compartida, permisos de Login, contrato con Taller y esquema V7.

Se distinguió entre código existente, código registrado/invocable y funcionalidad accesible desde la pantalla. Un componente aislado o un servicio correcto no acreditan que el trabajador pueda completar la tarea.

Se mantiene el alcance por etapas: no se exige terminar los demás módulos. Las reglas propias de Ficha y sus contratos con Login/Taller sí se evalúan. La base ubicada en el directorio de datos de la aplicación se abrió exclusivamente en modo lectura para comprobar restricciones: sigue en `user_version=700`, sin `id_zanja_prevista` y con el catálogo de motores V7.

No se editaron datos ni implementación del producto. Los archivos temporales de prueba se retiraron al terminar. No se realizó un recorrido manual de la ventana instalada ni una prueba completa por IPC de Tauri; tampoco se certifica la correspondencia del ejecutable instalado con los fuentes actuales.

## Nota con los mismos pesos de la primera auditoría

| Área | Anterior | Actual | Máximo |
|---|---:|---:|---:|
| Clientes, vehículos y validaciones | 11 | 12 | 20 |
| Historial público e identidad histórica | 6 | 6 | 20 |
| Roles, sesión y privacidad | 9 | 10 | 15 |
| Cambios administrativos y auditoría | 3 | 7 | 15 |
| Próximos cambios | 4 | 5 | 10 |
| Interfaz, errores y respuestas asíncronas | 3 | 4 | 10 |
| Pruebas y evidencia de aceptación | 4 | 4 | 10 |
| **Total** | **40** | **48** | **100** |

La nota es una valoración técnica ponderada del cumplimiento funcional. No es porcentaje de líneas cambiadas, cobertura ni tasa de pruebas. La mejora principal está en auditoría y servicios; varios flujos de pantalla siguen inaccesibles o bloqueados. Los nuevos errores de motor y configuración no se ignoran para aumentar la nota.

## Cambios comprobados como resueltos

1. **Tipo y número de documento:** búsqueda y reutilización ahora comparan ambos campos. Una prueba CE 12345678 no reutilizó el DNI 12345678 existente.
2. **Edición independiente de cliente en backend:** `actualizar_cliente` conserva ID, cambia documento/nombre y permite eliminar contactos opcionales. Se verificó en memoria. Su exposición a Tauri y pantalla sigue pendiente.
3. **Auditoría de cliente:** el alta ahora conserva usuario, acción y datos posteriores.
4. **Auditoría de corrección:** ahora persiste actor USUARIO, ID real y JSON anterior/nuevo. Se comprobaron los valores de km antes y después.
5. **Atomicidad de estado:** el comando utiliza transacción y propaga el error de auditoría. Se reprodujo un fallo de auditoría dentro de la misma estructura transaccional: la baja se revirtió y el vehículo quedó activo. Esta prueba llama al servicio dentro de una transacción equivalente a la del comando; no simula IPC.
6. **Atomicidad de las demás escrituras:** alta/edición de cliente, vehículo y corrección se ejecutan desde comandos con transacción y auditoría obligatoria. Se comprobó por lectura de código; no se inyectó un fallo en cada uno de esos comandos por separado.
7. **Placa con guion al obtener ficha:** la ficha pasa la placa normalizada al historial; ABC-123 ya conserva su atención en la prueba directa.
8. **Acceso de mecánico a candidatos:** el comando de próximos cambios ahora admite únicamente administrador/cajero.
9. **Estado inactivo en pantalla:** hay banner y botón Reactivar; se comprobó el botón en React.
10. **Acciones de inactivo:** Crear OT y Editar Ficha se ocultan; comprobado en React. El backend sigue exigiendo reactivar incluso al administrador para editar el vehículo.
11. **Año mínimo:** el formulario ya usa 1900. Alta Rust añade control de año, aunque usa zona del equipo y SQL conserva el control Lima.
12. **Lista aislada de candidatos:** implementa buscador por placa/cliente, filtros y páginas visuales de 20. Se comprobaron cambio de página y filtrado. Falta conectarla al módulo y corregir el orden de origen.
13. **Contrato frontend:** tipos de cliente se separaron en `cliente.ts` y se incorporó `clienteService`; TypeScript compila.

Se mantienen las garantías previas de unicidad, alta conjunta, exclusión de contactos mecánico, controles RUC/SQL y cálculo SQL de próximos cambios. La corrección sigue recalculando `km_proximo` sin alterar el km actual, comprobado de nuevo.

## Bloqueos actuales y regresiones

### B01 — El selector nuevo de motor impide registrar con sus valores ofrecidos

Referencias: `src/pages/vehiculo/components/ModalVehiculo.tsx:33`, `:320`; `src-tauri/migrations/0001_initial_schema_v7.sql:162`.

El formulario propone `4 Cilindros` y ofrece V6, V8, Diesel, Híbrido y Eléctrico. Ninguno coincide exactamente con los códigos SQL GASOLINA, DIESEL, GLP, GNV, HIBRIDO, ELECTRICO y OTRO. Tampoco hay opción vacía aunque motor sea opcional.

Se envió el valor predeterminado `4 Cilindros` al servicio real con el esquema V7: falló la restricción de motor. El formulario envía el valor sin convertirlo. Este defecto afecta al alta habitual aun cuando los otros campos sean válidos. Añadir un select no basta: deben coincidir las opciones funcionales, los códigos y el carácter opcional.

### B02 — Tres componentes nuevos todavía no tienen entrada desde la aplicación

`ModalCliente`, `ModalCorregirHistorialOT` y `FichaVehicularProximosCambios` no tienen referencias desde ninguna otra pantalla de producción en `src`. La página principal no los importa ni los monta.

Las pruebas de página confirmaron que no existe acceso Nuevo cliente, acceso a Próximos cambios ni acción Corregir desde el detalle. La prueba aislada de filtros/paginación de candidatos sí pasó: la pieza funciona parcialmente, pero el usuario no puede llegar a ella desde Ficha.

### B03 — Editar cliente tiene servicio y comando, pero el comando no está registrado

Referencias: `src-tauri/src/commands/cliente_commands.rs:61`, `src-tauri/src/lib.rs:63`, `src/services/clienteService.ts:11`.

`actualizar_cliente_cmd` existe y el frontend lo invoca. Falta en `tauri::generate_handler![...]`. Se comprobó el registro de handlers. Conectar el formulario sin registrar el comando deja la operación sin resolver en Tauri.

### B04 — Baja y reactivación aún no permiten escribir motivo

Referencias: `src/pages/vehiculo/FichaVehicularPage.tsx:71`, `:160`, `:189`, `:329`; `src/components/ConfirmModal.tsx:5`.

Se añadieron acción y función Reactivar, pero ambas operaciones leen `motivoAccion`, que inicia vacío y solo se restablece a vacío. Ningún campo lo modifica. ConfirmModal sigue sin input/textarea ni mecanismo de captura de motivo.

Las dos pruebas de diálogo fallaron por ausencia de campo. Las operaciones siguen bloqueadas antes del backend. Tampoco hay estado de envío conectado para evitar reiteraciones y el texto de confirmación por defecto sigue siendo «Sí, Eliminar», incluso al reactivar.

### B05 — Historial conserva sus defectos principales

Referencias: `src-tauri/src/services/ordenes_service.rs:562`, `:574`, `:607`; `src-tauri/src/dto/orden_trabajo_dto.rs:14`.

La consulta sigue sin filtrar FINALIZADO, usa placa histórica, no pagina y ordena por recepción. Devuelve km de ingreso, pedido inicial y fecha de ingreso; calcula ingreso + 5.000. El tipo de aceite continúa derivándose del nombre actual de un producto.

Se reprodujo nuevamente:

- OT abierta visible como mantenimiento.
- Sintético finalizado a 50.010 km devuelto con próximo 55.000 en vez de 60.010.
- Pedido inicial mostrado en lugar de resumen público final.
- Ausencia de `corregido` en el DTO serializado después de corregir.
- Pérdida de asociación del historial al cambiar la placa actual, conservando el ID y las OT.

Siguen faltando cliente/documento históricos, fecha/hora final, aceite/intervalo públicos correctos y páginas de 20. Corregir la consulta compartida que utiliza Ficha es parte de este módulo, aunque su archivo se llame `ordenes_service.rs`.

### B06 — Placa, propietario y motivo siguen incompletos

Referencias: `src/pages/vehiculo/components/ModalVehiculo.tsx:263`, `src-tauri/src/services/cliente_service.rs:469`.

La placa está deshabilitada al editar. El servicio busca por la placa recibida, ignora `id_vehiculo` y no ejecuta UPDATE de placa. Una nueva placa devuelve NotFound.

El cambio de propietario sigue aceptándose sin motivo y la reducción de km acepta 251 caracteres de motivo. Ambos fallos se reprodujeron. No hay confirmación específica de cambios administrativos ni distinción de campos por rol en el formulario.

El DTO permite enviar `id_cliente`, pero el servicio resuelve propietario mediante documento; debe unificarse el contrato. La edición de vehículo también sigue modificando datos del cliente asignado, en lugar de utilizar consistentemente la edición separada.

### B07 — Permisos de cajero/mecánico siguen desalineados

Referencia: `src-tauri/src/services/auth_service.rs:798`.

Login continúa entregando `puede_crear=false` y `puede_editar=false` para FICHA_VEHICULAR al cajero. Ficha consume esos permisos y oculta ambas acciones. El mecánico continúa recibiendo el permiso genérico de crear OT que la página emplea para mostrar recepción, aunque su comando la rechace.

Se resolvió específicamente el permiso de próximos cambios en el comando, pero no estas discrepancias.

### B08 — Las búsquedas aún presentan datos obsoletos y confunden fallo con ausencia

Referencia: `src/pages/vehiculo/FichaVehicularPage.tsx:91`.

Las pruebas React volvieron a reproducir una respuesta A antigua que reemplaza la ficha B reciente, y un error de almacenamiento que muestra también «Vehículo no encontrado». Siguen faltando identificación de la búsqueda vigente, clasificación de error/ausencia y Limpiar. La carga de órdenes sigue usando `catch(() => [])`.

### B09 — La normalización telefónica valida un valor, pero guarda otro

Referencia: `src-tauri/src/services/cliente_service.rs:65`, `:125`.

La validación ahora retira espacios/guiones/+51 para comprobar el número, pero el INSERT/UPDATE persiste el texto original solo recortado en extremos. La prueba con `+51 912-345-678` pasó la validación Rust y falló en el CHECK de SQLite.

El frontend conserva la eliminación de caracteres y truncado a nueve dígitos, por lo que pegar el prefijo puede guardar un número diferente. Debe normalizarse una sola representación y persistirla.

### B10 — Se añadió aceite semisintético fuera del contrato V7

Referencias: `src/pages/vehiculo/components/ModalCorregirHistorialOT.tsx:104`, `src-tauri/src/services/cliente_service.rs:731`, SQL `:256` y `:259`.

El nuevo formulario y Rust ofrecen SEMI_SINTETICO / 7.500 km, pero el PDF y SQL admiten Mineral / 5.000 y Sintético / 10.000. Esa opción no se puede guardar con el esquema actual y no es un requisito pendiente de la versión aprobada.

El formulario también propone SINTETICO cuando la atención no tenía aceite y no limita su edición a «si corresponde». Tiene estado inicial por `useState` que no se reinicia al cambiar la atención mientras el componente siga montado. Debe corregirse antes de conectarlo al detalle.

### B11 — Próximos cambios todavía entrega el orden equivocado

Referencia: `src-tauri/src/services/cliente_service.rs:890`.

El SQL del servicio sigue en `fecha_servicio DESC`, frente a antiguo primero e ID como desempate. El componente pagina visualmente de 20 pero descarga el vector completo; no hay contrato paginado en backend. No se declara por ese solo motivo incumplido el objetivo de 3 s: falta medirlo. `rows.flatten()` sigue descartando errores de fila.

Al actualizar y reducirse el conjunto, la página actual no se ajusta al nuevo total; puede mostrar un vacío aunque existan candidatos en páginas anteriores. Es un defecto adicional identificado por lectura del estado, no uno de los casos ejecutados.

### B12 — Recepción de OT sigue fallando contra el esquema activo

Referencias: `src/services/vehiculoService.ts:28`, `src-tauri/src/services/ordenes_service.rs:222`.

Se eliminó la invocación a `listar_vehiculos_cmd` inexistente. Ahora `listarVehiculos` reutiliza la lista de candidatos de aceite y construye marca/modelo vacíos. Esto permite que el modal haga búsqueda de respaldo de la placa inicial cuando no sea candidata, pero no equivale a listar todos los vehículos.

Crear OT sigue intentando insertar `id_zanja_prevista`: la prueba volvió a fallar porque no existe esa columna. Se confirmó también su ausencia en la base de la aplicación, en modo lectura. La elección obligatoria de mecánico/zanja en recepción sigue sin ajustarse al contrato simplificado.

Se registra como integración de Ficha con Taller, sin exigir terminar todo Taller ahora. No puede darse por funcional la recepción actual.

## Regresión detectada fuera de la nota de Ficha

Referencia: `src-tauri/src/commands/auth_commands.rs:321`.

Al adaptar la llamada al servicio de auditoría, `autorizar_descuento` cambió el módulo de POS a AUTORIZACION, eliminó el ID del autorizante y perdió la referencia de venta en el evento.

AUTORIZACION no está permitido por el CHECK de auditoría V7. Se reprodujo el rechazo SQL. El comando registra el error en logs y continúa devolviendo token, por lo que puede autorizar sin conservar ese evento.

Las 30 pruebas existentes de Login/regresión pasaron, pero no cubren esta llamada específica del comando. El hallazgo debe corregirse para no deteriorar el módulo ya revisado. No se utilizó como penalización adicional de Ficha.

## Matriz actual de requisitos

| Requisito | Reevaluación |
|---|---|
| RF-FVH-01 Buscar | Parcial: normalización completa de placa mejorada; error/ausencia, Limpiar y respuestas antiguas pendientes. |
| RF-FVH-02 Alta/reutilización | Parcial: tipo/número y auditoría corregidos; Nuevo cliente sigue inaccesible, motor rompe alta habitual, cajero sin permisos UI. |
| RF-FVH-03 Edición común | Parcial: backend de cliente independiente y borrado de contacto funcionan; falta registro Tauri, acceso UI, permisos y separación efectiva. |
| RF-FVH-04 Historial | No cumple: mantiene datos/estados/relación/paginación incorrectos. |
| RF-FVH-05 Cambios administrativos | No cumple: placa, motivo de propietario, límites y confirmación pendientes. |
| RF-FVH-06 Corrección | Parcial: auditoría y transacción mejoradas; falta conectar acción, valores corregidos en consulta y marca real; opción de aceite incompatible. |
| RF-FVH-07 Baja/reactivación | Parcial: backend y presentación de estado mejoran; ambas acciones sin captura de motivo. |
| RF-FVH-08 Próximos cambios | Parcial: permisos backend y filtros/paginación visual mejorados; inaccesible desde página y orden incorrecto. |
| RF-FVH-09 Recepción | Parcial con fallo integrado: inactivos ya sin botón, pero creación incompatible con V7. |
| RNF-FVH-01 Rendimiento | No acreditado: benchmark deshabilitado, sin medición de aceptación de 1.000 vehículos ni historial paginado. |
| RNF-FVH-02 Usabilidad | Parcial: año y estado mejoran; motor opcional, km cero, errores por campo y descarte siguen pendientes. |
| RNF-FVH-03 Privacidad | Parcial favorable: contactos filtrados; ahora mecánico excluido de candidatos; permisos de recepción y edición comunes desalineados. |
| RNF-FVH-04 Integridad | Mejorado: transacciones/auditoría obligatoria; identidad por tipo/número corregida; falta control de edición obsoleta y trazabilidad completa de todos los campos modificados. |
| RNF-FVH-05 Local/Lima/errores | Parcial: operación local y restricciones SQL; hora/año usan a veces zona del equipo, errores de lectura convertidos a ausencia u omitidos. |

## Otros pendientes que siguen abiertos

- Pestañas/organización del módulo, último servicio y último cambio visibles en la ficha.
- Regreso al origen autorizado: `onVolver` sigue declarado sin uso; App no entrega el callback correspondiente.
- Selección explícita de cliente existente y acceso a su registro al detectar duplicados.
- Confirmación única de descarte: los formularios no usan `setHayCambiosSinGuardar`.
- Kilometraje cero precargado como vacío mediante `valor || ''`.
- Campos administrativos exclusivos del administrador y motivo/confirmación correctos.
- Auditoría completa para todos los campos modificados: edición de vehículo solo conserva km/cliente; edición de cliente conserva tipo/documento/nombre, sin antes/después de contactos cambiados.
- Conteo de caracteres Rust mediante bytes, límites visibles y errores junto al campo.
- Control de versión esperada: los DTO/escrituras no rechazan formularios obsoletos.
- Error de OT abierta con referencia identificable, en lugar de un error SQL genérico.
- Fechas en Lima independientemente de la zona de Windows y pruebas de medianoche.
- Pruebas completas TC-FVH-01 a 10, evidencia de comandos reales y recorrido manual Tauri.

La validación de datos poblados presente en `lubricentro_demo_poblada.db.validacion.json` describe controles de datos; no prueba que estos recorridos de la aplicación funcionen. No se ejecutaron sus scripts de carga ni se modificó la base durante la revisión. Tampoco se utilizó la cantidad de archivos SQL o las copias originales como evidencia de cumplimiento funcional.

## Evidencia de ejecución

| Comprobación | Resultado |
|---|---|
| `npm run typecheck` sobre producto antes de pruebas temporales | Correcto |
| Pruebas existentes React de Ficha | 4/4 aprobadas |
| Pruebas existentes Rust de Ficha | 6/6 aprobadas |
| Pruebas existentes `auth_regression_tests` | 12/12 aprobadas |
| Pruebas existentes `auth_tests` | 18/18 aprobadas |
| Casos adicionales Rust | 20: 9 aprobados, 11 fallidos |
| Casos adicionales React | 11: 3 aprobados, 8 fallidos |
| Registro Tauri de actualizar cliente | Ausente, confirmado por inspección del handler |
| Entradas desde pantallas a tres componentes nuevos | Ninguna, confirmado mediante búsqueda en fuentes de producción |
| Restricción SQL de AUTORIZACION | Rechazo reproducido en base aislada |

Los ejecutables temporales Rust/React incluyeron además las 6/4 pruebas originales, respectivamente: sus totales fueron 26 y 15. La tabla descuenta esas repeticiones. Los conjuntos de casos adicionales no son idénticos a la primera auditoría y no se usa la variación del número de fallos para calcular progreso.

Casos nuevos Rust aprobados: reutilización tipo/número; editar documento manteniendo ID y borrar contacto; autor/antes/después de corrección; rollback de estado ante fallo de auditoría dentro de transacción; placa con guion; recálculo SQL sin alterar km actual; auditoría de alta de cliente; exclusión de contacto mecánico; antes/después de reducción de km.

Casos nuevos Rust fallidos: historial solo finalizadas; próximo km sintético final; resumen final; marca Corregido en DTO; cambio de placa por ID; propietario exige motivo; motivo de 251 rechazado; teléfono normalizado persistible; motor ofrecido por UI persistible; historial tras cambio de placa; creación OT con esquema V7.

Casos nuevos React aprobados: acción Reactivar visible; inactivo sin Crear OT/Editar Ficha; componente aislado con paginación y filtros. Fallidos: motivo de desactivar; motivo de reactivar; acceso a candidatos; acceso a Nuevo cliente; corregir desde detalle; respuesta antigua descartada; fallo distinto de ausencia; opciones de motor compatibles/opcional.

Las pruebas fallidas expresan requisitos esperados y reproducen defectos; no son errores de compilación del producto. El chequeo SQL de AUTORIZACION y la inspección de handlers no se cuentan como tests automáticos adicionales.

## Orden concreto para cerrar

1. Corregir códigos/opcionalidad de motor; es un bloqueo del alta habitual.
2. Conectar los tres componentes nuevos y registrar `actualizar_cliente_cmd`.
3. Añadir captura real de motivo y control de envío a baja/reactivación.
4. Sustituir la consulta/DTO de historial por los campos públicos finales correctos, ID estable y páginas de 20.
5. Completar placa/propietario/reducción con permisos por campo, motivo y confirmación.
6. Alinear permisos de cajero y recepción de mecánico con Login.
7. Persistir el teléfono normalizado y retirar semisintético del contrato V7 actual.
8. Corregir respuestas antiguas, ausencia/error, descarte y edición obsoleta.
9. Corregir orden de candidatos y contrato de recepción con Taller.
10. Ejecutar aceptación funcional, rendimiento y recorrido manual. Mantener pruebas económicas integradas con POS para su etapa, con el contrato de Ficha preparado.

La implementación nueva resuelve una parte real de los problemas de persistencia. El cierre sigue dependiendo de completar los recorridos accesibles y corregir el historial; no puede aprobarse todavía por la existencia de los nuevos archivos.
