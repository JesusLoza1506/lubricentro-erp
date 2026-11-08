import { invoke } from '@tauri-apps/api/core';

export type AccionCaja = 'abrir' | 'manual' | 'revertir' | 'cerrar' | 'aclarar';
export interface SolicitudCaja {
  accion: AccionCaja;
  payload: Record<string, string | number | null>;
}

export function cajaApi<T>(accion: string, payload: object = {}): Promise<T> {
  return invoke<T>('caja_operacion_cmd', { accion, payload });
}
const clave = (usuario: number, ambiente: string) => `gloria:caja:pendiente:${ambiente}:${usuario}`;
const escrituras = new Set(['abrir', 'manual', 'revertir', 'cerrar', 'aclarar']);

export function leerCajaPendiente(usuario: number, ambiente: string): SolicitudCaja | null {
  const guardado = localStorage.getItem(clave(usuario, ambiente));
  if (!guardado) return null;
  const r = JSON.parse(guardado) as SolicitudCaja;
  if (!escrituras.has(r?.accion) || typeof r.payload?.id_operacion !== 'string')
    throw new Error('No se pudo recuperar la solicitud guardada de Caja Chica.');
  return r;
}
export function mensajeCaja(e: unknown): string {
  const mensaje =
    typeof e === 'string'
      ? e
      : e instanceof Error
        ? e.message
        : 'No se pudo completar la operación. Inténtalo nuevamente.';
  return mensaje
    .replace(
      /^(VALIDATION_ERROR|ACCESS_DENIED|PERMISO_DENEGADO|NOT_FOUND|AUTH_ERROR|NO_AUTENTICADO|DATABASE_ERROR|DB_LOCK):\s*/,
      ''
    )
    .replace(/^RESUMEN_DESACTUALIZADO:\s*/, '');
}
export const esArqueoVencido = (e: unknown) =>
  typeof e === 'string' && e.startsWith('VALIDATION_ERROR: RESUMEN_DESACTUALIZADO:');

export async function ejecutarCaja<T>(
  usuario: number,
  ambiente: string,
  accion: AccionCaja,
  datos: SolicitudCaja['payload']
): Promise<T> {
  const guardado = leerCajaPendiente(usuario, ambiente);
  if (
    guardado &&
    (guardado.accion !== accion ||
      Object.entries(datos).some(([k, v]) => guardado.payload[k] !== v))
  )
    throw new Error('Primero recupera la operación pendiente de Caja Chica.');
  const payload = guardado?.payload ?? { ...datos, id_operacion: crypto.randomUUID() };
  const storageKey = clave(usuario, ambiente);
  localStorage.setItem(storageKey, JSON.stringify({ accion, payload }));
  let resultado: T;
  try {
    resultado = await cajaApi<T>(accion, payload);
  } catch (e) {
    // Una validación del servicio implica que no hubo una escritura nueva confirmada.
    if (typeof e === 'string' && /^(VALIDATION_ERROR|NOT_FOUND):/.test(e))
      localStorage.removeItem(storageKey);
    else if (
      !guardado &&
      typeof e === 'string' &&
      /^(PERMISO_DENEGADO|ACCESS_DENIED|NO_AUTENTICADO):/.test(e)
    )
      localStorage.removeItem(storageKey);
    throw e;
  }
  localStorage.removeItem(storageKey);
  return resultado;
}
