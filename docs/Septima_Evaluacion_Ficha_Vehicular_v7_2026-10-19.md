# Séptima evaluación — Ficha Vehicular e Historial V7

Fecha: 19 de octubre de 2026, America/Lima.

**Puntuación actual: 88/100. Anterior: 85/100. Estado: pendiente de cierre.**

## Alcance y puntuación

Se revisaron los cambios actuales de Ficha, DTO/servicios Rust, formularios de recepción y su contrato con el esquema V7. Se repitieron los escenarios de la sexta evaluación y se agregaron casos para el envío actual del modal, snapshots antiguos, orden y presentación del aceite. Se volvió a consultar el PDF específico, páginas 3 y 6, para confirmar las reglas distintas de orden del historial y de candidatos. Se conserva el marco de los dos PDF V7 y el alcance de Ficha como segundo módulo trabajado después de Login.

No se exige completar Taller, POS o Inventario: se revisa únicamente el contrato que Ficha ya ofrece hacia recepción y los datos públicos que consume. No se interpreta una declaración de corrección en comentarios como evidencia de aceptación.

| Área | Sexta | Séptima | Máximo |
|---|---:|---:|---:|
| Clientes, vehículos y validaciones | 17 | 17 | 20 |
| Historial público e identidad histórica | 17 | 18 | 20 |
| Roles, sesión y privacidad | 14 | 14 | 15 |
| Cambios administrativos y auditoría | 13 | 14 | 15 |
| Próximos cambios | 8 | 8 | 10 |
| Interfaz, errores y respuestas asíncronas | 9 | 9 | 10 |
| Pruebas y evidencia de aceptación | 7 | 8 | 10 |
| **Total** | **85** | **88** | **100** |

La nota es una valoración ponderada, no cobertura ni porcentaje de tests. Reconoce la corrección de los casos principales de identidad/aceite, el motivo de propietario y más evidencia de regresión. La regresión de orden y los recorridos aún incompletos impiden declarar cierre. Obtener 90 tampoco sustituye los criterios de aceptación documentales.

## Lo que ahora sí está corregido

1. **Identidad desde snapshot completo:** las consultas de listado, detalle e historial llaman a `extraer_identidad_cliente_snapshot`. El caso con snapshot completo muestra el cliente histórico y, tras editar nombre/documento actuales, conserva ambos valores originales. No se repite como pendiente general que el historial ignore todos los snapshots.
2. **Próximo km sin cambio real:** `calcular_proximo_kilometraje` ahora verifica primero el servicio marcado como cambio. El caso anterior con material ACEITE y próximo persistido NULL devuelve **None**, correctamente. Ya no se reporta que siga inventando 55010 en ese escenario.
3. **Motivo por propietario:** cambiar documento activa el motivo administrativo. La prueba antes fallida ahora pasa. Placa administrativa y reducción de km conservan esa exigencia.
4. **Reinicio de paginación:** el efecto por cambio de historial reinicia página. Pasa el caso de 21 a 1 fila estando en página 2, además de navegación 20/1.
5. **Recepción más flexible:** mecánico no se elige automáticamente y aceite inicial es vacío. Cuando las dos zanjas están ocupadas, el modal permite y envía espera libre sin zanja. Ese caso React pasa y el backend de recepción libre también pasa.

Se conservan teléfono +51, nullable en detalle/lista, separación visual de fallo/ausencia, páginas visuales de veinte, roles de lectura del mecánico, propagación de errores de filas, ID estable, restricciones de placa/propietario/km y auditoría administrativa comprobados anteriormente. Las 13 pruebas Rust existentes y las 4 React existentes pasan de nuevo.

## Fallos actuales y condiciones de cierre

### 1. La recepción normal sigue enviando una zanja libre automáticamente

Fuentes: `src/pages/ordenes/components/ModalCrearOT.tsx:47`, `:57`, `:120`, `:207`; `src-tauri/src/services/ordenes_service.rs:351`, `:366`.

`obtenerZanjaInicialValida()` elige 1 si está libre o 2 si la primera está ocupada. El efecto vuelve a establecer ese valor. Aunque el selector diga Opcional, el envío habitual contiene zanja de ejecución.

Prueba React del modal actual con placa precargada, mecánico opcional y aceite vacío: envía **zanja=1**. Prueba Rust del mismo contrato (`id_mecanico=0`, `zanja=Some(1)`, km=50010 y aceite vacío): se rechaza por el trigger **«OT nace en espera libre o asignada, sin ejecucion»**. El mensaje de Display se sanitiza como error de almacenamiento; el detalle interno del rechazo se comprobó mediante Debug en el diagnóstico, sin datos del negocio.

