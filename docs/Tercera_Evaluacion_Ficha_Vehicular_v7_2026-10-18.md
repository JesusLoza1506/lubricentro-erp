# Tercera evaluación — Ficha Vehicular e Historial V7

Fecha: 18 de octubre de 2026, America/Lima.

**Nota: 64/100. Evaluación anterior: 48/100. Estado: todavía no aprobar el cierre.**

## Alcance y evidencia

Revisión nueva de los fuentes actuales, cambios locales y archivos nuevos del módulo, sus comandos/DTO, auditoría, permisos de Login y contrato con Taller. Referencia: PDF específico de Ficha V7 y documento general alineado, ambos del 15/10/2026. Se contrastó también con los bloqueos de la segunda revisión.

Se ejecutaron pruebas sobre SQLite en memoria con el esquema real V7, pruebas React con servicios simulados, TypeScript y las suites existentes de Ficha/Login. Se comprobó en modo solo lectura la base del directorio de datos de la aplicación: versión 700 y ausencia de `id_zanja_prevista`. No se alteraron los datos ni la implementación; se retiraron las pruebas temporales al terminar.

No se hizo un recorrido manual de la ventana instalada ni IPC completo de Tauri. No se acredita rendimiento en el equipo objetivo ni correspondencia de su ejecutable con todos los fuentes actuales. No se ejecutaron scripts de ingesta ni migraciones sobre datos del usuario.

Se respeta el desarrollo por módulos: no se exige terminar POS, Caja, Inventario, Dashboard, Administración, Reportes ni Backup. La consulta de historial usada por Ficha y sus permisos propios sí deben cumplir el contrato, aunque su código se ubique en archivos compartidos con Taller/Login.

## Nota con los mismos pesos

| Área | Segunda revisión | Tercera revisión | Máximo |
|---|---:|---:|---:|
| Clientes, vehículos y validaciones | 12 | 15 | 20 |
| Historial público e identidad histórica | 6 | 12 | 20 |
| Roles, sesión y privacidad | 10 | 8 | 15 |
| Cambios administrativos y auditoría | 7 | 11 | 15 |
| Próximos cambios | 5 | 8 | 10 |
| Interfaz, errores y respuestas asíncronas | 4 | 5 | 10 |
| Pruebas y evidencia de aceptación | 4 | 5 | 10 |
| **Total** | **48** | **64** | **100** |

Es una valoración técnica ponderada, no cobertura ni tasa de tests. La bajada en permisos refleja dos problemas comprobados: Login ahora anuncia escritura de Ficha para mecánico y el backend permite cambiar placa al cajero. Una nota mayor no sustituye resolver los bloqueos de integridad y autorización.

## Mejoras verificadas desde la segunda evaluación

| Punto anterior | Resultado actual |
|---|---|
| Motor incompatible con SQL | Corregido: opciones GASOLINA/DIESEL/GLP/GNV/HIBRIDO/ELECTRICO/OTRO y opción vacía. Probadas contra SQLite. |
| Teléfono validado pero no normalizado al persistir | Corregido en backend: `+51 912-345-678` se guarda como `912345678`. El formulario de vehículo ya conserva el texto para normalizarlo. El modal de cliente aún trunca al capturar. |
| Comando de cliente no registrado | Corregido: `actualizar_cliente_cmd` está en `generate_handler!`. |
| Componentes aislados | Se conectaron Gestión Cliente, Corregir en filas de historial y pestaña Próximos cambios. No significa que todos sus flujos estén completos. |
| Motivo ausente en baja/reactivación | Corregido: ConfirmModal captura motivo y valida 5–250. Se probaron ambos envíos con su payload. |
| Historial incluye OT abiertas | Corregido: consulta solo FINALIZADO; prueba aprobada. |
| Pedido inicial en lugar de resumen | Corregido en historial: devuelve resumen público final/corregido. |
| Fecha de recepción en historial | Corregido en la consulta de historial: usa fecha final, aunque se entrega bajo el nombre `fecha_ingreso`. Falta mostrar hora y corregir la semántica del DTO. |
| Aceite tomado siempre del catálogo | Mejorado: prioriza `tipo_aceite` guardado en OT. Persiste un fallback al catálogo si no hay tipo. |
| Falta `corregido` en DTO Rust | Corregido y probado junto con el resumen vigente. |
| Cambiar placa pierde historial | Mejorado: consulta también por ID del vehículo actual; conserva su historia después del cambio. La condición OR introduce mezcla al reutilizar placa, descrita abajo. |
| Backend no actualiza placa por ID | Corregido funcionalmente: actualiza por ID y conserva identidad. Falta autorización/motivo y habilitación UI correcta. |
| Cambio de propietario sin motivo | Corregido: rechaza ausencia y controla 5–250. |
| Motivo de reducción mayor de 250 | Corregido: 251 rechazado. |
| Cajero sin alta/edición comunes | Corregido en permisos entregados por Login. Se comprobó con autenticación real en memoria. |
| Auditoría usa módulo AUTORIZACION inválido | Corregido a POS y restaurado `id_autorizante`. Sigue sin referencia de venta en `id_entidad`; revisar al integrar POS. |
| Pruebas existentes muy limitadas | Se añadieron cuatro pruebas Rust: ahora pasan 10. Siguen sin cubrir todos los criterios del PDF. |

