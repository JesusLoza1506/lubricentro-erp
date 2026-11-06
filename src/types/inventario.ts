export type CategoriaEnum = 'ACEITE' | 'FILTRO' | 'ADITIVO' | 'GRANEL' | 'OTRO';
export type UnidadMedida = 'UND' | 'ML';

// DTO Público (CAJERO / MECANICO)
export interface ProductoPublicoDTO {
  id_producto: number;
  codigo: string;
  nombre: string;
  descripcion?: string | null;
  categoria: CategoriaEnum;
  unidad_medida: UnidadMedida;
  stock_actual: number;
  stock_minimo: number;
  precio_venta_cent: number; // En céntimos
  activo: boolean;
  es_critico?: boolean;
}

// DTO Interno (ADMINISTRADOR)
export interface ProductoAdminDTO extends ProductoPublicoDTO {
  valor_stock_micro: number; // En microsoles
  ultimo_costo_micro?: number | null; // En microsoles
}

export type Producto = ProductoPublicoDTO | ProductoAdminDTO;

export interface ProductosPaginadosDTO {
  productos: ProductoAdminDTO[];
  total_registros: number;
}

export interface ProductosPublicosPaginadosDTO {
  productos: ProductoPublicoDTO[];
  total_registros: number;
}

export interface OrigenKardexDTO {
  tipo: string;
  codigo: string;
  fecha: string;
  estado: string;
  documento: string | null;
  resumen: string | null;
  lineas: { nombre: string; unidad_medida: string; cantidad: number; valor_micro: number | null }[];
  comprobantes: { id_comprobante: number; numero: string; tipo_comprobante: string; estado_envio: string; total_cent: number }[];
}

export interface CrearProductoDTO {
  codigo: string;
  nombre: string;
  descripcion?: string | null;
  categoria: CategoriaEnum;
  unidad_medida: UnidadMedida;
  stock_minimo: number;
  precio_venta_cent: number; // En céntimos
  cantidad_inicial?: number | null;
  costo_unitario_inicial_micro?: number | null; // En microsoles
  referencia_inicial?: string | null;
}

export interface EditarProductoDTO {
  id_producto: number;
  nombre: string;
  descripcion?: string | null;
  stock_minimo: number;
  precio_venta_cent: number; // En céntimos
}

export interface AjustarStockDTO {
  id_producto: number;
  tipo_ajuste: 'ENTRADA' | 'SALIDA';
  cantidad: number;
  costo_unitario_micro?: number | null;
  motivo: string;
}

export interface ConteoFisicoDTO {
  id_producto: number;
  stock_fisico_contado: number; // En UND o en Litros
  stock_observado?: number | null; // Concurrencia optimista
  costo_unitario_micro?: number | null;
  motivo: string;
}

export interface MovimientoKardexDTO {
  id_movimiento: number;
  id_producto: number;
  tipo_movimiento: string;
  unidad_medida: string;
  stock_anterior: number;
  diferencia: number;
  stock_resultante: number;
  valor_anterior_micro: number;
  valor_delta_micro: number;
  valor_resultante_micro: number;
  costo_unitario_micro: number;
  motivo?: string | null;
  fecha_movimiento: string;
  usuario_nombre?: string | null;
  id_ingreso: number | null;
  id_ot: number | null;
  id_venta: number | null;
  id_devolucion: number | null;
  codigo_origen: string | null;
}

export interface ServicioDTO {
  id_servicio: number;
  codigo: string;
  nombre: string;
  descripcion?: string | null;
  precio_venta_cent: number;
  es_cambio_aceite: boolean;
  activo: boolean;
}

export interface CrearServicioDTO {
  codigo: string;
  nombre: string;
  descripcion?: string | null;
  precio_venta_cent: number;
  es_cambio_aceite: boolean;
}

export interface ActualizarServicioDTO {
  id_servicio: number;
  nombre: string;
  descripcion?: string | null;
  precio_venta_cent: number;
  es_cambio_aceite: boolean;
}

export interface Categoria {
  id_categoria: number;
  nombre: CategoriaEnum;
  activo: number;
}

export interface ProveedorDTO {
  id_proveedor: number;
  tipo_documento: string;
  numero_documento: string;
  nombre_razon_social: string;
  telefono?: string | null;
  correo?: string | null;
  direccion?: string | null;
  contacto_asesor?: string | null;
}

export interface CrearProveedorDTO {
  tipo_documento: string;
  numero_documento: string;
  nombre_razon_social: string;
  telefono?: string | null;
  correo?: string | null;
  direccion?: string | null;
  contacto_asesor?: string | null;
}

export interface ActualizarProveedorDTO extends CrearProveedorDTO {
  id_proveedor: number;
}

export interface DetalleIngresoRequestDTO {
  id_producto: number;
  cantidad: number; // En UND o ML
  costo_efectivo_micro: number; // En microsoles
}

export interface RegistrarIngresoDTO {
  origen: 'COMPRA' | 'INICIAL';
  id_proveedor?: number | null;
  tipo_documento?: string | null; // FACTURA, BOLETA, OTRO, SIN_DOCUMENTO
  numero_documento?: string | null;
  fecha_documento?: string | null;
  referencia_inicial?: string | null;
  observacion?: string | null;
  detalles: DetalleIngresoRequestDTO[];
}

export interface AnularIngresoDTO {
  id_ingreso: number;
  motivo: string;
}

export interface DetalleIngresoResponseDTO {
  id_detalle: number;
  id_ingreso: number;
  id_producto: number;
  codigo_snapshot: string;
  nombre_snapshot: string;
  unidad_medida: string;
  cantidad: number;
  costo_efectivo_micro: number;
  valor_aporte_micro: number;
}

export interface IngresoStockResponseDTO {
  id_ingreso: number;
  id_operacion: string;
  codigo: string;
  origen: string;
  id_proveedor?: number | null;
  proveedor_nombre?: string | null;
  tipo_documento?: string | null;
  numero_documento?: string | null;
  fecha_documento?: string | null;
  estado: string;
  total_costo_micro: number;
  id_usuario: number;
  usuario_nombre?: string | null;
  observacion?: string | null;
  fecha_registro: string;
  fecha_anulacion?: string | null;
  motivo_anulacion?: string | null;
  detalles: DetalleIngresoResponseDTO[];
}

export interface ProductoForm {
  id_producto?: number;
  codigo: string;
  nombre: string;
  descripcion?: string;
  categoria: CategoriaEnum;
  unidad_medida: UnidadMedida;
  stock_minimo: number;
  precio_venta_soles: number; // En Soles UI
  tiene_stock_inicial: boolean;
  cantidad_inicial: number;
  costo_unitario_soles: number; // En Soles UI
  referencia_inicial: string;
}
