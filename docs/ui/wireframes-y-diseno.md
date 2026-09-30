# UI/UX & Especificación de Pantallas - ERP Lubricentro Gloria S.R.L. (v0.1.0)

Este documento define la arquitectura de información, guía de estilo, mapa de navegación y especificación detallada de cada pantalla del sistema antes de su construcción en React.

---

# 1. Guía de Estilo Base

## 1.1. Paleta de Colores

| Elemento                        | Color     | Uso                                           |
| ------------------------------- | --------- | --------------------------------------------- |
| **Primario / Acción principal** | `#2563EB` | Botones principales, enlaces y bordes activos |
| **Fondo principal**             | `#F8FAFC` | Fondo neutro de la aplicación                 |
| **Superficies / Cards**         | `#FFFFFF` | Tarjetas, tablas y modales                    |
| **Texto principal**             | `#0F172A` | Títulos y textos principales                  |
| **Texto secundario**            | `#64748B` | Subtítulos, labels y textos de ayuda          |

## 1.2. Colores de Estado

| Estado                      | Color     | Uso                                                            |
| --------------------------- | --------- | -------------------------------------------------------------- |
| **Éxito / ACEPTADO**        | `#16A34A` | Comprobantes aceptados por SUNAT, caja cuadrada y stock óptimo |
| **Advertencia / OBSERVADO** | `#D97706` | Comprobantes observados y stock bajo mínimo                    |
| **Peligro / RECHAZADO**     | `#DC2626` | Comprobantes rechazados, errores y eliminación                 |
| **Pendiente / EN_COLA**     | `#2563EB` | Comprobantes pendientes de envío a SUNAT                       |

## 1.3. Tipografía

- **Familia:** `Inter` / `System UI`
- **Título de pantalla:** `24px` / SemiBold (`font-semibold`)
- **Subtítulos / Headers de Card:** `18px` / Medium (`font-medium`)
- **Texto general / Tablas:** `14px` / Regular (`font-normal`)
- **Badges / Textos secundarios:** `12px` / Medium (`font-medium`)

---

# 2. Mapa de Navegación por Rol

```text
[ LOGIN ]
   │
   ├─► (ADMINISTRADOR) ──► Sidebar Completo:
   │                         ├─ Dashboard General
   │                         ├─ Ficha Vehicular & Historial
   │                         ├─ Taller (Órdenes de Trabajo)
   │                         ├─ Inventario & Stock
   │                         ├─ Punto de Venta (POS)
   │                         ├─ Caja Chica
   │                         ├─ Panel de Fidelización
   │                         ├─ Monitoreo Fiscal (Cola SUNAT)
   │                         └─ Backup & Restauración
   │
   ├─► (CAJERO) ─────────► Sidebar Restringido:
   │                         ├─ Dashboard Ventas
   │                         ├─ Ficha Vehicular
   │                         ├─ Punto de Venta (POS)
   │                         └─ Caja Chica
   │
   └─► (MECANICO) ───────► Vista Simplificada Taller:
                             └─ Taller (Tablero Kanban de Órdenes de Trabajo)
```

---

# 3. Especificación de Pantallas y Validación Cruzada con DTOs

## 3.1. Pantalla 1: Login

### Propósito

Autenticación inicial de usuarios del sistema.

### DTO de Entrada

`Credenciales`:

- `usuario: String`
- `password: String`

### DTO de Salida / Respaldo

`UsuarioSesionDTO`:

- `id_usuario`
- `nombre_completo`
- `usuario`
- `rol`
- `activo`

### Comando Tauri

`obtener_usuario_sesion_cmd`

### Componentes Visuales

- Formulario central con logo del Lubricentro.
- Input `Usuario`.
- Input `Contraseña`.
- Botón primario `Iniciar Sesión`.
- Mensaje de error:
  - `Credenciales inválidas`
  - `Usuario inactivo`

---

## 3.2. Pantalla 2: Dashboard / Inicio

### Propósito

Panel general con métricas rápidas según el rol del usuario autenticado.

### DTO de Salida / Respaldo

`UsuarioSesionDTO` + resúmenes de caja y OT.

### Componentes Visuales

#### Header

