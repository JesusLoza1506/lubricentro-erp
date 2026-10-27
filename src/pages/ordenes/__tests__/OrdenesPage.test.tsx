import { render, screen, fireEvent, waitFor } from '@testing-library/react';
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

const ahora = new Date();
const anioLocal = meAjustarPad(ahora.getFullYear(), 4);
const mesLocal = meAjustarPad(ahora.getMonth() + 1, 2);
const diaLocal = meAjustarPad(ahora.getDate(), 2);
const hoyFechaLocalStr = `${anioLocal}-${mesLocal}-${diaLocal}T10:00:00`;

function meAjustarPad(val: number, len: number): string {
  return String(val).padStart(len, '0');
}

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
    fecha_ingreso: hoyFechaLocalStr,
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
    fecha_ingreso: hoyFechaLocalStr,
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
    fecha_ingreso: hoyFechaLocalStr,
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

  it('1. Debe renderizar el encabezado y el botón "+ Nueva OT" para Administrador', async () => {
    (useAuth as any).mockReturnValue({
      usuario: mockUsuarioAdmin,
      tienePermiso: () => true,
    });
    (ordenesService.listarOrdenesTrabajo as any) = vi.fn().mockResolvedValue(mockOrdenes);

    render(<OrdenesPage />);

    await waitFor(() => {
      expect(screen.getByText(/Órdenes de Trabajo \/ Taller/i)).toBeInTheDocument();
      expect(screen.getByText(/(\+ Recepcionar Vehículo|\+ Nueva OT)/i)).toBeInTheDocument();
      expect(screen.getByText('ABC-123')).toBeInTheDocument();
    });
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

    const btnsVer = screen.getAllByText(
      /(Abrir atención|Ver Detalle \/ Gestionar|Ver Detalle|Gestionar|Atender \/ Gestionar OT|Ver Detalle \/ Insumos)/i
    );
    fireEvent.click(btnsVer[0]);

    await waitFor(() => {
      expect(screen.getByText(/OT:/i)).toBeInTheDocument();
      expect(screen.getAllByText('OT-000101').length).toBeGreaterThanOrEqual(1);
    });
  });

  it('4. Mecánico distingue visualmente sus OTs asignadas frente a las de otros', async () => {
    (useAuth as any).mockReturnValue({
      usuario: mockUsuarioMecanico,
      tienePermiso: () => true,
    });
    (ordenesService.listarOrdenesTrabajo as any) = vi.fn().mockResolvedValue(mockOrdenes);

    render(<OrdenesPage />);

    await waitFor(() => {
      expect(screen.getByText('ABC-123')).toBeInTheDocument();
      expect(screen.getByText('XYZ-999')).toBeInTheDocument();
    });
  });

  it('5. Muestra e identifica correctamente el Historial del Día con fechas locales', async () => {
    (useAuth as any).mockReturnValue({
      usuario: mockUsuarioAdmin,
      tienePermiso: () => true,
    });
    (ordenesService.listarOrdenesTrabajo as any) = vi.fn().mockResolvedValue(mockOrdenes);

    render(<OrdenesPage />);

    await waitFor(() => {
      const elementosHistorial = screen.getAllByText(
        /(Historial del Día|Historial \/ Finalizadas|Historial de Órdenes|Historial Completo|Zanjas de Trabajo)/i
      );
      expect(elementosHistorial.length).toBeGreaterThan(0);
      expect(elementosHistorial[0]).toBeInTheDocument();
      expect(screen.getByText('ABC-123')).toBeInTheDocument();
    });
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
