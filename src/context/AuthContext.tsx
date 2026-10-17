import React, {
  createContext,
  useContext,
  useState,
  useEffect,
  ReactNode,
  useCallback,
  useRef,
} from 'react';
import { EstadoSesionEnum, PermisoModulo, UsuarioSesion } from '../types/auth';
import {
  apiCerrarSesion,
  apiLoginUsuario,
  apiObtenerSesion,
  apiRegistrarActividad,
  apiObtenerVersionApp,
} from '../services/authService';

interface AuthContextType {
  usuario: UsuarioSesion | null;
  estadoSesion: EstadoSesionEnum;
  cargando: boolean;
  isCheckingSession: boolean;
  isLoggingIn: boolean;
  appVersion: string;
  ambienteActual: string;
  mensajeInactividad: string | null;
  hayCambiosSinGuardar: boolean;
  setHayCambiosSinGuardar: (dirty: boolean) => void;
  limpiarMensajeInactividad: () => void;
  iniciarSesion: (user: string, pass: string) => Promise<UsuarioSesion>;
  cerrarSesion: () => Promise<void>;
  completarCambioObligatorio: () => void;
  actualizarEstadoSesion: (nuevoEstado: EstadoSesionEnum) => void;
  tienePermiso: (modulo: string, accion: 'ver' | 'crear' | 'editar' | 'eliminar') => boolean;
  verificarSesion: () => Promise<void>;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

export const AuthProvider: React.FC<{ children: ReactNode }> = ({ children }) => {
  const [usuario, setUsuario] = useState<UsuarioSesion | null>(null);
  const [estadoSesion, setEstadoSesion] = useState<EstadoSesionEnum>('SIN_SESION');
  const [isCheckingSession, setIsCheckingSession] = useState<boolean>(true);
  const [isLoggingIn, setIsLoggingIn] = useState<boolean>(false);
  const [appVersion, setAppVersion] = useState<string>('7.0.0');
  const [ambienteActual, setAmbienteActual] = useState<string>('PRODUCCION');
  const [mensajeInactividad, setMensajeInactividad] = useState<string | null>(null);
  const [hayCambiosSinGuardar, setHayCambiosSinGuardar] = useState<boolean>(false);

  const ultimoRegistroRef = useRef<number>(0);
  const sessionInstanceIdRef = useRef<number>(0);

  const incrementarInstanciaSesion = () => {
    sessionInstanceIdRef.current += 1;
    return sessionInstanceIdRef.current;
  };

  useEffect(() => {
    apiObtenerVersionApp()
      .then((ver) => setAppVersion(ver))
      .catch(() => setAppVersion('7.0.0'));
  }, []);

  const limpiarMensajeInactividad = () => setMensajeInactividad(null);

  const verificarSesion = useCallback(async () => {
    const localInstanceId = sessionInstanceIdRef.current;
    setIsCheckingSession(true);
    try {
      const sesionActiva = await apiObtenerSesion();

      if (localInstanceId !== sessionInstanceIdRef.current) return;

      setUsuario({
        id_usuario: sesionActiva.id_usuario,
        username: sesionActiva.username,
        nombre_completo: sesionActiva.nombre_completo,
        rol: sesionActiva.rol,
        activo: true,
        debe_cambiar_clave: sesionActiva.estado_sesion === 'CAMBIO_OBLIGATORIO',
        estado_sesion: sesionActiva.estado_sesion as EstadoSesionEnum,
        permisos: sesionActiva.permisos,
        ambiente_actual: sesionActiva.ambiente_actual || 'PRODUCCION',
      });
      setAmbienteActual(sesionActiva.ambiente_actual || 'PRODUCCION');
      setEstadoSesion(sesionActiva.estado_sesion as EstadoSesionEnum);
    } catch {
      if (localInstanceId !== sessionInstanceIdRef.current) return;
      setUsuario(null);
      setEstadoSesion('SIN_SESION');
    } finally {
      if (localInstanceId === sessionInstanceIdRef.current) {
        setIsCheckingSession(false);
      }
    }
  }, []);

  useEffect(() => {
    verificarSesion();
  }, [verificarSesion]);

  useEffect(() => {
    if (estadoSesion === 'SIN_SESION') return;

    const manejarActividadHumana = () => {
      const ahora = Date.now();
      if (ahora - ultimoRegistroRef.current > 30000) {
        ultimoRegistroRef.current = ahora;
        const localInstanceId = sessionInstanceIdRef.current;
        apiRegistrarActividad().catch(() => {
          if (localInstanceId !== sessionInstanceIdRef.current) return;
          incrementarInstanciaSesion();
          setUsuario(null);
          setEstadoSesion('SIN_SESION');
          setMensajeInactividad('Sesión finalizada por inactividad');
        });
      }
    };

    window.addEventListener('keydown', manejarActividadHumana);
    window.addEventListener('mousemove', manejarActividadHumana);
    window.addEventListener('mousedown', manejarActividadHumana);
    window.addEventListener('touchstart', manejarActividadHumana);

    return () => {
      window.removeEventListener('keydown', manejarActividadHumana);
      window.removeEventListener('mousemove', manejarActividadHumana);
      window.removeEventListener('mousedown', manejarActividadHumana);
      window.removeEventListener('touchstart', manejarActividadHumana);
    };
  }, [estadoSesion]);

  useEffect(() => {
    if (estadoSesion === 'SIN_SESION') return;

    const interval = setInterval(async () => {
      const localInstanceId = sessionInstanceIdRef.current;
      try {
        await apiObtenerSesion();
      } catch {
        if (localInstanceId !== sessionInstanceIdRef.current) return;
        incrementarInstanciaSesion();
        setUsuario(null);
        setEstadoSesion('SIN_SESION');
        setMensajeInactividad('Sesión finalizada por inactividad');
      }
    }, 10000);

    return () => clearInterval(interval);
  }, [estadoSesion]);

  const iniciarSesion = async (user: string, pass: string): Promise<UsuarioSesion> => {
    if (pass.length > 128) {
      throw new Error('La contraseña no puede exceder los 128 caracteres.');
    }

    const localInstanceId = incrementarInstanciaSesion();
    setIsLoggingIn(true);
    setMensajeInactividad(null);
    try {
      const datos = await apiLoginUsuario({ username: user, password: pass });

      if (localInstanceId !== sessionInstanceIdRef.current) {
        throw new Error('Operación cancelada por cambio de instancia de sesión.');
      }

      setUsuario(datos);
      setAmbienteActual(datos.ambiente_actual || 'PRODUCCION');
      setEstadoSesion(datos.estado_sesion as EstadoSesionEnum);
      setHayCambiosSinGuardar(false);
      ultimoRegistroRef.current = Date.now();
      return datos;
    } catch (error) {
      if (localInstanceId === sessionInstanceIdRef.current) {
        setUsuario(null);
        setEstadoSesion('SIN_SESION');
      }
      throw error;
    } finally {
      if (localInstanceId === sessionInstanceIdRef.current) {
        setIsLoggingIn(false);
      }
    }
  };

  const cerrarSesion = async () => {
    incrementarInstanciaSesion();
    try {
      await apiCerrarSesion();
    } catch {
      // Ignorar errores de red al salir
    } finally {
      setUsuario(null);
      setEstadoSesion('SIN_SESION');
      setHayCambiosSinGuardar(false);
      setMensajeInactividad(null);
    }
  };

  const completarCambioObligatorio = () => {
    setEstadoSesion('NORMAL');
    if (usuario) {
      setUsuario({
        ...usuario,
        estado_sesion: 'NORMAL',
        debe_cambiar_clave: false,
      });
    }
  };

  const actualizarEstadoSesion = (nuevoEstado: EstadoSesionEnum) => {
    setEstadoSesion(nuevoEstado);
    if (usuario) {
      setUsuario({
        ...usuario,
        estado_sesion: nuevoEstado,
        debe_cambiar_clave: nuevoEstado === 'CAMBIO_OBLIGATORIO',
      });
    }
  };

  const tienePermiso = (
    modulo: string,
    accion: 'ver' | 'crear' | 'editar' | 'eliminar'
  ): boolean => {
    if (!usuario || !usuario.permisos || estadoSesion !== 'NORMAL') return false;

    const permisoModulo = usuario.permisos.find(
      (p: PermisoModulo) => p.modulo.toUpperCase() === modulo.toUpperCase()
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
    <AuthContext.Provider
      value={{
        usuario,
        estadoSesion,
        cargando: isCheckingSession,
        isCheckingSession,
        isLoggingIn,
        appVersion,
        ambienteActual,
        mensajeInactividad,
        hayCambiosSinGuardar,
        setHayCambiosSinGuardar,
        limpiarMensajeInactividad,
        iniciarSesion,
        cerrarSesion,
        completarCambioObligatorio,
        actualizarEstadoSesion,
        tienePermiso,
        verificarSesion,
      }}
    >
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
