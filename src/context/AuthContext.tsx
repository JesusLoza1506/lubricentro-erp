import React, { createContext, useContext, useState, ReactNode } from 'react';
import { UsuarioSesionDTO } from '../types/types';
import { obtenerUsuarioSesion } from '../services/authService';

interface AuthContextType {
  usuario: UsuarioSesionDTO | null;
  cargando: boolean;
  iniciarSesion: (user: string, pass: string) => Promise<void>;
  cerrarSesion: () => void;
  tienePermiso: (modulo: string, accion: 'ver' | 'crear' | 'editar' | 'eliminar') => boolean;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

export const AuthProvider: React.FC<{ children: ReactNode }> = ({ children }) => {
  const [usuario, setUsuario] = useState<UsuarioSesionDTO | null>(null);
  const [cargando, setCargando] = useState<boolean>(false);

  const iniciarSesion = async (user: string, pass: string) => {
    setCargando(true);
    try {
      const datosUsuario = await obtenerUsuarioSesion(user, pass);
      setUsuario(datosUsuario);
    } catch (error) {
      console.error('Error durante el inicio de sesión:', error);
      throw error;
    } finally {
      setCargando(false);
    }
  };

  const cerrarSesion = () => {
    setUsuario(null);
  };

  /**
   * Evaluador transversal de permisos dinámicos por módulo
   * Deniega el acceso por defecto si el usuario o el módulo no existen.
   */
  const tienePermiso = (
    modulo: string,
    accion: 'ver' | 'crear' | 'editar' | 'eliminar'
  ): boolean => {
    if (!usuario || !usuario.permisos) return false;

    const permisoModulo = usuario.permisos.find(
      (p) => p.modulo.toUpperCase() === modulo.toUpperCase()
    );

    if (!permisoModulo) return false;

    switch (accion) {
      case 'ver':
        return permisoModulo.puede_ver;
      case 'crear':
        return permisoModulo.puede_crear;
      case 'editar':
        return permisoModulo.puede_editar;
      case 'eliminar':
        return permisoModulo.puede_eliminar;
      default:
        return false;
    }
  };

  return (
    <AuthContext.Provider value={{ usuario, cargando, iniciarSesion, cerrarSesion, tienePermiso }}>
      {children}
    </AuthContext.Provider>
  );
};

export const useAuth = () => {
  const context = useContext(AuthContext);
  if (!context) {
    throw new Error('useAuth debe ser usado dentro de un AuthProvider');
  }
  return context;
};
