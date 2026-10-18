import { invoke } from '@tauri-apps/api/core';
import { getVersion } from '@tauri-apps/api/app';
import {
  AutorizarDescuentoPayload,
  CambiarClaveObligatoriaPayload,
  CambiarClaveVoluntariaPayload,
  InstalacionPayload,
  InstalacionResponseDto,
  LoginPayload,
  ReautenticarPayload,
  RecuperarPayload,
  SesionResumenDto,
  UsuarioSesion,
} from '../types/auth';

/**
 * Mapeador central de errores para convertir errores provenientes de Tauri en una estructura comprensible.
 * Refinado R7 (Punto 1 y 7): Detección por códigos numéricos/estructurados (e.g. NO_AUTENTICADO).
 */
export const esErrorNoAutenticado = (err: unknown): boolean => {
  if (!err) return false;
  const msg = typeof err === 'string' ? err : (err as any)?.message || JSON.stringify(err);
  return (
    msg.startsWith('NO_AUTENTICADO') ||
    msg.includes('NO_AUTENTICADO') ||
    msg.includes('NoAutenticado') ||
    msg.includes('Debes iniciar sesión') ||
    msg.includes('sin sesión') ||
    msg.includes('Sesión no válida') ||
    msg.includes('invalidada')
  );
};

export const apiObtenerVersionApp = async (): Promise<string> => {
  try {
    return await getVersion();
  } catch {
    return '7.0.0';
  }
};

export const apiObtenerAmbienteInicial = async (): Promise<string> => {
  return await invoke<string>('obtener_ambiente_inicial');
};

export const apiVerificarInstalacion = async (): Promise<boolean> => {
  return await invoke<boolean>('requiere_instalacion');
};

export const apiLoginUsuario = async (payload: LoginPayload): Promise<UsuarioSesion> => {
  return await invoke<UsuarioSesion>('login_usuario', { req: payload });
};

export const apiInstalacionInicial = async (
  payload: InstalacionPayload
): Promise<InstalacionResponseDto> => {
  return await invoke<InstalacionResponseDto>('instalacion_inicial', {
    req: payload,
  });
};

export const apiRecuperarAccesoAdmin = async (
  payload: RecuperarPayload
): Promise<InstalacionResponseDto> => {
  return await invoke<InstalacionResponseDto>('recuperar_acceso_admin', {
    req: payload,
  });
};

export const apiObtenerSesion = async (): Promise<SesionResumenDto> => {
  return await invoke<SesionResumenDto>('obtener_sesion');
};

export const apiCambiarClaveObligatoria = async (
  payload: CambiarClaveObligatoriaPayload
): Promise<void> => {
  await invoke<void>('cambiar_clave_obligatoria', { req: payload });
};

export const apiCambiarClaveVoluntaria = async (
  payload: CambiarClaveVoluntariaPayload
): Promise<void> => {
  await invoke<void>('cambiar_clave_voluntaria', { req: payload });
};

export const apiRegistrarActividad = async (deltaMs?: number): Promise<void> => {
  await invoke<void>('registrar_actividad', { deltaMs: deltaMs ?? null });
};

export const apiCerrarSesion = async (): Promise<void> => {
  await invoke<void>('cerrar_sesion');
};

export const apiReautenticar = async (payload: ReautenticarPayload): Promise<string> => {
  return await invoke<string>('reautenticar', { req: payload });
};

export const apiAutorizarDescuento = async (
  payload: AutorizarDescuentoPayload
): Promise<string> => {
  return await invoke<string>('autorizar_descuento', { req: payload });
};
