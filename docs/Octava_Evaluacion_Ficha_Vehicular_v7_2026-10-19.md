# Octava evaluación — Ficha Vehicular e Historial V7

Fecha: 19 de octubre de 2026, America/Lima.

**Puntuación actual: 90/100. Anterior: 88/100. Estado: umbral de 90 alcanzado; cierre todavía pendiente.**

## Alcance y criterio

Revisión del estado actual y de los cambios frente a la séptima evaluación. Incluye Ficha, formularios de vehículo/corrección pública, candidatos, recepción OT, DTO, servicios y restricciones V7. Se repitieron los diagnósticos anteriores y se añadieron pruebas de confirmación, doble clic, opción de aceite en recepción y orden de candidatos sobre datos reales del esquema de prueba.

Los dos PDF V7 continúan siendo el contrato. Se mantiene el trabajo por módulos: no se exige completar Taller, Inventario, POS o Dashboard para reconocer el avance de Ficha. Se evalúan los contratos que ya abre o consume este módulo. Los comentarios del código y los ejemplos de los documentos no acreditan pruebas por sí mismos.

| Área | Nota actual | Máximo |
|---|---:|---:|
| Clientes, vehículos y validaciones | 18 | 20 |
| Historial público e identidad histórica | 19 | 20 |
| Roles, sesión y privacidad | 14 | 15 |
| Cambios administrativos y auditoría | 14 | 15 |
| Próximos cambios | 8 | 10 |
| Interfaz, errores y respuestas asíncronas | 9 | 10 |
| Pruebas y evidencia de aceptación | 8 | 10 |
| **Total** | **90** | **100** |

La calificación es una valoración técnica ponderada, no porcentaje de cobertura o de tests aprobados. Reconoce que los casos históricos principales ya pasan y que guardar cambios sensibles ahora requiere confirmación. No equivale a aprobación automática: quedan incumplimientos reproducidos y aceptación final pendiente.

## Los cuatro fallos principales anteriores ya pasan

| Hallazgo de la séptima revisión | Resultado actual |
|---|---|
| Recepción seleccionaba zanja libre por defecto | **Corregido para el recorrido normal de Ficha.** El modal sin zanja inicial envía sin zanja, mecánico 0 y aceite vacío, tanto con zanjas libres como ocupadas. Backend libre también pasa. Quedan opciones manuales incompatibles, detalladas abajo. |
| Historial estaba ascendente | **Corregido.** Consulta por fecha final DESC e ID DESC; el caso de dos OT devuelve [2,1]. Esto no corrige por sí solo Próximos cambios. |
| Documento histórico faltante se rellenaba con actual | **Corregido.** Snapshot antiguo solo con nombre devuelve documento None, incluso tras editar el cliente actual. Snapshot completo también conserva nombre/documento originales. |
| Nombre del material aparecía como tipo de aceite | **Corregido.** Sin cambio real, material ACEITE devuelve tipo None y próximo None. El tipo ya se lee de la OT, sin fallback al nombre del producto. |

Se mantienen teléfono +51, detalle/lista seguros con null, páginas visuales de veinte y reinicio, motivo de propietario, placa administrativa, permisos del mecánico, filtrado de contactos, auditoría e ID estable. Los 12 casos adicionales de la séptima revisión pasan ahora, además de las suites existentes de vehículos y React.

## Confirmaciones nuevas: funcionan, pero falta bloquear la reiteración

Se verificó en ambos formularios:

- Guardar un cambio sensible abre confirmación sin llamar al servicio.
- Revisar/Cancelar la confirmación no escribe.
- Confirmar una vez llama una vez al guardado.
- El envío conserva el motivo y los datos esperados.

Por tanto, ya no se afirma que falte la confirmación de vehículo o de corrección pública. El problema residual es el procesamiento después de confirmar.

## Tres problemas actuales reproducidos

### 1. Doble clic en confirmar produce dos llamadas de guardado

Fuentes: `src/pages/vehiculo/components/ModalVehiculo.tsx:484`, `src/pages/vehiculo/components/ModalCorregirHistorialOT.tsx:210`.

Pruebas con el primer guardado aún pendiente:

