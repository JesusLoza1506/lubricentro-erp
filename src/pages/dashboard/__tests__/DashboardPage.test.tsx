import '@testing-library/jest-dom/vitest';
import { StrictMode } from 'react';
import { act, cleanup, fireEvent, render, screen, waitFor, within } from '@testing-library/react';
import { invoke } from '@tauri-apps/api/core';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { useAuth } from '../../../context/AuthContext';
import type { DashboardDTO } from '../../../services/dashboardService';
import { authDePrueba, sesionDePrueba } from '../../__tests__/authFixture';
import { DashboardPage } from '../DashboardPage';

vi.mock('@tauri-apps/api/core', () => ({ invoke: vi.fn() }));
vi.mock('../../../context/AuthContext', () => ({ useAuth: vi.fn() }));

const base = { ambiente: 'PRUEBA', fecha: '2026-11-10', actualizado_en: '2026-11-10 14:42:18' };
const admin: DashboardDTO = {
  ...base, rol: 'ADMINISTRADOR', ventas_netas_hoy_cent: 7000, trabajos_terminados_hoy: 8, mediana_cobro_seg: 2040,
  ventas_por_dia: [{ periodo: '2026-11-09', valor: -3000 }, { periodo: base.fecha, valor: 7000 }],
  trabajos_por_hora: [{ periodo: '2026-11-10 14', valor: 8 }],
};
const mecanico: DashboardDTO = {
  ...base, rol: 'MECANICO', trabajos_terminados_hoy: 4, trabajos_abiertos: 3, mediana_trabajo_seg: 2880,
  por_estado: { en_espera: 2, en_proceso: 1, finalizadas_hoy: 4 },
  trabajos_por_dia: [{ periodo: base.fecha, valor: 4 }],
};
const cajero: DashboardDTO = {
  ...base, rol: 'CAJERO', turno: { estado: 'PROPIO', apertura: '2026-11-09 23:00:00' },
  actividad_turno: { cobros: 2, cobros_por_hora: [
    { periodo: '2026-11-09 23', valor: 1 }, { periodo: '2026-11-10 00', valor: 1 },
  ] },
  pendientes: { total: 6, mayor_espera_seg: 5640, menos_30: 3, entre_30_y_60: 1, mas_60: 2 },
};

beforeEach(() => {
  vi.mocked(useAuth).mockReturnValue(authDePrueba());
  vi.mocked(invoke).mockReset().mockResolvedValue(admin);
});
afterEach(cleanup);