Saludo al usuario mostrando:

- `nombre_completo`
- `rol`

#### Tarjetas de Resumen (KPIs)

- **Total Ventas del Día:** `monto_total` acumulado.
- **Órdenes en Proceso:** conteo de OTs en estado `EN_PROCESO`.
- **Alertas de Stock Bajo:** conteo de productos donde `stock_actual <= stock_minimo`.

#### Accesos Rápidos

- `Nueva Venta (POS)`
- `Nueva Orden de Trabajo`

---

## 3.3. Pantalla 3: Ficha Vehicular e Historial

### Propósito

Consulta y registro de clientes con sus respectivos vehículos e historial de atenciones.

### DTO de Entrada

`doc: String`

Puede corresponder a DNI o RUC.

### DTO de Salida / Respaldo

`Option<ClienteDTO>`:

- `id_cliente`
- `tipo_documento`
- `numero_documento`
- `nombre_razon_social`
- `telefono`
- `direccion`

### Comando Tauri

`buscar_cliente_por_doc_cmd`

### Componentes Visuales

- **Buscador Superior:** Input para DNI/RUC + botón `Buscar`.
- **Tarjeta de Datos del Cliente:**
  - `tipo_documento`
  - `numero_documento`
  - `nombre_razon_social`
  - `telefono`
  - `direccion`

- **Tabla de Vehículos Asociados:**
  - Placa
  - Marca
  - Modelo
  - Kilometraje

- **Tabla de Historial:** atenciones pasadas realizadas en el Lubricentro.

---

## 3.4. Pantalla 4: Taller / Órdenes de Trabajo (OT)

### Propósito

Control de atenciones activas en zanjas/elevadores del taller.

### DTO de Entrada

`codigo: String`

Ejemplo:

`OT-2026-001`

### DTO de Salida / Respaldo

`OrdenTrabajoDTO`:

- `id_ot`
- `codigo_ot`
- `placa`
- `id_mecanico`
- `zanja`
- `estado`
- `kilometraje_ingreso`
- `proximo_kilometraje`
- `fecha_ingreso`

### Comando Tauri

`obtener_ot_por_codigo_cmd`

### Componentes Visuales

- **Tablero Kanban / Vista Zanjas**
  - `REGISTRADA`
  - `EN_PROCESO`
  - `FINALIZADA`

- **Card de OT**
  - `codigo_ot`
  - `placa`
  - `zanja`
  - `kilometraje_ingreso`
  - `estado`

- **Acción:** botón para cambiar estado o visualizar detalle.

---

## 3.5. Pantalla 5: Inventario y Alertas de Stock

### Propósito

Catálogo público de productos y monitoreo de niveles de insumos, como aceites, filtros y aditivos.

### DTO de Salida / Respaldo

`Vec<ProductoPublicoDTO>`:

- `id_producto`
- `id_categoria`
- `codigo_barras`
- `unidad_medida`
- `stock_actual`
- `stock_minimo`
- `precio_venta`

### Comando Tauri

`listar_productos_publicos_cmd`

### Seguridad de Capas

Excluye explícitamente `precio_compra` para no exponer costos a cajeros o mecánicos.

### Componentes Visuales

- **Tabla de Productos**
  - `codigo_barras`
  - `unidad_medida`
  - `stock_actual`
  - `stock_minimo`
  - `precio_venta`

- **Indicador de Alerta:** filas resaltadas si `stock_actual <= stock_minimo`.
- **Filtros:** búsqueda por código de barras o nombre.

---

# 4. Especificación de Pantallas y Validación Cruzada con DTOs

## 4.1. Pantalla 6: Punto de Venta (POS)

### Propósito

Emisión rápida de comprobantes de pago, incluyendo boletas y facturas, en mostrador.

### DTO de Entrada

`EmitirComprobanteDTO`:

- `id_cliente`
- `id_cajero`
- `tipo_comprobante`
- `medio_pago`
- `monto_total`

### DTO de Salida / Respaldo

`ComprobanteDTO`:

- `id_comprobante`
- `tipo_comprobante`
- `serie`
- `correlativo`
- `monto_subtotal`
- `monto_igv`
- `monto_total`
- `medio_pago`
- `estado_sunat`
- `fecha_emision`

