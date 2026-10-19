# Auditoría de Ficha Vehicular e Historial — versión 7

Fecha de revisión: 18 de octubre de 2026, America/Lima.

**Resultado: 40/100. No aprobar todavía el cierre del módulo.**

## Alcance y método

Se contrastó el código presente en el workspace, incluidos los cambios locales sin commit, con las 12 páginas de `03_Modulo_Ficha_Vehicular_e_Historial_v7_Simplificado.pdf` y las 15 páginas de `Informe_General_Software_Lubricentro_Gloria_v7_Alineado.pdf`, ambos del 15/10/2026. El informe general define reglas compartidas; los campos y excepciones se toman del documento específico.

La evaluación respeta la secuencia del proyecto: este es el segundo módulo que se está trabajando después de Login, aunque documentalmente sea el módulo 03. No exige terminar POS, Inventario, Caja, Dashboard, Administración, Reportes ni Backup para aprobar Ficha. Sí exige que Ficha cumpla sus reglas y que sus contratos de integración estén definidos y sean comprobables.

Se revisaron React, TypeScript, servicios de invocación, comandos Tauri, servicios Rust, DTO, esquema SQLite V7, permisos emitidos por Login y pruebas existentes. Se ejecutaron pruebas adicionales en memoria y React con servicios simulados. Se inspeccionó en modo solo lectura el esquema de la base situada en el directorio de datos de la aplicación: `user_version=700`, vistas de historial/candidatos presentes y ausencia de `id_zanja_prevista`.

No se realizó un recorrido manual de la ventana instalada ni una prueba completa mediante IPC real de Tauri. Tampoco se certificó que el ejecutable previamente compilado incorpore todos los cambios locales actuales. No se modificaron datos de la aplicación ni su implementación. Las pruebas temporales de diagnóstico se retiraron al terminar; este documento conserva los resultados.

## Calificación

La nota es una valoración técnica ponderada de requisitos, funcionamiento y evidencia. No es porcentaje de cobertura ni porcentaje de pruebas aprobadas. Las dependencias completas de otros módulos no reciben una penalización independiente.

| Área | Puntos obtenidos | Máximo | Nota del área |
|---|---:|---:|---:|
| Clientes, vehículos y validaciones | 11 | 20 | 55/100 |
| Historial público e identidad histórica | 6 | 20 | 30/100 |
| Roles, sesión y privacidad | 9 | 15 | 60/100 |
| Cambios administrativos y auditoría | 3 | 15 | 20/100 |
| Próximos cambios | 4 | 10 | 40/100 |
| Interfaz, errores y respuestas asíncronas | 3 | 10 | 30/100 |
| Pruebas y evidencia de aceptación | 4 | 10 | 40/100 |
| **Total** | **40** | **100** | **40/100** |

Los puntos de validaciones incluyen controles efectivos de SQLite: no se considera ausente una validación solo porque no esté repetida en React o Rust. Una función del backend que no se puede utilizar desde la interfaz recibe crédito parcial.

## Matriz de requisitos funcionales

