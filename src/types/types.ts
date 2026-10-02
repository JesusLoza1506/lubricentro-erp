export interface PermisoRolDTO {
  id_permiso: number;
  rol: 'ADMINISTRADOR' | 'CAJERO' | 'MECANICO';
  modulo: string;
  puede_ver: boolean;
  puede_crear: boolean;
  puede_editar: boolean;
  puede_eliminar: boolean;
}

// Ojehechauka hekopete UsuarioSesionDTO oúva Rust-gui
export interface UsuarioSesionDTO {
  id_usuario: number;
  nombre_completo: string;
  usuario: string;
  rol: 'ADMINISTRADOR' | 'CAJERO' | 'MECANICO';
  activo: boolean;
  permisos: PermisoRolDTO[];
}

// Ojehechauka umi mba'e ojeikotevẽva oñepyrũ hag̃ua sesión
export interface Credenciales {
  usuario: string;
  password: string;
}
