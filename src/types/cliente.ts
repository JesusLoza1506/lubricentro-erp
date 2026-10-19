export interface ClienteResumenDTO {
  id_cliente: number;
  tipo_documento?: string | null;
  numero_documento?: string | null;
  nombre_razon_social: string;
  telefono?: string | null;
  correo?: string | null;
  direccion?: string | null;
}

export interface ClienteDTO {
  id_cliente: number;
  tipo_documento: 'DNI' | 'RUC' | 'CE';
  numero_documento: string;
  nombre_razon_social: string;
  telefono?: string;
  correo?: string;
  direccion?: string;
  es_varios: boolean;
  fecha_creacion: string;
}

export interface CrearClienteDTO {
  tipo_documento: string;
  numero_documento: string;
  nombre_razon_social: string;
  telefono?: string;
  correo?: string;
  direccion?: string;
}

export interface EditarClienteDTO {
  id_cliente: number;
  tipo_documento: string;
  numero_documento: string;
  nombre_razon_social: string;
  telefono?: string;
  correo?: string;
  direccion?: string;
  motivo?: string;
}
