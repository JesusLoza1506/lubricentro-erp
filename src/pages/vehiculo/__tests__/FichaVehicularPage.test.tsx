import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import { FichaVehicularPage } from '../FichaVehicularPage';
import { vehiculoService } from '../../../services/vehiculoService';
import { FichaVehicularCompletaDTO } from '../../../types/vehiculo';

vi.mock('../../../services/vehiculoService', () => ({
  vehiculoService: {
    obtenerFichaVehicularCompleta: vi.fn(),
    registrarVehiculo: vi.fn(),
    actualizarVehiculo: vi.fn(),
    cambiarEstadoVehiculo: vi.fn(),
    corregirHistorialOt: vi.fn(),
    obtenerProximosCambiosAceite: vi.fn().mockResolvedValue([]),
  },
}));

vi.mock('../../../services/ordenesService', () => ({
  ordenesService: {
    listarOrdenesTrabajo: vi.fn().mockResolvedValue([]),
    crearOrdenTrabajo: vi.fn(),
  },
}));

vi.mock('../../../services/clienteService', () => ({
  clienteService: {
    buscarClientePorDocumento: vi.fn(),
    registrarCliente: vi.fn(),
    actualizarCliente: vi.fn(),
  },
}));

vi.mock('../../../context/AuthContext', () => ({
  useAuth: () => ({
    tienePermiso: () => true,
    esAdmin: true,
    usuario: { id_usuario: 1, nombre_completo: 'Administrador', rol: 'ADMINISTRADOR' },
  }),
}));

