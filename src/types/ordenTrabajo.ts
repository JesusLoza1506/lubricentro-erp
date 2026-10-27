export type EstadoOT = 'EN_ESPERA' | 'EN_PROCESO' | 'FINALIZADO' | 'CANCELADO';
export type TipoAceiteOT = 'MINERAL' | 'SINTETICO';

export interface ServicioInicialDTO {
  id_servicio: number;
  cantidad: number;
}

export interface ServicioOtDTO {
  id_detalle: number;
  id_servicio: number;
  codigo_snapshot: string;
  nombre_snapshot: string;
  cantidad: number;
  precio_unitario_cent: number;
  precio_unitario_soles: number;
  subtotal_cent: number;
  subtotal_soles: number;
  es_cambio_aceite: number;
}

export interface InsumoOtDTO {
  id_detalle: number;
  id_producto: number;
  codigo_snapshot: string;
  nombre_snapshot: string;
  unidad_medida: string;
  cantidad: number;
  cantidad_formateada: number;
  precio_unitario_cent: number;
  precio_unitario_soles: number;
  subtotal_cent: number;
  subtotal_soles: number;
  devuelto_ml_und: number;
  neto_ml_und: number;
  costo_unitario_micro?: number;
}

export interface OrdenTrabajoHistorialDTO {
  id_ot: number;
  codigo_ot: string;
  id_vehiculo: number;
  placa: string;
  id_cliente_atendido: number;
  cliente_nombre: string;
  cliente_documento?: string;
  id_creador: number;
  nombre_creador: string;
  id_mecanico?: number;
  nombre_mecanico?: string;
  id_mecanico_terminal?: number;
  id_zanja_ejecucion?: number;
  estado: EstadoOT;
  kilometraje_ingreso: number;
  kilometraje_salida?: number;
  pedido_cliente?: string;
  resumen_publico?: string;
  nota_interna?: string;
  tipo_aceite?: string;
  intervalo_aceite_km?: number;
  km_proximo?: number;
  corregido: number;
  fecha_ingreso: string;
  fecha_inicio?: string;
  fecha_fin?: string;
  fecha_cancelacion?: string;
  motivo_cancelacion?: string;
  servicios: ServicioOtDTO[];
  insumos: InsumoOtDTO[];
  total_servicios_cent: number;
  total_insumos_cent: number;
  total_general_cent: number;
  total_general_soles: number;
}

export interface ZanjaEstadoDTO {
  id_zanja: number;
  activa: number;
  ocupada: boolean;
  ot_actual?: OrdenTrabajoHistorialDTO;
}

export interface TableroZanjasDTO {
  zanjas: ZanjaEstadoDTO[];
  cola_espera: OrdenTrabajoHistorialDTO[];
}

export interface FiltroPaginacionOtPayload {
  pagina?: number;
  limite?: number;
  placa?: string;
  codigo_ot?: string;
  fecha_inicio?: string;
  fecha_fin?: string;
}

export interface ResultadoPaginadoOtDTO {
  items: OrdenTrabajoHistorialDTO[];
  total_items: number;
  pagina_actual: number;
  total_paginas: number;
}

export interface CrearOrdenTrabajoPayload {
  id_vehiculo: number;
  id_cliente_atendido: number;
  kilometraje_ingreso: number;
  id_mecanico?: number;
  pedido_cliente?: string;
  servicios_iniciales: ServicioInicialDTO[];
}

export interface IniciarTrabajoOtPayload {
  id_ot: number;
  id_zanja_ejecucion: number;
  id_mecanico?: number;
}

export interface ReasignarMecanicoOtPayload {
  id_ot: number;
  nuevo_id_mecanico: number;
  motivo: string;
}

export interface FinalizarOtPayload {
  id_ot: number;
  kilometraje_salida: number;
  resumen_publico: string;
  tipo_aceite?: TipoAceiteOT;
}

export interface CancelarOtPayload {
  id_ot: number;
  motivo: string;
}

export interface RegistrarInsumoOtPayload {
  id_ot: number;
  id_producto: number;
  cantidad: number;
  id_operacion?: string;
}

export interface DevolverInsumoOtPayload {
  id_ot: number;
  id_detalle_producto: number;
  cantidad_a_devolver: number;
  motivo: string;
  id_operacion?: string;
}

export interface AgregarServicioOtPayload {
  id_ot: number;
  id_servicio: number;
  cantidad: number;
}

export interface ModificarServicioOtPayload {
  id_ot: number;
  id_detalle_servicio: number;
  nueva_cantidad: number;
}