| Requisito | Estado | Evaluación |
|---|---|---|
| RF-FVH-01: búsqueda normalizada; ficha, ausencia y error distintos | Parcial | La interfaz normaliza placa y Enter busca. Al iniciar búsqueda limpia ficha/historial. Sin embargo, un fallo de almacenamiento también presenta «Vehículo no encontrado»; respuestas antiguas pueden reemplazar la selección actual. En llamada directa con guion, el vehículo se encuentra pero su historial queda vacío. Falta Limpiar. |
| RF-FVH-02: cliente solo/con vehículo y reutilización | Parcial | Hay comando de cliente solo y alta conjunta transaccional. La pantalla no ofrece Nuevo cliente ni búsqueda/selección por tipo y documento. La reutilización automática compara solo número y puede vincular un CE con el DNI de otra identidad. El cajero no recibe permisos de crear Ficha desde Login. |
| RF-FVH-03: edición común sin reescribir identidad histórica | Parcial | El administrador puede editar campos comunes. No hay Editar cliente separado; cambiar documento se interpreta como seleccionar/crear otro propietario. No existe operación de edición independiente de cliente. El cajero no recibe permiso de editar Ficha. Borrar un contacto no lo elimina por el uso de `COALESCE`. |
| RF-FVH-04: historial público paginado de OT finalizadas | No cumple | La consulta no filtra FINALIZADO, usa placa histórica en lugar de ID estable, carga todo, ordena por recepción y muestra km de ingreso y pedido del cliente. Faltan fecha/hora final, cliente histórico y marca Corregido en el DTO Rust. |
| RF-FVH-05: placa/propietario/reducción km con administrador y motivo | No cumple | La placa está deshabilitada para todos; el backend ignora `id_vehiculo` y no actualiza placa. El cambio de propietario permite guardar sin motivo. La reducción de km controla rol y motivo mínimo, pero admite más de 250 caracteres. No hay confirmación específica al guardar cambios sensibles. |
| RF-FVH-06: corrección pública y auditoría antes/después | No cumple | Hay comando y actualización SQL con recálculo, pero falta acción/formulario en el detalle. Los JSON antes/después se calculan y no se guardan. El actor termina sin identificar. El DTO no devuelve `corregido` ni los valores públicos corregidos. |
| RF-FVH-07: baja/reactivación del vehículo | Parcial, bloqueado en UI | El backend permite ambos estados, conserva consulta y bloquea baja con OT abierta mediante trigger. En pantalla no hay campo de motivo, así que nunca se supera la validación para desactivar. No hay Reactivar ni indicador claro de inactividad; Crear OT aparece para inactivos. |
| RF-FVH-08: candidatos por km/fecha con teléfono | Parcial, no accesible en UI | La vista SQL calcula correctamente el último cambio, intervalos, 500 km y 180 días. El componente existe pero no está importado ni montado en la página. Faltan filtros/buscador/páginas de 20; el servicio ordena de más nuevo a más antiguo y admite mecánicos. |
| RF-FVH-09: acceso a recepción autorizada sin efectos ajenos | Parcial con fallo de integración | Existe botón/modal y no se crea mantenimiento al abrirlo. El modal llama `listar_vehiculos_cmd`, inexistente; su fallo impide precargar la placa. El servicio de creación usa `id_zanja_prevista`, ausente en V7. El permiso de crear OT se muestra al mecánico aunque el comando lo rechaza. |

## Requisitos no funcionales

| Requisito | Estado y evidencia |
|---|---|
| RNF-FVH-01: hasta 3 s, 1.000 vehículos, páginas de 20 | No acreditado. No hay paginación en historial/candidatos. El benchmark existente está deshabilitado mediante `tests_deshabilitados_temporalmente`, usa otro volumen y contiene una llamada desactualizada. No se ejecutó una medición de aceptación en el equipo objetivo. |
| RNF-FVH-02: teclado, opcionales, unidades, captura ante errores | Parcial. Enter busca, los formularios emplean controles nativos y conservan captura tras errores. Faltan errores junto a cada campo, límites consistentes, selección de motor y confirmación de descarte. Ficha no registra cambios pendientes en el mecanismo global de Login. |
| RNF-FVH-03: privacidad real en backend | Parcial favorable. Comprobado que contactos se eliminan en ambos niveles del DTO del vehículo para mecánico. La consulta de Ficha no devuelve costos, márgenes ni `nota_interna`. El comando de candidatos permite mecánicos aunque el módulo prohíbe ese acceso. |
| RNF-FVH-04: atomicidad y unicidad persistente | Parcial. Placa y tipo/número son únicos en SQLite; comprobado rollback del cliente nuevo si falla el vehículo dentro de la transacción usada por el comando. Auditoría se ignora si falla; cambio de estado/corrección no se agrupan con ella en una transacción. No se comprueba versión esperada para rechazar una edición obsoleta. |
| RNF-FVH-05: local, Lima, km enteros, errores reales | Parcial. SQLite y comandos funcionan localmente; km enteros y rango controlados por tipos/SQL; fechas SQL usan -5 horas. Las consultas convierten algunos errores de lectura en NotFound y la lista usa `rows.flatten()`, que descarta errores. La interfaz usa fecha del equipo sin zona explícita; el caso de instantes externos requiere verificación. |

## Hallazgos que impiden aprobar