describe('FichaVehicularPage - Pruebas de Integración Frontend (Punto 15)', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('1. Renderiza correctamente el estado inicial vacío de búsqueda', () => {
    render(<FichaVehicularPage />);
    expect(screen.getByPlaceholderText(/Ej\. ABC123/i)).toBeDefined();
    expect(screen.getByText(/Consulta vehículos y su historial de mantenimiento/i)).toBeDefined();
  });

  it('2. Muestra estado de carga, renderiza la ficha vehicular y el cliente histórico atendido', async () => {
    const mockFicha: FichaVehicularCompletaDTO = {
      vehiculo: {
        id_vehiculo: 1,
        id_cliente: 1,
        placa: 'ABC123',
        marca: 'Toyota',
        modelo: 'Yaris',
        anio: 2020,
        tipo_motor: 'GASOLINA',
        kilometraje_actual: 45000,
        activo: 1,
        nombre_cliente: 'Juan Pérez (Propietario Actual)',
        tipo_documento_cliente: 'DNI',
        documento_cliente: '12345678',
        telefono_cliente: '987654321',
      },
      historial: {
        items: [
          {
            id_ot: 1,
            codigo_ot: 'OT-0001',
            placa: 'ABC123',
            estado: 'FINALIZADO',
            kilometraje_ingreso: 45000,
            kilometraje_salida: 45000,
            km_proximo: 50000,
            proximo_kilometraje: 50000,
            id_mecanico: 2,
            tipo_aceite: 'SINTETICO',
            resumen_publico: 'Cambio general de aceite y filtro realizado.',
            fecha_fin: '2026-06-01T11:30:00Z',
            nombre_cliente: 'Cliente Antiguo S.A.C.',
            documento_cliente: '20112233445',
            servicios: [],
            insumos: [],
          },
        ],
        total_registros: 1,
        pagina_actual: 1,
        total_paginas: 1,
      },
    };

    vi.mocked(vehiculoService.obtenerFichaVehicularCompleta).mockResolvedValue(mockFicha);

    render(<FichaVehicularPage />);

    const input = screen.getByPlaceholderText(/Ej\. ABC123/i);
    fireEvent.change(input, { target: { value: 'ABC123' } });

    const botonBuscar = screen.getByRole('button', { name: /buscar/i });
    fireEvent.click(botonBuscar);

    await waitFor(() => {
      expect(screen.getByText('Juan Pérez (Propietario Actual)')).toBeDefined();
      expect(screen.getByText('Cliente Antiguo S.A.C.')).toBeDefined();
      expect(screen.getByText(/20112233445/i)).toBeDefined();
      expect(screen.getByText((content) => content.includes('Toyota'))).toBeDefined();
      expect(screen.getAllByText((content) => content.includes('OT-0001')).length).toBeGreaterThan(
        0
      );
    });
  });

  it('3. Captura y muestra el mensaje de error cuando el vehículo no existe', async () => {
    vi.mocked(vehiculoService.obtenerFichaVehicularCompleta).mockRejectedValue(
      new Error('No se encontró vehículo registrado con la placa XYZ999.')
    );

    render(<FichaVehicularPage />);

    const input = screen.getByPlaceholderText(/Ej\. ABC123/i);
    fireEvent.change(input, { target: { value: 'XYZ999' } });

    const botonBuscar = screen.getByRole('button', { name: /buscar/i });
    fireEvent.click(botonBuscar);

    await waitFor(() => {
      expect(screen.getByText(/Vehículo no encontrado/i)).toBeDefined();
      expect(screen.getByText(/No existe registro para la placa/i)).toBeDefined();
      expect(screen.getByText('XYZ999')).toBeDefined();
      expect(screen.getByRole('button', { name: /Registrar XYZ999 Ahora/i })).toBeDefined();
    });
  });

  it('4. Realiza paginación remota al hacer clic en Siguiente página', async () => {
    const mockFichaPagina1: FichaVehicularCompletaDTO = {
      vehiculo: {
        id_vehiculo: 1,
        id_cliente: 1,
        placa: 'ABC123',
        marca: 'Toyota',
        modelo: 'Yaris',
        anio: 2020,
        tipo_motor: 'GASOLINA',
        kilometraje_actual: 45000,
        activo: 1,
        nombre_cliente: 'Juan Pérez',
      },
      historial: {
        items: Array.from({ length: 20 }, (_, i) => ({
          id_ot: i + 1,
          codigo_ot: `OT-${100 + i}`,
          placa: 'ABC123',
          estado: 'FINALIZADO',
          kilometraje_ingreso: 40000,
          kilometraje_salida: 40000,
          fecha_fin: '2026-06-01T11:30:00Z',
          resumen_publico: 'Mantenimiento preventivo',
        })),
        total_registros: 25,
        pagina_actual: 1,
        total_paginas: 2,
      },
    };

    const mockFichaPagina2: FichaVehicularCompletaDTO = {
      ...mockFichaPagina1,
      historial: {
        items: Array.from({ length: 5 }, (_, i) => ({
          id_ot: i + 21,
          codigo_ot: `OT-${120 + i}`,
          placa: 'ABC123',
          estado: 'FINALIZADO',
          kilometraje_ingreso: 30000,
          kilometraje_salida: 30000,
          fecha_fin: '2026-01-01T11:30:00Z',
          resumen_publico: 'Mantenimiento antiguo',
        })),
        total_registros: 25,
        pagina_actual: 2,
        total_paginas: 2,
      },
    };

    vi.mocked(vehiculoService.obtenerFichaVehicularCompleta)
      .mockResolvedValueOnce(mockFichaPagina1)
      .mockResolvedValueOnce(mockFichaPagina2);

    render(<FichaVehicularPage />);

    const input = screen.getByPlaceholderText(/Ej\. ABC123/i);
    fireEvent.change(input, { target: { value: 'ABC123' } });

    const botonBuscar = screen.getByRole('button', { name: /buscar/i });
    fireEvent.click(botonBuscar);

    await waitFor(() => {
      expect(screen.getAllByText(/Mostrando 1–20 de 25/i).length).toBeGreaterThan(0);
    });

    const botonesSiguiente = screen.getAllByRole('button', { name: /siguiente/i });
    fireEvent.click(botonesSiguiente[0]);

    await waitFor(() => {
      expect(vehiculoService.obtenerFichaVehicularCompleta).toHaveBeenLastCalledWith(
        'ABC123',
        2,
        20
      );
      expect(screen.getAllByText(/Mostrando 21–25 de 25/i).length).toBeGreaterThan(0);
    });
  });
});
