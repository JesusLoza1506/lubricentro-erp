import { invoke } from '@tauri-apps/api/core';
import type { CredencialesPos, ResultadoPos } from '../types/pos';

export const posApi = <T>(accion: string, payload: unknown = {}, credenciales?: CredencialesPos) =>
  invoke<T>('pos_operacion_cmd', { accion, payload, credenciales: credenciales ?? null });
export const documentoPos = (
  id: number,
  consultar = false,
  automatico = false,
  tokenEnvio?: string
) =>
  invoke<{ estado: string; mensaje: string }>('pos_documento_cmd', {
    idDocumento: id,
    consultar,
    automatico,
    tokenEnvio: tokenEnvio ?? null,
  });
export interface SolicitudPos {
  accion: 'confirmar' | 'devolver' | 'reemitir';
  payload: Record<string, unknown>;
}
const trabajando = new Set<string>();
const clave = (usuario: number, ambiente: string) => `pos.operacion.v1.${ambiente}.${usuario}`;
export function leerPendientePos(usuario: number, ambiente: string): SolicitudPos | null {
  const raw = localStorage.getItem(clave(usuario, ambiente));
  if (!raw) return null;
  const p = JSON.parse(raw) as SolicitudPos;
  if (
    !['confirmar', 'devolver', 'reemitir'].includes(p.accion) ||
    typeof p.payload?.id_operacion !== 'string'
  )
    throw new Error('La solicitud pendiente requiere revisión. No inicies otro cobro.');
  return p;
}
export async function ejecutarPos(
  usuario: number,
  ambiente: string,
  accion: SolicitudPos['accion'],
  payload: Record<string, unknown>,
  credenciales?: CredencialesPos
): Promise<ResultadoPos> {
  const k = clave(usuario, ambiente);
  if (trabajando.has(k)) throw new Error('Esta operación ya se está procesando.');
  const anterior = leerPendientePos(usuario, ambiente);
  if (anterior) payload = { ...payload, id_operacion: anterior.payload.id_operacion };
  if (
    anterior &&
    (anterior.accion !== accion || JSON.stringify(anterior.payload) !== JSON.stringify(payload))
  )
    throw new Error(
      'Primero recupera el resultado de la solicitud pendiente con sus datos originales.'
    );
  localStorage.setItem(k, JSON.stringify({ accion, payload }));
  trabajando.add(k);
  try {
    const r = await posApi<ResultadoPos>(accion, payload, credenciales);
    localStorage.removeItem(k);
    return r;
  } catch (e) {
    if (
      typeof e === 'string' &&
      /^(VALIDATION_ERROR|NOT_FOUND|ACCESS_DENIED|PERMISO_DENEGADO):/.test(e) &&
      !/Idempotencia|revisión|revision/i.test(e)
    )
      localStorage.removeItem(k);
    throw e;
  } finally {
    trabajando.delete(k);
  }
}
export const mensajePos = (e: unknown) =>
  e instanceof Error
    ? e.message
    : typeof e === 'string'
      ? e.replace(/^[A-Z_]+:\s*/, '')
      : 'No se pudo completar la operación. Conserva los datos y reintenta.';
