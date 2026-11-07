import { render, screen, fireEvent, waitFor, within } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import '@testing-library/jest-dom';
import { OrdenesPage } from '../OrdenesPage';
import { useAuth } from '../../../context/AuthContext';
import { ordenesService } from '../../../services/ordenesService';
import { inventarioService } from '../../../services/inventarioService';

vi.mock('../../../context/AuthContext');
vi.mock('../../../services/ordenesService');
vi.mock('../../../services/inventarioService');

const mockUsuarioAdmin = {
  id_usuario: 1,
  nombre_completo: 'Administrador Pruebas',
  rol: 'ADMINISTRADOR',
};

const mockUsuarioMecanico = {
  id_usuario: 2,
  nombre_completo: 'Mecánico Pruebas',
  rol: 'MECANICO',
};

const mockUsuarioCajero = {
  id_usuario: 3,
  nombre_completo: 'Cajero Pruebas',
  rol: 'CAJERO',
};

const fechaIngreso = '2026-11-06T10:00:00';

const mockOrdenes = [
  {
    id_ot: 101,
    codigo_ot: 'OT-000101',
    placa: 'ABC-123',
    id_mecanico: 2,
    nombre_mecanico: 'Mecánico Pruebas',
    id_zanja_ejecucion: 1,
    zanja: 1,
    estado: 'EN_PROCESO' as const,
    kilometraje_ingreso: 50000,
    proximo_kilometraje: 55000,
    tipo_aceite: 'MINERAL' as const,
    fecha_ingreso: fechaIngreso,
    servicios: [],
    insumos: [],
    total_servicios_cent: 0,
    total_insumos_cent: 0,
    total_general_cent: 0,
    total_general_soles: 0,
  },
  {
    id_ot: 102,
    codigo_ot: 'OT-000102',
    placa: 'XYZ-999',
    id_mecanico: 4,
    nombre_mecanico: 'Otro Mecánico',
    id_zanja_ejecucion: 2,
    zanja: 2,
    estado: 'EN_ESPERA' as const,
    kilometraje_ingreso: 30000,
    proximo_kilometraje: 35000,
    tipo_aceite: 'SINTETICO' as const,
    fecha_ingreso: fechaIngreso,
    servicios: [],
    insumos: [],
    total_servicios_cent: 0,
    total_insumos_cent: 0,
    total_general_cent: 0,
    total_general_soles: 0,
  },
  {
    id_ot: 103,
    codigo_ot: 'HIST-100',
    placa: 'HIST-100',
    id_mecanico: 2,
    nombre_mecanico: 'Mecánico Pruebas',
    id_zanja_ejecucion: undefined,
    zanja: undefined,
    estado: 'FINALIZADO' as const,
    kilometraje_ingreso: 60000,
    proximo_kilometraje: 65000,
    tipo_aceite: 'MINERAL' as const,
    fecha_ingreso: fechaIngreso,
    servicios: [],
    insumos: [],
    total_servicios_cent: 0,
    total_insumos_cent: 0,
    total_general_cent: 0,
    total_general_soles: 0,
  },
];

