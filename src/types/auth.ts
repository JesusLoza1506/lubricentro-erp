export type EstadoSesionEnum = 'SIN_SESION' | 'CAMBIO_OBLIGATORIO' | 'NORMAL';

export type EstadoAmbiente = 'CARGANDO' | 'CONFIRMADO' | 'ERROR';

export interface PermisoModulo {
  modulo: string;
  puede_ver: boolean;
  puede_crear: boolean;
  puede_editar: boolean;
  puede_eliminar: boolean;
}

export interface UsuarioSesion {
  id_usuario: number;
  nombre_completo: string;
  username: string;
  rol: string;
  activo: boolean;
  debe_cambiar_clave: boolean;
  estado_sesion: EstadoSesionEnum;
  permisos: PermisoModulo[];
  ambiente_actual?: string;
}

export interface SesionResumenDto {
  id_usuario: number;
  username: string;
  nombre_completo: string;
  rol: string;
  estado_sesion: EstadoSesionEnum;
  permisos: PermisoModulo[];
  ambiente_actual?: string;
}

export interface LoginPayload {
  username: string;
  password: string;
}

export interface InstalacionPayload {
  nombre_completo: string;
  username: string;
  password: string;
}

export interface InstalacionResponseDto {
  id_usuario?: number;
  username: string;
  codigo_recuperacion: string;
}

export interface RecuperarPayload {
  username: string;
  codigo_recuperacion: string;
  nueva_password: string;
}

export interface CambiarClaveObligatoriaPayload {
  nueva_password: string;
}

export interface CambiarClaveVoluntariaPayload {
  password_actual: string;
  nueva_password: string;
}

export interface ReautenticarPayload {
  password_actual: string;
  accion: string;
  referencia: string;
}

export interface AutorizarDescuentoPayload {
  username_admin: string;
  password_admin: string;
  referencia_venta: string;
  descuento_bp: number;
}
