// Datos del Cliente/Propietario
export interface ClienteDTO {
  id_cliente: number;
  nombre_razon_social: string;
  tipo_documento?: string; // 'DNI' | 'RUC' | 'CE' | 'PASAPORTE'
  numero_documento?: string;
  telefono?: string;
  email?: string;
  direccion?: string;
}

// Datos Completos del Vehículo (incluyendo info del cliente)
export interface VehiculoCompletoDTO {
  id_vehiculo?: number;
  placa: string;
  id_cliente: number;
  marca: string;
  modelo: string;
  anio?: number;
  tipo_motor?: string;
  kilometraje_actual: number;
  activo?: number;
  // Datos del Cliente Anidados o Aplanados
  cliente?: ClienteDTO;
  nombre_cliente?: string;
  tipo_documento_cliente?: string;
  documento_cliente?: string;
  telefono_cliente?: string;
  direccion_cliente?: string;
}

// Insumos/Servicios aplicados en una OT
export interface DetalleItemOT {
  id_detalle?: number;
  descripcion: string;
  cantidad: number;
  precio_unitario: number;
  subtotal: number;
  tipo: 'PRODUCTO' | 'SERVICIO';
}

// Historial Completo de Orden de Trabajo
export interface OrdenTrabajoHistorialDTO {
  id_ot: number;
  codigo_ot: string;
  placa: string;
  id_mecanico: number;
  nombre_mecanico?: string;
  zanja?: number;
  estado: string;
  kilometraje_ingreso: number;
  proximo_kilometraje: number;
  observaciones?: string;
  fecha_ingreso: string;
  tipo_aceite?: string;
  detalles?: DetalleItemOT[];
}

// DTO Unificado para Ficha Vehicular Completa (Vehículo + Historial)
export interface FichaVehicularCompletaDTO {
  vehiculo: VehiculoCompletoDTO;
  historial: OrdenTrabajoHistorialDTO[];
}

// DTO para Crear Cliente y Vehículo desde la Ficha
export interface CrearVehiculoDTO {
  placa: string;
  marca: string;
  modelo: string;
  anio?: number | null;
  tipo_motor?: string | null;
  kilometraje_actual: number;
  // Si el cliente es nuevo o existente
  id_cliente?: number | null;
  nombre_razon_social?: string;
  tipo_documento?: string;
  numero_documento?: string;
  telefono?: string;
  direccion?: string;
}

// DTO para Actualizar Vehículo
export interface EditarVehiculoDTO {
  id_vehiculo?: number;
  placa: string;
  marca: string;
  modelo: string;
  anio?: number | null;
  tipo_motor?: string | null;
  kilometraje_actual: number;
  id_cliente?: number | null;
  // Datos para actualizar/reasignar al cliente asociado
  nombre_razon_social?: string;
  tipo_documento?: string;
  numero_documento?: string;
  telefono?: string;
  direccion?: string;
}
