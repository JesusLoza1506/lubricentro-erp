export type EstadoOT = 'EN_ESPERA' | 'EN_PROCESO' | 'FINALIZADO' | 'CANCELADO';
export type TipoAceiteOT = 'MINERAL' | 'SINTETICO';

export interface DetalleItemOtDTO {
  id_detalle: number;
  descripcion: string;
  cantidad: number;
  precio_unitario: number;
  subtotal: number;
  tipo: 'PRODUCTO' | 'SERVICIO';
}

export interface OrdenTrabajoHistorialDTO {
  id_ot: number;
  codigo_ot: string;
  placa: string;
  id_mecanico: number;
  nombre_mecanico?: string;
  zanja?: number;
  estado: EstadoOT;
  kilometraje_ingreso: number;
  proximo_kilometraje: number;
  observaciones?: string;
  fecha_ingreso?: string;
  tipo_aceite?: string;
  detalles: DetalleItemOtDTO[];
}

export interface CrearOrdenTrabajoPayload {
  placa: string;
  id_mecanico: number;
  zanja?: number;
  kilometraje_ingreso: number;
  tipo_aceite: TipoAceiteOT;
  observaciones?: string;
}

export interface CambiarEstadoOtPayload {
  id_ot: number;
  nuevo_estado: EstadoOT;
  zanja?: number;
}

export interface ReasignarMecanicoOtPayload {
  id_ot: number;
  nuevo_id_mecanico: number;
}