La recepción libre sí funciona si se omite zanja; no se afirma que todo el backend de creación falle. Tampoco se repite que el formulario obligue siempre a mecánico/MINERAL: esos valores ya mejoraron.

**Cierre:** recepción siempre en cola sin zanja de ejecución ni aceite final, tanto con zanjas libres como ocupadas. Elegir zanja pertenece a Iniciar trabajo. Hacer opcionales los campos de ejecución no completa el contrato: mientras puedan enviarse en EN_ESPERA, siguen existiendo rutas rechazadas. El selector de aceite todavía permite elegir un valor incompatible con el nacimiento de la OT.

Se conserva como pendiente de integración `vehiculoService.listarVehiculos()`, que usa candidatos de aceite como listado general. No se exige terminar todo Taller para corregir estos contratos.

### 2. Se invirtió el historial; candidatos conserva el orden anterior incorrecto

Fuentes: `ordenes_service.rs:771` y `cliente_service.rs:957`.

Son reglas diferentes:

| Lista | PDF | Código actual |
|---|---|---|
| Historial de trabajos | Más reciente primero; ID de desempate, página 3, apartado 4.2 | Fecha ASC e ID ASC: **regresión** |
| Próximos cambios | Último cambio más antiguo primero; ID de desempate, página 6, RB08 | Fecha DESC, sin desempate explícito: **pendiente** |

La prueba de dos OT finalizadas devuelve IDs **[1,2]**, cuando el orden reciente/ID descendente esperado es **[2,1]**. El cambio ascendente se aplicó a la consulta de Historial, no a la de candidatos que se había señalado.

**Cierre:** historial por fecha final DESC e ID coherente como desempate; candidatos por fecha del último cambio ASC e ID estable. Mantener las páginas de veinte ya implementadas.

### 3. Snapshot antiguo incompleto todavía se rellena con documento actual

Fuente: `ordenes_service.rs:97`, especialmente `:115`.

El caso principal con snapshot completo ya pasa. El residual está en `.or(doc_relacional)` y equivalentes: un dato histórico ausente se sustituye por un valor presente de la tabla clientes.

Caso reproducido con el formato antiguo real de creación, `{"nombre":"Cliente historico"}`: después de cambiar el documento actual, la atención devuelve **87654321** como documento del cliente histórico, aunque ese documento nunca estuvo en el snapshot.

**Cierre:** ausencia histórica explícita cuando el dato no fue guardado. No completar silenciosamente el documento histórico con el actual. Si se pueden recuperar datos originales de otra fuente verificable, definir ese proceso; si no, mostrar No registrado. Para las pruebas de snapshot completo se usó la clave `documento` que el creador actual realmente escribe, separando ese caso de la compatibilidad antigua.

### 4. El próximo km ya es correcto, pero el tipo de aceite aún puede ser un producto

Fuente: `ordenes_service.rs:38`, `:48`.

El fallback de `obtener_tipo_aceite_ot` sigue consultando el nombre actual de un producto ACEITE/GRANEL cuando la OT tiene tipo null. Caso sin servicio de cambio y con material: próximo km **None**, correcto; tipo de aceite **Some("Aceite mineral")**, incorrecto como tipo histórico de cambio.

La pantalla muestra insignia de aceite cuando el campo tiene valor, de modo que puede sugerir cambio donde no lo hubo. Además, leer catálogo actual no acredita un dato histórico estable.

**Cierre:** tipo de aceite público null/sin cambio cuando no corresponde, y Mineral/Sintético persistido cuando sí corresponde. Evitar presentar nombre de material como tipo de cambio. Este residual no se confunde con el próximo inventado, que ya quedó resuelto.

## Pendientes conservados por lectura