### Comando Tauri

`emitir_comprobante_cmd`

### Componentes Visuales

- **Panel Izquierdo:** selección/búsqueda de cliente por DNI/RUC.
- **Panel Central:** grilla/lista de selección rápida de productos e insumos.
- **Panel Derecho:** carrito de compra con cálculo de IGV y subtotal.
- **Selector de Pago:**
  - `Efectivo`
  - `Yape/Plin`
  - `Tarjeta`

- **Acción Principal:** botón `Emitir Comprobante`.

---

## 4.2. Pantalla 7: Caja Chica y Arqueo

### Propósito

Apertura, registro de movimientos y cierre/arqueo diario de caja.

### DTO de Entrada

- `id_caja: i32`
- `monto_apertura: f64`
- `ventas_efectivo: f64`
- `efectivo_conteo: f64`
- `digital_conteo: f64`

### DTO de Salida / Respaldo

`CierreCajaDTO`:

- `id_caja`
- `monto_cierre_efectivo`
- `monto_cierre_digital`
- `monto_diferencia`
- `estado`

### Comando Tauri

`cerrar_caja_cmd`

### Componentes Visuales

- Formulario de ingreso de conteo físico de dinero.
- Resumen de balance proyectado vs. real.
- Badge de `Diferencia`:
  - Verde si es `0.0`.
  - Rojo si existe sobrante o faltante.

- Botón `Cerrar Caja`.

---

## 4.3. Pantalla 8: Panel de Fidelización

### Propósito

Seguimiento del programa de puntos y proyección de servicios según el kilometraje de los vehículos.

### DTO de Respaldo

Métricas de proyección vehicular:

- `kilometraje_actual`
- `proximo_kilometraje`

### Componentes Visuales

- Tabla de clientes frecuentes.
- Indicador de alertas de mantenimiento preventivo.
- Indicador de próximo cambio de aceite.
- Botón para enviar recordatorio/notificación.

---

## 4.4. Pantalla 9: Monitoreo de Cola Fiscal (SUNAT)

### Propósito

Supervisión en tiempo real del estado de envío de comprobantes hacia NubeFact/SUNAT mediante el worker asíncrono.

### DTO de Respaldo

Lista de cola de envíos:

- `id_cola`
- `serie`
- `correlativo`
- `estado_envio`
- `intentos`

### Componentes Visuales

- Tabla de comprobantes en cola.
- Badges de estado:
  - `PENDIENTE`
  - `ENVIADO`
  - `ERROR`

- Contador de reintentos con backoff exponencial.

---

## 4.5. Pantalla 10: Backup y Restauración

### Propósito

Gestión exclusiva de copias de seguridad de la base de datos SQLite para la administración.

### Componentes Visuales

- Botón primario `Generar Copia de Seguridad Local`.
- Historial de backups realizados:
  - Fecha
  - Hora
  - Tamaño del archivo `.db`

- Botón secundario `Restaurar desde Archivo`.

---

# 5. Validación Preliminar del RNF-04

## Usabilidad del POS ≤ 4 Clics

Para garantizar la meta de usabilidad del Requisito No Funcional RNF-04, el flujo de emisión en el Punto de Venta (POS) se optimiza en exactamente cuatro interacciones:

```text
[ Clic 1: Seleccionar Cliente ]
                │
                ▼
[ Clic 2: Agregar Producto / Servicio ]
                │
                ▼
[ Clic 3: Seleccionar Medio de Pago ]
                │
                ▼
[ Clic 4: Emitir Comprobante ]
```

### Flujo Detallado

1. **Clic 1:** Buscar/seleccionar cliente o utilizar `Cliente Varios` por defecto.
2. **Clic 2:** Seleccionar el producto/servicio desde la lista rápida.
3. **Clic 3:** Seleccionar el método de pago, por ejemplo `Efectivo`.
4. **Clic 4:** Presionar el botón `Emitir Comprobante`.

### Resultado Esperado

El flujo completo de una venta estándar puede ejecutarse en un máximo de **4 interacciones principales**, cumpliendo la meta establecida para el **RNF-04**.
