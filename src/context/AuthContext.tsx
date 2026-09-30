import React, { createContext, useContext, useState, ReactNode } from 'react';
import { UsuarioSesionDTO } from '../types/types';
import { obtenerUsuarioSesion } from '../services/authService';

interface AuthContextType {
  usuario: UsuarioSesionDTO | null;
  cargando: boolean;
  iniciarSesion: (user: string, pass: string) => Promise<void>;
  cerrarSesion: () => void;
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

  return (
    <AuthContext.Provider value={{ usuario, cargando, iniciarSesion, cerrarSesion }}>
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
