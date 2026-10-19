# Cuarta evaluación — Ficha Vehicular e Historial V7

Fecha de referencia: 18 de octubre de 2026, America/Lima.

**Nota: 74/100. Anterior: 64/100. Estado: todavía no aprobar el cierre.**

## Alcance y método

Nueva revisión de la implementación actual y sus cambios frente a la tercera evaluación, con los mismos PDF V7 específico/general como contrato. Se revisaron página, formularios, DTO de ambos lados, comandos y su registro, servicios Rust, consultas SQLite, permisos de Login y recepción hacia Taller. Incluye los archivos nuevos sin commit del módulo.

Se ejecutaron TypeScript, suites existentes de Ficha/Login y casos adicionales de requisitos sobre React con servicios simulados y SQLite en memoria con el esquema real de migración V7. Los datos de prueba son ficticios. No se modificaron datos del negocio ni la implementación. Los tests temporales se retiraron al terminar; este documento conserva sus resultados.

No se ejecutó un recorrido manual de la ventana instalada, IPC completo de Tauri ni aceptación de rendimiento con 1.000 vehículos en el equipo objetivo. No se certifica que el ejecutable instalado corresponda a los fuentes actuales. No se ejecutaron scripts de ingesta o migraciones sobre datos del usuario.

La evaluación sigue siendo por etapas: no exige completar otros módulos. El contrato de Ficha con Taller se comprueba sin evaluar toda la operación del taller/POS.

## Calificación con los mismos pesos

| Área | Tercera | Cuarta | Máximo |
|---|---:|---:|---:|
| Clientes, vehículos y validaciones | 15 | 16 | 20 |
| Historial público e identidad histórica | 12 | 15 | 20 |
| Roles, sesión y privacidad | 8 | 10 | 15 |
| Cambios administrativos y auditoría | 11 | 12 | 15 |
| Próximos cambios | 8 | 8 | 10 |
| Interfaz, errores y respuestas asíncronas | 5 | 7 | 10 |
| Pruebas y evidencia de aceptación | 5 | 6 | 10 |
| **Total** | **64** | **74** | **100** |

La nota es una valoración técnica ponderada, no cobertura ni porcentaje de tests aprobados. Los conjuntos adicionales cambian entre revisiones y no se compara su tasa de aprobación para calcular la nota. Los defectos de historial, permisos y UI impiden aprobar aunque haya mejora numérica.

## Lo resuelto desde la tercera evaluación

| Bloqueo anterior | Resultado actual y evidencia |
|---|---|
| Cajero podía cambiar placa | Resuelto en backend: prueba rechazada con rol/cuenta cajero. |
| Administrador cambiaba placa sin motivo | Resuelto: ahora exige 5–250; prueba sin motivo rechazada. Con motivo conserva ID y actualiza placa. |
| Historial mezclaba vehículos por reutilizar placa | Resuelto: consulta por ID exclusivo. Caso A ABC123 → NEW123, B ABC123 quedó aislado y A conservó su atención. |
| Próximo cambio desde km de ingreso | Resuelto en atención típica: ingreso 50.000/salida 50.010 sintético devuelve 60.010. |
| Corrección de km no reflejada | Resuelto: salida corregida 50.020 mineral devuelve 55.020 y marca Corregido. |
| Atención sin aceite tenía próximo inventado | Resuelto para atención sin servicio/material de aceite: Rust devuelve None/null. Se detectó un caso residual con material de categoría ACEITE, detallado abajo. |
| Dos actualizaciones de cliente | Resuelto: modal delega el guardado al callback y espera su resultado. Un envío produce una llamada, comprobado en React. |
| Nuevo cliente inaccesible | Resuelto: botón, modo CREAR y guardado sin vehículo accesibles. Prueba aprobada con una llamada de alta. |
| Respuesta antigua reemplazaba selección reciente | Resuelto para búsquedas A/B: contador de solicitudes protege éxito, error y fin de carga; prueba aprobada. |
| Próximos cambios visible para mecánico | Resuelto en pestaña/montaje: no se muestra ni se monta para ese rol; prueba aprobada. El permiso general de escritura de Ficha sigue mal. |
| Semisintético en corrección | Resuelto en formulario: solo Mineral/Sintético. |
| Corrección sin aceite proponía sintético | Resuelto cuando próximo es null: select deshabilitado y envío sin tipo. Prueba aprobada. |
| Corrección precargaba lectura de ingreso | Mejorado: backend entrega lectura final en el campo unificado y el modal precarga 50.010, comprobado. El campo sigue llamándose ingreso. |
| Pruebas Rust | Ahora existen 12 y pasan todas; se añadieron los casos de placa por cajero y sin motivo. |