| Acción | Esperado | Observado |
|---|---:|---:|
| Dos clics en «Sí, Aplicar Cambios» | 1 llamada | **2 llamadas** |
| Dos clics en «Sí, Corregir OT» | 1 llamada | **2 llamadas** |

Los botones originales de los formularios sí se bloquean con guardando/cargando. Los botones nuevos dentro de la confirmación no tienen ese bloqueo, y los handlers tampoco impiden entrar otra vez. Cancelar/Revisar continúa disponible durante el envío.

**Cierre:** bloquear confirmación y salida pertinente durante el envío, con una guarda de ejecución que impida reiteraciones. Ante error permitir reintentar de forma controlada. Repetir ambos casos hasta observar una llamada mientras el primer resultado no llega.

La prueba reproduce llamadas repetidas desde React con servicio simulado; no se afirma que se hayan duplicado filas o auditorías en la base real. El defecto es el envío repetido, contrario al estado Procesando descrito por el PDF.

Los ConfirmModal de desactivar/reactivar siguen sin recibir `cargando` desde Ficha, pendiente anterior confirmado por lectura. Resolver también esos usos del mismo requisito.

### 2. Recepción ya nace libre por defecto, pero aún permite seleccionar aceite/zanja de ejecución

Fuentes: `src/pages/ordenes/components/ModalCrearOT.tsx:48`, `:206`, `:208`, `:475`, `:596`; inserción en `ordenes_service.rs` y trigger `trg_ot_nace_espera`.

El caso normal corregido pasa y se reconoce. Sin embargo, el formulario conserva «Tipo de Aceite a Aplicar (Opcional)». Al seleccionar MINERAL y guardar, la prueba captura **tipo_aceite="MINERAL"** en una recepción EN_ESPERA. El servicio con ese contrato, sin mecánico y sin zanja, devuelve error por las restricciones V7.

El selector de zanja también permanece disponible y `zanjaInicial` puede precargarlo. La prueba de guardarraíl Rust confirma que una zanja de ejecución en el nacimiento es rechazada. Esto no se presenta como selección automática en el recorrido normal actual, ya corregido.

**Cierre:** recepción sin datos finales de ejecución en todos los recorridos ofrecidos; zanja se elige al iniciar y aceite final cuando corresponde. No basta dejarlos opcionales si las opciones siguen produciendo un envío incompatible con el esquema. Completar la coherencia entre modal, DTO y servicio, conservando la revalidación de vehículo activo y sin abierta.

Se conserva otro pendiente de contrato: `vehiculoService.listarVehiculos()` lista candidatos de aceite como si fueran todos los vehículos. La placa precargada dispone de consulta específica, pero ese listado general aún no representa todo el conjunto elegible.

### 3. Próximos cambios continúa del más reciente al antiguo

Fuente: `src-tauri/src/services/cliente_service.rs:957`.

La consulta mantiene `ORDER BY fecha_servicio DESC`, sin desempate explícito por ID. Se prepararon dos vehículos activos con cambios finalizados de 200 y 190 días, usando restricciones y vista reales V7. Ambos son candidatos por fecha.

Resultado: IDs **[2,1]**, es decir, 190 días antes que 200. RB08 exige el cambio más antiguo primero: **[1,2]**.

**Cierre:** orden ascendente por fecha del último cambio e ID estable como desempate. No modificar de nuevo Historial, cuyo orden descendente ya quedó correcto. El cambio actual de `FichaVehicularProximosCambios.tsx` respecto de HEAD solo retira un comentario; no cambia esta consulta.

## Pendientes adicionales conservados por lectura

