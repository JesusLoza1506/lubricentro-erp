export type UnidadMedida = 'ML' | 'UNIDAD' | 'LITRO' | 'GALON' | 'BARRIL';

// DTO Público (CAJERO / MECANICO) - Evita fugas de datos de costo
export interface ProductoPublicoDTO {
  id_producto: number;
  id_categoria: number;
  nombre_categoria?: string;
  categoria?: string;
  nombre: string;
  codigo_barras: string | null;
  codigo?: string;
  unidad_medida: string | null;
  stock_actual: number;
  stock_minimo: number;
  precio_venta: number; // En céntimos desde el backend
  precio_venta_soles?: number; // Calculado en frontend
  es_critico: boolean;
}

// DTO Interno (ADMINISTRADOR) - Incluye precio_compra y margen
export interface ProductoAdminDTO extends ProductoPublicoDTO {
  precio_compra: number; // En céntimos
  margen: number; // En céntimos
}

// Tipo flexible para vistas/tablas de React
export type Producto = ProductoPublicoDTO | ProductoAdminDTO;

export interface CrearProductoDTO {
  id_categoria: number;
  nombre: string;
  codigo_barras?: string | null;
  unidad_medida?: string | null;
  stock_actual: number;
  stock_minimo: number;
  precio_compra: number; // En céntimos
  precio_venta: number; // En céntimos
}

export interface EditarProductoDTO extends CrearProductoDTO {
  id_producto: number;
}

export interface DespacharProductoDTO {
  id_producto: number;
  cantidad: number;
  motivo?: string;
}

export interface AjustarStockDTO {
  id_producto: number;
  tipo_movimiento: 'ENTRADA' | 'AJUSTE';
  cantidad_nueva: number;
  motivo: string; // Obligatorio
}

export interface Categoria {
  id_categoria?: number;
  idCategoria?: number;
  nombre_categoria?: string;
  nombreCategoria?: string;
}

// Formulario reactivo para crear/editar productos
export interface ProductoForm {
  id_producto?: number;
  id_categoria: number;
  nombre: string;
  nombre_categoria?: string;
  codigo_barras?: string | null;
  unidad_medida?: string | null;
  stock_actual: number;
  stock_minimo: number;
  precio_compra: number; // En Soles en la UI
  precio_venta: number; // En Soles en la UI
}
