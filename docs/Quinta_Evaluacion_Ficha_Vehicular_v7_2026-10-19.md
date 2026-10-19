# Quinta evaluación — Ficha Vehicular e Historial V7

Fecha: 19 de octubre de 2026, America/Lima.

**Puntuación actual: 80/100. Anterior: 74/100. Todavía no se aprueba el cierre.**

## Alcance y criterio

Reevaluación del código actual, incluidos archivos nuevos sin commit y cambios compartidos de Login. Contrato: PDF específico `03_Modulo_Ficha_Vehicular_e_Historial_v7_Simplificado.pdf` y su alineación con `Informe_General_Software_Lubricentro_Gloria_v7_Alineado.pdf`, ambos V7 del 15 de octubre de 2026. Se volvió a extraer el PDF específico completo y a contrastar las secciones relevantes del general. Sus pasos y ejemplos son resultados esperados, no evidencia de ejecución.

Se mantiene el orden de trabajo indicado: Ficha es el segundo módulo que estamos evaluando después de Login. No se exige terminar Dashboard, Taller, Inventario o POS para subir la nota de Ficha. Sí se revisa el contrato de apertura/recepción OT que Ficha ofrece al usuario.

La nota es una valoración ponderada de cumplimiento con los mismos pesos anteriores; no es cobertura ni porcentaje de pruebas. No se toma como evidencia la estimación de avance del README. Un 90 numérico tampoco sustituye los criterios de aprobación del apartado 8.5 del PDF.

| Área | Cuarta | Quinta | Máximo |
|---|---:|---:|---:|
| Clientes, vehículos y validaciones | 16 | 16 | 20 |
| Historial público e identidad histórica | 15 | 16 | 20 |
| Roles, sesión y privacidad | 10 | 14 | 15 |
| Cambios administrativos y auditoría | 12 | 12 | 15 |
| Próximos cambios | 8 | 8 | 10 |
| Interfaz, errores y respuestas asíncronas | 7 | 8 | 10 |
| Pruebas y evidencia de aceptación | 6 | 6 | 10 |
| **Total** | **74** | **80** | **100** |

## Correcciones nuevas que sí se reconocen

1. **Contrato nullable corregido.** `src/types/vehiculo.ts:52` declara `number | null`. Tanto el historial como el detalle comprueban null/undefined. Dos casos React adicionales pasan: el detalle abre y la lista muestra «Sin cambio», sin error de renderizado ni próximo 0 km inventado.
2. **Login entrega Ficha de solo lectura al mecánico.** `auth_service.rs:804` restringe crear/editar a OT. Se comprobó mediante Login real contra SQLite en memoria: ver=true, crear/editar/eliminar=false en FICHA_VEHICULAR.
3. **Ficha oculta acciones al mecánico.** La página limita alta, edición, estado y recepción por rol; los accesos a cliente también tienen guardas. El caso React con mecánico pasa incluso simulando un proveedor genérico de permisos demasiado permisivo. Próximos cambios sigue oculto.
4. **Se añadió la presentación de identidad histórica.** El detalle y TypeScript ahora tienen campos de cliente histórico. Se reconoce el avance visual, pero este requisito sigue parcial: Rust no entrega esos campos. No se marca resuelto de extremo a extremo.

También se conservan las mejoras verificadas en revisiones anteriores: alta de cliente independiente, edición separada con un solo callback de guardado, motor opcional/códigos admitidos, teléfono normalizado en backend, documento compuesto y placa únicos, cambio administrativo de placa por ID estable y con motivo, historial por ID de vehículo y solo FINALIZADO, lectura final usada en el caso normal, corrección pública con marca y auditoría, bloqueo de bajas con OT abierta, contactos filtrados para mecánico y descarte de búsquedas antiguas.

Las pruebas actuales de vehículos vuelven a aprobar sus 12 escenarios, incluidos cajero sin cambio de placa, placa sin motivo y restricciones de propietario/km. Las 4 pruebas React existentes también pasan. Estas garantías reciben crédito; no prueban por sí solas todos los recorridos descritos en el PDF.

## Fallos reproducidos ahora

### 1. Cliente histórico: interfaz lista, respuesta real incompleta

Fuentes: `src/types/vehiculo.ts:44`, `src/pages/vehiculo/components/ModalDetalleOT.tsx:120`, `src-tauri/src/dto/orden_trabajo_dto.rs:14`, `src-tauri/src/services/ordenes_service.rs:635`.