### 1. El historial muestra hechos que todavía no son mantenimientos terminados

Origen: `src-tauri/src/services/ordenes_service.rs:562`.

La consulta filtra `placa_snapshot` y no incluye `estado='FINALIZADO'`. Una OT EN_ESPERA apareció como una fila del historial en la prueba adicional. Tampoco pagina ni usa fecha de finalización. El PDF exige exclusivamente trabajos finalizados, aunque no estén cobrados.

### 2. Los datos de mantenimiento presentados son distintos de los guardados al finalizar

Origen: `src-tauri/src/services/ordenes_service.rs:599` y `src-tauri/src/dto/orden_trabajo_dto.rs:14`.

La salida usa `kilometraje_ingreso`, `pedido_cliente` y `fecha_ingreso`. Calcula próximo km como ingreso + 5.000. Para un sintético con ingreso 50.000 y salida 50.010, devolvió **55.000** en lugar de **60.010**. El resumen fue el pedido de recepción en lugar del resumen final público.

El tipo de aceite se busca en el nombre actual del catálogo de productos, no en `tipo_aceite` de la OT. El resultado puede cambiar al renombrar un producto y no expresa necesariamente Mineral/Sintético. Falta cliente/documento histórico; `corregido` tampoco existe en el DTO Rust aunque React espere recibirlo.

### 3. Auditoría incompleta y no obligatoria para confirmar

Origen: `src-tauri/src/services/cliente_service.rs:555` y `src-tauri/src/services/auditoria_service.rs:55`.

Las llamadas de Ficha envían identidad del usuario sin snapshots de username/rol; el servicio degrada el actor a `NO_IDENTIFICADO` y elimina `id_usuario`. En corrección, `_antes_json` y `_despues_json` no se insertan. Las llamadas usan `let _ =`, ignorando fallos.

Se provocó un fallo de INSERT en auditoría: desactivar devolvió éxito y dejó el vehículo inactivo. El PDF exige guardar operación y trazabilidad completas o ninguna. El alta de cliente solo tampoco registra autor/auditoría.

### 4. Permisos de Login y acciones de Ficha no coinciden

Origen: `src-tauri/src/services/auth_service.rs:798` y `src/pages/vehiculo/FichaVehicularPage.tsx:45`.

El cajero recibe `puede_crear=false` y `puede_editar=false` para FICHA_VEHICULAR, aunque sus comandos permitan ambas acciones. El mecánico recibe permiso genérico de crear OT y Ficha lo utiliza para mostrar el botón de recepción; el comando de crear OT sí lo rechaza. Esto exige ajustar el contrato entre Login y Ficha, sin rehacer Login completo.

### 5. Las operaciones administrativas de placa y propietario están incompletas

Origen: `src/pages/vehiculo/components/ModalVehiculo.tsx:263` y `src-tauri/src/services/cliente_service.rs:289`.

La placa está deshabilitada al editar. El servicio busca el registro por la placa enviada y no utiliza su ID estable; intentar NEW123 para el ID existente devuelve «no encontrado». No hay UPDATE de placa.

Se cambió el propietario como administrador sin motivo y la operación fue aceptada. La razón solo se exige al reducir km y únicamente con longitud mínima. El formulario no distingue permisos por campo. Cambiar documento de un mismo cliente puede crear otro cliente y se procesa como transferencia de propiedad, no como edición de documento.

### 6. Baja/reactivación no se puede completar desde pantalla

Origen: `src/pages/vehiculo/FichaVehicularPage.tsx:157` y `:299`; `src/components/ConfirmModal.tsx:5`.

El motivo inicia vacío y solo puede restablecerse a vacío; el diálogo no admite input ni textarea. Desactivar siempre falla antes de llamar al backend. No existe acción Reactivar y los botones no se adaptan al estado operativo. La prueba de pantalla confirmó ambos problemas.

### 7. Próximos cambios existe como pieza aislada

Origen: `src/pages/vehiculo/components/FichaVehicularProximosCambios.tsx:10`, `src-tauri/src/services/cliente_service.rs:683` y `src-tauri/src/commands/cliente_commands.rs:197`.

