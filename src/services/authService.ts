import { invoke } from '@tauri-apps/api/core';
import { UsuarioSesionDTO } from '../types/types';

/**
 * Llama al comando Tauri `obtener_usuario_sesion_cmd`
 * enviando usuario y contraseña para autenticar al usuario.
 */
export const obtenerUsuarioSesion = async (
  usuario: string,
  password_hash: string
): Promise<UsuarioSesionDTO> => {
  try {
    const res = await invoke<UsuarioSesionDTO>('obtener_usuario_sesion_cmd', {
      usuario: usuario,
      passwordHash: password_hash,
    });
    return res;
  } catch (error) {
    console.error('Error al obtener la sesión del usuario:', error);
    throw new Error(String(error));
  }
};
