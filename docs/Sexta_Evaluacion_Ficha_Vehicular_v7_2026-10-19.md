# Sexta evaluación — Ficha Vehicular e Historial V7

Fecha: 19 de octubre de 2026, America/Lima.

**Nota actual: 85/100. Quinta evaluación: 80/100. Estado: pendiente de cierre.**

## Alcance

Se revisaron las diferencias actuales, los archivos nuevos y los recorridos de Ficha entre React, servicios TypeScript, comandos de sesión, DTO Rust, consultas y esquema V7. Se contrastó el resultado con los requisitos de los dos PDF V7 ya analizados y con la quinta evaluación. No se supone que un fallo anterior siga existiendo: se repitieron sus casos adaptando el test a las nuevas interfaces.

El segundo módulo que estamos cerrando después de Login es Ficha, aunque el documento lo numere 03. No se exige completar los otros módulos para reconocer el avance. Taller se incluye únicamente por la recepción accesible desde Ficha; inventario, base de datos y Login se inspeccionan en las dependencias pertinentes. Las instrucciones y ejemplos de los PDF se tratan como especificación, no como pruebas ejecutadas.

La base de comparación del repositorio es HEAD `48c5dee`, además de cambios sin commit. Se revisaron los cambios visuales de Login para comprobar compilación y se repitieron sus suites Rust, pero no se hace aquí otra auditoría funcional completa de Login ni de Inventario.

## Puntuación con los mismos pesos

| Área | Quinta | Sexta | Máximo |
|---|---:|---:|---:|
| Clientes, vehículos y validaciones | 16 | 17 | 20 |
| Historial público e identidad histórica | 16 | 17 | 20 |
| Roles, sesión y privacidad | 14 | 14 | 15 |
| Cambios administrativos y auditoría | 12 | 13 | 15 |
| Próximos cambios | 8 | 8 | 10 |
| Interfaz, errores y respuestas asíncronas | 8 | 9 | 10 |
| Pruebas y evidencia de aceptación | 6 | 7 | 10 |
| **Total** | **80** | **85** | **100** |

Es una valoración técnica ponderada, no cobertura ni porcentaje de tests. Los puntos nuevos reconocen teléfono, paginación visual, placa administrativa, separación de error/ausencia y nueva evidencia con 1000 vehículos. Los avances parciales de identidad y recepción se reconocen sin certificarlos como completos.

## Lo corregido desde la quinta revisión

| Hallazgo anterior | Resultado actual |
|---|---|
| Placa bloqueada también para administrador | **Resuelto en la habilitación.** El modal recibe `esAdmin` de la página. Pruebas: habilitada para admin, bloqueada para cajero. Cambiar placa hace aparecer el motivo. |
| +51 truncado en ModalCliente | **Resuelto para el caso reportado.** `+51 912 345 678` queda en `912345678`. El modal de vehículo ahora comparte esa normalización. |
| Historial muestra 21 filas juntas | **Resuelto en pantalla.** Primera página de 20, segunda de 1 y regreso a la primera comprobados. La consulta sigue devolviendo el conjunto completo. |
| Fallo técnico muestra también vehículo inexistente | **Resuelto para los escenarios probados.** Error técnico excluye el estado vacío; ausencia real permite registro sin mensaje técnico. |
| `rows.flatten()` oculta errores de candidatos | **Resuelto.** Ahora usa `r?`. Una vista ficticia con tipo de dato incompatible provoca error, en lugar de devolver una lista vacía. |
| DTO Rust omite campos del cliente | **Resuelto en el contrato de transporte.** Ahora devuelve nombre/documento/tipo y el detalle puede mostrarlos. El origen histórico sigue siendo incorrecto, explicado abajo. |
| Snapshot nuevo solo conserva nombre | **Mejorado y comprobado.** Crear OT guarda JSON válido con documento y tipo, además de otros datos. No se utiliza ese snapshot al consultar el historial. |
| Backend no admite recepción libre | **Mejorado y comprobado.** Envío directo con mecánico 0, zanja None y aceite vacío crea EN_ESPERA y conserva snapshot. El formulario todavía envía otro contrato. |
| Falta prueba de volumen ejecutable | **Mejorado.** Nueva prueba de 1000 vehículos en `vehiculo_tests.rs` ejecutada y aprobada. Tiene el alcance concreto descrito abajo. |