| Pendiente | Estado y condición de aceptación |
|---|---|
| Contexto de candidatos | Filtros y página viven en componente desmontado al abrir Ficha; Actualizar no ajusta una página que queda fuera de rango al reducir resultados. Conservar contexto autorizado y página válida. |
| Captura sin guardar | Formularios no registran estado de cambios para una sola confirmación de descarte. La expiración automática debe salir sin esperar al usuario. |
| Escrituras obsoletas | Revalidar estado y comparar la base de la captura para pedir recargar si los datos cambiaron, evitando sobrescritura silenciosa. No se exige un visor de versiones. |
| Selección de propietario | Seleccionar explícitamente el cliente destino, separado de editar sus datos. El modal sigue enviando el ID anterior mientras Rust resuelve por documento y puede actualizar nombre/contactos del destino. |
| Errores | `listarOrdenesTrabajo().catch(() => [])` y algunas lecturas convierten fallos en ausencia. La propagación `r?` de candidatos y la separación visual de error/ausencia sí se dan por corregidas. |
| Presentación/navegación | Limpiar, volver autorizado, resumen de último servicio/último cambio, hora final y rótulo km de atención; Abrir OT y corrección desde detalle. |
| Estado/validación | Edición del inactivo con distinción por rol; km cero precargado mediante `|| ''`; errores por campo y longitudes Unicode frente a bytes. |
| Hora Lima | Validar medianoche/cambio de año con Windows en otra zona, revisando Local::now y formateo sin zona explícita, sin doble conversión de fechas locales. |
| Consulta/rendimiento | Rust aún carga todo el historial y detalles; completar cinco búsquedas/primeras páginas reales con 1000 vehículos, objetivo hasta 3 s y equipo/versión. La paginación visual ya funciona. |

Se conserva la garantía de auditoría con actor real y transacciones en comandos. No se exige almacenar contactos completos innecesariamente en eventos, completar otros módulos, registrar llamadas u ocultar candidatos.

## Pruebas ejecutadas ahora

| Comprobación | Resultado |
|---|---|
| `npm run typecheck` | Pasa |
| `npm run build` | Pasa |
| Rust `vehiculo_tests` | 13/13 pasan |
| React Ficha existente | 4/4 pasan |
| Rust adicional | 14 casos: **13 pasan / 1 falla** |
| React adicional | 22 casos: **19 pasan / 3 fallan** |

Rust adicional fallido: candidatos más antiguos primero. Los 12 anteriores pasan, incluido snapshot antiguo, tipo de aceite sin cambio, orden de historial y permiso real de mecánico tras Login. El caso adicional de rechazo de aceite final confirma la incompatibilidad del envío manual permitido por recepción.

React adicional fallido: doble envío de vehículo, doble envío de corrección y selección de aceite final en recepción. Los 15 escenarios de la séptima revisión pasan. Se añaden cuatro positivos de confirmación/guardar/cancelar y los tres fallidos descritos.

La ejecución Rust temporal incluye los 13 originales: **27 tests, 26 aprobados/1 fallido**. React incluye los 4 originales: **26 tests, 23 aprobados/3 fallidos**. No se calcula la nota con esas proporciones ni se comparan tasas de suites diferentes como si fueran cobertura total.

Volumen: el test original con 1000 vehículos vuelve a pasar. En la ejecución temporal visible: inserción 288,2204 ms; búsqueda **83,3 µs (0,0833 ms)**. Es SQLite en memoria, no rendimiento end-to-end de Tauri. La matriz TC-FVH, límites 179/180 y 84499/84500 y aceptación manual siguen pendientes.

Los diagnósticos quedan como texto en `docs/diagnosticos-ficha-octava-2026-10-19/`. Para repetir, copiar `.rs.txt` a `src-tauri/tests/octava_revision_tmp.rs` y `.tsx.txt` a `src/pages/vehiculo/__tests__/octava_revision_tmp.test.tsx`, ejecutar y retirar copias. Rust incluye los tests originales del módulo.

Pruebas con datos ficticios, SQLite en memoria y migración real; React con servicios simulados. No se abrió ni modificó la base del negocio. No se ejecutaron ingestas ni migraciones reales. No hubo recorrido manual de la ventana instalada ni IPC completo. No se repitieron las 30 pruebas completas de Login sin cambios; sí su caso real de permiso mecánico dentro de los diagnósticos.

## Decisión

**90/100 reconoce un módulo muy avanzado y los cuatro bloqueos principales anteriores corregidos.** Todavía no aprobaría cierre formal: corregir envíos reiterados, retirar las opciones de ejecución incompatibles de recepción y ordenar candidatos; después completar los pendientes de aceptación señalados. La revisión no modificó implementación funcional; solo conserva este informe y diagnósticos reproducibles.
