import { invoke } from '@tauri-apps/api/core';
import { Producto, CrearProductoDTO, EditarProductoDTO, Categoria } from '../types/inventario';

export const inventarioService = {
  listarProductos: async (): Promise<Producto[]> => {
    return await invoke<Producto[]>('listar_productos_cmd');
  },

  listarProductosPublicos: async (): Promise<Producto[]> => {
    return await invoke<Producto[]>('listar_productos_publicos_cmd');
  },

  listarProductosCriticos: async (): Promise<Producto[]> => {
    return await invoke<Producto[]>('listar_productos_criticos_cmd');
  },

  registrarProducto: async (producto: CrearProductoDTO): Promise<void> => {
    console.log('🚀 [FRONTEND] Registrando producto:', producto.nombre);
    return await invoke('registrar_producto_cmd', { payload: producto });
  },

  actualizarProducto: async (producto: EditarProductoDTO): Promise<void> => {
    console.log('🚀 [FRONTEND] Actualizando producto ID:', producto.id_producto);
    return await invoke('actualizar_producto_cmd', { payload: producto });
  },

  eliminarProducto: async (idProducto: number, idUsuario?: number | null): Promise<void> => {
    console.log(
      '🚀 [FRONTEND] Solicitando eliminar producto ID:',
      idProducto,
      'por usuario:',
      idUsuario ?? 1
    );
    try {
      await invoke('eliminar_producto_cmd', {
        idProducto,
        idUsuario: idUsuario ?? 1,
      });
      console.log('✅ [FRONTEND] Producto eliminado correctamente en Backend');
    } catch (error) {
      console.error('❌ [FRONTEND] Error en eliminarProducto:', error);
      throw error;
    }
  },

  listarCategorias: async (): Promise<Categoria[]> => {
    return await invoke<Categoria[]>('listar_categorias_cmd');
  },
};