Se mantienen las mejoras ya verificadas en auditoría identificada, antes/después, transacciones, reutilización por tipo/número, privacidad del mecánico y estado de inactivos.

## Bloqueos prioritarios

### 1. El cajero puede cambiar placa desde backend

Referencias: `src-tauri/src/services/cliente_service.rs:482`, `:525`; `src-tauri/src/commands/cliente_commands.rs:141`.

La nueva rama de placa solo comprueba duplicados. No exige ADMINISTRADOR ni motivo. El comando admite CAJERO y pasa su rol al servicio, que no lo comprueba para la placa.

Se creó una cuenta cajero ficticia y se envió una edición con ID estable y nueva placa: la operación fue aceptada. También se aceptó el cambio como administrador sin motivo. El PDF exige rol administrativo y motivo en ambos casos.

La interfaz sigue deshabilitando placa para todos (`ModalVehiculo.tsx:270`), por lo que falta además la edición autorizada de ese campo en pantalla. Ocultarlo no sustituye el control backend.

### 2. La búsqueda de historial puede mezclar vehículos distintos

Referencia: `src-tauri/src/services/ordenes_service.rs:602`.

La condición ahora es placa histórica coincidente **o** ID actual coincidente. Conserva la historia del vehículo cuya placa cambió, pero también incorpora OT de un ID diferente que tuvo esa placa antes.

Escenario reproducido: vehículo A con atención ABC123; A se corrige a NEW123; se registra vehículo B con ABC123. La ficha de B muestra la atención de A aunque B no tiene ninguna finalizada. Debe resolverse primero el vehículo actual y consultar exclusivamente sus OT por ID estable. La placa histórica debe mostrarse como dato de la atención.

### 3. Próximo kilometraje todavía se calcula desde el ingreso

Referencias: `src-tauri/src/services/ordenes_service.rs:47`, `:628`.

La selección del intervalo sintético mejoró, pero la base continúa siendo `kilometraje_ingreso`. No se lee el `km_proximo` ya calculado en SQLite a partir de la salida y el intervalo guardado.

Resultados reproducidos:

| Escenario | Esperado | Actual |
|---|---:|---:|
| Ingreso 50.000, salida 50.010, sintético | 60.010 | 60.000 |
| Corrección de salida a 50.020, mineral | 55.020 | 55.000 |
| Atención sin cambio de aceite | Sin próximo cambio registrado | 55.000 |

El DTO tampoco entrega `kilometraje_salida`; interfaz y formulario de corrección siguen usando el de ingreso. Una corrección solo de texto puede enviar ese kilometraje precargado y sustituir la lectura final original. Este último riesgo está identificado por lectura del formulario; no se simuló su commit completo por IPC.

### 4. Editar cliente hace dos solicitudes de actualización

Referencias: `src/pages/vehiculo/components/ModalCliente.tsx:100`, `:110`; `src/pages/vehiculo/FichaVehicularPage.tsx:210`, `:469`.