| Pendiente | Qué falta para cerrarlo |
|---|---|
| Confirmación sensible | Motivo de propietario ahora presente; aún faltan confirmación al guardar cambios sensibles/corrección pública y separación explícita entre elegir propietario y editar sus datos. |
| Cliente destino | Buscar/seleccionar cliente y respetar su identidad/contactos; no utilizar transferencia para modificar el cliente destino. Consulta navegable del duplicado. |
| Envíos de estado | Conectar `cargando` a ConfirmModal para impedir reiteraciones; resolver resultado incierto antes de repetir. |
| Captura | Confirmación única de descarte mediante estado de cambios sin guardar; expiración automática sin esperar; detectar captura obsoleta antes de sobrescribir. |
| Candidatos | Conservar filtros/contexto al regresar y limitar página cuando Actualizar reduce resultados. El reinicio nuevo corrige Historial, no esta lista. |
| Errores | Ficha todavía usa `listarOrdenesTrabajo().catch(() => [])`; helpers y consultas mantienen conversiones amplias de errores a ausencia. La propagación por `r?` de candidatos sí está resuelta. |
| Presentación/navegación | Limpiar y volver al origen autorizado; resumen último servicio/último cambio, hora final, rótulo km de atención en lugar de Ingreso, Abrir OT/corregir desde detalle según permisos. |
| Estado/validación | Distinción de edición del inactivo por rol; precarga km=0; errores por campo; longitudes Unicode frente a bytes. |
| Hora Lima | Probar medianoche y cambio de año con zona Windows distinta; retirar dependencia de `Local::now()`/formateo sin zona explícita donde corresponda. |
| Consulta paginada/rendimiento | React pagina, pero Rust aún carga todo el historial y detalles. Completar cinco búsquedas/primeras páginas reales con 1000 vehículos, equipo/versión y objetivo de 3 s. |

La prueba de 1000 vehículos vuelve a pasar: en la ejecución temporal visible insertó en 216,3934 ms y consultó un vehículo en **89,2 µs (0,0892 ms)**. Es consulta SQLite en memoria, no tiempo Tauri con historial, candidatos y renderizado. Se conserva el crédito por la prueba parcial sin presentarla como aceptación completa.

## Pruebas ejecutadas

| Comprobación | Resultado |
|---|---|
| `npm run typecheck` | Pasa |
| `npm run build` | Pasa |
| Rust `vehiculo_tests` | 13/13 pasan |
| React Ficha existente | 4/4 pasan |
| Rust adicional | 12 casos: 9 pasan, 3 fallan |
| React adicional | 15 casos: 14 pasan, 1 falla |

Rust adicional aprobado: snapshot completo; invariancia de nombre/documento con snapshot completo; sin cambio null; material sin próximo inventado; sintético desde lectura final; Login mecánico solo lectura; recepción libre con snapshot; propagación de error de filas; rechazo del envío actual con zanja de ejecución.

Rust fallido: orden de historial; documento histórico ausente reemplazado por actual; nombre de material como tipo de aceite. React fallido: envío normal de recepción con zanja. El caso con ambas zanjas ocupadas sí pasa.

La primera ejecución Rust temporal incluyó los 13 originales: 25 tests, 22 aprobados/3 fallidos. Se ajustó después el diagnóstico del rechazo para usar exactamente el envío nuevo del modal (sin mecánico ni aceite, con zanja) y se ejecutó ese caso aislado. Su primera aserción buscaba el trigger en Display, que sanitiza el mensaje; se corrigió el diagnóstico para inspeccionar Debug y aprobó. No se cuenta esa aserción del diagnóstico como defecto adicional del producto ni como escenario distinto. La ejecución React tuvo 19 tests: 18 aprobados/1 fallido.

Login completo no se volvió a ejecutar: no cambió desde la sexta revisión. El caso real de permiso mecánico sí se repitió en los diagnósticos. No se presentan los 30 resultados anteriores como nueva ejecución.

Se preservan los diagnósticos como texto en `docs/diagnosticos-ficha-septima-2026-10-19/`. Para repetirlos, copiar el `.rs.txt` a `src-tauri/tests/septima_revision_tmp.rs` y el `.tsx.txt` a `src/pages/vehiculo/__tests__/septima_revision_tmp.test.tsx`, ejecutar y retirar las copias. El Rust incluye los tests originales. SQLite usa migración real V7 en memoria y datos ficticios; React usa servicios simulados.

No se modificó código funcional ni datos del negocio. No se ejecutaron ingestas/migraciones reales ni se hizo recorrido manual de la ventana instalada o IPC completo. La matriz TC-FVH, límites de 179/180 días y 84499/84500 y evidencia del equipo objetivo siguen siendo requisitos de aceptación pendientes; la integración económica se valida al trabajar POS.

## Decisión

El avance es comprobable y los fallos generales anteriores de snapshot completo, motivo de propietario y próximo km ya no se mantienen como si nada hubiera cambiado. Para superar 90 y cerrar con confianza, priorizar recepción sin ejecución anticipada, corregir el orden de ambas listas, definir ausencias históricas y retirar el tipo de aceite inferido de materiales; después completar las confirmaciones y evidencia de aceptación señaladas. **Resultado: 88/100; todavía pendiente de cierre.**
