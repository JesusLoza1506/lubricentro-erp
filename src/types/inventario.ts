export interface Producto {
  id_producto: number;
  id_categoria: number;
  nombre_categoria?: string;
  nombre: string;
  codigo_barras: string | null;
  unidad_medida: string | null;
  stock_actual: number;
  stock_minimo: number;
  precio_compra: number;
  precio_venta: number;
}

export interface CrearProductoDTO {
  id_categoria: number;
  nombre: string;
  codigo_barras?: string | null;
  unidad_medida?: string | null;
  stock_actual: number;
  stock_minimo: number;
  precio_compra: number;
  precio_venta: number;
}

export interface EditarProductoDTO extends CrearProductoDTO {
  id_producto: number;
}

export interface Categoria {
  id_categoria?: number;
  idCategoria?: number;
  nombre_categoria?: string;
  nombreCategoria?: string;
}

// Tipo unificado para el formulario en React (soporta tanto camelCase como snake_case)
export interface ProductoForm {
  id_producto?: number;
  idProducto?: number;
  id_categoria: number;
  idCategoria?: number;
  nombre: string;
  nombre_categoria?: string;
  nombreCategoria?: string;
  codigo_barras?: string | null;
  codigoBarras?: string | null;
  unidad_medida?: string | null;
  unidadMedida?: string | null;
  stock_actual: number;
  stockActual?: number;
  stock_minimo: number;
  stockMinimo?: number;
  precio_compra: number;
  precioCompra?: number;
  precio_venta: number;
  precioVenta?: number;
}
