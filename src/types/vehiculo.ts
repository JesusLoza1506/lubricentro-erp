import { ClienteDTO, ClienteResumenDTO } from './cliente';

export type { ClienteDTO, ClienteResumenDTO };

export type TipoMotor = 'GASOLINA' | 'DIESEL' | 'GLP' | 'GNV' | 'HIBRIDO' | 'ELECTRICO' | 'OTRO';
export type TipoAceite = 'MINERAL' | 'SINTETICO';

export interface ResultadoPaginadoDTO<T> {
  items: T[];
  total_registros: number;
  pagina_actual: number;
  total_paginas: number;
}

export interface VehiculoCompletoDTO {
  id_vehiculo: number;
  placa: string;
  id_cliente: number;
  marca: string;
  modelo: string;
  anio?: number;
  tipo_motor?: TipoMotor | string;
  kilometraje_actual: number;
  activo: number;
  cliente?: ClienteResumenDTO;
  nombre_cliente?: string;
  tipo_documento_cliente?: string;
  documento_cliente?: string;
  telefono_cliente?: string;
  correo_cliente?: string;
  direccion_cliente?: string;
}

export interface DetalleItemOT {
  id_detalle?: number;
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
  marca_vehiculo?: string;
  modelo_vehiculo?: string;
  id_cliente?: number;
  nombre_cliente?: string;
  tipo_documento_cliente?: string;
  documento_cliente?: string;
  id_mecanico: number;
  nombre_mecanico?: string;
  zanja?: number;
  estado: string;
  kilometraje_ingreso: number;
  proximo_kilometraje: number | null;
  observaciones?: string;
  fecha_ingreso: string;
  tipo_aceite?: TipoAceite | string | null;
  corregido?: number;
  detalles?: DetalleItemOT[];
}

// DTOs Públicos Sanitizados para Ficha Vehicular
export interface ServicioOTPublicoDTO {
  id_detalle: number;
  id_servicio?: number;
  codigo_snapshot?: string;
  nombre_snapshot: string;
  cantidad: number;
  es_cambio_aceite: number;
}

export interface InsumoOTPublicoDTO {
  id_detalle: number;
  id_producto?: number;
  codigo_snapshot?: string;
  nombre_snapshot: string;
  unidad_medida?: string;
  cantidad_formateada: string | number;
}

export interface HistorialOTPublicoDTO {
  id_ot: number;
  codigo_ot: string;
  placa_snapshot?: string;
  placa?: string;
  estado?: string;
  fecha_fin?: string;
  fecha_ingreso?: string;
  kilometraje_salida?: number;
  kilometraje_ingreso?: number;
  resumen_publico?: string;
  observaciones?: string;
  tipo_aceite?: string;
  km_proximo?: number | null;
  proximo_kilometraje?: number | null;
  id_mecanico?: number;
  corregido?: number;
  nombre_cliente?: string;
  cliente_nombre?: string;
  documento_cliente?: string;
  cliente_documento?: string;
  tipo_documento_cliente?: string;
  nombre_mecanico?: string;
  servicios?: ServicioOTPublicoDTO[];
  insumos?: InsumoOTPublicoDTO[];
  detalles?: DetalleItemOT[];
}

export interface FichaVehicularCompletaDTO {
  vehiculo: VehiculoCompletoDTO;
  historial: ResultadoPaginadoDTO<HistorialOTPublicoDTO>;
}

export interface ProximoCambioAceiteDTO {
  id_vehiculo: number;
  placa: string;
  kilometraje_actual: number;
  id_cliente: number;
  nombre_razon_social: string;
  telefono?: string;
  id_ot: number;
  fecha_servicio: string;
  tipo_aceite?: TipoAceite | string | null;
  intervalo_aceite_km?: number | null;
  km_proximo?: number | null;
  causa: 'POR_KM' | 'POR_FECHA' | 'AMBOS';
}

export interface CrearVehiculoDTO {
  placa: string;
  marca: string;
  modelo: string;
  anio?: number | null;
  tipo_motor?: TipoMotor | null;
  kilometraje_actual: number;
  id_cliente?: number | null;
  nombre_razon_social?: string;
  tipo_documento?: string;
  numero_documento?: string;
  telefono?: string;
  correo?: string;
  direccion?: string;
}

export interface EditarVehiculoDTO {
  id_vehiculo?: number;
  placa: string;
  marca: string;
  modelo: string;
  anio?: number | null;
  tipo_motor?: TipoMotor | null;
  kilometraje_actual: number;
  id_cliente?: number | null;
  nombre_razon_social?: string;
  tipo_documento?: string;
  numero_documento?: string;
  telefono?: string;
  correo?: string;
  direccion?: string;
  motivo?: string;
}

export interface CambiarEstadoVehiculoPayloadDTO {
  placa: string;
  activo: number;
  motivo: string;
}

export interface CorregirHistorialOtPayloadDTO {
  id_ot: number;
  resumen_publico: string;
  kilometraje_salida: number;
  tipo_aceite?: TipoAceite | null;
  motivo: string;
}
