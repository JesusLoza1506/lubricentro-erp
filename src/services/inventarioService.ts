import { invoke } from '@tauri-apps/api/core';
import {
  Producto,
  ProductoPublicoDTO,
  CrearProductoDTO,
  EditarProductoDTO,
  AjustarStockDTO,
  Categoria,
  ProveedorDTO,
  CrearProveedorDTO,
  RegistrarIngresoDTO,
} from '../types/inventario';

export const inventarioService = {
  listarProductos: async (): Promise<Producto[]> => {
    return await invoke<Producto[]>('listar_productos_cmd');
  },

  listarProductosPublicos: async (): Promise<ProductoPublicoDTO[]> => {
    return await invoke<ProductoPublicoDTO[]>('listar_productos_publicos_cmd');
  },

  listarProductosCriticos: async (): Promise<Producto[]> => {
    return await invoke<Producto[]>('listar_productos_criticos_cmd');
  },

  registrarProducto: async (payload: CrearProductoDTO): Promise<number> => {
    return await invoke<number>('registrar_producto_cmd', { payload });
  },

  actualizarProducto: async (payload: EditarProductoDTO): Promise<void> => {
    return await invoke('actualizar_producto_cmd', { payload });
  },

  eliminarProducto: async (idProducto: number): Promise<void> => {
    return await invoke('eliminar_producto_cmd', { idProducto });
  },

  despacharProductoGranel: async (
    idProducto: number,
    cantidad: number,
    motivo?: string
  ): Promise<void> => {
    return await invoke('despachar_producto_granel_cmd', { idProducto, cantidad, motivo });
  },

  ajustarStock: async (payload: AjustarStockDTO): Promise<void> => {
    return await invoke('ajustar_stock_cmd', { payload });
  },

  registrarIngresoCompra: async (payload: RegistrarIngresoDTO): Promise<number> => {
    return await invoke<number>('registrar_ingreso_compra_cmd', { payload });
  },

  listarCategorias: async (): Promise<Categoria[]> => {
    return await invoke<Categoria[]>('listar_categorias_cmd');
  },

  listarProveedores: async (): Promise<ProveedorDTO[]> => {
    return await invoke<ProveedorDTO[]>('listar_proveedores_cmd');
  },

  crearProveedor: async (payload: CrearProveedorDTO): Promise<number> => {
    return await invoke<number>('crear_proveedor_cmd', { payload });
  },
};
