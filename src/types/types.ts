// Define la estructura exacta de UsuarioSesionDTO que viene de Rust
export interface UsuarioSesionDTO {
  id_usuario: number;
  nombre_completo: string;
  usuario: string;
  rol: 'ADMINISTRADOR' | 'CAJERO' | 'MECANICO';
  activo: boolean;
}

// Define la credencial de entrada para la autenticación
export interface Credenciales {
  usuario: string;
  password_hash: string;
}
