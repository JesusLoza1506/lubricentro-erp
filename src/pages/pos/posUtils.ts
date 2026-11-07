import type { MedioPago, PagoPos } from '../../types/pos';
export const MEDIOS: MedioPago[] = ['EFECTIVO', 'TARJETA', 'YAPE', 'PLIN', 'TRANSFERENCIA'];
export const soles = (cent: number) =>
  new Intl.NumberFormat('es-PE', { style: 'currency', currency: 'PEN' }).format(cent / 100);
export const cantidad = (base: number, unidad: string) =>
  unidad === 'ML'
    ? `${(base / 1000).toLocaleString('es-PE', { maximumFractionDigits: 3 })} L`
    : `${base} UND`;
export function decimalBase(texto: string, decimales: number, permitirCero = false): number {
  const t = texto.trim().replace(',', '.');
  if (!(decimales === 0 ? /^\d+$/ : new RegExp(`^\\d+(?:\\.\\d{1,${decimales}})?$`)).test(t))
    throw new Error(`Ingresa un número con hasta ${decimales} decimales.`);
  const [ent, frac = ''] = t.split('.');
  const valor = Number(ent) * 10 ** decimales + Number(frac.padEnd(decimales, '0'));
  if (!Number.isSafeInteger(valor) || valor < (permitirCero ? 0 : 1))
    throw new Error('El importe o cantidad debe ser válido y positivo.');
  return valor;
}
export function construirPagos(
  total: number,
  medio: MedioPago,
  segundo: MedioPago | null,
  importeSegundo: string,
  recibido: string,
  referencia: string
): PagoPos[] {
  const dos = segundo ? decimalBase(importeSegundo, 2) : 0;
  if (segundo && (segundo === medio || dos >= total))
    throw new Error('Selecciona dos medios distintos con importes positivos.');
  const principal = total - dos;
  const efectivo = medio === 'EFECTIVO' ? principal : segundo === 'EFECTIVO' ? dos : 0;
  const rec = efectivo ? decimalBase(recibido, 2) : null;
  if (efectivo && rec !== null && rec < efectivo)
    throw new Error('El efectivo recibido no cubre el aplicado.');
  const pago = (m: MedioPago, n: number): PagoPos => ({
    medio: m,
    monto_cent: n,
    recibido_cent: m === 'EFECTIVO' ? rec : null,
    referencia: referencia.trim() || null,
  });
  return [pago(medio, principal), ...(segundo ? [pago(segundo, dos)] : [])];
}
