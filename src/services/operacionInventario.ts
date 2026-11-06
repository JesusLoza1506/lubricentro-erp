import { invoke } from '@tauri-apps/api/core';

interface Solicitud {
  id: string;
  payload: unknown;
}
interface Contexto {
  usuario: number;
  ambiente: string;
}
let contexto: Contexto | null = null;
const enCurso = new Set<string>();

export function configurarContextoInventario(usuario: number, ambiente: string) {
  contexto = { usuario, ambiente };
}

function clave(comando: string): string {
  if (!contexto)
    throw new Error('No se ha identificado la sesión de inventario. Vuelve a abrir el módulo.');
  return `inventario.solicitud.v1.${contexto.ambiente}.${contexto.usuario}.${comando}`;
}

export function solicitudPendiente(comando: string): Solicitud | null {
  const valor = localStorage.getItem(clave(comando));
  if (!valor) return null;
  const solicitud = JSON.parse(valor) as Solicitud;
  if (typeof solicitud.id !== 'string' || !('payload' in solicitud)) {
    throw new Error(
      'No se pudo leer la solicitud pendiente. Conserva los datos y solicita revisión.'
    );
  }
  return solicitud;
}

/** Guarda antes de enviar: una respuesta perdida conserva ID y datos originales incluso al reabrir. */
export async function ejecutarOperacionInventario<T>(
  comando: string,
  payload: unknown,
  idOperacion?: string
): Promise<T> {
  const llave = clave(comando);
  if (enCurso.has(llave)) throw new Error('Esta operación ya se está procesando.');
  const anterior = solicitudPendiente(comando);
  if (anterior && JSON.stringify(anterior.payload) !== JSON.stringify(payload)) {
    throw new Error(
      'Existe una solicitud sin respuesta confirmada. Reabre el formulario para reintentar los datos originales antes de iniciar otra operación.'
    );
  }
  const solicitud = anterior ?? {
    id: idOperacion ?? crypto.randomUUID(),
    payload: JSON.parse(JSON.stringify(payload)),
  };
  localStorage.setItem(llave, JSON.stringify(solicitud));
  enCurso.add(llave);
  try {
    const resultado = await invoke<T>(comando, {
      payload: solicitud.payload,
      idOperacion: solicitud.id,
    });
    localStorage.removeItem(llave);
    return resultado;
  } catch (error) {
    // Solo una validación de negocio definitiva libera la captura.
    // Sesión, almacenamiento o un resultado previo ilegible conservan el ID: todavía puede existir un éxito anterior.
    if (
      typeof error === 'string' &&
      /^(VALIDATION_ERROR|NOT_FOUND):/.test(error) &&
      !/Idempotencia|PENDING|Requiere revisión|Requiere revision/i.test(error)
    ) {
      localStorage.removeItem(llave);
      throw error;
    }
    throw new Error(
      `No se confirmó el resultado. Reintenta esta misma solicitud; se conservaron su identificador y sus datos.${typeof error === 'string' ? ` ${error}` : ''}`
    );
  } finally {
    enCurso.delete(llave);
  }
}

export function solesAMicro(valor: number, permitirCero = false): number {
  const centimos = valor * 100;
  if (
    !Number.isFinite(valor) ||
    valor < (permitirCero ? 0 : 0.01) ||
    valor > 999999.99 ||
    Math.abs(centimos - Math.round(centimos)) > 0.000001
  ) {
    throw new Error(
      'Ingresa un importe válido con un máximo de dos decimales (hasta S/ 999999.99).'
    );
  }
  return Math.round(centimos) * 10000;
}
