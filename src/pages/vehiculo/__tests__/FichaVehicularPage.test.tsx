import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import { FichaVehicularPage } from '../FichaVehicularPage';
import { vehiculoService } from '../../../services/vehiculoService';
import { FichaVehicularCompletaDTO } from '../../../types/vehiculo';

// Mock de los servicios de Tauri / Backend
vi.mock('../../../services/vehiculoService', () => ({
  vehiculoService: {
    obtenerFichaVehicularCompleta: vi.fn(),
    registrarVehiculo: vi.fn(),
    actualizarVehiculo: vi.fn(),
    eliminarVehiculo: vi.fn(),
  },
}));

// Mock del contexto de Autenticación con permisos completos
vi.mock('../../../context/AuthContext', () => ({
  useAuth: () => ({
    tienePermiso: () => true,
    usuario: { id_usuario: 1, nombre_completo: 'Administrador', rol: 'ADMINISTRADOR' },
  }),
}));

describe('FichaVehicularPage - Pruebas de Integración Frontend', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renderiza correctamente el estado inicial vacío de búsqueda', () => {
    render(<FichaVehicularPage />);
    expect(screen.getByPlaceholderText(/Ej\. ABC-123/i)).toBeDefined();
    expect(screen.getByText(/Consulta la Hoja de Vida Vehicular/i)).toBeDefined();
  });

  it('muestra estado de carga y renderiza la ficha vehicular cuando la búsqueda es exitosa', async () => {
    const mockFicha: FichaVehicularCompletaDTO = {
      vehiculo: {
        id_cliente: 1,
        placa: 'ABC-123',
        marca: 'Toyota',
        modelo: 'Yaris',
        anio: 2020,
        tipo_motor: '1.5L',
        kilometraje_actual: 45000,
        nombre_cliente: 'Juan Pérez',
      },
      historial: [
        {
          id_ot: 1,
          codigo_ot: 'OT-0001',
          placa: 'ABC-123',
          estado: 'FINALIZADO',
          kilometraje_ingreso: 45000,
          proximo_kilometraje: 50000,
          id_mecanico: 2,
          tipo_aceite: 'Aceite Sintetico 5W-30',
          observaciones: 'Cambio general realizado con éxito.',
          fecha_ingreso: '2026-06-01T10:00:00Z',
          detalles: [],
        },
      ],
    };

    vi.mocked(vehiculoService.obtenerFichaVehicularCompleta).mockResolvedValue(mockFicha);

    render(<FichaVehicularPage />);

    const input = screen.getByPlaceholderText(/Ej\. ABC-123/i);
    fireEvent.change(input, { target: { value: 'ABC-123' } });

    const botonBuscar = screen.getByRole('button', { name: /buscar/i });
    fireEvent.click(botonBuscar);

    // Validamos usando matchers flexibles para textos compuestos
    await waitFor(() => {
      expect(screen.getByText('Juan Pérez')).toBeDefined();
      expect(screen.getByText((content) => content.includes('Toyota'))).toBeDefined();
      expect(screen.getByText('OT-0001')).toBeDefined();
      expect(screen.getByText('Aceite Sintetico 5W-30')).toBeDefined();
    });
  });

  it('captura y muestra el mensaje de error cuando el backend falla', async () => {
    vi.mocked(vehiculoService.obtenerFichaVehicularCompleta).mockRejectedValue(
      new Error('Vehículo no encontrado en la base de datos.')
    );

    render(<FichaVehicularPage />);

    const input = screen.getByPlaceholderText(/Ej\. ABC-123/i);
    fireEvent.change(input, { target: { value: 'XYZ-999' } });

    const botonBuscar = screen.getByRole('button', { name: /buscar/i });
    fireEvent.click(botonBuscar);

    await waitFor(() => {
      expect(screen.getByText(/Vehículo no encontrado en la base de datos/i)).toBeDefined();
    });
  });
});