describe('OrdenesPage - Batería Frontend FASE 4 (Vitest React)', () => {
  beforeEach(() => {
    vi.clearAllMocks();

    (ordenesService.listarMecanicos as any) = vi.fn().mockResolvedValue([
      { id_usuario: 2, nombre_completo: 'Mecánico Pruebas' },
      { id_usuario: 4, nombre_completo: 'Otro Mecánico' },
    ]);
    (ordenesService.listarServicios as any) = vi.fn().mockResolvedValue([]);
    (inventarioService.listarProductosPublicos as any) = vi.fn().mockResolvedValue([]);

    (ordenesService.obtenerTableroZanjas as any) = vi.fn().mockResolvedValue({
      zanjas: [
        { id_zanja: 1, activa: 1, ocupada: true, ot_actual: mockOrdenes[0] },
        { id_zanja: 2, activa: 1, ocupada: false, ot_actual: null },
      ],
      cola_espera: [mockOrdenes[1]],
    });

    (ordenesService.listarOrdenesTrabajoPaginado as any) = vi.fn().mockResolvedValue({
      items: mockOrdenes,
      total_registros: mockOrdenes.length,
      pagina_actual: 1,
      total_paginas: 1,
    });
  });

  it('muestra las zanjas disponibles y ocupadas junto con la cola de espera', async () => {
    (useAuth as any).mockReturnValue({ usuario: mockUsuarioAdmin, tienePermiso: () => true });
    render(<OrdenesPage />);
    const tablero = within(await screen.findByRole('region', { name: 'Zanjas de trabajo' }));
    expect(tablero.getByText('1 disponible')).toBeInTheDocument();
    expect(tablero.getByText('1 ocupada')).toBeInTheDocument();
    expect(tablero.getByText('ABC-123')).toBeInTheDocument();
    const cola = within(screen.getByRole('region', { name: 'Vehículos en espera' }));
    expect(cola.getByText('XYZ-999')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: '+ Recepcionar vehículo' })).toBeEnabled();
  });

  it('2. Debe ocultar el botón de crear OT si el usuario no tiene permisos', async () => {
    (useAuth as any).mockReturnValue({
      usuario: mockUsuarioMecanico,
      tienePermiso: () => false,
    });
    (ordenesService.listarOrdenesTrabajo as any) = vi.fn().mockResolvedValue(mockOrdenes);

    render(<OrdenesPage />);

    await waitFor(() => {
      expect(screen.queryByText('+ Nueva OT')).not.toBeInTheDocument();
      expect(screen.queryByText('+ Recepcionar Vehículo')).not.toBeInTheDocument();
    });
  });

  it('3. Debe abrir el modal de detalle al interactuar con una orden', async () => {
    (useAuth as any).mockReturnValue({
      usuario: mockUsuarioAdmin,
      tienePermiso: () => true,
    });
    (ordenesService.listarOrdenesTrabajo as any) = vi.fn().mockResolvedValue(mockOrdenes);

    render(<OrdenesPage />);

    await waitFor(() => {
      expect(screen.getByText('ABC-123')).toBeInTheDocument();
    });

    const tablero = within(screen.getByRole('region', { name: 'Zanjas de trabajo' }));
    fireEvent.click(tablero.getByRole('button', { name: 'Abrir atención' }));
    const detalle = within(await screen.findByRole('dialog', { name: 'Detalle de orden de trabajo' }));
    expect(detalle.getByRole('heading', { name: 'OT: OT-000101' })).toBeInTheDocument();
    expect(detalle.getByText('ABC-123')).toBeInTheDocument();
    fireEvent.click(detalle.getByRole('button', { name: 'Cerrar detalle' }));
    expect(screen.queryByRole('dialog', { name: 'Detalle de orden de trabajo' })).not.toBeInTheDocument();
  });

  it('envía el filtro Mis órdenes cuando el mecánico lo selecciona', async () => {
    (useAuth as any).mockReturnValue({ usuario: mockUsuarioMecanico, tienePermiso: () => true });
    render(<OrdenesPage />);
    fireEvent.click(screen.getByRole('button', { name: 'Historial de órdenes' }));
    const propias = screen.getByRole('button', { name: 'Mis órdenes' });
    fireEvent.click(propias);
    expect(propias).toHaveAttribute('aria-pressed', 'true');
    await waitFor(() => expect(ordenesService.listarOrdenesTrabajoPaginado).toHaveBeenLastCalledWith(
      expect.objectContaining({ pagina: 1, limite: 20, solo_propias: true })
    ));
    expect(await screen.findByRole('region', { name: 'Resultados de órdenes' })).toBeInTheDocument();
  });

  it('consulta el historial y muestra la fecha local registrada para cada orden', async () => {
    (useAuth as any).mockReturnValue({ usuario: mockUsuarioAdmin, tienePermiso: () => true });
    render(<OrdenesPage />);
    const pestaña = screen.getByRole('button', { name: 'Historial de órdenes' });
    expect(ordenesService.listarOrdenesTrabajoPaginado).not.toHaveBeenCalled();
    fireEvent.click(pestaña);
    expect(pestaña).toHaveAttribute('aria-current', 'page');
    const historial = within(await screen.findByRole('region', { name: 'Resultados de órdenes' }));
    const fila = within(historial.getByRole('row', { name: /OT-000101/ }));
    expect(fila.getByText('06/11/2026 · 10:00')).toBeInTheDocument();
    expect(fila.getByText('En proceso')).toBeInTheDocument();
    expect(ordenesService.listarOrdenesTrabajoPaginado).toHaveBeenCalledWith(
      expect.objectContaining({ pagina: 1, limite: 20 })
    );
  });

  it('6. Cajero puede visualizar la interfaz de órdenes correctamente', async () => {
    (useAuth as any).mockReturnValue({
      usuario: mockUsuarioCajero,
      tienePermiso: () => true,
    });
    (ordenesService.listarOrdenesTrabajo as any) = vi.fn().mockResolvedValue(mockOrdenes);

    render(<OrdenesPage />);

    await waitFor(() => {
      expect(screen.getByText(/Órdenes de Trabajo \/ Taller/i)).toBeInTheDocument();
      expect(screen.getByText('ABC-123')).toBeInTheDocument();
    });
  });
});