Se creó una OT finalizada con `cliente_snapshot` de un cliente ficticio histórico y documento. Se consultó Ficha y se serializó el DTO real: no contiene el nombre ni documento del cliente atendido. El DTO Rust no tiene esos campos y el SELECT no los recupera. El detalle utiliza `nombre_cliente || 'N/A'`, por lo que agregar propiedades opcionales a TypeScript no completa el recorrido.

Además, la creación actual de OT guarda un snapshot con nombre solamente (`ordenes_service.rs:270`). Hay que capturar la identidad necesaria al recibir y devolverla desde ese snapshot; consultar el cliente actual no preserva el documento anterior si luego cambia. Debe definirse cómo mostrar ausencia en datos históricos que no tengan documento guardado, sin inventarlo.

**Aceptación:** recibir con cliente A, cambiar propietario y datos actuales, consultar la misma OT: mostrar cliente/documento y placa de aquella atención, con una fila y el mismo ID de vehículo. No tomar los valores actuales como sustituto histórico.

### 2. Un material ACEITE todavía fabrica un próximo cambio

Fuentes: `ordenes_service.rs:25`, `:47`, `:74`.

Caso con servicio no marcado como cambio, material de categoría ACEITE y tipo/intervalo nulos. La base conserva `km_proximo=NULL`, pero el historial devuelve `Some(55010)` para lectura final 50010. El helper infiere un cambio desde la categoría actual del producto y recalcula un intervalo por defecto.

**Corrección requerida:** respetar el hecho y el intervalo guardados en la atención. Evitar que una categoría o nombre actual de catálogo reinterprete el historial. Propagar errores de consulta en lugar de tratarlos como cero/ausencia. Historial y candidatos deben coincidir sobre si existió cambio.

**Aceptación:** el caso sin cambio sigue null aunque incluya material ACEITE; el sintético real a 50010 devuelve 60010. Este último caso ya pasa.

### 3. Editar placa continúa bloqueado para administrador

Fuente: `src/pages/vehiculo/components/ModalVehiculo.tsx:270`.

El input conserva `disabled={modo === 'EDITAR'}`. La prueba renderizada como administrador obtiene disabled=true. El backend de placa sí tiene las protecciones añadidas anteriormente; el pendiente está en completar el recorrido de pantalla.

Por lectura, el motivo se muestra solo cuando disminuye km (`:110`, `:353`), no al transferir propietario. El cliente destino no se selecciona explícitamente: el modal envía el ID anterior y el servicio resuelve cambios por documento. Tampoco existe confirmación adicional del efecto sensible al guardar, requerida por el PDF, sin pedir otra contraseña.

**Aceptación:** administrador cambia placa o selecciona propietario con motivo 5–250 y confirmación; cajero no puede editar esos campos; una OT abierta bloquea con identificación de esa OT; el historial conserva identidad anterior. Separar esta operación de editar el nombre/contactos del cliente.

### 4. Pegar +51 cambia el número de teléfono

Fuente: `src/pages/vehiculo/components/ModalCliente.tsx:159`.

Entrada `+51 912 345 678`: la prueba obtiene **519123456**, cuando debería obtener **912345678**. Se eliminan separadores y se corta a nueve antes de normalizar el prefijo. El backend y el modal de vehículo no corrigen un número que este formulario ya alteró.

**Aceptación:** teléfono opcional, normalización +51/separadores sin truncado silencioso y validación final de 7 o 9 dígitos. Probar tanto alta como edición del cliente.

### 5. Historial no tiene páginas de veinte

Fuentes: `FichaVehicularHistorial.tsx:43`, `ordenes_service.rs:635`.

La prueba entrega 21 OT y se renderizan las 21. El servicio tampoco limita páginas y carga los detalles de cada atención. La paginación de Próximos cambios no resuelve la de Historial.

**Aceptación:** 21 atenciones generan páginas de 20 y 1, orden de fecha final descendente e ID estable como desempate, sin duplicados al navegar. Alinear consulta y pantalla para no traer todos los detalles en la primera página.

### 6. Un fallo técnico se anuncia también como vehículo inexistente

Fuente: `src/pages/vehiculo/FichaVehicularPage.tsx:399`.

