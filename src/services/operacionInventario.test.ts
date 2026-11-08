import { beforeEach, describe, expect, it, vi } from 'vitest';
import { invoke } from '@tauri-apps/api/core';
import {
  configurarContextoInventario,
  ejecutarOperacionInventario,
  solicitudPendiente,
  solesAMicro,
} from './operacionInventario';
vi.mock('@tauri-apps/api/core', () => ({ invoke: vi.fn() }));
beforeEach(() => {
  localStorage.clear();
  vi.resetAllMocks();
  configurarContextoInventario(1, 'PRUEBA');
});
describe('Solicitudes de inventario', () => {
  it.each([
    'registrar_ingreso_compra_cmd',
    'ajustar_stock_cmd',
    'procesar_conteo_fisico_cmd',
    'anular_ingreso_compra_cmd',
  ])('conserva ID y captura en reintento incierto de %s', async (comando) => {
    vi.mocked(invoke)
      .mockRejectedValueOnce(new Error('Respuesta perdida'))
      .mockResolvedValueOnce({ status: 'OK' });
    const original = { id_producto: 1, cantidad: 9 };
    await expect(ejecutarOperacionInventario(comando, original)).rejects.toThrow('No se confirmó');
    const guardado = solicitudPendiente(comando);
    expect(guardado?.id).toBeTruthy();
    await expect(
      ejecutarOperacionInventario(comando, { ...original, cantidad: 8 })
    ).rejects.toThrow('solicitud sin respuesta');
    expect(invoke).toHaveBeenCalledTimes(1);
    await ejecutarOperacionInventario(comando, original);
    expect(vi.mocked(invoke).mock.calls[1]).toEqual(vi.mocked(invoke).mock.calls[0]);
    expect(solicitudPendiente(comando)).toBeNull();
  });
  it('una captura nueva recibe otro ID después del éxito', async () => {
    vi.mocked(invoke).mockResolvedValue({ status: 'OK' });
    await ejecutarOperacionInventario('ajustar_stock_cmd', { cantidad: 1 });
    await ejecutarOperacionInventario('ajustar_stock_cmd', { cantidad: 1 });
    expect((vi.mocked(invoke).mock.calls[0][1] as Record<string, unknown>)?.idOperacion).not.toBe(
      (vi.mocked(invoke).mock.calls[1][1] as Record<string, unknown>)?.idOperacion
    );
  });
  it('bloquea doble envío simultáneo antes de esperar al backend', async () => {
    let resolver!: (v: unknown) => void;
    vi.mocked(invoke).mockImplementationOnce(
      () =>
        new Promise((r) => {
          resolver = r;
        }) as never
    );
    const uno = ejecutarOperacionInventario('procesar_conteo_fisico_cmd', { cantidad: 9 });
    await expect(
      ejecutarOperacionInventario('procesar_conteo_fisico_cmd', { cantidad: 9 })
    ).rejects.toThrow('ya se está procesando');
    expect(invoke).toHaveBeenCalledTimes(1);
    resolver({ status: 'OK' });
    await uno;
  });
  it('una validación definitiva libera la captura para corregirla', async () => {
    vi.mocked(invoke)
      .mockRejectedValueOnce('VALIDATION_ERROR: Stock modificado')
      .mockResolvedValueOnce({ status: 'OK' });
    await expect(
      ejecutarOperacionInventario('procesar_conteo_fisico_cmd', { cantidad: 9 })
    ).rejects.toBe('VALIDATION_ERROR: Stock modificado');
    expect(solicitudPendiente('procesar_conteo_fisico_cmd')).toBeNull();
    await ejecutarOperacionInventario('procesar_conteo_fisico_cmd', { cantidad: 8 });
    expect((vi.mocked(invoke).mock.calls[0][1] as Record<string, unknown>)?.idOperacion).not.toBe(
      (vi.mocked(invoke).mock.calls[1][1] as Record<string, unknown>)?.idOperacion
    );
  });
  it('no comparte solicitudes pendientes entre usuario ni ambiente', async () => {
    vi.mocked(invoke).mockRejectedValue(new Error('Respuesta perdida'));
    await expect(
      ejecutarOperacionInventario('ajustar_stock_cmd', { cantidad: 1 })
    ).rejects.toThrow();
    const anterior = solicitudPendiente('ajustar_stock_cmd');
    configurarContextoInventario(2, 'PRUEBA');
    expect(solicitudPendiente('ajustar_stock_cmd')).toBeNull();
    configurarContextoInventario(1, 'PRODUCCION');
    expect(solicitudPendiente('ajustar_stock_cmd')).toBeNull();
    configurarContextoInventario(1, 'PRUEBA');
    expect(solicitudPendiente('ajustar_stock_cmd')).toEqual(anterior);
  });
  it('si no puede persistir no envía al backend', async () => {
    const spy = vi.spyOn(Storage.prototype, 'setItem').mockImplementationOnce(() => {
      throw new Error('Sin almacenamiento');
    });
    await expect(ejecutarOperacionInventario('ajustar_stock_cmd', { cantidad: 1 })).rejects.toThrow(
      'Sin almacenamiento'
    );
    expect(invoke).not.toHaveBeenCalled();
    spy.mockRestore();
  });
  it('rechaza exceso de decimales sin truncar y acepta importes comerciales', () => {
    for (const v of [1.234567, 0, -1, Infinity, NaN, 1000000])
      expect(() => solesAMicro(v)).toThrow();
    expect(solesAMicro(1.23)).toBe(1230000);
    expect(solesAMicro(19.99)).toBe(19990000);
    expect(solesAMicro(999999.99)).toBe(999999990000);
  });
  it('conserva el identificador si vence la sesión después de una respuesta perdida', async () => {
    vi.mocked(invoke)
      .mockRejectedValueOnce(new Error('Respuesta perdida'))
      .mockRejectedValueOnce('NO_AUTENTICADO: Inicia sesión')
      .mockResolvedValueOnce({ status: 'OK' });
    const payload = { cantidad: 1 };
    await expect(ejecutarOperacionInventario('ajustar_stock_cmd', payload)).rejects.toThrow();
    const id = solicitudPendiente('ajustar_stock_cmd')?.id;
    await expect(ejecutarOperacionInventario('ajustar_stock_cmd', payload)).rejects.toThrow(
      'NO_AUTENTICADO'
    );
    expect(solicitudPendiente('ajustar_stock_cmd')?.id).toBe(id);
    configurarContextoInventario(1, 'PRUEBA');
    await ejecutarOperacionInventario('ajustar_stock_cmd', payload);
    expect(
      vi
        .mocked(invoke)
        .mock.calls.every((c) => (c[1] as Record<string, unknown>).idOperacion === id)
    ).toBe(true);
  });
  it('un resultado heredado PENDING no libera el identificador para repetir efectos', async () => {
    vi.mocked(invoke).mockRejectedValue('VALIDATION_ERROR: Resultado PENDING. Requiere revisión.');
    await expect(ejecutarOperacionInventario('ajustar_stock_cmd', { cantidad: 1 })).rejects.toThrow(
      'PENDING'
    );
    expect(solicitudPendiente('ajustar_stock_cmd')).not.toBeNull();
  });
});
