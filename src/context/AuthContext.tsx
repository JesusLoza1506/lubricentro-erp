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
  apiObtenerAmbienteInicial,
  esErrorNoAutenticado,
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
  const [ambienteActual, setAmbienteActual] = useState<string>('CARGANDO...');
  const [mensajeInactividad, setMensajeInactividad] = useState<string | null>(null);
  const [hayCambiosSinGuardar, setHayCambiosSinGuardar] = useState<boolean>(false);

  const sesionActividadRef = useRef<string | null>(null);
  const sessionInstanceIdRef = useRef<number>(0);

  const incrementarInstanciaSesion = () => {
    sesionActividadRef.current = null;
    setIsCheckingSession(false);
    setIsLoggingIn(false);
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

      const env = sesionActiva.ambiente_actual || 'DESCONOCIDO';
      setUsuario({
        session_id: sesionActiva.session_id,
        id_usuario: sesionActiva.id_usuario,
        username: sesionActiva.username,
        nombre_completo: sesionActiva.nombre_completo,
        rol: sesionActiva.rol,
        activo: true,
        debe_cambiar_clave: sesionActiva.estado_sesion === 'CAMBIO_OBLIGATORIO',
        estado_sesion: sesionActiva.estado_sesion as EstadoSesionEnum,
        permisos: sesionActiva.permisos,
        ambiente_actual: env,
      });
      setAmbienteActual(env);
      setEstadoSesion(sesionActiva.estado_sesion as EstadoSesionEnum);
      sesionActividadRef.current = sesionActiva.session_id;
    } catch (err: unknown) {
      if (localInstanceId !== sessionInstanceIdRef.current) return;

      if (esErrorNoAutenticado(err)) {
        sesionActividadRef.current = null;
        setUsuario(null);
        setEstadoSesion('SIN_SESION');

        try {
          const envPreLogin = await apiObtenerAmbienteInicial();
          if (localInstanceId === sessionInstanceIdRef.current) {
            setAmbienteActual(envPreLogin);
          }
        } catch {
          if (localInstanceId === sessionInstanceIdRef.current) {
            setAmbienteActual('ERROR_CONEXION');
          }
        }
      }
    } finally {
      if (localInstanceId === sessionInstanceIdRef.current) {
        setIsCheckingSession(false);
      }
    }
  }, []);

  useEffect(() => {
    verificarSesion();
  }, [verificarSesion]);

  // Solo los eventos humanos registran actividad; sin agrupación ni envíos tardíos.
  useEffect(() => {
    if (estadoSesion === 'SIN_SESION') return;

    const capturarInteraccionHumana = () => {
      const sesionId = sesionActividadRef.current;
      if (!sesionId) return;
      const localInstanceId = sessionInstanceIdRef.current;
      apiRegistrarActividad(sesionId).catch((err: unknown) => {
        if (localInstanceId !== sessionInstanceIdRef.current) return;
        if (esErrorNoAutenticado(err)) {
          incrementarInstanciaSesion();
          setUsuario(null);
          setEstadoSesion('SIN_SESION');
          setMensajeInactividad('Sesión finalizada por inactividad o expiración.');
        }
      });
    };

    window.addEventListener('keydown', capturarInteraccionHumana);
    window.addEventListener('mousemove', capturarInteraccionHumana);
    window.addEventListener('mousedown', capturarInteraccionHumana);
    window.addEventListener('touchstart', capturarInteraccionHumana);

    return () => {
      window.removeEventListener('keydown', capturarInteraccionHumana);
      window.removeEventListener('mousemove', capturarInteraccionHumana);
      window.removeEventListener('mousedown', capturarInteraccionHumana);
      window.removeEventListener('touchstart', capturarInteraccionHumana);
    };
  }, [estadoSesion]);

  // Consultar la sesión nunca renueva actividad humana.
  useEffect(() => {
    if (estadoSesion === 'SIN_SESION') return;

    const interval = setInterval(async () => {
      const localInstanceId = sessionInstanceIdRef.current;
      try {
        const sesionActiva = await apiObtenerSesion();
        if (localInstanceId !== sessionInstanceIdRef.current) return;

        const env = sesionActiva.ambiente_actual || 'DESCONOCIDO';
        setAmbienteActual(env);
        setEstadoSesion(sesionActiva.estado_sesion as EstadoSesionEnum);
        setUsuario((prev) => {
          if (!prev) return null;
          return {
            ...prev,
            nombre_completo: sesionActiva.nombre_completo,
            rol: sesionActiva.rol,
            permisos: sesionActiva.permisos,
            estado_sesion: sesionActiva.estado_sesion as EstadoSesionEnum,
            debe_cambiar_clave: sesionActiva.estado_sesion === 'CAMBIO_OBLIGATORIO',
            ambiente_actual: env,
          };
        });
      } catch (err: unknown) {
        if (localInstanceId !== sessionInstanceIdRef.current) return;

        if (esErrorNoAutenticado(err)) {
          incrementarInstanciaSesion();
          setUsuario(null);
          setEstadoSesion('SIN_SESION');
          setMensajeInactividad('Sesión finalizada por inactividad o revocación.');
        }
      }
    }, 10000);

    return () => clearInterval(interval);
  }, [estadoSesion]);

  const iniciarSesion = async (user: string, pass: string): Promise<UsuarioSesion> => {
    if ([...pass].length > 128) {
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

      const env = datos.ambiente_actual || 'DESCONOCIDO';
      setUsuario(datos);
      setAmbienteActual(env);
      setEstadoSesion(datos.estado_sesion as EstadoSesionEnum);
      setHayCambiosSinGuardar(false);
      sesionActividadRef.current = datos.session_id;
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
    const localInstanceId = incrementarInstanciaSesion();
    try {
      await apiCerrarSesion();
    } catch {
      // Ignorar errores al cerrar sesión
    } finally {
      if (localInstanceId !== sessionInstanceIdRef.current) return;
      setUsuario(null);
      setEstadoSesion('SIN_SESION');
      setHayCambiosSinGuardar(false);
      setMensajeInactividad(null);
      try {
        const envPreLogin = await apiObtenerAmbienteInicial();
        if (localInstanceId === sessionInstanceIdRef.current) setAmbienteActual(envPreLogin);
      } catch {
        if (localInstanceId === sessionInstanceIdRef.current) setAmbienteActual('ERROR_CONEXION');
      }
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