Se mantienen los arreglos anteriores de null en historial/detalle, permisos de mecánico de solo lectura, filtrado de contactos, comandos con sesión real, ID estable, protección administrativa de placa y km, corrección pública auditada y consultas de OT finalizadas independientes del pago. La suite de vehículos ahora tiene 13 casos y pasa completa.

## Bloqueos funcionales reproducidos

### 1. El cliente mostrado como histórico sigue siendo el actual de la tabla clientes

Fuentes: `src-tauri/src/services/ordenes_service.rs:710`, `:717`, `:772`; consultas equivalentes en listar OT y obtener por ID.

El DTO ya devuelve identidad, por lo que no se repite el hallazgo anterior como «faltan los campos». El problema actual es su procedencia: `LEFT JOIN clientes c ON ot.id_cliente_atendido = c.id_cliente` lee nombre/documento vigentes. No lee `ot.cliente_snapshot`.

Dos pruebas lo demuestran:

1. Snapshot de la atención con nombre ficticio «Cliente historico»: la respuesta trae «Juan Perez», de la tabla actual.
2. Cambiar el nombre actual a «Cliente cambiado»: la misma atención pasada cambia de «Juan Perez» a «Cliente cambiado» en su respuesta.

La OT no se reescribió físicamente; lo que cambia es la identidad que muestra la consulta. Por tanto, no basta usar `id_cliente_atendido`: debe mostrarse la identidad guardada al recibir. Completar lectura del JSON en los recorridos públicos y documentar ausencia para snapshots antiguos incompletos, sin presentarla como identidad histórica confirmada. La placa ya viene de snapshot y debe conservarse así.

**Cierre:** editar cliente actual, cambiar propietario y cambiar placa no altera cliente/documento/placa mostrados en la atención anterior.

### 2. Sigue apareciendo un próximo cambio inexistente por un material ACEITE

Fuentes: `ordenes_service.rs:38`, `:47`, `:73`, `:83`.

Se reconoce que `tiene_cambio_aceite` ya dejó de contar productos. Sin embargo, `obtener_tipo_aceite_ot` sigue recuperando el nombre del producto ACEITE/GRANEL cuando el tipo de la OT es null. El cálculo acepta ese nombre no vacío y el caso por defecto suma 5000.

La nueva condición `!tiene_cambio_aceite && t_aceite.trim().is_empty()` no lo evita: inmediatamente antes se retornó para cualquier cadena vacía, por lo que ese segundo término ya es falso.

Caso repetido: servicio no marcado como cambio, material ACEITE, lectura final 50010 y `km_proximo=NULL` persistido. Resultado del historial: **Some(55010)**. El fallo anterior permanece pese al cambio del helper.

**Cierre:** consultar el hecho/tipo/intervalo/próximo guardados de la atención; no reinterpretar desde catálogo actual. El caso sin cambio mantiene null y el sintético real a 50010 conserva 60010, caso que ya pasa.

### 3. Propietario y confirmación sensible siguen incompletos en el formulario

Fuente: `src/pages/vehiculo/components/ModalVehiculo.tsx:124`.

`requiereMotivo` contempla disminución de km o cambio de placa, pero no cambio de propietario. Se modificó el documento del propietario como administrador, manteniendo placa/km: no aparece el motivo. El backend lo exige, de modo que ese recorrido sigue incompleto.

La placa ya no se considera bloqueada. Lo pendiente es selección explícita del cliente destino, motivo también para propietario y confirmación del efecto sensible al guardar. El modal sigue enviando el ID anterior, mientras Rust decide por documento y puede actualizar nombre/contactos del cliente destino. Separar elegir propietario de editar sus datos actuales, según el formulario de cliente independiente.

La corrección pública también envía directamente sin la confirmación descrita en el apartado 3.3 del PDF. Los dos ConfirmModal de estado disponen de soporte `cargando`, pero Ficha no lo conecta al envío. Estos últimos puntos se verificaron por lectura.

**Cierre:** propietario/placa/reducción con motivo y confirmación en sesión admin; cajero sin campos administrativos; OT abierta identificada al bloquear; guardar una vez mientras procesa.

