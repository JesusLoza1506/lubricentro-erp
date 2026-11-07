import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import { cleanup, fireEvent, render, screen, waitFor } from '@testing-library/react';
import { invoke } from '@tauri-apps/api/core';
import { PosPage } from '../PosPage';
vi.mock('@tauri-apps/api/core', () => ({ invoke: vi.fn() }));
vi.mock('@tauri-apps/plugin-opener', () => ({ openUrl: vi.fn() }));
const dirty = vi.fn();
vi.mock('../../../context/AuthContext', () => ({
  useAuth: () => ({
    usuario: { id_usuario: 2, nombre_completo: 'Cajero de prueba', rol: 'CAJERO' },
    ambienteActual: 'PRUEBA',
    setHayCambiosSinGuardar: dirty,
  }),
}));
const producto = {
  tipo: 'PRODUCTO',
  id: 1,
  codigo: 'FIL-01',
  nombre: 'Filtro de aceite',
  unidad: 'UND',
  precio_cent: 2000,
  stock: 10,
};
const detalle = {
  id: 1,
  id_ot: null,
  id_emisor: 2,
  emisor: 'Cajero de prueba',
  caja: 'CAJA-1',
  fecha: '2026-11-06 12:00:00',
  tipo: 'BOLETA',
  receptor: { nombre: 'Cliente varios' },
  bruto_cent: 2000,
  descuento_cent: 0,
  base_cent: 1695,
  igv_cent: 305,
  total_cent: 2000,
  devuelto_cent: 0,
  lineas: [],
  pagos: [],
  documentos: [],
  devoluciones: [],
};
let incierto = false;
beforeEach(() => {
  localStorage.clear();
  vi.clearAllMocks();
  incierto = false;
  vi.mocked(invoke).mockImplementation(async (comando, args) => {
    const a = args as { accion?: string; payload?: any };
    if (comando === 'pos_documento_cmd') throw 'VALIDATION_ERROR: Sin conexión de prueba';
    if (a.accion === 'inicio')
      return {
        ambiente: 'PRUEBA',
        empresa: 'Lubricentro',
        configurada: true,
        caja_propia: 1,
        caja: { id: 1, codigo: 'CAJA-1', titular: 'Cajero', propia: 1, efectivo_cent: 10000 },
        hoy: '2026-11-06',
        series: [],
      };
    if (a.accion === 'catalogo') return { filas: [producto], total: 1 };
    if (a.accion === 'cotizar')
      return {
        ...detalle,
        lineas: [
          {
            ...producto,
            id_producto: 1,
            id_servicio: null,
            cantidad: 1,
            total_cent: 2000,
            base_cent: 1695,
          },
        ],
      };
    if (a.accion === 'confirmar') {
      if (incierto) {
        incierto = false;
        throw 'DATABASE_ERROR: Respuesta incierta';
      }
      return { id_venta: 1, id_documento: 1, total_cent: 2000 };
    }
    if (a.accion === 'detalle') return detalle;
    if (a.accion === 'listado') return { filas: [], total: 0 };
    throw new Error(`Comando inesperado: ${comando} ${a.accion}`);
  });
});
afterEach(cleanup);
async function prepararCarrito() {
  render(<PosPage />);
  fireEvent.click(await screen.findByRole('button', { name: 'Agregar Filtro de aceite' }));
  await waitFor(() =>
    expect(
      (screen.getByRole('button', { name: 'Cobrar venta' }) as HTMLButtonElement).disabled
    ).toBe(false)
  );
  fireEvent.click(screen.getByRole('button', { name: 'Cobrar venta' }));
}
it('integra catálogo, confirmación y aviso de venta guardada aunque falle NubeFact', async () => {
  await prepararCarrito();
  fireEvent.click(screen.getByRole('button', { name: 'Confirmar cobro' }));
  await screen.findByRole('dialog', { name: 'Venta V-1' });
  expect(screen.getByText(/Venta guardada; documento pendiente/)).toBeTruthy();
  const ventas = vi.mocked(invoke).mock.calls.filter((c) => (c[1] as any)?.accion === 'confirmar');
  expect(ventas).toHaveLength(1);
  expect(ventas[0][1]).toMatchObject({
    payload: {
      id_cliente: 1,
      tipo_comprobante: 'BOLETA',
      lineas: [{ tipo: 'PRODUCTO', id: 1, cantidad: 1, precio_esperado_cent: 2000 }],
      pagos: [{ medio: 'EFECTIVO', monto_cent: 2000, recibido_cent: 2000 }],
    },
  });
});
it('reintenta desde el cobro con el mismo ID después de una respuesta incierta', async () => {
  await prepararCarrito();
  incierto = true;
  fireEvent.click(screen.getByRole('button', { name: 'Confirmar cobro' }));
  await screen.findByText('Respuesta incierta');
  fireEvent.click(screen.getByRole('button', { name: 'Confirmar cobro' }));
  await screen.findByRole('dialog', { name: 'Venta V-1' });
  const llamadas = vi
    .mocked(invoke)
    .mock.calls.filter((c) => (c[1] as any)?.accion === 'confirmar');
  expect(llamadas).toHaveLength(2);
  expect((llamadas[0][1] as any).payload.id_operacion).toBe(
    (llamadas[1][1] as any).payload.id_operacion
  );
});

it('permite retirar la captura cuando falla la cotización inicial', async () => {
  const original = vi.mocked(invoke).getMockImplementation()!;
  vi.mocked(invoke).mockImplementation(async (c, a) => {
    if ((a as any)?.accion === 'cotizar') throw 'VALIDATION_ERROR: Stock insuficiente';
    return original(c, a);
  });
  render(<PosPage />);
  fireEvent.click(await screen.findByRole('button', { name: 'Agregar Filtro de aceite' }));
  await screen.findByText('Stock insuficiente');
  fireEvent.click(screen.getByRole('button', { name: 'Quitar Filtro de aceite' }));
  await waitFor(() =>
    expect(screen.queryByRole('button', { name: 'Quitar Filtro de aceite' })).toBeNull()
  );
});
