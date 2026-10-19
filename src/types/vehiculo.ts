import { ClienteDTO, ClienteResumenDTO } from './cliente';

export type { ClienteDTO, ClienteResumenDTO };

export type TipoMotor = 'GASOLINA' | 'DIESEL' | 'GLP' | 'GNV' | 'HIBRIDO' | 'ELECTRICO' | 'OTRO';
export type TipoAceite = 'MINERAL' | 'SINTETICO';

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

// ✅ CORREGIDO (Punto 2): Incorporación de campos de identidad histórica en la OT
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

export interface FichaVehicularCompletaDTO {
  vehiculo: VehiculoCompletoDTO;
  historial: OrdenTrabajoHistorialDTO[];
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