El modal guarda con `actualizarCliente`, construye el cliente actualizado y llama `onClienteGuardado`. La página recibe ese callback con `handleGuardarCliente`, que vuelve a llamar `actualizarCliente`.

Una prueba React con datos ya cargados y un envío de formulario produjo **dos llamadas**. Ambas pueden crear auditoría; si la segunda falla, puede quedar una primera escritura confirmada y un tratamiento de error incoherente. El callback está tipado `void` y no se espera su promesa. Debe haber un solo responsable del guardado y un callback de éxito que actualice la pantalla.

### 5. Los permisos nuevos del mecánico son incorrectos

Referencia: `src-tauri/src/services/auth_service.rs:798`.

Al corregir al cajero, también se añadió FICHA_VEHICULAR a `puede_crear` y `puede_editar` del mecánico. Se comprobó mediante `AuthService::login` con credenciales ficticias y hash real. El resultado anuncia escritura de Ficha para ese rol, contrario al PDF.

Los comandos de escritura siguen rechazando mecánico; no se afirma que pueda completar esas escrituras. Sin embargo, ve acciones que terminan en denegación. Además, la pestaña Próximos cambios se muestra sin filtro de rol; su comando sí lo rechaza. La prueba de ocultación de esa pestaña falló.

### 6. Cliente solo aún no tiene entrada desde la pantalla

Existe el modo CREAR del modal y el comando de alta. La página solo monta el modal cuando hay `clienteEditar` y siempre con `modo="EDITAR"`. No existe Nuevo cliente accesible sin vehículo. La prueba correspondiente volvió a fallar.

También faltan búsqueda/selección explícita de cliente existente para alta de vehículo, resolución navegable de duplicados y separación completa de la edición de propietario frente a editar su identidad.

### 7. La corrección conectada sigue ofreciendo aceite incompatible

Referencia: `src/pages/vehiculo/components/ModalCorregirHistorialOT.tsx:97`.

El backend retiró SEMI_SINTETICO/7.500, pero el select todavía lo ofrece y fuerza el tipo con `as any`. PDF/SQL solo contemplan Mineral/Sintético; elegir esa opción falla en persistencia.

Además, una atención sin aceite recibe SINTETICO por defecto. Debe mantenerse sin tipo y habilitar el campo solamente si hubo cambio. La corrección aún carece de confirmación administrativa específica y límites de campos consistentes. Sus valores iniciales no se actualizan si cambia la OT manteniendo montado el componente.

### 8. Búsqueda antigua y error/ausencia siguen sin resolver

Referencia: `src/pages/vehiculo/FichaVehicularPage.tsx:109`.

Las pruebas reprodujeron de nuevo A/B resueltas en orden inverso que terminan mostrando A, y fallo de almacenamiento presentado simultáneamente como «Vehículo no encontrado». Falta identificación de la consulta vigente y distinguir NOT_FOUND de fallo real.

Siguen faltando Limpiar, descarte único de captura y comprobación de versión esperada antes de guardar un formulario antiguo. `catch(() => [])` de órdenes oculta errores.

### 9. Historial aún no cumple todo el contrato público

- Falta cliente/documento histórico de la atención: no se devuelven `cliente_snapshot`/`id_cliente_atendido` ni otra representación equivalente.
- Falta km final y próximo km correcto, descritos arriba.
- Falta paginación de 20; React renderiza el vector completo y Rust carga todos los detalles.
- Se muestra fecha sin hora y los campos reutilizan nombres de recepción.
- La corrección se abre desde fila; el detalle público no ofrece todavía el flujo completo previsto con identidad histórica y acceso autorizado al origen.
- El fallback de aceite al nombre actual de producto puede inventar un tipo para una atención que no tenga cambio registrado.

### 10. Recepción continúa incompatible con el esquema activo

Referencia: `src-tauri/src/services/ordenes_service.rs:242`.

