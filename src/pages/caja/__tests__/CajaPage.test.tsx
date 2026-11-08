import { cleanup, fireEvent, render, screen, waitFor, within } from '@testing-library/react';
import { invoke } from '@tauri-apps/api/core';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { CajaPage } from '../CajaPage';
import type { CajaDetalle, CajaInicio } from '../../../types/caja';

const marcar = vi.hoisted(() => vi.fn());
vi.mock('../../../context/AuthContext', () => ({
  useAuth: () => ({
    usuario: { id_usuario: 2, rol: 'CAJERO', nombre_completo: 'Cajero de prueba' },
    ambienteActual: 'PRUEBA',
    setHayCambiosSinGuardar: marcar,
  }),
}));
vi.mock('@tauri-apps/api/core', () => ({ invoke: vi.fn() }));
const llamadas = vi.mocked(invoke);
const totales = { cobros_cent: 0, reintegros_cent: 0, neto_cent: 0 };
const sinTurno: CajaInicio = {
  ambiente: 'PRUEBA',
  hoy: '2026-11-07',
  ocupada: false,
  puede_abrir: true,
  recuperacion: false,
  fondo_sugerido_cent: 0,
  detalle: null,
  responsables: [],
};
function turno(version: number, esperado: number): CajaDetalle {
  return {
    puede_mover: true,
    puede_aclarar: false,
    modo_cierre: 'NORMAL',
    turno: {
      id_caja: 1,
      codigo_caja: 'CAJA-1',
      id_usuario: 2,
      titular: 'Cajero de prueba',
      ambiente: 'PRUEBA',
      monto_apertura_cent: 10_000,
      estado: 'ABIERTA',
      por_conciliar: false,
      fecha_apertura: '2026-11-07 12:00:00',
      fecha_cierre: null,
      quien_cerro: null,
      tipo_cierre: null,
      motivo_cierre: null,
      efectivo_esperado_cent: null,
      efectivo_contado_cent: null,
      diferencia_efectivo_cent: null,
      fondo_siguiente_cent: null,
      retiro_cierre_cent: null,
      aclaracion: null,
      autor_aclaracion: null,
      fecha_aclaracion: null,
      version,
    },
    resumen: {
      efectivo_esperado_cent: esperado,
      efectivo: totales,
      tarjeta: totales,
      yape: totales,
      plin: totales,
      transferencia: totales,
      ingresos_manuales_cent: 0,
      egresos_manuales_cent: 0,
    },
  };
}

beforeEach(() => {
  marcar.mockReset();
  llamadas.mockReset();
  localStorage.clear();
  if (!HTMLDialogElement.prototype.showModal)
    Object.defineProperty(HTMLDialogElement.prototype, 'showModal', {
      configurable: true,
      value(this: HTMLDialogElement) {
        this.setAttribute('open', '');
      },
    });
  if (!HTMLDialogElement.prototype.close)
    Object.defineProperty(HTMLDialogElement.prototype, 'close', {
      configurable: true,
      value(this: HTMLDialogElement) {
        this.removeAttribute('open');
      },
    });
});
afterEach(() => {
  cleanup();
  vi.restoreAllMocks();
});

describe('Caja Chica', () => {
  it('muestra solo ocupación para una caja ajena y no consulta sus movimientos', async () => {
    llamadas.mockResolvedValue({ ...sinTurno, ocupada: true, puede_abrir: false });
    render(<CajaPage onVenta={vi.fn()} />);
    await screen.findByRole('heading', { name: 'Caja ocupada' });
    expect(screen.queryByRole('button', { name: 'Abrir turno' })).toBeNull();
    expect(screen.queryByText('Efectivo del turno')).toBeNull();
    expect(llamadas).toHaveBeenCalledTimes(1);
  });

  it('recupera una apertura con el mismo identificador después de volver al módulo', async () => {
    let intentos = 0;
    const enviadas: object[] = [];
    llamadas.mockImplementation(async (_cmd, raw) => {
      const args = raw as { accion: string; payload: object } | undefined;
      if (args?.accion === 'inicio') return sinTurno;
      if (args?.accion === 'abrir') {
        enviadas.push(args.payload as object);
        if (++intentos === 1) throw new Error('Respuesta incierta.');
        return { id_caja: 1 };
      }
      throw new Error('Acción inesperada');
    });
    const app = render(<CajaPage onVenta={vi.fn()} />);
    fireEvent.click(await screen.findByRole('button', { name: 'Abrir turno' }));
    const modal = screen.getByRole('dialog');
    expect(marcar).not.toHaveBeenCalledWith(true);
    fireEvent.change(within(modal).getByLabelText('Fondo de apertura (S/)'), {
      target: { value: '100' },
    });
    expect(marcar).toHaveBeenCalledWith(true);
    fireEvent.click(within(modal).getByRole('button', { name: 'Abrir turno' }));
    await screen.findByText('Respuesta incierta.');
    app.unmount();
    render(<CajaPage onVenta={vi.fn()} />);
    fireEvent.click(await screen.findByRole('button', { name: 'Reintentar operación' }));
    await screen.findByText('Turno abierto correctamente.');
    expect(enviadas).toHaveLength(2);
    expect(enviadas[1]).toEqual(enviadas[0]);
    expect(enviadas[0]).toMatchObject({
      monto_apertura_cent: 10_000,
      id_operacion: expect.any(String),
    });
  });

  it('conserva el contado ante una versión desactualizada y exige motivo por un céntimo', async () => {
    let vigente = turno(0, 10_000);
    const enviadas: object[] = [];
    llamadas.mockImplementation(async (_cmd, raw) => {
      const args = raw as { accion: string; payload: object } | undefined;
      switch (args?.accion) {
        case 'inicio':
          return { ...sinTurno, ocupada: true, puede_abrir: false, detalle: vigente };
        case 'detalle':
          return vigente;
        case 'movimientos':
          return { datos: [], total: 0, pagina: 1, por_pagina: 20 };
        case 'cerrar':
          enviadas.push(args.payload as object);
          if (enviadas.length === 1) {
            vigente = turno(1, 10_001);
            throw 'VALIDATION_ERROR: RESUMEN_DESACTUALIZADO: Revisa el resumen actualizado.';
          }
          return { ...vigente, modo_cierre: null, turno: { ...vigente.turno, estado: 'CERRADA' } };
        default:
          throw new Error('Acción inesperada');
      }
    });
    render(<CajaPage onVenta={vi.fn()} />);
    fireEvent.click(await screen.findByRole('button', { name: 'Cerrar turno' }));
    const modal = screen.getByRole('dialog');
    const contado = within(modal).getByLabelText('Efectivo contado (S/)') as HTMLInputElement;
    expect(contado.value).toBe('');
    fireEvent.change(contado, { target: { value: '100' } });
    fireEvent.click(within(modal).getByRole('button', { name: 'Confirmar cierre' }));
    const motivo = await within(modal).findByLabelText('Motivo obligatorio');
    expect(contado.value).toBe('100');
    fireEvent.click(within(modal).getByRole('button', { name: 'Confirmar cierre' }));
    expect(enviadas).toHaveLength(1);
    fireEvent.change(motivo, { target: { value: 'Faltante en el conteo' } });
    fireEvent.click(within(modal).getByRole('button', { name: 'Confirmar cierre' }));
    await waitFor(() => expect(enviadas).toHaveLength(2));
    expect(enviadas[1]).toMatchObject({
      version_esperada: 1,
      efectivo_contado_cent: 10_000,
      motivo: 'Faltante en el conteo',
    });
  });
});