describe('Inicio Dashboard v7.1', () => {
  it('carga el corte una vez, muestra tres tarjetas y dos gráficos y explora un día negativo localmente', async () => {
    let resolver!: (dto: DashboardDTO) => void;
    vi.mocked(invoke).mockImplementation(() => new Promise<DashboardDTO>((ok) => { resolver = ok; }));
    render(<StrictMode><DashboardPage /></StrictMode>);
    expect(screen.getByRole('status')).toHaveTextContent('Cargando resumen');
    expect(screen.queryByText('Ventas netas hoy')).not.toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Actualizar' })).toBeDisabled();
    await waitFor(() => expect(invoke).toHaveBeenCalledExactlyOnceWith('obtener_dashboard_cmd', { sessionId: 'sesion-prueba' }));
    await act(async () => resolver(admin));
    expect(within(screen.getByRole('region', { name: 'Resumen de hoy' })).getByText('34 min')).toBeInTheDocument();
    const graficos = within(screen.getByRole('region', { name: 'Actividad en gráficos' }));
    expect(graficos.getAllByRole('region')).toHaveLength(2);
    expect(screen.getAllByRole('term')).toHaveLength(3);
    fireEvent.click(graficos.getByRole('button', { name: /09\/11.*30/ }));
    expect(graficos.getByRole('button', { name: /09\/11.*30/ })).toHaveAttribute('aria-pressed', 'true');
    expect(graficos.getByText(/−S\/ 30/)).toBeInTheDocument();
    expect(invoke).toHaveBeenCalledTimes(1);
    expect(screen.queryByText(/Requiere atención|Accesos rápidos|Nueva OT/i)).not.toBeInTheDocument();
  });

  it('muestra al mecánico su jornada y sus finalizaciones sin exponer cifras de ventas', async () => {
    vi.mocked(useAuth).mockReturnValue(authDePrueba({ usuario: sesionDePrueba({ rol: 'MECANICO' }) }));
    vi.mocked(invoke).mockResolvedValue(mecanico);
    render(<DashboardPage />);
    expect(await screen.findByText('48 min')).toBeInTheDocument();
    expect(screen.getByRole('region', { name: 'Mi jornada por estado' })).toBeInTheDocument();
    expect(screen.getByRole('region', { name: 'Mis trabajos terminados por día' })).toBeInTheDocument();
    expect(screen.queryByText('Ventas netas hoy')).not.toBeInTheDocument();
  });

  it('distingue fechas en los cobros de un turno que cruza medianoche', async () => {
    vi.mocked(useAuth).mockReturnValue(authDePrueba({ usuario: sesionDePrueba({ rol: 'CAJERO' }) }));
    vi.mocked(invoke).mockResolvedValue(cajero);
    render(<DashboardPage />);
    expect(await screen.findByText('1 h 34 min')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /09\/11 · 23:00/ })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /10\/11 · 00:00/ })).toBeInTheDocument();
    expect(screen.getByText('Desde las 23:00 del 09/11 · Lima')).toBeInTheDocument();
  });

  it('distingue caja ajena, no aplica y ausencia real de pendientes', async () => {
    vi.mocked(useAuth).mockReturnValue(authDePrueba({ usuario: sesionDePrueba({ rol: 'CAJERO' }) }));
    vi.mocked(invoke).mockResolvedValue({ ...cajero, turno: { estado: 'AJENO' }, actividad_turno: null,
      pendientes: { total: 0, mayor_espera_seg: null, menos_30: 0, entre_30_y_60: 0, mas_60: 0 } });
    render(<DashboardPage />);
    expect(await screen.findByText('Caja a cargo de otro usuario')).toBeInTheDocument();
    expect(screen.getByText('No aplica sin un turno propio abierto.')).toBeInTheDocument();
    expect(screen.getByText('Sin pendientes')).toBeInTheDocument();
  });

  it('reintenta un fallo inicial y conserva la hora previa si falla un refresco', async () => {
    vi.mocked(invoke).mockRejectedValueOnce('DB_LOCK').mockResolvedValueOnce(admin).mockRejectedValueOnce('DB_LOCK');
    render(<DashboardPage />);
    expect(await screen.findByRole('alert')).toHaveTextContent('No se pudo cargar Inicio');
    expect(screen.queryByText('Ventas netas hoy')).not.toBeInTheDocument();
    fireEvent.click(screen.getByRole('button', { name: 'Reintentar' }));
    expect(await screen.findByText('Ventas netas hoy')).toBeInTheDocument();
    fireEvent.click(screen.getByRole('button', { name: 'Actualizar' }));
    expect(await screen.findByRole('alert')).toHaveTextContent('Sin actualizar');
    expect(screen.getByText(/14:42:18/)).toBeInTheDocument();
    expect(invoke).toHaveBeenCalledTimes(3);
  });

  it('descarta una respuesta tardía tras cambiar de usuario y no publica otra combinación de rol y ambiente', async () => {
    let resolver!: (dato: DashboardDTO) => void;
    vi.mocked(invoke).mockImplementationOnce(() => new Promise<DashboardDTO>((ok) => { resolver = ok; }))
      .mockResolvedValueOnce({ ...mecanico, ambiente: 'PRODUCCION' }).mockResolvedValueOnce(mecanico);
    const { rerender } = render(<DashboardPage />);
    await waitFor(() => expect(invoke).toHaveBeenCalledTimes(1));
    vi.mocked(useAuth).mockReturnValue(authDePrueba({ usuario: sesionDePrueba({ session_id: 'sesion-nueva', rol: 'MECANICO' }) }));
    rerender(<DashboardPage />);
    expect(await screen.findByRole('alert')).toBeInTheDocument();
    await act(async () => resolver(admin));
    expect(screen.queryByText('Ventas netas hoy')).not.toBeInTheDocument();
    expect(screen.queryByText('48 min')).not.toBeInTheDocument();
    fireEvent.click(screen.getByRole('button', { name: 'Reintentar' }));
    expect(await screen.findByText('48 min')).toBeInTheDocument();
    expect(invoke).toHaveBeenLastCalledWith('obtener_dashboard_cmd', { sessionId: 'sesion-nueva' });
  });

  it('vuelve a consultar al salir y regresar a Inicio', async () => {
    vi.mocked(invoke).mockResolvedValueOnce(admin).mockResolvedValueOnce({
      ...admin, actualizado_en: '2026-11-10 15:00:00',
    });
    const primeraVisita = render(<DashboardPage />);
    expect(await screen.findByText(/14:42:18/)).toBeInTheDocument();
    primeraVisita.unmount();
    render(<DashboardPage />);
    expect(await screen.findByText(/15:00:00/)).toBeInTheDocument();
    expect(invoke).toHaveBeenCalledTimes(2);
  });

  it('oculta el corte al expirar la sesión y solicita verificar la sesión real', async () => {
    const verificarSesion = vi.fn().mockResolvedValue(undefined);
    vi.mocked(useAuth).mockReturnValue(authDePrueba({ verificarSesion }));
    vi.mocked(invoke).mockResolvedValueOnce(admin).mockRejectedValueOnce('NO_AUTENTICADO: venció');
    render(<DashboardPage />);
    expect(await screen.findByText('Ventas netas hoy')).toBeInTheDocument();
    fireEvent.click(screen.getByRole('button', { name: 'Actualizar' }));
    expect(await screen.findByRole('alert')).toBeInTheDocument();
    expect(verificarSesion).toHaveBeenCalledOnce();
    expect(screen.queryByText('Ventas netas hoy')).not.toBeInTheDocument();
  });
});