Se reprodujo nuevamente `table ordenes_trabajo has no column named id_zanja_prevista`. La base real de la aplicación, leída sin modificarla, tampoco contiene esa columna. El modal sigue exigiendo mecánico/zanja en recepción y `listarVehiculos` usa solamente candidatos de aceite.

Se registra como contrato de integración con Taller. No se exige terminar todo Taller, pero este flujo no puede presentarse como funcional. Finalización y los efectos económicos completos deben verificarse en sus etapas correspondientes.

## Pendientes restantes por requisitos

| Requisito | Evaluación actual |
|---|---|
| RF-FVH-01 Buscar | Parcial: normaliza placa y limpia selección al iniciar; faltan Limpiar, ausencia/error y respuestas vigentes. |
| RF-FVH-02 Cliente/vehículo | Mejorado: motor y teléfono de vehículo correctos, tipo/número y atomicidad; falta Nuevo cliente y selección explícita. |
| RF-FVH-03 Edición común | Mejorado: Gestión Cliente conectado, registro Tauri correcto, cajero habilitado; doble envío y permisos mecánico incorrectos. |
| RF-FVH-04 Historial | Mejorado: finalizadas, resumen, fecha, aceite, Corregido; pendientes km, cliente histórico, aislamiento por ID y páginas. |
| RF-FVH-05 Cambios sensibles | Parcial: ID estable y motivo de propietario/reducción correctos; placa sin rol/motivo backend y deshabilitada UI. |
| RF-FVH-06 Corrección pública | Mejorado: accesible, auditada y transaccional; datos de lectura/km, aceite opcional y confirmación incompletos. |
| RF-FVH-07 Estado | Función principal conectada: baja/reactivación con motivo y recarga; falta bloqueo de envíos reiterados e identificar OT que bloquea. |
| RF-FVH-08 Candidatos | Accesible, filtros y páginas visuales; falta ocultar al mecánico, ordenar por antigüedad ASC/ID y ajustar página tras refresco. |
| RF-FVH-09 Recepción | Parcial, bloqueado por dependencia real de Taller; inactivos ya sin botón, mecánico aún lo ve por permiso genérico. |
| RNF-FVH-01 Rendimiento | No acreditado. Benchmark deshabilitado; no medición de cinco consultas/primeras páginas con 1.000 vehículos y equipo objetivo. |
| RNF-FVH-02 Usabilidad | Mejorado en motor/estado/motivo; faltan errores por campo, km cero precargado, descarte y confirmaciones. |
| RNF-FVH-03 Privacidad | Contactos y notas privadas siguen protegidos en backend; permisos UI y asociación del historial requieren corrección. |
| RNF-FVH-04 Integridad | Auditoría/transacciones mejoradas; faltan aislamiento de identidades, un envío por edición de cliente y control de formularios obsoletos. |
| RNF-FVH-05 Operación local | Local y SQL Lima conservados; faltan contrato de fechas/instantes uniforme, no ocultar errores y pruebas de zona/medianoche. |

Otros detalles que siguen pendientes:

- `onVolver` permanece declarado sin uso; App no entrega regreso autorizado al origen.
- No hay resumen completo de último servicio y último cambio en la ficha principal.
- Estado modificado de formularios no utiliza `setHayCambiosSinGuardar`.
- Edición precarga km cero con `valor || ''`, convirtiéndolo en vacío.
- Motor ya tiene valores correctos y es opcional; este punto debe retirarse de las listas de fallos anteriores.
- Backend normaliza teléfono; `ModalCliente` todavía elimina caracteres y trunca a nueve dígitos al capturar, por lo que un prefijo pegado puede alterar el número.
- Cliente/vehículo aún se editan conjuntamente en ModalVehiculo, permitiendo modificar datos del cliente destino durante una transferencia.
- Auditoría de edición de vehículo conserva placa/km/cliente, pero no todos los demás campos cambiados; cliente no conserva antes/después de contactos.
- Longitudes Rust cuentan bytes en algunos campos; falta verificar caracteres multibyte y validación de límites por campo.
- Candidatos siguen en `ORDER BY fecha_servicio DESC`; falta ASC e ID. `rows.flatten()` omite fallos de conversión.
- La paginación de candidatos no ajusta página actual si Actualizar reduce los resultados.
- No hay guardado en curso conectado a los diálogos de estado; pueden reenviar antes de recibir resultado.
- Errores de OT abierta siguen sin identificar la orden bloqueante.