La prueba hace fallar la lectura: aparecen el error técnico, «Vehículo no encontrado», «No existe registro…» y la opción Registrar. La condición del estado vacío no excluye error. Se volvió a comprobar con una aserción que cuenta los dos mensajes incorrectos.

**Aceptación:** distinguir ausencia real de error técnico, permitir reintentar y conservar una captura válida. No ofrecer alta como conclusión de un fallo de lectura. Revisar también `listarOrdenesTrabajo().catch(() => [])` (`:127`) y conversiones indiscriminadas a NotFound en Rust.

### 7. Recepción OT sigue chocando con el esquema V7

Fuentes: `ordenes_service.rs:278`, `CrearOrdenTrabajoDto`, `ModalCrearOT.tsx`, trigger `trg_ot_nace_espera`.

Se llamó al servicio real con vehículo activo, mecánico válido y zanja disponible. SQLite rechazó la operación con **«OT nace en espera libre o asignada, sin ejecucion»**. La inserción coloca zanja de ejecución y tipo de aceite en una orden EN_ESPERA.

Ya no se usa la columna antigua `id_zanja_prevista`; ese error anterior no se repite. El problema actual es el contrato de nacimiento de la OT.

**Aceptación del contrato:** desde Ficha abrir recepción con vehículo precargado, revalidar permiso/activo/sin abierta en destino, admitir responsable opcional y crear espera sin datos de ejecución ni aceite final. La elección de zanja pertenece a Iniciar trabajo. La selección general de vehículos no debe depender de ser candidato a cambio de aceite. No se exige aquí implementar todo Taller o POS.

## Pendientes confirmados por lectura, sin nuevo caso ejecutado

| Pendiente | Evidencia y condición para cerrarlo |
|---|---|
| Orden de candidatos | `cliente_service.rs:957` mantiene `ORDER BY fecha_servicio DESC`. RB08 exige cambio más antiguo primero e ID como desempate. |
| Contexto/página de candidatos | `FichaVehicularProximosCambios.tsx` resetea página por filtros, pero no ajusta una página fuera de rango al reducir datos con Actualizar. Sus filtros viven en el componente desmontado al abrir Ficha. Conservar el contexto autorizado al regresar y refrescar datos. |
| Errores de filas | `cliente_service.rs:987` usa `rows.flatten()`, omitiendo errores. Debe fallar de manera explícita, sin presentar un conjunto parcial como completo. |
| Confirmar durante envío | ConfirmModal soporta `cargando`, pero los dos usos de estado en Ficha no lo conectan. Bloquear reiteraciones y gestionar resultado incierto antes de repetir. |
| Descarte de captura | Los formularios de Ficha no registran cambios con `setHayCambiosSinGuardar`. Conectar una sola confirmación al abandonar captura; la expiración automática debe salir sin esperar. |
| Escritura con captura antigua | Los DTO no transmiten un estado/valor esperado que permita detectar una edición basada en datos que cambiaron. La relectura actual valida restricciones, pero no demuestra detección de sobrescritura obsoleta. Resolver sin exigir un sistema de versiones del historial. |
| Vehículo inactivo | `cliente_service.rs:518` bloquea toda edición, también del administrador; ajustar la distinción por rol documentada. Consulta e historial deben seguir disponibles y nueva OT bloqueada. |
| Presentación de atención | El campo llamado ingreso ahora transporta lectura final y la UI aún dice «Ingreso». Mostrar km de atención con contrato claro; fecha y hora final. Completar resumen de último servicio/último cambio y vacíos «No registrado». |
| Navegación | `onVolver` está declarado y no usado. Completar regreso autorizado al origen y Abrir OT desde detalle, validando permiso en destino. La corrección pública está accesible desde lista; falta ajustarla al recorrido del detalle descrito. |
| Alta/reutilización | Backend reutiliza documento, pero falta búsqueda/selección explícita de cliente y enlace para consultar duplicado desde la validación, conservando la captura. No modificar al cliente existente al registrar otro vehículo. |
| Validación de pantalla | Errores junto al campo, Limpiar del buscador y precarga correcta de km=0; `valor || ''` vacía el cero. Revisar longitudes Unicode: `.len()` Rust mide bytes y el contrato habla de caracteres. |
| Hora independiente de Windows | `Local::now()` en validación de año/código y `toLocaleDateString('es-PE')` sin zona explícita no acreditan America/Lima independiente de Windows. Definir si cada valor es instante o fecha local y probar medianoche/cambio de año sin doble conversión. |

