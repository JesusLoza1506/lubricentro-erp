import { invoke } from '@tauri-apps/api/core';
import {
  Producto,
  ProductoPublicoDTO,
  CrearProductoDTO,
  EditarProductoDTO,
  DespacharProductoDTO,
  AjustarStockDTO,
  Categoria,
} from '../types/inventario';

export const inventarioService = {
  listarProductos: async (idUsuario: number = 1): Promise<Producto[]> => {
    return await invoke<Producto[]>('listar_productos_cmd', { idUsuario });
  },

  listarProductosPublicos: async (idUsuario: number = 1): Promise<ProductoPublicoDTO[]> => {
    return await invoke<ProductoPublicoDTO[]>('listar_productos_publicos_cmd', { idUsuario });
  },

  listarProductosCriticos: async (idUsuario: number = 1): Promise<Producto[]> => {
    return await invoke<Producto[]>('listar_productos_criticos_cmd', { idUsuario });
  },

  registrarProducto: async (producto: CrearProductoDTO, idUsuario: number = 1): Promise<number> => {
    console.log('🚀 [FRONTEND] Registrando producto:', producto.nombre);
    return await invoke<number>('registrar_producto_cmd', { payload: producto, idUsuario });
  },

  actualizarProducto: async (producto: EditarProductoDTO, idUsuario: number = 1): Promise<void> => {
    console.log('🚀 [FRONTEND] Actualizando producto ID:', producto.id_producto);
    return await invoke('actualizar_producto_cmd', { payload: producto, idUsuario });
  },

  eliminarProducto: async (idProducto: number, idUsuario: number = 1): Promise<void> => {
    console.log('🚀 [FRONTEND] Eliminando producto ID:', idProducto);
    return await invoke('eliminar_producto_cmd', { idProducto, idUsuario });
  },

  despacharProductoGranel: async (
    payload: DespacharProductoDTO,
    idUsuario: number = 1
  ): Promise<void> => {
    console.log('🚀 [FRONTEND] Despachando producto granel ID:', payload.id_producto);
    return await invoke('despachar_producto_granel_cmd', { payload, idUsuario });
  },

  ajustarStock: async (payload: AjustarStockDTO, idUsuario: number = 1): Promise<void> => {
    console.log('🚀 [FRONTEND] Ajustando stock producto ID:', payload.id_producto);
    return await invoke('ajustar_stock_cmd', { payload, idUsuario });
  },

  listarCategorias: async (idUsuario: number = 1): Promise<Categoria[]> => {
    return await invoke<Categoria[]>('listar_categorias_cmd', { idUsuario });
  },
};