No hay import ni render del componente desde la página ni pestaña de acceso. No existen búsqueda por cliente/placa, filtros por razón o paginación. El servicio ordena DESC, en lugar de antigüedad ASC e ID de desempate. El comando admite MECANICO, aunque le retire el teléfono. No hay llamadas ni ocultación, lo cual sí coincide con el alcance simplificado.

### 8. Identidades distintas pueden reutilizarse incorrectamente

Origen: `src-tauri/src/services/cliente_service.rs:138`, `:218` y `:344`.

La unicidad SQL usa tipo/número, pero búsqueda y reutilización usan solo número. Se registró un vehículo para CE 12345678 y quedó vinculado al cliente DNI 12345678 existente. Es una asociación incorrecta, no una mera validación visual pendiente.

### 9. Las respuestas antiguas sustituyen resultados recientes

Origen: `src/pages/vehiculo/FichaVehicularPage.tsx:90`.

No hay contador de búsqueda, cancelación ni comprobación de selección al resolver. Se lanzaron consultas A y B, se resolvió B primero y A después: la pantalla terminó mostrando A con el buscador de B. El botón Buscar bloqueado no evita carreras por cambios de `placaInicial`.

### 10. Un error se presenta también como ausencia

Origen: `src/pages/vehiculo/FichaVehicularPage.tsx:219`, `src/pages/vehiculo/components/FichaVehicularEmptyState.tsx:116` y `src-tauri/src/services/cliente_service.rs:645`.

Ante un fallo de almacenamiento simulado aparecen a la vez el mensaje de error y «Vehículo no encontrado», con invitación a registrar. El frontend no clasifica ausencia frente a fallo. Algunas lecturas Rust convierten cualquier error de fila en NotFound y la carga de órdenes utiliza `catch(() => [])`.

## Otros pendientes del propio módulo

- Falta Nuevo cliente accesible y Editar cliente independiente de Editar vehículo.
- Falta consultar/seleccionar cliente por tipo/documento antes del alta; ante duplicados no se ofrece navegar al existente.
- Falta corregir mantenimiento desde su detalle, confirmación administrativa y actualización de ficha/lista posterior.
- Falta cabecera o navegación equivalente con contexto de negocio/usuario/ambiente y pestañas previstas. Existe información de sesión en el contenedor, pero no se acredita todo el diseño requerido en Ficha.
- Faltan resumen de último servicio y último cambio en la ficha principal.
- Falta regreso al origen: `onVolver` está declarado pero no se utiliza; App tampoco entrega los callbacks de regreso/navegación de esa página.
- La edición de un contacto vacío conserva el anterior por `COALESCE`, impidiendo retirarlo.
- El formulario permite motor libre, por ejemplo «1.5L 1NZ-FE», pero SQLite acepta solo GASOLINA/DIESEL/GLP/GNV/HIBRIDO/ELECTRICO/OTRO. Un texto sugerido por la propia UI puede fallar al guardar.
- Año usa mínimo 1980 en React, frente a 1900 en el PDF/SQL. El máximo SQL sí controla año Lima + 1.
- El teléfono no normaliza +51: el backend rechaza el formato y React elimina separadores y trunca a nueve dígitos, pudiendo guardar un número distinto si se pega el prefijo.
- Nombre/marca/modelo y longitudes de contactos tienen restricciones SQL, pero no presentación consistente de errores por campo en React.
- La reducción de km admite motivo de 251 caracteres; debe respetar 5–250. El conteo de cadenas Rust usa bytes, aspecto pendiente para nombres/motivos con caracteres multibyte.
- El kilometraje cero se convierte en campo vacío al precargar edición mediante `valor || ''`.
- El estado modificado de los formularios no se comunica a `setHayCambiosSinGuardar`; Cancelar/navegar puede perder captura sin la confirmación requerida.
- No hay control de versión esperada en las escrituras. Bloquear conexiones con un Mutex no detecta que el usuario está guardando un formulario antiguo.
- No hay páginas de 20 y el historial hace consultas de detalles/aceite por cada OT, sin límite.
- La lista de candidatos usa `rows.flatten()`: un error de conversión de una fila se omite en lugar de propagarse.
- Las bajas bloqueadas por trigger terminan como error genérico de almacenamiento, sin identificar la OT abierta que debe resolverse.
- La consulta directa de ficha con ABC-123 normaliza el vehículo pero pasa el valor original al historial. La ruta frontend habitual lo normaliza antes y evita ese caso, pero el contrato backend sigue incompleto.
- Si se cambia placa en SQL conservando ID e identidad histórica, la ficha nueva pierde su historial por filtrar `placa_snapshot`. No se borran las OT; se pierde su asociación en la consulta.
- Las fechas React usan configuración del equipo. Se comprobó que un instante con Z cambia de día entre America/Lima y Asia/Tokyo. Las cadenas SQL locales sin zona no se desplazan en esa comprobación; falta estandarizar el contrato y validar medianoche Lima.