### 4. Recepción libre funciona por servicio directo, pero el formulario sigue enviando ejecución

Fuentes: `ordenes_service.rs:238`, `:318`, `:333`, `src/pages/ordenes/components/ModalCrearOT.tsx`.

Prueba positiva nueva: `id_mecanico=0`, `zanja=None`, `tipo_aceite=""` crea una OT EN_ESPERA sin asignación ni aceite final; snapshot contiene documento/tipo. No se afirma que todo el backend de creación esté bloqueado.

Prueba del contrato que todavía representa el formulario: mecánico válido, zanja 1, MINERAL. Sigue fallando con **«OT nace en espera libre o asignada, sin ejecucion»**. El modal inicializa zanja y MINERAL, selecciona un mecánico y condiciona recepción a disponibilidad de zanjas. El servicio sigue insertando zanja y tipo si los recibe.

**Cierre:** alinear modal, DTO y servicio con recepción en cola sin datos finales de ejecución; responsable opcional. Revalidar activo/sin abierta en destino. `vehiculoService.listarVehiculos()` todavía usa candidatos de aceite como listado general; corregir ese contrato sin exigir aquí todo Taller.

## Paginación y rendimiento: avance real, aceptación parcial

La paginación React funciona en el recorrido 20/1. Rust continúa devolviendo todas las OT finalizadas y cargando sus detalles; no es una consulta paginada. Queda por ajustar el contrato o demostrar que cumple los requisitos documentados con las primeras páginas reales.

Un caso adicional de robustez del componente falla al cambiar sus props de 21 a 1 estando en página 2: conserva la página y no muestra la fila. **No se presenta como fallo confirmado del recorrido actual de búsqueda:** Ficha desmonta el historial mientras vuelve a consultar, lo que reinicia su estado. Conviene ajustar o acotar ese comportamiento para futuras actualizaciones en sitio; no es un nuevo bloqueo principal ni justifica por sí solo impedir el cierre.

La prueba nueva inserta 1000 vehículos, confirma 1001 contando el inicial y mide una búsqueda de vehículo sin historial. En la ejecución con salida visible: inserción **390,4193 ms**, consulta **120,8 µs (0,1208 ms)**. Pasa su umbral de 10 ms en este entorno. Es evidencia de búsqueda indexada rápida sobre SQLite en memoria.

No equivale todavía a cinco búsquedas/primeras páginas con historial/candidatos, IPC, renderizado y equipo objetivo hasta 3 s. Ese conjunto queda pendiente. Ya no corresponde decir «no existe prueba ejecutada con 1000 vehículos»; corresponde decir «prueba parcial aprobada».

## Pendientes conservados por lectura

| Pendiente | Estado actual y criterio |
|---|---|
| Orden de candidatos | `cliente_service.rs:957` mantiene fecha DESC. RB08 exige último cambio más antiguo primero e ID como desempate. |
| Contexto de candidatos | Filtros en componente desmontado; Actualizar no limita página cuando el conjunto se reduce. Conservar contexto autorizado y página válida. |
| Otros errores ocultos | Ficha conserva `listarOrdenesTrabajo().catch(() => [])`; Rust mantiene conversiones amplias a NotFound y helpers de aceite con errores ignorados. La corrección `r?` sí se da por realizada. |
| Clasificación de ausencia | La separación visual pasa los casos actuales; reconocer ausencia por código estructurado resulta más estable que una frase concreta. No se exige reescribir arquitectura para obtener crédito por lo corregido. |
| Captura y sesión | Formularios sin conexión a cambios sin guardar; falta confirmación única de descarte y prueba con respuesta pendiente al terminar sesión. La salida automática no debe esperar esa confirmación. |
| Datos obsoletos al guardar | No hay comparación del estado/base esperados de la captura para avisar de cambios y recargar. Revalidar restricciones actuales no demuestra protección frente a sobrescritura silenciosa. |
| Inactivo | `cliente_service.rs:518` bloquea toda edición del vehículo, también admin. Aplicar la distinción por rol documentada; mantener consulta e impedir recepción. |
| Presentación | Km final todavía rotulado «Ingreso»; fecha sin hora; resumen de último servicio/último cambio pendiente y vacíos coherentes. El detalle no tiene navegación al origen OT ni corrección desde el propio detalle. |
| Navegación y validación | `onVolver` sin usar, Limpiar pendiente, selección explícita de cliente/consulta de duplicados y errores junto al campo. Km cero se precarga vacío mediante `|| ''`; longitudes Rust por bytes. |
| Fechas Lima | `Local::now()` y formatos locales sin zona explícita requieren validar medianoche/cambio de año con Windows en otra zona, sin doble conversión de valores locales. |

