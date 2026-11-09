import { invoke } from '@tauri-apps/api/core';

export type Rol = 'ADMINISTRADOR' | 'CAJERO' | 'MECANICO';
export type Usuario = {
  id: number;
  login: string;
  nombre: string;
  rol: Rol;
  activo: boolean;
  bloqueado: boolean;
  debe_cambiar_clave: boolean;
  version: number;
};
export type FiltrosUsuarios = {
  busqueda: string;
  rol: string;
  estado: string;
  pagina: number;
};

export const FILTROS_USUARIOS_INICIALES: FiltrosUsuarios = {
  busqueda: '',
  rol: '',
  estado: '',
  pagina: 1,
};

export type PaginaUsuarios = {
  usuarios: Usuario[];
  actor: number;
  pagina: number;
  paginas: number;
  total: number;
};
export type Dependencias = { caja: string | null; ordenes: string[] };
export type DetalleUsuario = { usuario: Usuario; pendientes: Dependencias };
export type Entrega = { login: string; temporal?: string; repetida: boolean };
export type DatosEmpresa = {
  ruc: string | null;
  razon_social: string | null;
  nombre_comercial: string | null;
  direccion: string | null;
  ubigeo: string | null;
  telefono: string | null;
  correo: string | null;
  version: number;
  ruc_fijo: boolean;
};
export type TipoSerie = 'BOLETA' | 'FACTURA' | 'NC_BOLETA' | 'NC_FACTURA';
export type Serie = {
  id: number;
  tipo: TipoSerie;
  codigo: string;
  activa: boolean;
  pendiente: boolean;
  ultimo: number;
  siguiente: number | null;
};
export type Facturacion = {
  ruta: string | null;
  version: number;
  token_configurado: boolean;
  configuracion_completa: boolean;
  series: Serie[];
};
export type DatosTaller = {
  cantidad: number;
  zanjas: { id: number; activa: boolean; ot: string | null }[];
};

export function admin<T>(accion: string, datos: Record<string, unknown> = {}) {
  return invoke<T>('admin_operacion_cmd', { req: { accion, ...datos } });
}
export function cambiarAmbiente(destino: string) {
  return invoke<void>('admin_cambiar_ambiente_cmd', { destino, confirmado: true });
}
export function mensajeError(error: unknown) {
  return (error instanceof Error ? error.message : String(error)).replace(/^\[[^\]]+\]\s*/, '');
}
