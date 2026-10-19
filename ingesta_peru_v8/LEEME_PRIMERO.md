# Lubricentro Gloria — carga peruana ampliada v8

## Lo que cambia

Esta version parte de tu BD RESTAURADA: administrador ID 1, Cliente varios ID 1, configuracion ID 1 en PRODUCCION, seis zanjas y auditoria hasta ID 10. No necesitas restaurarla otra vez si ya esta asi.

Se quitaron de los campos habituales los nombres con DEMO, las descripciones de catalogo simulado, los correos example.invalid, los telefonos vacios y los textos de operaciones de prueba. Se reemplazaron por nombres completos, razones sociales, contactos, domicilios completos, codigos internos y descripciones naturales de un lubricentro peruano. Tambien se ampliaron los registros y la variedad de trabajos.

Los datos siguen siendo GENERADOS y no verificados: su apariencia natural no acredita identidades, direcciones, actividad comercial ni estado tributario. No son una lista de clientes o proveedores reales. Telefonos, correos y documentos pueden coincidir accidentalmente con datos existentes; no deben utilizarse para contactar personas o emitir comprobantes reales.

## Ejecutar

Cierra el software y desconecta la conexion de BD de VS Code. Extrae el ZIP en `C:\Projects\lubricentro-erp`; debe aparecer la carpeta `ingesta_peru_v8`.

En PowerShell:

```powershell
cd "C:\Projects\lubricentro-erp"
node .\ingesta_peru_v8\cargar_original.cjs --bd "C:\Users\lozay\AppData\Roaming\com.lubricentro.app\lubricentro_secure.db"
```

Requiere Node.js 22.13 o posterior con node:sqlite; no requiere instalar paquetes npm ni Python.

El ejecutor trabaja DIRECTAMENTE sobre esa BD original. Conserva exactamente el administrador y su contrasena, configuracion completa en PRODUCCION/America-Lima, codigo de recuperacion, seis zanjas y Cliente varios. Conserva la auditoria anterior, aunque haya mas de diez eventos por nuevos accesos.

Antes de insertar crea un respaldo fisico real, lo verifica y calcula tamano/SHA256. Todos los datos nuevos se insertan en una sola transaccion con verificaciones antes del COMMIT. Si hay un error se revierte la carga; no se borra ni renumera lo anterior.

El respaldo se guarda en `respaldos_ingesta`, junto a la BD. La ruta exacta aparece en consola y se registra en `backups` como PREVIA. Contiene el estado anterior a esta carga. El informe final queda junto a la BD como `lubricentro_secure.db.ingesta_peru_v8.validacion.json`.

Si alguna tabla operativa ya tiene datos, el ejecutor se detiene antes de cargar. Este paquete se usa UNA VEZ sobre la base restaurada, no encima de la ingesta anterior. No es necesario ni correcto ejecutar otra vez el paquete v7.

## Conteos finales

| Tabla | Filas finales |
|---|---:|
| usuarios | 11: tu administrador + 5 cajeros y 5 mecanicos |
| configuracion | 1, sin cambios |
| zanjas | 6, sin cambios |
| clientes | 401: Cliente varios + 320 personas DNI + 80 empresas RUC |
| proveedores | 30 |
| vehiculos | 500: 450 activos y 50 inactivos |
| productos | 120: 36 aceites envasados, 40 filtros, 20 aditivos, 16 insumos y 8 granel |
| servicios | 20 |
| ingresos_stock | 32: dos entradas iniciales y 30 compras |
| detalle_ingreso | 240 |
| movimientos_inventario | 1250 |
| ordenes_trabajo | 665: 630 finalizadas, 23 en espera, 2 en proceso y 10 canceladas |
| detalle_ot_servicios | 1232 |
| detalle_ot_productos | 693 |
| caja_chica | 63, cerradas |
| ventas | 945: 630 procedentes de OT y 315 directas |
| detalle_venta | 2205 |
| movimientos_caja | 1176 |
| series_comprobante | 4 |
| comprobantes | 945, PENDIENTE y sin envio |
| solicitudes_operacion | 945 |
| auditoria | 2998 si partias de 10; agrega 2988 eventos a los que ya tenias |
| backups | 1 respaldo fisico PREVIO real |
| devoluciones | 0 |
| detalle_devolucion | 0 |
| restauraciones | 0 |
| conciliaciones_series | 0 |

Siguen siendo 27 tablas, 23 con datos y 4 sin operaciones que las justifiquen. No se modifica el esquema, ni se desactivan claves foraneas o triggers. Las tablas vacias permanecen asi por los requisitos fiscales y de recuperacion explicados mas abajo.

## IDs y accesos

