<div align="center">

# 🚗 Lubricentro Gloria

### Gestión de taller, inventario y ventas en una aplicación de escritorio

**Proyecto académico · Ingeniería de Sistemas · Perú**

![Tauri](https://img.shields.io/badge/Tauri-24C8D8?style=for-the-badge&logo=tauri&logoColor=black)
![React](https://img.shields.io/badge/React-20232A?style=for-the-badge&logo=react&logoColor=61DAFB)
![TypeScript](https://img.shields.io/badge/TypeScript-3178C6?style=for-the-badge&logo=typescript&logoColor=white)
![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![SQLite](https://img.shields.io/badge/SQLite-003B57?style=for-the-badge&logo=sqlite&logoColor=white)

![Estado](https://img.shields.io/badge/Estado-En_desarrollo-E9A23B?style=flat-square)
![Plataforma](https://img.shields.io/badge/Plataforma-Windows_11-2563EB?style=flat-square)
![Módulos](https://img.shields.io/badge/Alcance-10_m%C3%B3dulos-0F766E?style=flat-square)

[Progreso](#estado) · [Login](#modulo-login-y-sesion) · [Ficha vehicular](#modulo-ficha-vehicular-e-historial) · [Taller](#modulo-ordenes-de-trabajo-taller) · [Inventario](#modulo-inventario-y-stock) · [Instalación](#instalacion) · [Documentación](#documentacion)

</div>

<a id="presentacion"></a>

## 1. Presentación

**Lubricentro Gloria** es una aplicación de escritorio para conectar atención vehicular, taller, inventario, ventas y caja en **Lubricentro Gloria S.R.L.** Busca evitar registros dispersos, consumos duplicados y cobros desvinculados del trabajo realizado.

**Objetivo:** facilitar la gestión diaria mediante procesos claros, datos relacionados y permisos por rol.

| Tecnología             | Uso en el proyecto                                                        |
| ---------------------- | ------------------------------------------------------------------------- |
| **Tauri**              | Aplicación de escritorio y comunicación entre interfaz y backend.         |
| **React + TypeScript** | Pantallas, formularios y navegación.                                      |
| **Vite**               | Herramientas de desarrollo y compilación del frontend.                    |
| **Rust**               | Comandos, validaciones y lógica del negocio.                              |
| **SQLite**             | Almacenamiento local de los datos.                                        |
| **NubeFact**           | Integración definida para comprobantes electrónicos vinculados con SUNAT. |

**Alcance:** un establecimiento, una PC con Windows 11, una sesión activa y una caja abierta a la vez. Moneda PEN y hora `America/Lima` (UTC−05:00). La operación local no requiere conexión permanente; NubeFact sí la requiere para envíos, consultas y archivos oficiales.

<a id="estado"></a>

## 2. Estado del proyecto

Desarrollo secuencial, módulo por módulo, conforme al alcance funcional v7. **Login y Sesión (95/100)**, **Ficha Vehicular e Historial (93/100)**, **Órdenes de Trabajo Taller (93/100)** e **Inventario y Stock (95/100)** se encuentran en **ajustes finales**. Sus funciones principales ya están implementadas; los pendientes de cierre y la validación en escritorio se detallan en cada módulo.

**La siguiente etapa será la implementación de Punto de Venta POS**, integrando ventas directas, cobro de órdenes y comprobantes sin duplicar los consumos del taller. Inicio Dashboard se desarrollará al final para reunir datos de los demás módulos.

| Orden | Módulo | Estado | Evaluación |
| --- | --- | --- | --- |
| 01 | [Login y Sesión](#modulo-login-y-sesion) | 🟢 Ajustes finales | 95/100 |
| 02 | [Ficha Vehicular e Historial](#modulo-ficha-vehicular-e-historial) | 🟢 Ajustes finales | 93/100 |
| 03 | [Órdenes de Trabajo Taller](#modulo-ordenes-de-trabajo-taller) | 🟢 Ajustes finales | 93/100 |
| 04 | [Inventario y Stock](#modulo-inventario-y-stock) | 🟢 Ajustes finales | 95/100 |
| 05 | Punto de Venta POS | ⚪ Pendiente · próximo módulo | — |
| 06 | Caja Chica | ⚪ Pendiente | — |
| 07 | Backup y Copias | ⚪ Pendiente | — |
| 08 | Administración | ⚪ Pendiente | — |
| 09 | Reportes | ⚪ Pendiente | — |
| 10 | Inicio Dashboard | ⚪ Pendiente; se desarrollará al final | — |

**Leyenda:** ⚪ Pendiente · 🟡 En curso · 🟢 Ajustes finales · ✅ Completado.

> **Evaluación:** los puntajes reflejan implementación y alineación con los PDF de los módulos; no son porcentajes de avance, cobertura de pruebas ni aprobación de todos los recorridos en escritorio. Las filas pendientes pueden tener código previo. Los roles, flujos y diagramas describen el alcance definido.

<a id="modulos"></a>

## 3. Detalle de los módulos

<a id="modulo-login-y-sesion"></a>

<details>
<summary><b>01 · Login y Sesión</b> — 🟢 Ajustes finales · Evaluación: 95/100</summary>

**Qué hace:** controla el acceso mediante cuentas individuales y mantiene la sesión del trabajador. Identifica su rol y reúne los flujos de primera instalación, cambio y recuperación de contraseña.

**Funciones principales:**

- **Acceso individual:** identifica al trabajador, habilita los accesos de Administrador, Cajero o Mecánico y rechaza cuentas inactivas o bloqueadas.
- **Primer administrador:** permite crearlo únicamente cuando no existen cuentas, con contraseña definitiva y código de recuperación.
- **Cambio de contraseña:** admite cambio voluntario con la clave actual y exige reemplazar las temporales antes de operar, sin volver a pedir la temporal ya autenticada.
- **Recuperación administrativa:** verifica el código vigente, cambia la contraseña y renueva el código para invalidar el anterior.
- **Control de sesión:** una sesión por aplicación, vencimiento tras 15 minutos de inactividad y rechazo de acceso si la cuenta pierde vigencia. Salir no cierra caja ni cancela órdenes automáticamente.
- **Protección de acceso:** bloqueo de 15 minutos tras cinco fallos consecutivos de una cuenta existente.
- **Autorización administrativa:** comprueba credenciales desde la operación que las requiere y conserva la sesión del trabajador solicitante.

**Fortalezas de implementación:**

- Rust valida identidad, estado de sesión y restricciones; el control no depende solo de la interfaz.
- Contraseñas protegidas con hashes **Argon2id**, sin almacenar claves en texto.
- Instalación, cambio de contraseña y recuperación coordinados mediante transacciones y auditoría para evitar éxitos parciales.
- **30 pruebas de autenticación aprobadas** en la última revisión reportada, incluyendo bloqueo, recuperación, revocación y fallos de escritura.

**Acceso por rol:**

| Usuario                 | Alcance                                                                                                 |
| ----------------------- | ------------------------------------------------------------------------------------------------------- |
| Administrador           | Sesión propia, cambio de clave y recuperación con código; otras cuentas se gestionan en Administración. |
| Mecánico                | Sesión propia, cambio de clave y módulos autorizados.                                                   |
| Cajero                  | Sesión propia, cambio de clave y módulos autorizados.                                                   |
| Instalación sin cuentas | Creación exclusiva del primer administrador.                                                            |

Se requiere una cuenta activa y completar el cambio obligatorio antes de operar. Las contraseñas se protegen mediante hashes Argon2id.

**Capturas:**

_Inicio de sesión_

![Pantalla de inicio de sesión](docs/images/01-login.png)

_Recuperación de acceso_

![Recuperación de acceso](docs/images/01-login-recuperacion.png)

_Creación del primer administrador_

![Creación del primer administrador](docs/images/01-login-primer-administrador.png)

_Cambio de contraseña_

![Cambio de contraseña](docs/images/01-login-cambio-clave.png)

**Pendiente para cerrar:**

- Mostrar un error claro y permitir reintentar si falla la comprobación inicial de instalación.
- Activar el aviso de captura pendiente únicamente cuando existan modificaciones sin guardar.
- Completar evidencia de navegación por teclado, recorridos en escritorio y mediciones de tiempo de ingreso exigidas.

**Estado:** **95/100 en la última revisión**, con ajustes y validación de escritorio pendientes antes del cierre.

</details>

<a id="modulo-ficha-vehicular-e-historial"></a>

<details>
<summary><b>02 · Ficha Vehicular e Historial</b> — 🟢 Ajustes finales · Evaluación: 93/100</summary>

**Qué hace:** centraliza la identificación del vehículo, su propietario y las atenciones realizadas. Permite consultar la ficha y mantener sus datos relacionados con las órdenes del taller.

**Funciones principales:**

- **Buscar vehículos:** normaliza la placa y muestra vehículo, propietario, kilometraje y estado; permite limpiar la selección y reintentar consultas fallidas.
- **Registrar clientes y vehículos:** admite clientes sin vehículo, reutiliza clientes existentes y permite el registro conjunto de forma atómica.
- **Editar información:** administrador y cajero modifican datos comunes; cambiar placa, propietario o reducir kilometraje exige administrador y motivo.
- **Desactivar y reactivar:** conserva el historial, bloquea la desactivación si existe una OT abierta e impide recibir en Taller un vehículo inactivo.
- **Consultar mantenimientos:** muestra OT finalizadas, aunque estén pendientes de cobro, con **20 registros por página**, servicios, insumos, resumen, kilometraje, aceite y cliente histórico. «Ver atención» abre el detalle completo.
- **Corregir datos públicos:** el administrador puede ajustar kilometraje de atención, resumen o aceite cuando corresponde; la atención queda marcada como corregida y se registra auditoría.
- **Consultar próximos cambios:** búsqueda por placa o cliente, filtros por kilometraje, fecha o ambos, teléfono y acceso a la ficha. No registra llamadas ni oculta candidatos.
- **Abrir recepción:** lleva el vehículo seleccionado al formulario de OT y comprueba su actividad y la existencia de otra orden abierta.

**Fortalezas de implementación:**

- Historial vinculado al **ID del vehículo**: corregir la placa no desvincula las atenciones anteriores.
- Respuestas públicas específicas, sin notas técnicas ni costos; el mecánico tampoco recibe contactos del propietario.
- Paginación, filtros y conteos en SQLite, con orden definido.
- Restricciones persistentes, transacciones y auditoría para altas y cambios administrativos.
- Corregir una atención no modifica automáticamente el kilometraje actual ni mueve stock o dinero.
- **16 pruebas de vehículos aprobadas** en la última revisión reportada, además de comprobaciones adicionales de las correcciones recientes.

**Acceso por rol:**

| Rol           | Alcance                                                                                                                          |
| ------------- | -------------------------------------------------------------------------------------------------------------------------------- |
| Administrador | Gestión de ficha, cambio de placa o propietario, reducción de kilometraje, correcciones y desactivación/reactivación con motivo. |
| Cajero        | Registro, edición de datos comunes y consulta autorizada.                                                                        |
| Mecánico      | Consulta de información pública del vehículo y sus atenciones, sin contactos del propietario.                                    |

**Capturas:**

_Ficha vehicular principal_

![Ficha vehicular principal](docs/images/ficha-vehicular.png)

_Historial de atenciones_

![Historial de atenciones del vehículo](docs/images/ficha-vehicular-historial.png)

_Registro o edición del vehículo_

![Formulario de registro o edición del vehículo](docs/images/ficha-vehicular-registro.png)

_Próximos cambios de aceite_

![Próximos cambios de aceite](docs/images/ficha-vehicular-proximos-cambios-aceite.png)

**Pendiente para cerrar:**

- Activar el indicador de captura pendiente al modificar datos, no al abrir el formulario.
- Incorporar a las pruebas permanentes los casos recientes de filtros, paginación, resumen y recepción.
- Completar los recorridos en escritorio y las cinco mediciones con el volumen de prueba definido.

**Estado:** **93/100 en la última revisión**. Las funciones principales están alineadas; el cierre se concentra en precisión de estados, tratamiento de errores y evidencia de validación.

</details>

<a id="modulo-ordenes-de-trabajo-taller"></a>

<details>
<summary><b>03 · Órdenes de Trabajo Taller</b> — 🟢 Ajustes finales · Evaluación: 93/100</summary>

**Qué hace:** organiza la atención del vehículo desde la recepción hasta la finalización o cancelación. Reúne servicios, responsable, zanja, materiales y resultado del trabajo, vinculados con Ficha Vehicular e Inventario.

**Funciones principales:**

- **Recepcionar vehículos:** selecciona un vehículo activo, registra kilometraje, pedido y servicio inicial; permite elegir mecánico o dejar la orden libre, evitando otra OT abierta para el mismo vehículo.
- **Consultar el tablero:** muestra primero las zanjas habilitadas y su ocupación, seguidas de una cola común de espera compacta. El mecánico consulta propias y libres; los trabajos ajenos se presentan sin información privada.
- **Buscar y listar órdenes:** ofrece páginas de **20 registros**, búsqueda por placa o código, filtros por estado, responsable y fechas, además de vistas de propias y libres. Propone la fecha de Lima y limita el rango a 366 días.
- **Iniciar trabajo:** exige una zanja habilitada y libre. El mecánico que inicia una orden libre queda asignado en la misma operación; el administrador selecciona responsable cuando falta.
- **Reasignar responsable:** el administrador puede cambiar al mecánico con motivo, conservando el creador y los datos originales.
- **Gestionar servicios:** administrador y mecánico propietario pueden agregar, modificar cantidades o retirar servicios mientras la OT esté abierta, manteniendo al menos uno. Los repetidos aumentan su línea al precio histórico.
- **Editar la nota interna:** permite una observación privada por orden, editable por administrador o mecánico responsable mientras esté abierta; queda separada del resumen público.
- **Registrar consumos:** descuenta materiales realmente utilizados durante la ejecución, conserva cantidades, precios y costos históricos y genera su movimiento de inventario.
- **Devolver materiales recuperados:** el administrador registra cantidad y motivo, sin superar el consumo neto; repone stock al costo de origen y conserva el retiro original.
- **Finalizar la atención:** propone kilometraje y resumen de servicios; registra aceite cuando corresponde, calcula próximo cambio, actualiza Ficha y libera la zanja.
- **Cancelar una orden:** el administrador confirma con motivo. Libera la zanja, conserva el consumo neto como costo interno y no devuelve stock automáticamente ni modifica el kilometraje del vehículo.
- **Recibir avisos de resultado:** muestra éxito después de confirmar el guardado y la causa de los rechazos, con el mismo componente de avisos de Ficha Vehicular. Los errores permanecen en el formulario y conservan lo escrito; si falla la recarga después de guardar, lo informa sin pedir repetir la operación.

**Fortalezas de implementación:**

- Validaciones de sesión, rol, responsable y estado desde Rust.
- Restricciones persistentes para una OT abierta por vehículo y una en proceso por zanja.
- Transacciones para mantener consistentes estado, inventario y kilometraje.
- Auditoría transaccional de recepción, inicio, nota, reasignación, finalización y cancelación.
- Privacidad según rol y propietario: cajero sin notas ni costos; mecánico sin notas ajenas ni costos internos.
- Cantidades de granel almacenadas en mililitros, con conversión y valoración verificadas.
- Identificador de operación en consumos y devoluciones: el backend evita repetir efectos cuando recibe el mismo identificador. Su conservación al reintentar desde la interfaz sigue pendiente.
- Modal organizado en hook, contenedor y componentes de acciones, servicios e insumos.
- **13 pruebas de Órdenes aprobadas** en la última revisión reportada, además de comprobaciones adicionales de auditoría, reversión e identificadores repetidos.

**Acceso por rol:**

| Rol | Alcance |
| --- | --- |
| Administrador | Recepción y gestión del taller; asignación y reasignación, notas, servicios, consumos, devolución de materiales y cancelación con motivo. |
| Mecánico | Consulta de propias y libres; inicio de las permitidas y ejecución de sus órdenes. Gestiona servicios y nota interna en las condiciones autorizadas, sin costos internos ni notas ajenas. |
| Cajero | Recepción y consulta pública autorizada, sin notas internas ni costos ni operaciones de ejecución reservadas. |

**Capturas:**

_Tablero: zanjas y cola de espera_

![Tablero de zanjas y cola de espera del taller](docs/images/03-ordenes-tablero.png)

_Recepción del vehículo_

![Formulario de recepción de una orden de trabajo](docs/images/03-ordenes-recepcion.png)

_Detalle: acciones, servicios e insumos_

![Detalle de una orden de trabajo con servicios e insumos](docs/images/03-ordenes-detalle.png)

_Historial y filtros_

![Historial paginado y filtros de órdenes de trabajo](docs/images/03-ordenes-historial.png)

_Aviso de resultado de una operación_

![Aviso de éxito o rechazo en Órdenes de Trabajo](docs/images/03-ordenes-avisos.png)

**Pendiente para cerrar:**

- Conservar el mismo identificador al reintentar un retiro o devolución tras una respuesta incierta; la protección de Inventario no se aplica automáticamente a estos formularios.
- Completar la auditoría de agregar, modificar y retirar servicios de la OT, con pruebas permanentes de reintentos y reversión ante fallos.
- Completar los recorridos en escritorio y las cinco mediciones con **2.000 OT**.

**Estado:** 🟢 **Ajustes finales · Evaluación: 93/100**. El ciclo principal del taller está implementado y mantiene su integración con Ficha e Inventario. El cierre se concentra en reintentos desde la interfaz, auditoría de servicios y evidencia de validación.

</details>

<a id="modulo-inventario-y-stock"></a>

<details>
<summary><b>04 · Inventario y Stock</b> — 🟢 Ajustes finales · Evaluación: 95/100</summary>

**Qué hace:** administra los productos, servicios y proveedores del lubricentro. Controla las existencias, las entradas de mercadería y su valoración, compartiendo el saldo con las órdenes del taller y conservando la trazabilidad de los movimientos.

**Funciones principales:**

- **Consultar productos:** permite buscar por código o nombre, filtrar por categoría y disponibilidad, y consultar **20 registros por página**. El administrador también puede consultar productos inactivos.
- **Identificar stock crítico:** muestra productos activos cuyo stock es menor o igual al mínimo, incluye el faltante individual y prioriza los productos sin existencias.
- **Registrar productos:** propone un código y un mínimo de cero. Permite incorporar existencias iniciales con su costo de adquisición; producto, entrada inicial y movimiento se guardan juntos, con referencia automática.
- **Editar y cambiar estado:** el administrador mantiene los datos del catálogo y desactiva o reactiva con motivo. Se conserva el historial y se bloquea la desactivación cuando existe consumo neto en una OT abierta.
- **Administrar servicios:** permite registrar y editar servicios, establecer precios y señalar cuáles corresponden a cambio de aceite. Los servicios no tienen stock ni descuentan aceite automáticamente.
- **Gestionar proveedores:** permite registrar, buscar, editar y reutilizar proveedores desde Entradas o crearlos durante una compra. Admite DNI, RUC y CE, además de contactos opcionales.
- **Registrar compras recibidas:** incorpora múltiples productos con cantidades y costos por unidad o litro, proveedor y documento opcional. Guarda todas las líneas o ninguna y actualiza la valoración sin registrar pagos de caja.
- **Consultar entradas:** muestra ingresos iniciales y compras, sus detalles y estado, con filtros por origen, estado y fechas. Permite anular con motivo únicamente cuando no existen movimientos posteriores de sus productos.
- **Revisar el kardex:** presenta entradas, salidas, cantidades, costos, valores y responsables. Distingue saldo inicial del período, saldo final y saldo actual, y permite consultar el origen del movimiento.
- **Realizar conteos físicos:** compara la cantidad física total con el saldo registrado y calcula la diferencia. Sin diferencias no genera movimiento ni exige motivo; con diferencias registra el ajuste autorizado.
- **Recuperar solicitudes inciertas:** conserva el identificador y los datos originales de compras, conteos, ajustes y anulaciones para reintentar sin duplicar efectos. El caso particular del conteo sin diferencias se detalla entre los pendientes.
- **Recibir avisos de resultado:** presenta mensajes de éxito tras confirmar una operación y errores en el formulario correspondiente, conservando los datos para corregir o reintentar, con el estilo de Ficha Vehicular y Taller.

**Fortalezas de implementación:**

- **Permisos comprobados en Rust:** costos, valoración, proveedores, entradas y kardex quedan restringidos al administrador; los roles de consulta reciben respuestas públicas específicas, sin costos ni valor de stock.
- **Transacciones y auditoría:** los movimientos, las cantidades, los valores y sus registros relacionados se confirman juntos; los fallos revierten la operación.
- **Unidades diferenciadas:** productos comunes utilizan UND; granel se muestra en litros y se almacena en mililitros, con validación de precisión.
- **Valoración e historial:** calcula el promedio de adquisición, conserva los costos históricos de las salidas y deja cantidad y valor en cero al agotar existencias.
- **Reintentos protegidos:** la interfaz persiste las solicitudes por usuario y ambiente; el backend reconoce operaciones confirmadas y rechaza reutilizar un identificador con datos diferentes.
- **Consultas consistentes:** paginación y filtros en SQLite, orden definido y protección frente a respuestas antiguas en las consultas principales.
- **Validación ampliada:** las **13 pruebas nuevas de cierre de inventario pasaron** en la última revisión, además de las pruebas existentes de inventario, idempotencia e integración con Taller.
- **Rendimiento del backend comprobado:** cinco muestras con **1.000 productos y 10.000 movimientos** registraron aproximadamente **0,2 a 1,8 ms** en catálogo, críticos y kardex utilizando SQLite en memoria. Falta medir el recorrido completo en escritorio.

**Acceso por rol:**

| Rol | Alcance |
| --- | --- |
| Administrador | Gestión de productos, servicios y proveedores; compras, entradas, costos, valoración, kardex, conteos y cambios de estado o anulaciones con motivo. |
| Mecánico | Consulta autorizada de productos y servicios activos, stock y precios de venta; sin costos, valoración ni operaciones administrativas. Los consumos del taller se registran desde OT. |
| Cajero | Consulta autorizada de productos y servicios activos, stock y precios de venta; sin costos, valoración ni operaciones administrativas de Inventario. |

**Capturas:**

_Catálogo de productos, filtros y existencias_

![Catálogo de productos y stock](docs/images/04-inventario-productos.png)

_Servicios del taller_

![Catálogo de servicios del taller](docs/images/04-inventario-servicios.png)

_Entradas, compras y acceso a proveedores_

![Historial de entradas y compras](docs/images/04-inventario-entradas.png)

_Registro o edición de un producto_

![Formulario de registro o edición de producto](docs/images/04-inventario-producto.png)

_Registro de compra recibida_

![Compra a proveedor con sus productos y costos](docs/images/04-inventario-compra.png)

_Conteo físico y diferencia de existencias_

![Formulario de conteo físico de stock](docs/images/04-inventario-conteo.png)

_Kardex y origen de movimientos_

![Kardex de producto con cantidades, valores y origen](docs/images/04-inventario-kardex.png)

_Aviso de resultado de una operación_

![Aviso de éxito o error en Inventario y Stock](docs/images/04-inventario-avisos.png)

**Pendiente para cerrar:**

- Permitir un sobrante desde stock cero utilizando el costo previo, sin que el formulario obligue indirectamente a capturar otro importe.
- Dar prioridad al último costo válido cuando existe; solicitar un costo nuevo únicamente cuando no haya antecedente.
- Permitir recuperar un conteo sin diferencias cuya respuesta fue incierta, aunque después haya cambiado el saldo.
- Confirmar el descarte al abandonar formularios modificados e integrar el indicador global de captura pendiente.
- Deshabilitar y mostrar el procesamiento del botón Guardar producto durante el envío.
- Incorporar los casos adicionales detectados a las pruebas permanentes y completar los recorridos y mediciones con SQLite en disco y la aplicación de escritorio.

**Estado:** 🟢 **Ajustes finales · Evaluación: 95/100**. Las funciones principales están ampliamente alineadas con el PDF. El cierre se concentra en casos particulares del conteo, protección de formularios y evidencia de validación en escritorio. La siguiente etapa de implementación corresponde a **Punto de Venta POS**.

</details>

**Próximo módulo: Punto de Venta POS.** Se trabajará sobre la base de Login, Ficha Vehicular, Taller e Inventario, integrando ventas y cobros con los consumos ya registrados. Los cuatro módulos anteriores permanecen en ajustes finales, con seguimiento de los pendientes documentados.

<a id="roles"></a>

## 4. Roles y permisos

| Módulo                      | Administrador                                              | Mecánico                                                    | Cajero                                         |
| --------------------------- | ---------------------------------------------------------- | ----------------------------------------------------------- | ---------------------------------------------- |
| Login y Sesión              | Cuenta propia y gestión autorizada.                        | Cuenta propia.                                              | Cuenta propia.                                 |
| Ficha Vehicular e Historial | Gestión y correcciones autorizadas.                        | Consulta de información pública de atención.                | Registro, edición de datos comunes y consulta. |
| Órdenes de Trabajo Taller | Gestión del taller, reasignación, devolución y cancelación con motivo. | Inicio de OT permitidas y ejecución de propias; sin notas ajenas ni costos. | Recepción y consulta pública; sin notas internas ni costos. |
| Inventario y Stock          | Catálogos, proveedores, entradas, costos, kardex y conteos.             | Consulta de activos, stock y precios.                       | Consulta de activos, stock y precios.          |
| Punto de Venta POS          | Operaciones con caja propia y gestión documental.          | Sin acceso.                                                 | Ventas propias; excepciones con autorización.  |
| Caja Chica                  | Turno propio; consulta y cierre administrativo autorizado. | Sin acceso.                                                 | Turnos propios.                                |
| Backup y Copias             | Configuración, copias y recuperación.                      | Sin acceso.                                                 | Sin acceso.                                    |
| Administración              | Usuarios, empresa, facturación y taller.                   | Sin acceso.                                                 | Sin acceso.                                    |
| Reportes                    | Consulta y exportación.                                    | Sin acceso.                                                 | Sin acceso.                                    |
| Inicio Dashboard            | Resumen del negocio.                                       | Resumen de trabajos propios y disponibles.                  | Resumen de caja y OT por cobrar.               |

Las autorizaciones se validan en Rust. Los costos y las notas internas se restringen por rol; ocultar botones no sustituye el control del backend.

<a id="flujo"></a>

## 5. Flujo de trabajo

| Paso                     | Acción                                                              | Resultado esperado                                                     |
| ------------------------ | ------------------------------------------------------------------- | ---------------------------------------------------------------------- |
| **1. Identificar**       | Buscar la placa y verificar cliente y kilometraje.                  | Ficha seleccionada o registro de un vehículo nuevo.                    |
| **2. Recibir**           | Crear una OT con servicio inicial y mecánico opcional.                     | Orden en espera, sin reserva ni descuento de stock.                    |
| **3. Ejecutar**          | Iniciar trabajo en una zanja libre y registrar consumos reales.     | Materiales descontados una vez y trabajo asignado.                     |
| **4. Finalizar**         | Confirmar kilometraje, resumen y tipo de aceite cuando corresponda. | OT finalizada, historial actualizado y zanja libre.                    |
| **5. Cobrar**            | Recuperar la OT desde POS y confirmar el pago.                      | Venta y cobro registrados sin repetir la salida de insumos del taller. |
| **6. Cerrar y proteger** | Contar efectivo, cerrar el turno y gestionar una copia.             | Cierre conservado y datos protegidos según la política de copias.      |

**Venta directa:** el POS contempla productos y servicios sin crear una OT.

**Comprobante electrónico:** el envío ocurre después del guardado local. Las incidencias se atienden en POS sin otra venta ni cobro; devolver exige original y notas anteriores aceptados.

<a id="arquitectura"></a>

## 6. Arquitectura y base de datos

### 6.1. Organización del código

**Arquitectura en capas dentro de un monolito de escritorio organizado por módulos.** React/TypeScript presenta la interfaz y Rust aplica las reglas y consulta SQLite mediante comandos Tauri. El acceso a datos todavía comparte servicios con la lógica del negocio.

| Capa              | Ubicación                                               | Responsabilidad                                                         |
| ----------------- | ------------------------------------------------------- | ----------------------------------------------------------------------- |
| Interfaz          | `src/`                                                  | Pantallas React, componentes, contexto, tipos y servicios del frontend. |
| Comandos          | `src-tauri/src/commands/`                               | Entrada a las operaciones del backend.                                  |
| Negocio           | `src-tauri/src/services/`                               | Validaciones y coordinación de procesos.                                |
| Datos y contratos | `src-tauri/src/entities/` y `src-tauri/src/dto/`        | Representación de datos y contratos de intercambio.                     |
| Base de datos     | `src-tauri/src/db.rs` y `src-tauri/migrations/`         | Conexión y evolución del esquema SQLite.                                |
| Integración       | `src-tauri/src/nubefact.rs`                             | Comunicación con NubeFact.                                              |
| Pruebas           | `src-tauri/tests/` y carpetas `__tests__/` del frontend | Casos automatizados de las capas correspondientes.                      |

### 6.2. Guardado local e integración

```mermaid
sequenceDiagram
    participant U as Interfaz React
    participant R as Backend Rust
    participant D as SQLite local
    participant N as NubeFact
    U->>R: Confirmar venta
    R->>R: Validar sesión, caja y datos
    R->>D: Guardar venta, pagos, stock y documento
    D-->>R: Confirmar transacción local
    R-->>U: Mostrar venta registrada
    R->>N: Intentar enviar comprobante
    alt Respuesta recibida
        N-->>R: Aceptación, rechazo o error
        R->>D: Conservar estado y respuesta
    else Sin respuesta definitiva
        R->>D: Conservar incertidumbre y exigir consulta
    end
```

### 6.3. Esquema SQL v7

**27 tablas de aplicación y 6 vistas.** Historial desde OT finalizadas; estado de envío en comprobantes; cierre con conteo de efectivo y resúmenes digitales. Dinero comercial en céntimos, cantidades UND/ml y costos internos con seis decimales. Bases separadas para Prueba y Producción.

**El SQL inicial es exclusivamente para una base vacía.** Una instalación con datos necesita migración específica y adaptación del código; no reemplazar una migración ya aplicada.

<a id="instalacion"></a>

## 7. Instalación y ejecución

### 7.1. Requisitos

Windows 11 de 64 bits; Node.js/npm compatibles con el proyecto; Rust/Cargo; Microsoft C++ Build Tools con desarrollo de escritorio C++; WebView2 Runtime y Git para clonar. Ver [requisitos oficiales de Tauri](https://v2.tauri.app/start/prerequisites/).

Ejecutar desde la raíz del repositorio, donde está `package.json`.

### 7.2. Instalar dependencias

```powershell
npm install
```

### 7.3. Ejecutar en desarrollo

```powershell
npx tauri dev
```

### 7.4. Compilar

```powershell
npx tauri build
```

La configuración actual de Tauri invoca `npm run dev` antes del desarrollo y `npm run build` antes de compilar. Los scripts existen en `package.json`; el arranque y el instalador se validan en el equipo Windows objetivo.

### 7.5. Configuración inicial definida

1. Empezar en Prueba y crear el primer administrador, sin credenciales predeterminadas.
2. Configurar empresa, NubeFact, series y zanjas; registrar catálogos y existencias.
3. Preparar la protección de copias y validar atención, cobro y cierre antes de Producción.

<a id="pruebas"></a>

## 8. Pruebas y calidad

### 8.1. Qué se verifica

En Login: acceso, sesiones, permisos, bloqueo, recuperación, revocación y consistencia ante fallos. En Ficha Vehicular: registro, edición, identidad histórica, permisos, correcciones, consultas y paginación. En Taller: recepción, estados, responsable, zanjas, servicios, consumos, devoluciones, aceite, auditoría y avisos de resultado. En Inventario: DTO públicos sin costos, catálogos, proveedores, compras, conteos, valoración de granel, kardex, filtros y reintentos sin duplicación de efectos. En las siguientes etapas se validarán también ventas, cobros, cierre, comprobantes, recuperación, numeración y fechas de Lima.

### 8.2. Cómo ejecutarlas

Desde la raíz:

```powershell
npm run typecheck
npm test
npm run build
cargo test --manifest-path src-tauri/Cargo.toml
```

Las pruebas de integración externas deben usar datos y credenciales de Prueba. El flujo de [GitHub Actions](.github/workflows/ci.yml) define comprobaciones del frontend y backend; sus resultados se consultan en [Actions](https://github.com/JesusLoza1506/lubricentro-erp/actions).

### 8.3. Estado de la validación

La última evaluación reportó los siguientes resultados. Corresponden a los casos ejecutados, no a una aprobación completa de todos los recorridos:

| Comprobación | Resultado reportado |
| --- | --- |
| TypeScript | Aprobado. |
| Pruebas permanentes de frontend y servicios | **78 aprobadas**. |
| Pruebas Rust seleccionadas de los cuatro módulos y biblioteca | **99 aprobadas**. |
| Autenticación y regresión, incluidas en el total Rust | **30 aprobadas**. |
| Ficha Vehicular, incluidas en el total Rust | **16 aprobadas**. |
| Órdenes de Trabajo, incluidas en el total Rust | **13 aprobadas**. |
| Nuevas pruebas de cierre de Inventario, incluidas en el total Rust | **13 aprobadas**, además de sus pruebas existentes. |
| Rendimiento, ejecutado por separado | **1 prueba aprobada**, con cinco muestras. |
| Comprobaciones adicionales de conteo | **Tres defectos reproducidos**, conservados en los pendientes de Inventario. |

Las mediciones de catálogo, críticos y kardex con **1.000 productos y 10.000 movimientos** estuvieron aproximadamente entre **0,2 y 1,8 ms**, usando SQLite **en memoria**. No sustituyen las mediciones con base en disco, comunicación Tauri y presentación de la pantalla. Las pruebas de interfaz con servicios simulados tampoco acreditan por sí solas la integración completa en escritorio.

**Evaluación de implementación y alineación con los PDF:** Login **95/100**, Ficha Vehicular **93/100**, Taller **93/100** e Inventario y Stock **95/100**. Los cuatro están en ajustes finales. Se mantienen los recorridos y mediciones pendientes de cada módulo; Taller requiere cinco mediciones con **2.000 OT**. La siguiente implementación será **Punto de Venta POS**.

**Esquema verificado en SQLite:** 27 casos satisfactorios sobre creación, restricciones, relaciones y flujos seleccionados, correspondientes al esquema revisado. Esta evidencia se distingue de las pruebas de los módulos y no acredita la integración completa con NubeFact. Las revisiones de Login disponibles se enlazan en Documentación.

<a id="documentacion"></a>

## 9. Documentación

| Recurso                                                                  | Contenido                          |
| ------------------------------------------------------------------------ | ---------------------------------- |
| [Carpeta docs](docs/)                                                    | Documentos actualmente publicados. |
| [Revisión de Login v7 — R4](docs/Reevaluacion_Login_v7_2026-10-17_R4.md) | Revisión del módulo de acceso.     |
| [Comandos Tauri](docs/api/tauri-commands.md)                             | Referencia técnica existente.      |
| [Wireframes y diseño](docs/ui/wireframes-y-diseno.md)                    | Referencia de interfaz existente.  |
| [Esquema SQL v7](src-tauri/migrations/0001_initial_schema_v7.sql)        | Modelo inicial de datos.           |

El informe general y las diez especificaciones funcionales v7 son documentos de referencia; sus enlaces se incorporarán cuando estén publicados en el repositorio. Los cambios de reglas deben reflejarse en código, documentación y esquema cuando corresponda.

<a id="hoja-de-ruta"></a>

## 10. Hoja de ruta

Se mantiene la siguiente secuencia de desarrollo. Los cuatro primeros módulos están en ajustes finales; **el siguiente módulo será Punto de Venta POS**.

1. **Login y Sesión — ajustes finales (95/100):** completar recuperación del error inicial y evidencias de teclado, foco, salida y tiempos de ingreso.
2. **Ficha Vehicular e Historial — ajustes finales (93/100):** activar cambios pendientes al modificar datos, consolidar pruebas de interfaz y completar mediciones de escritorio.
3. **Órdenes de Trabajo Taller — ajustes finales (93/100):** conservar identificadores al reintentar, completar auditoría de servicios y validar recorridos y rendimiento.
4. **Inventario y Stock — ajustes finales (95/100):** resolver los casos particulares de conteo y costos, proteger capturas modificadas y completar pruebas y mediciones en escritorio.
5. **Punto de Venta POS — próximo módulo:** implementar ventas directas, cobro de OT sin descontar nuevamente sus insumos, pagos y gestión de comprobantes con NubeFact.
6. **Caja Chica:** completar aperturas, movimientos y cierres.
7. **Backup y Copias:** completar copias de seguridad y recuperación.
8. **Administración:** completar la configuración y gestión autorizada.
9. **Reportes:** integrar las consultas y exportaciones de los módulos anteriores.
10. **Inicio Dashboard:** integrar al final los indicadores y resúmenes por rol con los datos de los demás módulos.

Cada módulo tendrá su detalle plegable, capturas y evidencias conforme avance. Pasar a la siguiente etapa no elimina los pendientes de los módulos anteriores. Este orden de desarrollo no modifica el flujo operativo del taller descrito en la sección 5.

<a id="autor"></a>

## 11. Autores y licencia

- **Jesús Ángel Loza Yataco**
- **Esteban Ricardo Puma Palomino**
- **Reyes Esterripa Carlos Edén**
- **Magallanes Llanos Juan Jesús**

**Estudiantes de Ingeniería de Sistemas — Perú.** Proyecto académico para aplicar desarrollo, diseño de datos, integración de servicios y aseguramiento de la calidad a la gestión de un lubricentro.

**Licencia:** este README no incorpora una licencia de uso o distribución; su elección corresponde al acuerdo de los autores.
