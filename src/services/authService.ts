import { invoke } from '@tauri-apps/api/core';
import { UsuarioSesionDTO } from '../types/types';

/**
 * Llama al comando Tauri `login_cmd`
 * enviando el objeto `req` con las claves exactas que espera Serde en Rust: `username` y `password`.
 */
export const obtenerUsuarioSesion = async (
  usuario: string,
  password: string
): Promise<UsuarioSesionDTO> => {
  try {
    const res = await invoke<UsuarioSesionDTO>('login_cmd', {
      req: {
        username: usuario,
        password: password,
      },
    });
    return res;
  } catch (error) {
    console.error('Error al obtener la sesión del usuario:', error);
    throw new Error(String(error));
  }
};