## Resultados de pruebas

| Comprobación | Resultado |
|---|---|
| TypeScript del producto antes de pruebas temporales | Correcto |
| Pruebas existentes Rust Ficha | 10/10 aprobadas |
| Pruebas existentes React Ficha | 4/4 aprobadas |
| `auth_regression_tests` | 12/12 aprobadas |
| `auth_tests` | 18/18 aprobadas |
| Casos adicionales Rust | 22: 13 aprobados, 9 fallidos |
| Casos adicionales React | 11: 5 aprobados, 6 fallidos |

La primera ejecución Rust adicional incluyó las diez pruebas originales: 30 casos, 22 aprobados/8 fallidos; después se ejecutaron dos casos nuevos de permisos de Login, uno aprobado y uno fallido. La tabla descuenta las repeticiones. React incluyó los cuatro originales: 15 casos, 9 aprobados/6 fallidos. Se corrigió el selector del botón de Gestión Cliente y se esperó la carga del formulario antes del envío; se informa el resultado final, que reproduce dos llamadas.

Casos Rust adicionales aprobados: exclusión OT abierta; resumen final; aceite guardado; fecha final; Corregido/resumen vigente; historial tras cambiar placa; teléfono normalizado; motivo de reducción de 251 rechazado; propietario sin motivo rechazado; auditoría de corrección y km actual intacto; rollback de baja ante fallo de auditoría; todos los motores/opcional contra SQL; permisos comunes de cajero tras Login.

Casos Rust adicionales fallidos: km próximo desde salida; km corregido reflejado; placa reutilizada sin mezcla; cajero bloqueado al cambiar placa; motivo de placa exigido al admin; cliente histórico en DTO; ausencia de próximo cambio sin aceite; recepción V7; permisos de solo lectura para mecánico en Login.

Casos React adicionales aprobados: baja con motivo; reactivación con motivo; opciones de motor compatibles/opcional; acceso a candidatos; apertura de corrección. Fallidos: actualización única de cliente; Nuevo cliente accesible; candidatos ocultos para mecánico; tipos de aceite V7; respuesta antigua descartada; error distinto de ausencia.

Las suites existentes no detectaron los permisos de Ficha entregados al mecánico; ese fallo sí se comprobó con una prueba adicional de autenticación real. Un test existente llamado autorización continúa comparando una lista fija y no verifica el contrato completo.

No se presentan como aprobados el recorrido manual Windows/WebView2, IPC real, rendimiento, medianoche Lima ni los diez TC-FVH completos. Las pruebas de integración con cobro/devolución/rechazo fiscal se mantienen para POS, sin dar por comprobados esos efectos.

## Orden para cerrar

1. Bloquear cambio de placa por cajero y exigir motivo al administrador; habilitar la edición administrativa correcta desde UI.
2. Consultar historial exclusivamente por ID del vehículo resuelto, eliminando la mezcla por placa reutilizada.
3. Leer km final/intervalo/km próximo guardados, admitir ausencia y devolver identidad histórica pública.
4. Hacer una sola actualización de cliente por envío y esperar su resultado.
5. Restaurar solo lectura del mecánico y ocultar acciones/pestañas no permitidas.
6. Completar Nuevo cliente y selección/reutilización desde pantalla.
7. Ajustar corrección de aceite/km, eliminar semisintético del formulario V7 y añadir confirmación.
8. Resolver respuestas obsoletas, errores/ausencia, descarte y control de envío.
9. Terminar paginación/orden del historial y candidatos, y contrato de recepción con Taller.
10. Ejecutar aceptación con evidencia de interfaz, backend y persistencia; medir rendimiento y recorrer la aplicación de escritorio.

La mejora desde 48 a 64 refleja funciones que ahora sí están conectadas y verificadas. El cierre permanece pendiente por defectos concretos de permisos, identidad histórica y cálculo, no por exigir que los siguientes módulos estén completos.
