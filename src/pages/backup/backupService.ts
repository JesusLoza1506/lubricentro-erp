import { invoke } from '@tauri-apps/api/core';

export interface ContextoBackup {
  inicial?: boolean;
  pendiente?: boolean;
  aviso?: boolean;
  ambiente?: string;
  generacion?: string | null;
}
export interface SerieRecuperada {
  id: number;
  serie: string;
  tipo: string;
  local: number;
  previo: number;
  remoto: number;
  pendiente: 0 | 1;
  siguiente: number;
}
export interface Recuperacion {
  pendiente: boolean;
  caja: boolean;
  token: boolean;
  series: SerieRecuperada[];
}
export interface Copia {
  id: number;
  nombre: string;
  tipo: string;
  estado: string;
  externo: string;
  fecha: string | null;
  creado: string;
  tamano: number | null;
  verificado: string | null;
}
export interface EstadoBackup {
  configurada: boolean;
  ultima: string | null;
  vencida: boolean;
  proxima: string;
  diaria: string | null;
  copias: Copia[];
  total: number;
  recuperacion: Recuperacion;
  ultima_restauracion?: { nombre: string; estado: string; fecha: string; previa?: string };
}
export interface SeleccionBackup {
  nombre: string;
  tamano: number;
  seleccion: string;
  compatible: boolean;
  caja_abierta: boolean;
  manifiesto: {
    identidad: { ambiente: string; ruc: string | null };
    fecha: string;
    generado: string;
    tipo: string;
    autor: string;
    app: string;
    esquema: number;
    formato: number;
    archivos: { nombre: string }[];
  };
}

export const contextoBackup = () => invoke<ContextoBackup>('backup_contexto_cmd');
export const elegirBackup = (carpeta: boolean, inicial: boolean) =>
  invoke<string | null>('backup_elegir_cmd', { carpeta, inicial });
export const backup = <T>(accion: string, datos: Record<string, unknown> = {}) =>
  invoke<T>('backup_operacion_cmd', { req: { accion, ...datos } });
export const errorBackup = (error: unknown) =>
  String(error instanceof Error ? error.message : error).replace(/^[A-Z_]+: /, '');

// El servicio devuelve fechas de Lima sin zona; solo se cambia su presentación.
export function fechaBackup(fecha: string) {
  return fecha.replace(/^(\d{4})-(\d{2})-(\d{2})[ T](\d{2}:\d{2})(?::\d{2})?$/, '$3/$2/$1 · $4');
}

// Captures in WebView belong to the database generation where they were made.
export function sincronizarCapturasBackup({ ambiente, generacion }: ContextoBackup) {
  if (!ambiente || !generacion) return;
  const marca = `gloria:backup:generacion:${ambiente}`;
  if (localStorage.getItem(marca) === generacion) return;
  const prefijos = [
    `pos.operacion.v1.${ambiente}.`,
    `gloria:caja:pendiente:${ambiente}:`,
    `inventario.solicitud.v1.${ambiente}.`,
  ];
  for (let i = localStorage.length - 1; i >= 0; i--) {
    const k = localStorage.key(i);
    if (k && prefijos.some((p) => k.startsWith(p))) localStorage.removeItem(k);
  }
  localStorage.setItem(marca, generacion);
}

export interface BackupArranque {
  bloqueado: boolean;
  motivo?: string | null;
  ambiente: string;
  previa: boolean;
}

export const consultarArranqueBackup = () => invoke<BackupArranque>('backup_arranque_cmd');
export const recuperarArranqueBackup = (req: { usuario: string; clave: string; frase: string; confirmado: boolean }) =>
  invoke<void>('backup_recuperar_arranque_cmd', { req });

export const progresoBackup = () => invoke<{ fase: number; no_cerrar: boolean }>('backup_progreso_cmd');

export const etapasBackup: Record<number, string> = {
  1: 'Preparando y cifrando el respaldo…',
  2: 'Comprobando autenticidad e integridad…',
  3: 'Guardando la copia comprobada…',
  4: 'Preparando la base y los archivos recuperados…',
  5: 'Activando el conjunto restaurado…',
};

export interface ContextoDesdeCaja {
  tipo: string;
  pagina: number;
  seleccion: SeleccionBackup | null;
}
