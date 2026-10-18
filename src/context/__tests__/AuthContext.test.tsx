import { act, cleanup, render } from '@testing-library/react';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import { AuthProvider, useAuth } from '../AuthContext';
import * as api from '../../services/authService';
vi.mock('../../services/authService', async original => ({
  ...await original<typeof import('../../services/authService')>(),
  apiObtenerSesion: vi.fn(), apiObtenerVersionApp: vi.fn(), apiObtenerAmbienteInicial: vi.fn(),
  apiRegistrarActividad: vi.fn(), apiCerrarSesion: vi.fn(), apiLoginUsuario: vi.fn(),
}));
const noAuth = 'NO_AUTENTICADO: Debes iniciar sesión para realizar esta acción.';
const session = { session_id: 'session-1', id_usuario: 1, username: 'admin', nombre_completo: 'Admin', rol: 'ADMINISTRADOR', estado_sesion: 'NORMAL', permisos: [], ambiente_actual: 'PRUEBA' };
let auth: ReturnType<typeof useAuth>;
function Probe() { auth = useAuth(); return null; }
async function mount() { await act(async () => { render(<AuthProvider><Probe /></AuthProvider>); }); }
beforeEach(() => {
  vi.resetAllMocks(); vi.useFakeTimers(); vi.setSystemTime(new Date('2026-10-18T08:00:00Z'));
  vi.mocked(api.apiObtenerVersionApp).mockResolvedValue('7');
  vi.mocked(api.apiObtenerAmbienteInicial).mockResolvedValue('PRUEBA');
  vi.mocked(api.apiObtenerSesion).mockResolvedValue(session as any);
  vi.mocked(api.apiRegistrarActividad).mockResolvedValue();
});
afterEach(() => { cleanup(); vi.useRealTimers(); });
it('uses the actual error helper and actual serialized error', () => {
  expect(api.esErrorNoAutenticado(noAuth)).toBe(true);
  expect(api.esErrorNoAutenticado('DATABASE_ERROR: Error interno.')).toBe(false);
});
it('clears an existing identity when polling receives no-auth', async () => {
  await mount(); expect(auth.usuario?.username).toBe('admin');
  vi.mocked(api.apiObtenerSesion).mockRejectedValue(noAuth);
  await act(async () => { await vi.advanceTimersByTimeAsync(10000); });
  expect(auth.usuario).toBeNull(); expect(auth.estadoSesion).toBe('SIN_SESION');
});
it('synchronizes changed session data on a real polling tick without activity', async () => {
  await mount();
  vi.mocked(api.apiObtenerSesion).mockResolvedValue({...session, nombre_completo:'Actualizado', ambiente_actual:'PRODUCCION'} as any);
  await act(async () => { await vi.advanceTimersByTimeAsync(10000); });
  expect(auth.usuario?.nombre_completo).toBe('Actualizado');
  expect(auth.ambienteActual).toBe('PRODUCCION'); expect(api.apiRegistrarActividad).not.toHaveBeenCalled();
});
it('does not invent production if prelogin environment fails', async () => {
  vi.mocked(api.apiObtenerSesion).mockRejectedValue(noAuth);
  vi.mocked(api.apiObtenerAmbienteInicial).mockRejectedValue('DATABASE_ERROR');
  await mount(); expect(auth.ambienteActual).toBe('ERROR_CONEXION');
});
it('does not revoke identity for an explicit storage error from polling', async () => {
  await mount(); vi.mocked(api.apiObtenerSesion).mockRejectedValue('DATABASE_ERROR: Error interno.');
  await act(async () => { await vi.advanceTimersByTimeAsync(10000); });
  expect(auth.estadoSesion).toBe('NORMAL');
});
it('ignores a delayed successful polling response after logout', async () => {
  await mount(); let resolve!: (v: any) => void;
  vi.mocked(api.apiObtenerSesion).mockReturnValue(new Promise(r => { resolve = r; }));
  await act(async () => { await vi.advanceTimersByTimeAsync(10000); });
  await act(async () => { await auth.cerrarSesion(); });
  await act(async () => { resolve(session); });
  expect(auth.estadoSesion).toBe('SIN_SESION'); expect(auth.usuario).toBeNull();
});
it('reports a fresh interaction at 14m30s for its originating session', async () => {
  await mount();
  await act(async () => { await vi.advanceTimersByTimeAsync(870000); window.dispatchEvent(new Event('keydown')); });
  expect(api.apiRegistrarActividad).toHaveBeenCalledExactlyOnceWith('session-1');
});
it('retains an interaction inside the thirty-second window', async () => {
  await mount();
  await act(async () => {
    await vi.advanceTimersByTimeAsync(20000); window.dispatchEvent(new Event('keydown'));
    await vi.advanceTimersByTimeAsync(12000);
  });
  expect(api.apiRegistrarActividad).toHaveBeenCalled();
});
it('reports every interaction and never sends activity after logout', async () => {
  await mount();
  await act(async () => {
    window.dispatchEvent(new Event('keydown'));
    window.dispatchEvent(new Event('mousemove'));
    window.dispatchEvent(new Event('mousedown'));
    window.dispatchEvent(new Event('touchstart'));
  });
  expect(api.apiRegistrarActividad).toHaveBeenCalledTimes(4);
  await act(async () => { await auth.cerrarSesion(); });
  await act(async () => { window.dispatchEvent(new Event('keydown')); await vi.advanceTimersByTimeAsync(60000); });
  expect(api.apiRegistrarActividad).toHaveBeenCalledTimes(4);
});
it('ignores a rejection of activity from an older session', async () => {
  await mount(); let reject!: (e: unknown) => void;
  vi.mocked(api.apiRegistrarActividad).mockReturnValue(new Promise((_resolve, r) => { reject = r; }));
  await act(async () => { window.dispatchEvent(new Event('keydown')); });
  await act(async () => { await auth.cerrarSesion(); });
  vi.mocked(api.apiLoginUsuario).mockResolvedValue({...session, session_id:'session-2', activo:true, debe_cambiar_clave:false} as any);
  await act(async () => { await auth.iniciarSesion('admin', 'Original123'); });
  await act(async () => { reject(noAuth); });
  expect(auth.estadoSesion).toBe('NORMAL'); expect(auth.usuario?.session_id).toBe('session-2');
});
it('keeps login failure without identity and exposes a controlled error', async () => {
  vi.mocked(api.apiObtenerSesion).mockRejectedValue(noAuth); await mount();
  vi.mocked(api.apiLoginUsuario).mockRejectedValue('AUTH_ERROR: Credenciales inválidas.');
  await act(async () => { await expect(auth.iniciarSesion('admin', 'wrong')).rejects.toBe('AUTH_ERROR: Credenciales inválidas.'); });
  expect(auth.usuario).toBeNull(); expect(auth.estadoSesion).toBe('SIN_SESION');
});
it('only classifies the exact public error code as no-auth', () => {
  expect(api.esErrorNoAutenticado('DATABASE_ERROR: NO_AUTENTICADO in detail')).toBe(false);
  expect(api.esErrorNoAutenticado({code:'NO_AUTENTICADO'})).toBe(false);
  expect(api.esErrorNoAutenticado(new Error(noAuth))).toBe(true);
  expect(api.mensajeErrorAuth(noAuth, 'Error')).toBe('Debes iniciar sesión para realizar esta acción.');
});
it('logout clears the loading flag even if a session check is pending', async () => {
  await mount(); let resolve!: (v: any) => void;
  vi.mocked(api.apiObtenerSesion).mockReturnValue(new Promise(r => { resolve=r; }));
  let pending!: Promise<void>;
  await act(async () => { pending=auth.verificarSesion(); });
  expect(auth.isCheckingSession).toBe(true);
  await act(async () => { await auth.cerrarSesion(); });
  expect(auth.isCheckingSession).toBe(false);
  await act(async () => { resolve(session); await pending; });
  expect(auth.usuario).toBeNull();
});
