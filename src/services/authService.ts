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
 * Obtener versión real de la aplicación
 */
export const apiObtenerVersionApp = async (): Promise<string> => {
  try {
    return await getVersion();
  } catch {
    return '7.0.0';
  }
};

/**
 * 0. Verificar si el sistema requiere instalación inicial (requiere_instalacion)
 */
export const apiVerificarInstalacion = async (): Promise<boolean> => {
  return await invoke<boolean>('requiere_instalacion');
};

/**
 * 1. Autenticar usuario y abrir sesión en Rust (login_usuario)
 */
export const apiLoginUsuario = async (payload: LoginPayload): Promise<UsuarioSesion> => {
  return await invoke<UsuarioSesion>('login_usuario', { req: payload });
};

/**
 * 2. Primer arranque / Instalación inicial (instalacion_inicial)
 */
export const apiInstalacionInicial = async (
  payload: InstalacionPayload
): Promise<InstalacionResponseDto> => {
  return await invoke<InstalacionResponseDto>('instalacion_inicial', {
    req: payload,
  });
};

/**
 * 3. Recuperar acceso de Administrador con código de instalación (recuperar_acceso_admin)
 */
export const apiRecuperarAccesoAdmin = async (
  payload: RecuperarPayload
): Promise<InstalacionResponseDto> => {
  return await invoke<InstalacionResponseDto>('recuperar_acceso_admin', {
    req: payload,
  });
};

/**
 * 4. Obtener estado de la sesión activa en memoria (obtener_sesion)
 */
export const apiObtenerSesion = async (): Promise<SesionResumenDto> => {
  return await invoke<SesionResumenDto>('obtener_sesion');
};

/**
 * 5.a. Cambiar contraseña obligatoria (RF-LOG-03): Solo exige nueva clave y confirmación
 */
export const apiCambiarClaveObligatoria = async (
  payload: CambiarClaveObligatoriaPayload
): Promise<void> => {
  await invoke<void>('cambiar_clave_obligatoria', { req: payload });
};

/**
 * 5.b. Cambiar contraseña voluntaria (RF-LOG-04): Exige clave actual, nueva y confirmación
 */
export const apiCambiarClaveVoluntaria = async (
  payload: CambiarClaveVoluntariaPayload
): Promise<void> => {
  await invoke<void>('cambiar_clave_voluntaria', { req: payload });
};

/**
 * 6. Registrar interacción humana para mantener viva la sesión (registrar_actividad)
 */
export const apiRegistrarActividad = async (): Promise<void> => {
  await invoke<void>('registrar_actividad');
};

/**
 * 7. Cerrar sesión activa en Rust (cerrar_sesion)
 */
export const apiCerrarSesion = async (): Promise<void> => {
  await invoke<void>('cerrar_sesion');
};

/**
 * 8. Reautenticar al usuario actual para operaciones sensibles (reautenticar)
 */
export const apiReautenticar = async (payload: ReautenticarPayload): Promise<string> => {
  return await invoke<string>('reautenticar', { req: payload });
};

/**
 * 9. Autorizar un descuento de venta por un Administrador (autorizar_descuento)
 */
export const apiAutorizarDescuento = async (
  payload: AutorizarDescuentoPayload
): Promise<string> => {
  return await invoke<string>('autorizar_descuento', { req: payload });
};
