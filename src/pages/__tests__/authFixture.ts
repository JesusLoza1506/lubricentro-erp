import { vi } from 'vitest';
import type { useAuth } from '../../context/AuthContext';
import type { UsuarioSesion } from '../../types/auth';

export function sesionDePrueba(overrides: Partial<UsuarioSesion> = {}): UsuarioSesion {
  return {
    session_id: 'sesion-prueba',
    id_usuario: 7,
    nombre_completo: 'Ana Torres',
    username: 'ana',
    rol: 'ADMINISTRADOR',
    activo: true,
    debe_cambiar_clave: false,
    estado_sesion: 'NORMAL',
    permisos: [],
    ...overrides,
  };
}

export function authDePrueba(overrides: Partial<ReturnType<typeof useAuth>> = {}) {
  const sesion = sesionDePrueba();
  return {
    usuario: sesion,
    estadoSesion: 'NORMAL',
    cargando: false,
    isCheckingSession: false,
    isLoggingIn: false,
    appVersion: '7.0.0',
    ambienteActual: 'PRUEBA',
    mensajeInactividad: null,
    hayCambiosSinGuardar: false,
    setHayCambiosSinGuardar: vi.fn(),
    limpiarMensajeInactividad: vi.fn(),
    iniciarSesion: vi.fn<ReturnType<typeof useAuth>['iniciarSesion']>().mockResolvedValue(sesion),
    cerrarSesion: vi.fn().mockResolvedValue(undefined),
    completarCambioObligatorio: vi.fn(),
    actualizarEstadoSesion: vi.fn(),
    tienePermiso: vi.fn().mockReturnValue(false),
    verificarSesion: vi.fn().mockResolvedValue(undefined),
    ...overrides,
  } satisfies ReturnType<typeof useAuth>;
}
