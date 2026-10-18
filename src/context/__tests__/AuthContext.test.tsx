import { render, waitFor } from '@testing-library/react';
import { describe, test, expect, beforeEach, vi } from 'vitest';
import { AuthProvider, useAuth } from '../AuthContext';
import * as authService from '../../services/authService';

vi.mock('../../services/authService', () => ({
  apiObtenerVersionApp: vi.fn(),
  apiObtenerAmbienteInicial: vi.fn(),
  apiObtenerSesion: vi.fn(),
  apiRegistrarActividad: vi.fn(),
  apiCerrarSesion: vi.fn(),
  esErrorNoAutenticado: (err: unknown): boolean => {
    if (!err) return false;
    const msg = typeof err === 'string' ? err : (err as any)?.message || JSON.stringify(err);
    return (
      msg.includes('NO_AUTENTICADO') ||
      msg.includes('NoAutenticado') ||
      msg.includes('Debes iniciar sesión') ||
      msg.includes('sin sesión')
    );
  },
}));

const TestConsumer = () => {
  const { usuario, estadoSesion, ambienteActual } = useAuth();
  return (
    <div>
      <span data-testid="estado">{estadoSesion}</span>
      <span data-testid="usuario">{usuario ? usuario.username : 'null'}</span>
      <span data-testid="ambiente">{ambienteActual}</span>
    </div>
  );
};

describe('AuthContext - Pruebas de Regresión R7', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    vi.mocked(authService.apiObtenerVersionApp).mockResolvedValue('7.0.0');
    vi.mocked(authService.apiObtenerAmbienteInicial).mockResolvedValue('DESARROLLO');
  });

  test('Expiración y revocación retiran la identidad del usuario', async () => {
    vi.mocked(authService.apiObtenerSesion).mockRejectedValue('NO_AUTENTICADO: Sesión expirada');

    const { getByTestId } = render(
      <AuthProvider>
        <TestConsumer />
      </AuthProvider>
    );

    await waitFor(() => {
      expect(getByTestId('estado').textContent).toBe('SIN_SESION');
      expect(getByTestId('usuario').textContent).toBe('null');
      expect(getByTestId('ambiente').textContent).toBe('DESARROLLO');
    });
  });

  test('Fallo de obtención de ambiente inicial asigna ERROR_CONEXION sin asumir Producción', async () => {
    vi.mocked(authService.apiObtenerSesion).mockRejectedValue('NO_AUTENTICADO');
    vi.mocked(authService.apiObtenerAmbienteInicial).mockRejectedValue(
      new Error('Conexión fallida')
    );

    const { getByTestId } = render(
      <AuthProvider>
        <TestConsumer />
      </AuthProvider>
    );

    await waitFor(() => {
      expect(getByTestId('ambiente').textContent).toBe('ERROR_CONEXION');
      expect(getByTestId('estado').textContent).toBe('SIN_SESION');
    });
  });

  test('El polling exitoso sincroniza el ambiente sin registrar falsa actividad', async () => {
    vi.mocked(authService.apiObtenerSesion).mockResolvedValue({
      id_usuario: 1,
      username: 'cajero1',
      nombre_completo: 'Cajero Test',
      rol: 'CAJERO',
      estado_sesion: 'NORMAL',
      permisos: [],
      ambiente_actual: 'PRODUCCION',
    });

    const { getByTestId } = render(
      <AuthProvider>
        <TestConsumer />
      </AuthProvider>
    );

    await waitFor(() => {
      expect(getByTestId('estado').textContent).toBe('NORMAL');
      expect(getByTestId('ambiente').textContent).toBe('PRODUCCION');
    });

    expect(authService.apiRegistrarActividad).not.toHaveBeenCalled();
  });
});
