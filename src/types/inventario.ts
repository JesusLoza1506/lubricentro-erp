export type CategoriaEnum = 'ACEITE' | 'FILTRO' | 'ADITIVO' | 'GRANEL' | 'OTRO';
export type UnidadMedida = 'UND' | 'ML';

// DTO Público (CAJERO / MECANICO) - Oculta costo y valoración de stock
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
  activo: number;
  es_critico?: boolean;
}

// DTO Interno (ADMINISTRADOR) - Incluye valoración y costos
export interface ProductoAdminDTO extends ProductoPublicoDTO {
  valor_stock_micro: number; // En microsoles
  ultimo_costo_micro?: number | null; // En microsoles
}

export type Producto = ProductoPublicoDTO | ProductoAdminDTO;

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

export interface DespacharProductoDTO {
  id_producto: number;
  cantidad: number;
  motivo?: string;
}

export interface AjustarStockDTO {
  id_producto: number;
  tipo_ajuste: 'ENTRADA' | 'SALIDA';
  cantidad: number;
  costo_unitario_micro?: number | null;
  motivo: string;
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

export interface DetalleIngresoRequestDTO {
  id_producto: number;
  cantidad: number;
  costo_efectivo_micro: number; // En microsoles
}

export interface RegistrarIngresoDTO {
  origen: 'COMPRA' | 'INICIAL';
  id_proveedor?: number | null;
  tipo_documento?: string | null;
  numero_documento?: string | null;
  fecha_documento?: string | null;
  referencia_inicial?: string | null;
  observacion?: string | null;
  detalles: DetalleIngresoRequestDTO[];
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
  // Campos opcionales para Alta con Stock Inicial
  tiene_stock_inicial: boolean;
  cantidad_inicial: number;
  costo_unitario_soles: number; // En Soles UI
  referencia_inicial: string;
}
