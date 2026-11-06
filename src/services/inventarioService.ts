import { ejecutarOperacionInventario } from './operacionInventario';
import { invoke } from '@tauri-apps/api/core';
import {
  ProductoAdminDTO,
  ProductoPublicoDTO,
  ProductosPaginadosDTO,
  ProductosPublicosPaginadosDTO,
  OrigenKardexDTO,
  CrearProductoDTO,
  EditarProductoDTO,
  AjustarStockDTO,
  ConteoFisicoDTO,
  MovimientoKardexDTO,
  Categoria,
  ProveedorDTO,
  CrearProveedorDTO,
  ActualizarProveedorDTO,
  RegistrarIngresoDTO,
  AnularIngresoDTO,
  IngresoStockResponseDTO,
  ServicioDTO,
  CrearServicioDTO,
  ActualizarServicioDTO,
} from '../types/inventario';

export interface ResultadoOperacionStock {
  status: string;
  message?: string;
  sin_diferencias?: boolean;
  id_ingreso?: number;
  stock_resultante?: number;
}

export interface ResumenSaldosKardexDTO {
  saldo_inicial: number;
  saldo_final: number;
  saldo_actual: number;
}

export const inventarioService = {
  proponerCodigo: (tipo: 'PRODUCTO' | 'SERVICIO'): Promise<string> =>
    invoke('proponer_codigo_inventario_cmd', { tipo }),
  listarProductos: async (
    incluirInactivos: boolean = false
  ): Promise<(ProductoAdminDTO | ProductoPublicoDTO)[]> => {
    return await invoke<(ProductoAdminDTO | ProductoPublicoDTO)[]>('listar_productos_cmd', {
      incluirInactivos,
    });
  },

  listarProductosAdminPaginado: async (
    busqueda?: string,
    categoria?: string,
    incluirInactivos?: boolean,
    limite: number = 20,
    offset: number = 0,
    soloCriticos: boolean = false,
    stock?: string,
    estado?: string
  ): Promise<ProductosPaginadosDTO> => {
    return await invoke<ProductosPaginadosDTO>('listar_productos_admin_paginado_cmd', {
      busqueda,
      categoria,
      incluirInactivos,
      limite,
      offset,
      soloCriticos,
      stock,
      estado,
    });
  },

  listarProductosPublicos: async (): Promise<ProductoPublicoDTO[]> => {
    return await invoke<ProductoPublicoDTO[]>('listar_productos_publicos_cmd');
  },

  listarProductosPublicosPaginado: async (
    busqueda?: string,
    categoria?: string,
    limite: number = 20,
    offset: number = 0,
    soloCriticos: boolean = false,
    stock?: string,
    estado?: string
  ): Promise<ProductosPublicosPaginadosDTO> => {
    return await invoke<ProductosPublicosPaginadosDTO>('listar_productos_publicos_paginado_cmd', {
      busqueda,
      categoria,
      limite,
      offset,
      soloCriticos,
      stock,
      estado,
    });
  },

  listarProductosCriticos: async (): Promise<(ProductoAdminDTO | ProductoPublicoDTO)[]> => {
    return await invoke<(ProductoAdminDTO | ProductoPublicoDTO)[]>('listar_productos_criticos_cmd');
  },

  registrarProducto: async (payload: CrearProductoDTO): Promise<number> => {
    return await invoke<number>('registrar_producto_cmd', { payload });
  },

  actualizarProducto: async (payload: EditarProductoDTO): Promise<void> => {
    return await invoke('actualizar_producto_cmd', { payload });
  },

  eliminarProducto: async (idProducto: number, motivo: string): Promise<void> => {
    return await invoke('eliminar_producto_cmd', { idProducto, motivo });
  },

  reactivarProducto: async (idProducto: number, motivo: string): Promise<void> => {
    return await invoke('reactivar_producto_cmd', { idProducto, motivo });
  },

  despacharProductoGranel: async (
    idProducto: number,
    cantidad: number,
    motivo?: string
  ): Promise<void> => {
    return await invoke('despachar_producto_granel_cmd', { idProducto, cantidad, motivo });
  },

  ajustarStock: async (
    payload: AjustarStockDTO,
    idOperacion?: string
  ): Promise<ResultadoOperacionStock> => {
    return ejecutarOperacionInventario<ResultadoOperacionStock>(
      'ajustar_stock_cmd',
      payload,
      idOperacion
    );
  },

  procesarConteoFisico: async (
    payload: ConteoFisicoDTO,
    idOperacion?: string
  ): Promise<ResultadoOperacionStock> => {
    return ejecutarOperacionInventario<ResultadoOperacionStock>(
      'procesar_conteo_fisico_cmd',
      payload,
      idOperacion
    );
  },

  obtenerOrigenKardex: async (idMovimiento: number): Promise<OrigenKardexDTO> => {
    return await invoke<OrigenKardexDTO>('obtener_origen_kardex_cmd', { idMovimiento });
  },

  listarKardexProducto: async (
    idProducto: number,
    limite: number = 20,
    offset: number = 0,
    fechaInicio?: string,
    fechaFin?: string,
    tipoMovimiento?: string
  ): Promise<MovimientoKardexDTO[]> => {
    return await invoke<MovimientoKardexDTO[]>('listar_kardex_producto_cmd', {
      idProducto,
      limite,
      offset,
      fechaInicio,
      fechaFin,
      tipoMovimiento,
    });
  },

  obtenerResumenSaldosKardex: async (
    idProducto: number,
    fechaInicio?: string,
    fechaFin?: string,
    tipoMovimiento?: string
  ): Promise<ResumenSaldosKardexDTO> => {
    return await invoke<ResumenSaldosKardexDTO>('obtener_resumen_saldos_kardex_cmd', {
      idProducto,
      fechaInicio,
      fechaFin,
      tipoMovimiento,
    });
  },

  registrarIngresoCompra: async (
    payload: RegistrarIngresoDTO,
    idOperacion?: string
  ): Promise<ResultadoOperacionStock> => {
    return ejecutarOperacionInventario<ResultadoOperacionStock>(
      'registrar_ingreso_compra_cmd',
      payload,
      idOperacion
    );
  },

  listarIngresosStock: async (
    limite: number = 20,
    offset: number = 0,
    estado?: string,
    origen?: string,
    fechaInicio?: string,
    fechaFin?: string
  ): Promise<IngresoStockResponseDTO[]> => {
    return await invoke<IngresoStockResponseDTO[]>('listar_ingresos_stock_cmd', {
      limite,
      offset,
      estado,
      origen,
      fechaInicio,
      fechaFin,
    });
  },

  anularIngresoCompra: async (
    payload: AnularIngresoDTO,
    idOperacion?: string
  ): Promise<ResultadoOperacionStock> => {
    return ejecutarOperacionInventario<ResultadoOperacionStock>(
      'anular_ingreso_compra_cmd',
      payload,
      idOperacion
    );
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

  actualizarProveedor: async (payload: ActualizarProveedorDTO): Promise<void> => {
    return await invoke('actualizar_proveedor_cmd', { payload });
  },

  listarServiciosAdmin: async (): Promise<ServicioDTO[]> => {
    return await invoke<ServicioDTO[]>('listar_servicios_admin_cmd');
  },

  listarServiciosPublicos: async (): Promise<ServicioDTO[]> => {
    return await invoke<ServicioDTO[]>('listar_servicios_publicos_cmd');
  },

  crearServicio: async (payload: CrearServicioDTO): Promise<number> => {
    return await invoke<number>('crear_servicio_cmd', { payload });
  },

  actualizarServicio: async (payload: ActualizarServicioDTO): Promise<void> => {
    return await invoke('actualizar_servicio_cmd', { payload });
  },

  cambiarEstadoServicio: async (
    idServicio: number,
    activo: boolean,
    motivo: string
  ): Promise<void> => {
    return await invoke('cambiar_estado_servicio_cmd', { idServicio, activo, motivo });
  },
};
