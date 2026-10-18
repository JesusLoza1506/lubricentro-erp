import React from 'react';
import { act, cleanup, render } from '@testing-library/react';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import { AuthProvider, useAuth } from './AuthContext';
import * as api from '../services/authService';
vi.mock('../services/authService', async original => ({
  ...await original<typeof import('../services/authService')>(),
  apiObtenerSesion: vi.fn(), apiObtenerVersionApp: vi.fn(), apiObtenerAmbienteInicial: vi.fn(),
  apiRegistrarActividad: vi.fn(), apiCerrarSesion: vi.fn(), apiLoginUsuario: vi.fn(),
}));
const noAuth = 'NO_AUTENTICADO: Debes iniciar sesión para realizar esta acción.';
const session = { id_usuario: 1, username: 'admin', nombre_completo: 'Admin', rol: 'ADMINISTRADOR', estado_sesion: 'NORMAL', permisos: [], ambiente_actual: 'PRUEBA' };
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
it('reports the observed delta after an interaction at 14m30s', async () => {
  await mount();
  await act(async () => { await vi.advanceTimersByTimeAsync(870000); window.dispatchEvent(new Event('keydown')); });
  expect(api.apiRegistrarActividad).toHaveBeenCalledExactlyOnceWith(870000);
});
it('retains an interaction inside the thirty-second window', async () => {
  await mount();
  await act(async () => {
    await vi.advanceTimersByTimeAsync(20000); window.dispatchEvent(new Event('keydown'));
    await vi.advanceTimersByTimeAsync(12000);
  });
  expect(api.apiRegistrarActividad).toHaveBeenCalled();
});