Se mantienen las garantías anteriores de motor opcional/códigos correctos, teléfono normalizado en backend, identidad tipo/número, auditoría identificada y transacciones, privacidad de contactos, estado inactivo y consultas de candidatos. La aceptación completa no se da por realizada por repetir estas garantías ya comprobadas.

## Bloqueos actuales

### 1. El detalle de una atención sin aceite falla al renderizar

Referencias: `src-tauri/src/dto/orden_trabajo_dto.rs:23`, `src/types/vehiculo.ts:51`, `src/pages/vehiculo/components/ModalDetalleOT.tsx:119`.

Rust ahora devuelve `proximo_kilometraje: null` cuando corresponde, pero TypeScript sigue declarando `number`. El detalle ejecuta `.toLocaleString()` sin comprobar null. Se reprodujo el fallo mediante un error boundary de prueba que recibió el error de renderizado.

Debe actualizarse el contrato TypeScript a `number | null`, revisar sus consumidores y mostrar «No registrado»/«No aplica» según el contexto. El historial usa `?? 0`, evitando el error pero mostrando «Próximo servicio: 0 km», que tampoco representa ausencia.

Que `npm run typecheck` pase no acredita este contrato: el compilador no verifica el JSON emitido por Rust.

### 2. Sigue faltando la identidad histórica pública del cliente

Referencia: `src-tauri/src/dto/orden_trabajo_dto.rs:14` y consulta de historial en `ordenes_service.rs:635`.

No se devuelven el nombre/documento del cliente de aquella atención ni una representación equivalente. La ficha muestra propietario actual y la base tiene snapshot, pero el detalle no puede presentar la distinción exigida.

Prueba: se guardó un snapshot ficticio con nombre «Cliente historico» y documento; el historial serializado no contiene esa identidad. Cambiar propietario o documento actual exige conservar y mostrar la identidad atendida en cada fecha.

### 3. Un material ACEITE puede volver a inventar un cambio que no existe en la atención

Referencias: `src-tauri/src/services/ordenes_service.rs:47`, `:74` y fallback de `obtener_tipo_aceite_ot`.

`tiene_cambio_aceite` considera también productos cuya categoría actual sea ACEITE/GRANEL. El helper recalcula en lugar de usar directamente el próximo km guardado en la atención.

Se preparó una OT terminada con servicio no marcado como cambio, un material ACEITE y `tipo_aceite/intervalo/km_proximo` nulos. SQLite conserva `km_proximo=NULL`, pero el historial devuelve **55.010**. La consulta pública debe respetar el hecho guardado y ser coherente con la vista usada para candidatos; la categoría actual de un producto no debe reinterpretarlo.

### 4. Login todavía anuncia escritura de Ficha al mecánico

Referencia: `src-tauri/src/services/auth_service.rs:804`.

La prueba de autenticación real en memoria volvió a obtener `puede_crear=true` y `puede_editar=true` en FICHA_VEHICULAR para el mecánico. La página consume esos permisos y muestra Nuevo cliente/vehículo y edición. También sigue tomando el permiso genérico de OT para mostrar recepción.

Los comandos de escritura siguen rechazando mecánico; no se afirma que pueda completar esas operaciones. La ocultación de Próximos cambios sí quedó corregida, pero el resto de la matriz de solo lectura sigue pendiente.

### 5. La edición administrativa funciona en backend, pero no se puede completar correctamente desde UI

Referencia: `src/pages/vehiculo/components/ModalVehiculo.tsx:270`.

La placa sigue deshabilitada incluso para administrador; la prueba de habilitación falló. Falta habilitarla solo para el rol autorizado con motivo y confirmación.

El motivo del formulario aparece únicamente al disminuir km, no al cambiar propietario. Como el backend ya lo exige para propietario, el recorrido habitual queda incompleto. Se debe separar edición común de campos administrativos y seleccionar al nuevo cliente explícitamente, sin modificar su identidad/contactos mediante una transferencia.

El requisito de confirmación al guardar cambios sensibles/correcciones todavía no está completado. Los diálogos de estado sí tienen confirmación/motivo, pero siguen sin un estado de envío conectado que impida reiteraciones mientras espera respuesta.

### 6. El modal de cliente trunca teléfonos al pegar +51

Referencia: `src/pages/vehiculo/components/ModalCliente.tsx:159`.

