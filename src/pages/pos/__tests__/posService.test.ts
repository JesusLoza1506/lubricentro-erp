import { beforeEach, describe, expect, it, vi } from 'vitest';
import { invoke } from '@tauri-apps/api/core';
import { ejecutarPos, leerPendientePos } from '../../../services/posService';
import { construirPagos, decimalBase } from '../posUtils';
vi.mock('@tauri-apps/api/core', () => ({ invoke: vi.fn() }));
beforeEach(() => {
  localStorage.clear();
  vi.clearAllMocks();
});
describe('capturas y operaciones económicas POS', () => {
  it('representa litros y céntimos sin aceptar truncamiento', () => {
    expect(decimalBase('4,500', 3)).toBe(4500);
    expect(decimalBase('1.23', 2)).toBe(123);
    expect(decimalBase('3', 0)).toBe(3);
    expect(() => decimalBase('1.234', 2)).toThrow();
    expect(() => decimalBase('1.5', 0)).toThrow();
    expect(() => decimalBase('NaN', 2)).toThrow();
  });
  it('calcula aplicado y recibido por separado en pago mixto', () => {
    const p = construirPagos(8000, 'EFECTIVO', 'YAPE', '30', '100', '');
    expect(p.map((x) => x.monto_cent)).toEqual([5000, 3000]);
    expect(p[0].recibido_cent).toBe(10000);
    expect(p[1].recibido_cent).toBeNull();
  });
  it('rechaza medios iguales, exceso digital y efectivo insuficiente', () => {
    expect(() => construirPagos(8000, 'YAPE', 'YAPE', '30', '', '')).toThrow();
    expect(() => construirPagos(8000, 'YAPE', 'TARJETA', '90', '', '')).toThrow();
    expect(() => construirPagos(8000, 'EFECTIVO', null, '', '70', '')).toThrow();
  });
  it('conserva identificador y datos originales ante respuesta incierta, sin guardar credenciales', async () => {
    vi.mocked(invoke)
      .mockRejectedValueOnce('DATABASE_ERROR: respuesta no confirmada')
      .mockResolvedValueOnce({ id_venta: 1, id_documento: 1 });
    const payload = { id_operacion: 'operacion-original-01', total: 100 };
    await expect(
      ejecutarPos(2, 'PRUEBA', 'confirmar', payload, { username: 'admin', password: 'secreto' })
    ).rejects.toBeTruthy();
    expect(JSON.stringify(leerPendientePos(2, 'PRUEBA'))).not.toContain('secreto');
    await ejecutarPos(2, 'PRUEBA', 'confirmar', { ...payload, id_operacion: 'otro-uuid-nuevo' });
    expect(vi.mocked(invoke).mock.calls[1][1]).toMatchObject({ payload });
    expect(leerPendientePos(2, 'PRUEBA')).toBeNull();
  });
  it('no permite sustituir datos de una solicitud incierta', async () => {
    vi.mocked(invoke).mockRejectedValue('DATABASE_ERROR: incierto');
    await expect(
      ejecutarPos(2, 'PRUEBA', 'confirmar', { id_operacion: 'operacion-original-02', total: 100 })
    ).rejects.toBeTruthy();
    await expect(
      ejecutarPos(2, 'PRUEBA', 'confirmar', { id_operacion: 'operacion-nueva-02', total: 200 })
    ).rejects.toThrow('datos originales');
    expect(invoke).toHaveBeenCalledTimes(1);
  });
  it('separa usuario y ambiente', async () => {
    vi.mocked(invoke).mockRejectedValue('DATABASE_ERROR: incierto');
    await expect(
      ejecutarPos(2, 'PRUEBA', 'confirmar', { id_operacion: 'operacion-original-03' })
    ).rejects.toBeTruthy();
    expect(leerPendientePos(3, 'PRUEBA')).toBeNull();
    expect(leerPendientePos(2, 'PRODUCCION')).toBeNull();
  });
  it('bloquea doble envío concurrente antes de recibir respuesta', async () => {
    let completar: (v: unknown) => void = () => {};
    vi.mocked(invoke).mockReturnValueOnce(
      new Promise((r) => {
        completar = r;
      })
    );
    const p = { id_operacion: 'operacion-original-04' };
    const primero = ejecutarPos(2, 'PRUEBA', 'confirmar', p);
    await expect(ejecutarPos(2, 'PRUEBA', 'confirmar', p)).rejects.toThrow('procesando');
    completar({ id_venta: 1, id_documento: 1 });
    await primero;
    expect(invoke).toHaveBeenCalledTimes(1);
  });
});
