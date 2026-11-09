import '@testing-library/jest-dom/vitest';
import { act, cleanup, fireEvent, render, screen, within } from '@testing-library/react';
import { invoke } from '@tauri-apps/api/core';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { useAuth } from '../../../context/AuthContext';
import type { DashboardDTO } from '../../../services/dashboardService';
import { authDePrueba, sesionDePrueba } from '../../__tests__/authFixture';
import { DashboardPage } from '../DashboardPage';

vi.mock('@tauri-apps/api/core', () => ({ invoke: vi.fn() }));
vi.mock('../../../context/AuthContext', () => ({ useAuth: vi.fn() }));

const metricas: DashboardDTO = {
  ventas_del_dia: 189.5,
  comprobantes_emitidos: 1,
  ots_activas: 2,
  productos_stock_critico: 1,
  estado_caja: 'Abierta',
};

const respaldo = {
  configurada: true,
  ultima: '2026-11-06 23:00:00',
  vencida: true,
  recuperacion: { pendiente: false },
};

function permisos(...modulos: string[]) {
  return (modulo: string, accion: string) => accion === 'ver' && modulos.includes(modulo);
}

beforeEach(() => {
  vi.mocked(invoke).mockReset().mockImplementation(async (cmd) =>
    cmd === 'backup_operacion_cmd' ? respaldo : metricas
  );
});

afterEach(() => {
  cleanup();
  vi.restoreAllMocks();
});

describe('Dashboard de inicio', () => {
  it('distingue datos cargando de métricas reales y abre el módulo elegido', async () => {
    vi.mocked(useAuth).mockReturnValue(
      authDePrueba({
        tienePermiso: permisos(
          'BACKUP',
          'COLA_SUNAT',
          'OT',
          'POS',
          'FICHA_VEHICULAR',
          'INVENTARIO'
        ),
      })
    );
    let resolver!: (dto: DashboardDTO) => void;
    vi.mocked(invoke).mockImplementation((cmd) =>
      cmd === 'backup_operacion_cmd' ? Promise.resolve(respaldo)
        : new Promise<DashboardDTO>((resolve) => { resolver = resolve; })
    );
    const navegar = vi.fn();
    render(<DashboardPage setModuloActual={navegar} />);

    expect(screen.getByRole('heading', { name: 'Hola, Ana' })).toBeInTheDocument();
    const resumen = within(screen.getByRole('region', { name: 'Resumen de hoy' }));
    expect(resumen.getAllByText('Cargando…')).toHaveLength(4);
    expect(resumen.queryByText('S/ 0.00')).not.toBeInTheDocument();
    expect(invoke).toHaveBeenCalledWith('obtener_dashboard_cmd', { idUsuario: 7 });

    await act(async () => resolver(metricas));
    expect(resumen.queryByText('Cargando…')).not.toBeInTheDocument();
    expect(resumen.getByText('S/ 189.50')).toBeInTheDocument();
    expect(resumen.getByText('1 comprobante emitido')).toBeInTheDocument();
    expect(resumen.getByText('Turno en curso')).toBeInTheDocument();
    const accesos = within(screen.getByRole('navigation', { name: 'Accesos rápidos' }));
    fireEvent.click(accesos.getByRole('button', { name: 'Punto de Venta' }));
    expect(navegar).toHaveBeenCalledExactlyOnceWith('pos');
  });

  it('un mecánico solo ve sus órdenes y los accesos permitidos', async () => {
    vi.mocked(useAuth).mockReturnValue(
      authDePrueba({
        usuario: sesionDePrueba({ id_usuario: 3, rol: 'MECANICO' }),
        tienePermiso: permisos('OT'),
      })
    );
    const navegar = vi.fn();
    render(<DashboardPage setModuloActual={navegar} />);
    const ordenes = within(screen.getByRole('region', { name: 'Mis órdenes activas' }));
    expect(await ordenes.findByText('2')).toBeInTheDocument();
    expect(invoke).toHaveBeenCalledExactlyOnceWith('obtener_dashboard_cmd', { idUsuario: 3 });
    expect(screen.queryByRole('region', { name: 'Resumen de hoy' })).not.toBeInTheDocument();
    expect(screen.queryByText('S/ 189.50')).not.toBeInTheDocument();
    const accesos = within(screen.getByRole('navigation', { name: 'Accesos rápidos' }));
    expect(accesos.getAllByRole('button')).toHaveLength(1);
    fireEvent.click(accesos.getByRole('button', { name: 'Órdenes de Trabajo' }));
    expect(navegar).toHaveBeenCalledExactlyOnceWith('taller');
  });

  it('si falla la consulta muestra datos no disponibles sin bloquear los accesos', async () => {
    vi.spyOn(console, 'error').mockImplementation(() => {});
    vi.mocked(invoke).mockImplementation(async (cmd) => {
      if (cmd === 'obtener_dashboard_cmd') throw new Error('Sin respuesta');
      return respaldo;
    });
    vi.mocked(useAuth).mockReturnValue(
      authDePrueba({
        tienePermiso: permisos('BACKUP', 'COLA_SUNAT', 'INVENTARIO'),
      })
    );
    const navegar = vi.fn();
    render(<DashboardPage setModuloActual={navegar} />);
    const resumen = within(screen.getByRole('region', { name: 'Resumen de hoy' }));
    expect((await resumen.findAllByText('Dato no disponible')).length).toBe(4);
    expect(resumen.queryByText('S/ 0.00')).not.toBeInTheDocument();
    fireEvent.click(screen.getByRole('button', { name: 'Inventario y Stock' }));
    expect(navegar).toHaveBeenCalledExactlyOnceWith('inventario');
  });

  it('muestra la fecha y el estado del respaldo y abre Backup desde Inicio', async () => {
    vi.mocked(useAuth).mockReturnValue(authDePrueba({ tienePermiso: permisos('BACKUP') }));
    const navegar = vi.fn();
    render(<DashboardPage setModuloActual={navegar} />);
    const acceso = within(screen.getByRole('button', { name: 'Backup y Copias' }));
    expect(await acceso.findByText('Copia de más de 24 h')).toBeInTheDocument();
    expect(acceso.getByText('Última copia válida: 06/11/2026 · 23:00 (Lima)')).toBeInTheDocument();
    expect(invoke).toHaveBeenCalledWith('backup_operacion_cmd', { req: { accion: 'estado' } });
    fireEvent.click(screen.getByRole('button', { name: 'Backup y Copias' }));
    expect(navegar).toHaveBeenCalledExactlyOnceWith('backup');
  });

  it('abre Administración desde Inicio si la sesión es administrativa', async () => {
    vi.mocked(useAuth).mockReturnValue(authDePrueba({ tienePermiso: permisos('ADMIN') }));
    const navegar = vi.fn();
    render(<DashboardPage setModuloActual={navegar} />);
    const accesos = within(screen.getByRole('navigation', { name: 'Accesos rápidos' }));
    fireEvent.click(accesos.getByRole('button', { name: 'Administración' }));
    expect(navegar).toHaveBeenCalledExactlyOnceWith('administracion');
    await act(async () => {});
  });

  it('no muestra Administración a un cajero aunque figure el permiso', async () => {
    vi.mocked(useAuth).mockReturnValue(
      authDePrueba({ usuario: sesionDePrueba({ rol: 'CAJERO' }), tienePermiso: permisos('ADMIN', 'CAJA') })
    );
    render(<DashboardPage setModuloActual={vi.fn()} />);
    expect(screen.queryByRole('button', { name: 'Administración' })).not.toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Caja Chica' })).toBeInTheDocument();
    await act(async () => {});
  });

});
