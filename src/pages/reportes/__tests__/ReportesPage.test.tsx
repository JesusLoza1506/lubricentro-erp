import '@testing-library/jest-dom/vitest';
import { act, cleanup, fireEvent, render, screen, within } from '@testing-library/react';
import { invoke } from '@tauri-apps/api/core';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import { useAuth } from '../../../context/AuthContext';
import { authDePrueba, sesionDePrueba } from '../../__tests__/authFixture';
import { ReportesPage } from '../ReportesPage';
import type { Filtros, Pagina, Reporte } from '../reportesApi';

vi.mock('@tauri-apps/api/core', () => ({ invoke: vi.fn() }));
vi.mock('../../../context/AuthContext', () => ({ useAuth: vi.fn() }));

function resultado(reporte: Reporte = 'R01', filtros: Filtros = { periodo: 'este_mes' }): Pagina {
  return {
    id: 'corte-vigente',
    reporte,
    filtros,
    ambiente: 'PRUEBA',
    hora: '2026-11-09 15:00:00',
    criterio: 'Fecha económica',
    rango: ['2026-11-01', '2026-11-09'],
    filtros_texto: 'Sin filtros adicionales',
    usuarios: [],
    columnas: ['Fecha'],
    filas: [{ celdas: [{ tipo: 'texto', valor: '2026-11-08' }], origenes: [] }],
    pagina: 1,
    total_filas: 21,
    disponibles: null,
    totales: [['Neto (S/)', { tipo: 'numero', valor: '63.00' }]],
  };
}
beforeEach(() => {
  vi.mocked(useAuth).mockReturnValue(authDePrueba());
  vi.mocked(invoke).mockReset().mockResolvedValue(null);
});
afterEach(cleanup);

it('consulta R01 al entrar y conserva el corte al fallar un cambio de reporte', async () => {
  vi.mocked(invoke)
    .mockResolvedValueOnce(resultado())
    .mockRejectedValueOnce('VALIDATION_ERROR: Consulta de prueba fallida');
  render(<ReportesPage onOrigen={vi.fn()} />);
  await screen.findByRole('heading', { name: 'Ventas por periodo' });
  expect(invoke).toHaveBeenCalledWith('reportes_operacion_cmd', {
    req: { accion: 'consultar', reporte: 'R01', filtros: { periodo: 'este_mes' } },
  });
  fireEvent.change(screen.getByLabelText('Periodo'), { target: { value: 'ayer' } });
  expect(screen.getByRole('button', { name: 'Exportar Excel' })).toBeDisabled();
  fireEvent.change(screen.getByLabelText('Reporte'), { target: { value: 'R02' } });
  expect(await screen.findByRole('alert')).toHaveTextContent('Consulta de prueba fallida');
  expect(invoke).toHaveBeenLastCalledWith('reportes_operacion_cmd', {
    req: { accion: 'consultar', reporte: 'R02', filtros: { periodo: 'ayer' } },
  });
  expect(screen.getByText('2026-11-09 15:00:00 (Lima)')).toBeInTheDocument();
  expect(screen.getByRole('button', { name: 'Exportar PDF' })).toBeDisabled();
  vi.mocked(invoke).mockResolvedValueOnce(resultado('R02', { periodo: 'ayer' }));
  fireEvent.click(screen.getByRole('button', { name: 'Reintentar' }));
  expect(await screen.findByRole('heading', { name: 'Cobros y devoluciones' })).toBeInTheDocument();
  expect(screen.getByRole('button', { name: 'Exportar PDF' })).toBeEnabled();
});

it('vincula segunda página, detalle y origen al mismo corte; exportar usa ese ID', async () => {
  const origen = { destino: 'pos', id: 7 };
  const onOrigen = vi.fn();
  vi.mocked(invoke).mockImplementation(async (_cmd, args) => {
    const req = (args as { req: { accion: string } }).req;
    if (req.accion === 'pagina') return { ...resultado(), pagina: 2 };
    if (req.accion === 'detalle')
      return {
        columnas: ['Venta'],
        filas: [{ celdas: [{ tipo: 'numero', valor: '7' }], origenes: [origen] }],
      };
    if (req.accion === 'origen') return origen;
    if (req.accion === 'exportar') return { guardado: false };
    return resultado();
  });
  render(<ReportesPage onOrigen={onOrigen} />);
  fireEvent.click(await screen.findByRole('button', { name: 'Siguiente' }));
  await screen.findByText('Página 2 de 2');
  fireEvent.click(screen.getByRole('button', { name: 'Ver detalle' }));
  const tabla = await screen.findByRole('table', { name: 'Filas del detalle' });
  expect(screen.getByRole('heading', { name: 'Detalle al corte' })).toHaveFocus();
  expect(invoke).toHaveBeenLastCalledWith('reportes_operacion_cmd', {
    req: { accion: 'detalle', id: 'corte-vigente', fila: 20 },
  });
  await act(async () =>
    fireEvent.click(within(tabla).getByRole('button', { name: 'Punto de Venta' }))
  );
  expect(onOrigen).toHaveBeenCalledWith(origen);
  expect(invoke).toHaveBeenLastCalledWith('reportes_operacion_cmd', {
    req: { accion: 'origen', id: 'corte-vigente', fila: 20, detalle: 0, indice: 0 },
  });
  fireEvent.click(screen.getByRole('button', { name: 'Volver al resultado' }));
  expect(screen.getByText('Página 2 de 2')).toBeVisible();
  await act(async () => fireEvent.click(screen.getByRole('button', { name: 'Exportar Excel' })));
  expect(invoke).toHaveBeenLastCalledWith('reportes_operacion_cmd', {
    req: {
      accion: 'exportar',
      id: 'corte-vigente',
      reporte: 'R01',
      filtros: { periodo: 'este_mes' },
      formato: 'xlsx',
    },
  });
  expect(screen.queryByText('Reporte Excel guardado.')).not.toBeInTheDocument();
});

it('bloquea la vista a roles operativos y oculta el corte si vence la sesión', async () => {
  const auth = authDePrueba({ usuario: sesionDePrueba({ rol: 'CAJERO' }) });
  vi.mocked(useAuth).mockReturnValue(auth);
  const onOrigen = vi.fn();
  const { rerender } = render(<ReportesPage onOrigen={onOrigen} />);
  await act(async () => {});
  expect(invoke).not.toHaveBeenCalled();
  const admin = authDePrueba();
  vi.mocked(useAuth).mockReturnValue(admin);
  vi.mocked(invoke)
    .mockResolvedValueOnce(resultado())
    .mockRejectedValueOnce('NO_AUTENTICADO: Sesión vencida');
  rerender(<ReportesPage onOrigen={onOrigen} />);
  fireEvent.click(await screen.findByRole('button', { name: 'Siguiente' }));
  expect(await screen.findByRole('alert')).toHaveTextContent('Sesión vencida');
  expect(screen.queryByRole('region', { name: 'Resultado del reporte' })).not.toBeInTheDocument();
  expect(admin.verificarSesion).toHaveBeenCalledOnce();
});