La auditoría ya registra actor real y las operaciones administrativas con motivo/antes/después dentro de las transacciones de comandos. Al completar campos pendientes hay que conservar esas garantías. No se exige almacenar teléfonos/correos completos innecesariamente en eventos: el PDF también limita datos personales en auditoría. No se repite como obligación ampliar indiscriminadamente el registro de contactos.

## Pruebas ejecutadas en esta revisión

| Comprobación | Resultado |
|---|---|
| `npm run typecheck` del producto | Pasa |
| Rust `vehiculo_tests` | 12/12 pasan |
| React `FichaVehicularPage.test.tsx` | 4/4 pasan |
| Rust `auth_tests` | 18/18 pasan |
| Rust `auth_regression_tests` | 12/12 pasan |
| Casos adicionales Rust | 6: 3 pasan / 3 fallan |
| Casos adicionales React | 7: 3 pasan / 4 fallan |

Rust adicional aprobado: sin aceite devuelve null; sintético parte de lectura final; Login mecánico devuelve Ficha solo lectura. Fallido: identidad histórica en JSON real; material sin cambio devuelve próximo inventado; recepción contra esquema real.

React adicional aprobado: detalle null; historial null; mecánico sin acciones/candidatos. Fallido: placa editable admin; +51 sin truncado; historial 20 filas; fallo técnico distinto de ausencia.

La primera ejecución Rust temporal incluyó también los 12 tests originales: 17 casos, 14 aprobados y 3 fallidos. El sexto caso adicional, Login mecánico, se ejecutó después con filtro y aprobó. El caso React de error se repitió aisladamente tras ajustar la consulta de la aserción para contar ambos mensajes. No se suman repeticiones como escenarios distintos ni se considera un error del test inicial como defecto del producto.

Las pruebas adicionales usan datos ficticios y SQLite en memoria con `0001_initial_schema_v7.sql`; React usa servicios simulados. Las copias de diagnóstico quedan como texto en `docs/diagnosticos-ficha-2026-10-19/`; los archivos temporales ejecutables se retiraron. Para repetirlos, copiar el `.rs.txt` a `src-tauri/tests/quinta_revision_tmp.rs` y el `.tsx.txt` a `src/pages/vehiculo/__tests__/quinta_revision_tmp.test.tsx`, ejecutar las suites y retirar esas copias. El Rust incluye los 12 tests del archivo original mediante `include!`.

No se ejecutaron scripts de ingesta ni migraciones sobre la base del negocio. No se modificó código funcional. Esta revisión no acredita el ejecutable instalado, IPC completo ni una prueba manual de la ventana Tauri.

## Qué falta para 90+ y qué falta para aprobar

**Prioridad 1:** cerrar los siete fallos reproducidos: snapshot/DTO histórico, cambio de aceite persistido, edición administrativa completa, teléfono, historial paginado, error frente a ausencia y contrato de recepción. Convertir esos casos en regresiones permanentes después de corregir. Son los principales obstáculos funcionales para superar 90; no se garantiza una nota por editar archivos sin verificar resultados.

**Prioridad 2:** ordenar candidatos con desempate, conservar contexto y ajustar página; completar confirmación/envío/descarte; detectar captura obsoleta; terminar presentación/navegación y reglas de inactivo. Repetir los recorridos por los tres roles.

**Evidencia para el cierre:** completar los diez TC-FVH con preparación, esperado, observado y PASÓ/FALLÓ/NO EJECUTADO; validar 84499/84500 y 179/180 días con reloj controlable; cinco búsquedas/primeras páginas con 1000 vehículos, objetivo hasta 3 s y equipo/versión registrados; recorrido Tauri y sesión expirada con respuesta pendiente. El benchmark existente está deshabilitado por feature en `src-tauri/tests/benchmark_tests.rs:1`, por lo que su presencia no acredita ejecución.

La parte de cobro/devolución de TC-FVH-05 se verifica al integrar POS, conservando ahora el contrato de historial independiente del pago. No se exige terminar POS para trabajar el siguiente módulo, pero esa prueba no se marca ejecutada ni se certifica una integración aún pendiente.

El avance desde 74 es real. Quedan fallos observables en recorridos ofrecidos hoy y evidencia de aceptación pendiente; por eso el estado es **80/100, pendiente de cierre**.
