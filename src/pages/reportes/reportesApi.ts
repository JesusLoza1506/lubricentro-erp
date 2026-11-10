import type { Producto } from '../../types/inventario';
import { invoke } from '@tauri-apps/api/core';

export const REPORTES = {
  R01: 'Ventas por periodo',
  R02: 'Cobros y devoluciones',
  R03: 'Productos y margen',
  R04: 'Stock actual y crítico',
  R05: 'Cierres de caja',
  R06: 'Trabajo del taller',
  R07: 'Comprobantes',
  R08: 'Auditoría',
} as const;

export type Reporte = keyof typeof REPORTES;
export type Periodo =
  | 'hoy'
  | 'ayer'
  | 'esta_semana'
  | 'este_mes'
  | 'mes_anterior'
  | 'personalizado';
export type Formato = 'pdf' | 'xlsx';
export type Filtros = {
  periodo: Periodo;
  desde?: string;
  hasta?: string;
  medio?: string;
  tipo?: string;
  ver_todos?: boolean;
  buscar?: string;
  categoria?: string;
  estado?: string;
  solo_criticos?: boolean;
  usuario?: number;
  con_diferencia?: boolean;
  actividad?: string;
  actor?: string;
  modulo?: string;
};
export type Origen = {
  destino: 'pos' | 'caja' | 'inventario' | 'taller' | 'administracion' | 'backup';
  id: number;
  producto?: Pick<Producto, 'id_producto' | 'codigo' | 'nombre' | 'unidad_medida'>;
};
export type Celda = { tipo: 'texto' | 'numero'; valor: string } | { tipo: 'no_aplica' };
export type Fila = { celdas: Celda[]; origenes: Origen[] };
export type Detalle = { columnas: string[]; filas: Fila[] };
export type UsuarioFiltro = { id: number; nombre: string; rol: string; responsable_ot: boolean };
export type Pagina = Detalle & {
  id: string;
  reporte: Reporte;
  ambiente: string;
  hora: string;
  criterio: string;
  rango: [string, string] | null;
  filtros: Filtros;
  filtros_texto: string;
  usuarios: UsuarioFiltro[];
  pagina: number;
  total_filas: number;
  disponibles: number | null;
  totales: [string, Celda][];
};
export const POR_PAGINA = 20;

export function valor(v: Celda) {
  return v.tipo === 'no_aplica' ? 'No aplica' : v.valor;
}

export function inicial(anterior?: Filtros): Filtros {
  return {
    periodo: anterior?.periodo ?? 'este_mes',
    ...(anterior?.periodo === 'personalizado'
      ? { desde: anterior.desde, hasta: anterior.hasta }
      : {}),
  };
}

export function filtrosIguales(a: Filtros, b: Filtros) {
  const campos = new Set([...Object.keys(a), ...Object.keys(b)] as (keyof Filtros)[]);
  return [...campos].every((k) => (a[k] ?? defecto(k)) === (b[k] ?? defecto(k)));
}
function defecto(k: keyof Filtros) {
  if (k === 'buscar') return '';
  if (['ver_todos', 'solo_criticos', 'con_diferencia'].includes(k)) return false;
  return undefined;
}
type Req =
  | { accion: 'consultar'; reporte: Reporte; filtros: Filtros }
  | { accion: 'pagina'; id: string; pagina: number }
  | { accion: 'detalle'; id: string; fila: number }
  | { accion: 'origen'; id: string; fila: number; detalle?: number; indice: number }
  | { accion: 'exportar'; id: string; reporte: Reporte; filtros: Filtros; formato: Formato }
  | { accion: 'descartar'; id: string };
export function reportes<T>(req: Req) {
  return invoke<T>('reportes_operacion_cmd', { req });
}
export function errorReporte(e: unknown) {
  const msg = typeof e === 'string' ? e : e instanceof Error ? e.message : '';
  return msg.replace(/^[A-Z_]+:\s*/, '') || 'No se pudo completar la consulta. Inténtalo de nuevo.';
}