El backend y el formulario de vehículo permiten normalización, pero el modal de cliente elimina no dígitos y corta a nueve. La prueba de pegar `+51 912-345-678` dejó **519123456**, un número diferente que satisface longitud y puede persistirse.

Debe conservar el texto para normalizarlo en backend o normalizar el prefijo antes de limitar longitud. No se debe truncar silenciosamente.

### 7. Historial todavía no pagina de 20 y no está completa su presentación

La prueba con 21 trabajos mostró las 21 filas a la vez. Rust también devuelve el conjunto completo y carga detalles de cada atención. Falta paginación y validación del rendimiento requerido.

Aunque ahora `kilometraje_ingreso` transporta km final, interfaz y detalle siguen rotulándolo «Ingreso». Debe aclararse el contrato para presentar «Km de atención» y no confundir datos históricos. La fecha final se muestra sin hora. También faltan resumen de último servicio/último cambio y acceso autorizado al origen desde detalle.

### 8. Fallo y ausencia continúan mezclándose

Referencia: `src/pages/vehiculo/FichaVehicularPage.tsx:401`.

Se corrigió la concurrencia A/B, pero ante un error de almacenamiento se muestra tanto su mensaje como «Vehículo no encontrado», con opción de alta. Prueba React fallida. Debe diferenciarse NOT_FOUND de error real.

Persisten `catch(() => [])` para órdenes y algunas conversiones indiscriminadas a NotFound. Faltan Limpiar, confirmación única de descarte de captura y comprobación de versión esperada para formularios obsoletos. El contador nuevo no sustituye el control de escritura obsoleta ni acredita por sí solo todos los casos de fin de sesión.

### 9. Próximos cambios conserva orden y refresco incompletos

Referencia: `src-tauri/src/services/cliente_service.rs:957`.

Se conserva `ORDER BY fecha_servicio DESC`; el PDF exige más antiguo primero e ID como desempate. La búsqueda/filtros/páginas visuales existentes y el acceso por rol sí reciben crédito.

`rows.flatten()` sigue omitiendo errores de lectura. La página actual no se ajusta si Actualizar reduce el conjunto. Los filtros se pierden al desmontar la lista para abrir una ficha y volver; falta conservar el contexto autorizado según el recorrido previsto.

### 10. Crear OT cambió de error, pero todavía falla

Referencia: `src-tauri/src/services/ordenes_service.rs:279` y trigger V7 `trg_ot_nace_espera`.

Ya se retiró el uso de `id_zanja_prevista`: no se repite ese hallazgo como si siguiera igual. Ahora se inserta `id_zanja_ejecucion` y tipo de aceite en EN_ESPERA. El SQL prohíbe que una OT nazca con datos de ejecución/cambio final.

La prueba actual falló con **«OT nace en espera libre o asignada, sin ejecucion»**. Debe nacer en cola, responsable opcional y sin zanja de ejecución ni aceite final; estos se asignan/confirman en sus etapas. El DTO/modal todavía exigen mecánico/aceite y condicionan recepción a zanjas. La lista de vehículos del modal continúa limitada a candidatos de aceite.

Se mantiene como contrato de integración con Taller, sin exigir completar todo Taller. No se modifica el SQL para admitir un flujo contrario a la simplificación documental.

## Matriz de requisitos

| Requisito | Cuarta revisión |
|---|---|
| RF-FVH-01 Búsqueda | Mejorado: respuestas antiguas descartadas; faltan Limpiar y distinguir ausencia/error. |
| RF-FVH-02 Alta/reutilización | Mejorado: Nuevo cliente operativo y una llamada, alta conjunta y tipo/número; faltan selección explícita y resolución navegable de duplicados. |
| RF-FVH-03 Edición común | Mejorado: un guardado y espera de resultado; permisos de mecánico, campos/identidad y teléfono UI incompletos. |
| RF-FVH-04 Historial | Mejorado: ID estable, finalizadas, lectura final, próximo típico y corregido; faltan identidad histórica, páginas y robustez null/materiales. |
| RF-FVH-05 Sensibles | Backend de placa/propietario/reducción protegido; UI de placa/motivo/confirmación pendiente. |
| RF-FVH-06 Corrección | Mejora en lectura final, aceite opcional y sin semisintético; falta confirmación y contrato completo de datos/ausencias. |
| RF-FVH-07 Estado | Backend/diálogos con motivo conservados; falta control de envío reiterado y referencia de OT bloqueante. |
| RF-FVH-08 Candidatos | Acceso, filtros y páginas visuales disponibles; orden/contexto/errores aún pendientes. |
| RF-FVH-09 Recepción | Sigue bloqueada por contrato con Taller; error de columna corregido, trigger de nacimiento aún falla. |
| RNF-FVH-01 Rendimiento | No acreditado: sin paginación de historial ni cinco mediciones con 1.000 vehículos/equipo objetivo; benchmark deshabilitado. |
| RNF-FVH-02 Usabilidad | Mejora con flujos conectados y esperas; faltan errores por campo, descarte, km cero precargado y presentación correcta de ausencias. |
| RNF-FVH-03 Privacidad | Contactos backend protegidos y candidatos ocultos al mecánico; su matriz general y la identidad pública todavía requieren corrección. |
| RNF-FVH-04 Integridad | Mejora importante de ID y permisos de placa; transacciones/auditoría y un guardado conservados; falta control de ediciones obsoletas. |
| RNF-FVH-05 Local/Lima/errores | Local y SQL Lima conservados; falta validar instantes/medianoche y propagar errores sin convertirlos en ausencia. |