## Dependencias y límite de la aprobación por etapas

**Integración con Taller, sin evaluar todo Taller:**

1. `ModalCrearOT` llama `listar_vehiculos_cmd`, que no está implementado ni registrado en Tauri. El catch solo escribe consola y no ejecuta la búsqueda de placa de respaldo; por ello no se selecciona automáticamente el vehículo de origen.
2. `OrdenesService::crear_orden_trabajo` intenta insertar `id_zanja_prevista`. Se reprodujo el error «table ordenes_trabajo has no column named id_zanja_prevista» con el esquema real V7. También se confirmó su ausencia en la base de la aplicación, leída sin cambios.
3. El modal exige mecánico, zanja disponible y tipo de aceite en recepción, contrario al destino simplificado con responsable opcional y cola sin reserva de zanja. Esto debe resolverse al alinear el contrato de recepción, sin ampliar esta auditoría a todo Taller.
4. La finalización actual pone `id_zanja_ejecucion=NULL`, aunque el esquema V7 exige conservar esa referencia para FINALIZADO. Es un riesgo adicional detectado por lectura; no se ejecutó como parte de las pruebas adicionales. Las pruebas existentes de Ficha crean la atención finalizada mediante SQL directo, por lo que no acreditan el flujo real de Taller.

La persistencia del historial frente a cobro, devolución y rechazo fiscal debe comprobarse más adelante en integración con POS. En esta etapa puede aprobarse el contrato mediante datos controlados y comprobar que Ficha no depende de una venta para listar una OT terminada. Hoy ya hay incumplimientos propios de esa consulta, independientemente de POS.

Dashboard y Administración se registran como dependencias posteriores: no se exige terminar sus pantallas aquí. La producción de auditoría completa sí corresponde a Ficha; su visor se verificará al trabajar el módulo propietario. No se da por realizada la migración de bases antiguas ni por terminada la separación global de ambientes en esta auditoría.

## Comprobaciones favorables

- Fuente del historial basada en OT, sin crear una tabla de mantenimiento paralelo en este flujo.
- Unicidad de placa y de tipo/número de documento en persistencia.
- Cliente sin vehículo admitido en backend y esquema.
- Alta conjunta transaccional desde el comando; fallo por placa duplicada no deja cliente parcial.
- Reutilizar cliente durante alta no actualiza sus datos existentes.
- Placas normalizadas en la ruta habitual de interfaz.
- Datos opcionales admitidos y km enteros/rango respaldados por SQLite.
- Dígito de control del RUC comprobado en triggers; prueba adicional de RUC incorrecto rechazada.
- Contactos del mecánico excluidos del DTO anidado y de los campos superiores.
- Sin costos/márgenes ni nota interna en la respuesta de Ficha revisada.
- La baja con OT abierta se bloquea; la consulta de vehículos inactivos se conserva.
- La vista de candidatos utiliza último cambio finalizado por fecha e ID, vehículos activos y propietario actual.
- Se comprobaron los cuatro límites: 179 días/59.509 km sin candidato, 179/59.510 Por km, 180/59.509 Por fecha y 180/59.510 Ambos, para próximo 60.010.
- Corregir tipo/km recalcula `km_proximo` mediante columna generada y no cambia km actual del vehículo.
- SQLite rechaza convertir una atención sin cambio de aceite en cambio de aceite mediante corrección.
- Los comandos de Ficha validan sesión operativa y roles; el mecánico no tiene comandos de escritura de Ficha autorizados.
- No se exige contraseña adicional en una sesión administrativa vigente.
- No hay registro de llamadas, ocultación por 30 días ni estimación de recorridos en el flujo revisado.
- Las escrituras propias de Ficha revisadas no consumen stock, cobran ni emiten comprobantes.

