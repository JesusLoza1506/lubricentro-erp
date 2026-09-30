# Contrato de Comandos Tauri (v0.1.0) - ERP Lubricentro

Este documento define la interfaz oficial entre el Frontend
(React/TypeScript) y el Backend (Rust/Tauri). Todos los datos expuestos
utilizan exclusivamente DTOs limpios (Data Transfer Objects).

---

## 1. Módulo: Autenticación y Usuarios

### `obtener_usuario_sesion_cmd`

- **Descripción**: Retorna la sesión del usuario activo. Excluye
  explícitamente `password_hash`.
- **Entrada**: N/A
- **Salida (`UsuarioSesionDTO`)**:

```json
{
  "id_usuario": 1,
  "nombre_completo": "Administrador General",
  "usuario": "admin",
  "rol": "ADMINISTRADOR",
  "activo": true
}
```

---

## 2. Módulo: Clientes y Vehículos

### `buscar_cliente_por_doc_cmd`

- **Descripción**: Busca un cliente registrado por su número de
  DNI/RUC.
- **Entrada**: `doc: String`
- **Salida (`Option<ClienteDTO>`)**:

```json
{
  "id_cliente": 1,
  "tipo_documento": "RUC",
  "numero_documento": "20600695771",
  "nombre_razon_social": "NUBEFACT SA",
  "telefono": "987654321",
  "direccion": "AV. LIBERTAD 123"
}
```

---

## 3. Módulo: Inventario y Productos

### `listar_productos_publicos_cmd`

- **Descripción**: Lista el catálogo de productos disponible para
  venta. Excluye el campo sensible `precio_compra`.
- **Entrada**: N/A
- **Salida (`Vec<ProductoPublicoDTO>`)**:

```json
[
  {
    "id_producto": 1,
    "id_categoria": 1,
    "codigo_barras": "7750001001",
    "unidad_medida": "Litro",
    "stock_actual": 45.5,
    "stock_minimo": 10.0,
    "precio_venta": 28.0
  }
]
```

---

## 4. Módulo: Órdenes de Trabajo (OT)

### `obtener_ot_por_codigo_cmd`

- **Descripción**: Retorna el estado e información de una orden de
  trabajo.
- **Entrada**: `codigo: String`
- **Salida (`OrdenTrabajoDTO`)**:

```json
{
  "id_ot": 1,
  "codigo_ot": "OT-2026-001",
  "placa": "ABC-123",
  "id_mecanico": 2,
  "zanja": 1,
  "estado": "EN_PROCESO",
  "kilometraje_ingreso": 45000,
  "proximo_kilometraje": 50000,
  "fecha_ingreso": "2026-09-29 10:00:00"
}
```

---

## 5. Módulo: Comprobantes y Facturación

### `emitir_comprobante_cmd`

- **Descripción**: Procesa la emisión de un comprobante local y
  calcula subtotales/IGV de forma automática.
- **Entrada (`EmitirComprobanteDTO`)**:

```json
{
  "id_cliente": 1,
  "id_cajero": 1,
  "tipo_comprobante": "01",
  "medio_pago": "Efectivo",
  "monto_total": 118.0
}
```

- **Salida (`ComprobanteDTO`)**:

```json
{
  "id_comprobante": 100,
  "tipo_comprobante": "01",
  "serie": "FFF1",
  "correlativo": 69,
  "monto_subtotal": 100.0,
  "monto_igv": 18.0,
  "monto_total": 118.0,
  "medio_pago": "Efectivo",
  "estado_sunat": "PENDIENTE",
  "fecha_emision": "2026-09-29 22:30:00"
}
```

---

## 6. Módulo: Caja Chica

### `cerrar_caja_cmd`

- **Descripción**: Realiza el cálculo del arqueo de caja y calcula
  diferencias cuadradas contra apertura y ventas.
- **Entrada**: `id_caja: i32`, `monto_apertura: f64`,
  `ventas_efectivo: f64`, `efectivo_conteo: f64`,
  `digital_conteo: f64`
- **Salida (`CierreCajaDTO`)**:

```json
{
  "id_caja": 1,
  "monto_cierre_efectivo": 300.0,
  "monto_cierre_digital": 50.0,
  "monto_diferencia": 0.0,
  "estado": "CERRADA"
}
```