Otros pendientes ya identificados, sin ampliar alcance: `onVolver` declarado sin uso; formularios no enlazados a `setHayCambiosSinGuardar`; km cero se precarga vacío; motivo y longitudes Rust por bytes; auditoría no conserva todos los contactos/campos comunes modificados; estado inactivo bloquea toda edición de vehículo en lugar de aplicar correctamente la distinción por rol. No se evaluaron de nuevo todos esos casos por ejecución; se confirmó su presencia por lectura.

## Pruebas ejecutadas

| Comprobación | Resultado |
|---|---|
| TypeScript del producto antes de tests temporales | Correcto |
| Pruebas existentes Rust Ficha | 12/12 aprobadas |
| Pruebas existentes React Ficha | 4/4 aprobadas |
| `auth_regression_tests` | 12/12 aprobadas |
| `auth_tests` | 18/18 aprobadas |
| Casos adicionales Rust | 14: 10 aprobados, 4 fallidos |
| Casos adicionales React | 12: 6 aprobados, 6 fallidos |

La ejecución Rust temporal incluyó 12 originales: total 26, 22 aprobados/4 fallidos. React incluyó 4 originales: total 16, 10 aprobados/6 fallidos. La tabla descuenta las repeticiones. Los casos adicionales expresan requisitos esperados; los fallos no son errores de compilación del producto. No se usa el número de casos para deducir cobertura total.

Rust aprobado: bloqueo de placa por cajero; motivo de placa; placa admin con motivo e ID conservado; aislamiento tras reutilizar placa; km final/próximo sintético; km corregido; próximo null en atención sin cambio/material; teléfono normalizado; auditoría y km actual intacto; contacto mecánico filtrado.

Rust fallido: identidad histórica pública; permisos mecánico tras Login; próximo inventado por material; recepción contra esquema actual.

React aprobado: alta de cliente sin vehículo con una llamada; edición con una llamada; descarte de respuesta antigua; corrección sin aceite sin agregar tipo; lectura final/opciones de aceite; candidatos ocultos al mecánico.

React fallido: detalle con null; mecánico sin acciones de escritura; fallo distinto de ausencia; placa habilitada para admin; teléfono +51 sin truncado; primera página de historial limitada a 20.

El caso de detalle usa un error boundary temporal para detectar el error de renderizado sin interrumpir el resto de la suite. Los mocks de interfaz no se presentan como IPC real; el permiso incorrecto de mecánico también se verificó en la prueba Rust de Login.

## Condiciones concretas de cierre

1. Armonizar DTO TypeScript con null y hacer seguros detalle/lista ante ausencia de próximo cambio.
2. Entregar y mostrar identidad histórica del cliente y paginar el historial.
3. Respetar el cambio/intervalo/próximo guardados; retirar reinterpretación por categoría actual de materiales.
4. Corregir permisos de escritura del mecánico y su botón de recepción.
5. Completar UI administrativa de placa/propietario/motivo/confirmación.
6. Corregir teléfono del modal de cliente y error frente a ausencia.
7. Completar orden/contexto de candidatos, descarte, envíos y formularios obsoletos.
8. Alinear recepción con el esquema y el contrato del siguiente módulo.
9. Ejecutar los diez TC-FVH aplicables, cinco mediciones de rendimiento, validación de fechas y recorrido manual Tauri. La integración económica completa se comprobará al trabajar POS.

La nota reconoce correcciones reales de los bloqueos más graves de la revisión anterior. El módulo permanece pendiente por incumplimientos identificables y una regresión de renderizado, no por exigir terminar los demás módulos del ERP.