## Pruebas ejecutadas y límites

| Comprobación | Resultado |
|---|---|
| `npm run typecheck` | Correcto |
| Pruebas existentes de Ficha en React | 4 aprobadas de 4 |
| `cargo test --test vehiculo_tests --offline` | 6 aprobadas de 6 |
| Pruebas adicionales Rust, aisladas en memoria | 20: 6 aprobadas y 14 fallidas |
| Pruebas adicionales React, con servicios simulados | 5: las 5 fallaron |
| Esquema de la base de la aplicación | Lectura correcta; V7 y columna de recepción ausente |
| Fecha con instante externo y zonas distintas | Se confirmó que la presentación actual depende de la zona del equipo |

Las pruebas adicionales expresaron el comportamiento esperado por el PDF. Sus fallos reproducen defectos; no son fallos de compilación del producto. Se corrigieron preparaciones de fixtures y esperas asíncronas antes de obtener los resultados definitivos anteriores. Las repeticiones incluyeron las pruebas originales: sus aprobaciones no se cuentan como casos nuevos.

Los 14 fallos Rust fueron: historial con OT abierta; próximo km sintético; resumen final público; autor/antes/después de corrección; propietario sin motivo; cambio de placa por ID; normalización telefónica; rollback ante fallo de auditoría; reutilización por tipo y número; historial con placa con guion; marca Corregido en DTO; límite de motivo de reducción; historial tras cambio de placa; recepción compatible con V7.

Los 6 casos Rust adicionales aprobados fueron: exclusión de contactos mecánico; dígito RUC; prohibición de agregar aceite a una atención sin cambio; recálculo SQL sin alterar km actual; límites km/fecha; rollback del alta conjunta.

Los 5 fallos React fueron: ingresar motivo de baja; ofrecer Reactivar; acceder a Próximos cambios; distinguir fallo de ausencia; descartar respuesta antigua.

Las pruebas existentes tienen cobertura funcional limitada: Auth está simulado con todos los permisos habilitados, el caso Rust llamado autorización verifica una lista fija de roles y la atención finalizada se prepara con SQL directo. No verifican los nueve RF, cinco RNF ni los diez TC del PDF de extremo a extremo.

No se ejecutó aceptación manual en Windows/WebView2, medición de rendimiento con 1.000 vehículos, flujo IPC completo con los tres roles, restauración/cambio de ambiente ni integración fiscal/económica. Esos resultados permanecen **NO EJECUTADOS**, sin presentarlos como aprobados ni como fallos funcionales demostrados.

## Condiciones para pasar al siguiente módulo

Antes de considerar Ficha cerrada:

1. Alinear permisos de cajero/mecánico y separar campos comunes y administrativos.
2. Completar cliente solo, edición independiente y selección inequívoca por tipo/documento.
3. Consultar historial FINALIZADO por ID estable, con DTO público correcto, datos finales, snapshot histórico y páginas de 20.
4. Habilitar placa/propietario/reducción km con motivo, confirmación y controles en backend.
5. Completar corrección desde el detalle y persistir autor, motivo y antes/después de forma atómica.
6. Hacer utilizables baja/reactivación, estado visible y elegibilidad de nueva OT.
7. Conectar Próximos cambios con permisos, filtros, orden y paginación requeridos.
8. Resolver respuestas antiguas, fallos frente a ausencia y descarte de captura.
9. Corregir el enlace de recepción o dejar un contrato verificable preparado para Taller, documentando qué prueba integrada queda pendiente. No declarar funcional el flujo actual que falla contra V7.
10. Ejecutar los diez TC-FVH y RNF aplicables con evidencia de UI, backend y persistencia. La validación económica completa se repetirá al abordar POS.

El avance posterior de otros módulos puede continuar si el responsable decide priorizarlo, pero Ficha debe conservar estado de pendiente: la nota o la existencia de pantallas no sustituyen los criterios de aceptación del PDF.