- Administrador ID 1, sin cambios.
- Nuevos usuarios IDs 2 a 11.
- Cliente varios ID 1 y clientes nuevos IDs 2 a 401.
- Vehiculos IDs 1 a 500. Las demas tablas previamente vacias tambien comienzan desde 1.
- Auditoria continua con AUTOINCREMENT desde el ultimo evento existente.
- Usuarios `cajero01` a `cajero05` y `mecanico01` a `mecanico05`, contrasena `q123456`, con el hash Argon2id que proporcionaste y que se verifico. Tu administrador conserva su propia contrasena.

## Contexto peruano y variedad

- Nombres y apellidos completos sin numeracion artificial. Razones sociales con S.A.C., S.R.L. o E.I.R.L.
- Domicilios con via, numero, urbanizacion, distrito, provincia y departamento. Distritos de Chincha, Pisco, Ica y Canete.
- Celulares peruanos de nueve digitos y correos con formato habitual. Son generados; no se valido que pertenezcan a alguien ni que existan.
- DNI de ocho digitos y RUC de once con digito de control. No se consulto SUNAT ni RENIEC.
- 280 clientes con un vehiculo, 110 con dos y 10 sin vehiculos. Cliente varios no es propietario de ninguno.
- Marcas/modelos habituales: Toyota, Nissan, Hyundai, Kia, Chevrolet, Suzuki, Mitsubishi, Volkswagen, Renault y JAC. Placas y propietarios generados; sin validacion registral.
- Catalogo con lubricantes, presentaciones envasadas, filtros de aceite/aire/combustible/cabina, aditivos, refrigerantes, liquidos de frenos, grasas, limpiadores y granel. Los codigos de filtros son internos, no referencias OEM ni garantias de compatibilidad. Las presentaciones y precios son referenciales, sin cotejo comercial.
- Trabajos de cambio de aceite mineral y sintetico, cambio de filtros, refrigerante, transmision, engrase, revision de niveles y limpieza de bornes. Los consumos corresponden al servicio y heredan costo y precio al cobrarse en POS.
- 63 jornadas, de marzo a octubre de 2026, con la mayor parte entre agosto y octubre. Hay 126 cambios de aceite; no todas las atenciones se marcan como cambios de aceite.
- Un vehiculo tiene 23 atenciones para probar paginacion. Hay vehiculos sin historial, recordatorios por fecha y kilometraje, estados activos/inactivos y una correccion publica documentada.
- Pagos por efectivo, Yape, Plin, tarjeta y transferencia, algunas ventas con dos medios, descuentos del 5 %, gastos, aportes, diferencias de caja justificadas y cierres conciliados.

Las operaciones historicas se incorporan como historial generado; las fechas anteriores a la instalacion no prueban actividad real del administrador en ese momento. La auditoria de importacion utiliza actor SISTEMA.

## Comprobantes y tablas que permanecen vacias

Se conserva PRODUCCION tal como pediste. El ejecutor no tiene llamadas de red y exige que NubeFact siga sin configurar. Los documentos quedan PENDIENTE; el payload local incluye `no_enviar` y no constituye una solicitud fiscal valida. No hay aceptaciones SUNAT, XML, CDR o codigos oficiales inventados.

No configures NubeFact sobre esta BD con operaciones generadas. El campo tecnico `no_enviar` informa del alcance de la carga; no garantiza que el backend existente lo respete si despues se conecta un emisor. Las series B001/F001/BC01/FC01 de este conjunto no deben asumirse como series reales disponibles o conciliadas.

`devoluciones` y `detalle_devolucion` permanecen vacias porque el esquema exige comprobantes aceptados. `restauraciones` y `conciliaciones_series` tambien quedan vacias porque no se ejecuta una restauracion ni se contrasta un correlativo con un emisor externo. No se inventan esos hechos para llenar tablas. El respaldo previo si corresponde a un archivo real.

## Archivos y comprobaciones

4571 SQL, separados por tabla y etapa, con su orden en `plan.json` y resumen en `INDICE_ARCHIVOS.csv`. El ejecutor los procesa automaticamente en un solo comando; no necesitas abrirlos uno por uno.

Una tabla puede reaparecer en varias fases para respetar sus dependencias: OT -> servicios -> inicio -> consumo -> kardex -> saldo -> fin, o venta -> detalle -> kardex/saldo -> cobro -> confirmacion -> reserva de serie -> documento. No agrupar manualmente todos los archivos de una tabla ni ejecutarlos aisladamente. El ultimo SQL de backups recibe parametros reales del archivo creado.

Se comprueban integridad SQLite, relaciones, seis vistas, stock/valor contra kardex, importes y detalles de ventas, cobros, cinco medios de pago en caja, correlativos y preservacion exacta de tus datos iniciales. Tambien se probaron rechazo de una carga repetida y ROLLBACK completo ante un error tardio.

`PRUEBAS_REALIZADAS.txt` describe las comprobaciones y el entorno real. `VALIDAR_SOLO_LECTURA.sql` permite revisar el resultado. Estas pruebas validan la ingesta; no sustituyen probar comandos Rust y pantallas Tauri.