`db.rs` activa foreign_keys, busy_timeout y WAL antes de la migración y aplica V7 transaccionalmente. La prueba de volumen ya configura estos PRAGMA también. No se abrió ni migró la base real del negocio y no se certifica su versión instalada a partir de la prueba en memoria. No se ejecutó el test de inicialización con keyring real. La revisión de Inventario no se utiliza para exigir su cierre en esta nota.

Se conserva auditoría con identidad real y transacciones en comandos administrativos. No se pide guardar contactos completos innecesariamente en eventos. Tampoco se pide registrar llamadas, ocultar candidatos, crear mantenimiento independiente ni construir un visor de versiones.

## Pruebas de esta revisión

| Comprobación | Resultado |
|---|---|
| `npm run typecheck` | Pasa |
| `npm run build` | Pasa, incluidos recursos visuales de Login |
| Rust `vehiculo_tests` | 13/13 pasan |
| React Ficha existente | 4/4 pasan |
| Rust `auth_tests` y `auth_regression_tests` | 18/18 y 12/12 pasan |
| Rust adicional | 9 casos: 5 pasan, 4 fallan |
| React adicional | 13 casos: 11 pasan, 2 fallan |

Rust adicional aprobado: sin cambio null; sintético desde km final; Login mecánico solo lectura; recepción libre con snapshot; propagación de error de fila de candidatos. Fallido: snapshot histórico distinto del cliente actual; invariancia al editar cliente actual; material sin cambio; contrato antiguo de recepción. Los dos fallos de identidad prueban el mismo defecto desde ángulos distintos.

React adicional aprobado: siete escenarios anteriores, navegación 20/1, bloqueo de placa para cajero, motivo de placa para admin y ausencia real. Fallido: motivo de propietario y ajuste de página al reducir props, con la limitación de alcance explicada arriba.

La ejecución temporal Rust incluye los 13 casos originales: 22 en total, 18 aprobados/4 fallidos. React ejecutó 4 originales y 13 adicionales: 17, con 15 aprobados/2 fallidos. No se usa esa proporción para calcular la nota.

Pruebas con datos ficticios: SQLite en memoria con migración real; React con servicios simulados. Diagnósticos conservados como texto en `docs/diagnosticos-ficha-sexta-2026-10-19/`. Para repetir: copiar `.rs.txt` a `src-tauri/tests/sexta_revision_tmp.rs` y `.tsx.txt` a `src/pages/vehiculo/__tests__/sexta_revision_tmp.test.tsx`, ejecutar y retirar copias. El Rust incluye el archivo original de vehículos.

No se modificó implementación funcional, no se tocaron datos del negocio ni scripts de ingesta. No hubo recorrido manual de la ventana Tauri ni certificación del ejecutable instalado. Los cambios de diseño de Login compilan; eso no sustituye su aceptación visual manual.

## Ruta concreta a 90+ y cierre

1. Usar snapshot al leer identidad histórica y repetir ambos casos fallidos.
2. Retirar la reinterpretación de aceite desde productos y hacer coincidir historial con el próximo persistido.
3. Terminar propietario/motivo/confirmación y separación de datos del cliente; probar flujo completo por rol.
4. Conectar el formulario de recepción al contrato libre que el backend ya admite.
5. Ordenar candidatos, completar contexto/envíos/descarte y atender los pendientes de usabilidad/integridad señalados.
6. Completar matriz TC-FVH, límites 84499/84500 y 179/180 días, cinco mediciones y recorrido Tauri. La parte económica de integración se valida al trabajar POS, sin marcarla ejecutada ahora.

Son correcciones y pruebas verificables; no se promete 90 por el número de archivos editados. Con los bloqueos actuales, la conclusión es **85/100, avance comprobado, pendiente de cierre**.
