import React, { useState, useEffect, useRef } from 'react';
import { useAuth } from '../../context/AuthContext';
import { ShieldAlert } from 'lucide-react';
import { loginStyles as s } from '../../styles/loginStyles';
import {
  apiCambiarClaveObligatoria,
  apiInstalacionInicial,
  apiRecuperarAccesoAdmin,
  apiVerificarInstalacion,
} from '../../services/authService';

import { LoginBrandPanel } from './components/LoginBrandPanel';
import { LoginForm } from './components/LoginForm';
import { LoginPasswordChange } from './components/LoginPasswordChange';
import { LoginInstallForm } from './components/LoginInstallForm';
import { LoginRecoverForm } from './components/LoginRecoverForm';

type ModoPantalla = 'LOGIN' | 'INSTALACION' | 'RECUPERACION' | 'CAMBIO_OBLIGATORIO';

export const LoginPage: React.FC = () => {
  const {
    iniciarSesion,
    isLoggingIn,
    appVersion,
    estadoSesion,
    cerrarSesion,
    completarCambioObligatorio,
    mensajeInactividad,
    limpiarMensajeInactividad,
  } = useAuth();

  const usuarioInputRef = useRef<HTMLInputElement>(null);

  const [modo, setModo] = useState<ModoPantalla>('LOGIN');
  const [mostrarPassword, setMostrarPassword] = useState(false);

  // Formulario Login
  const [usuarioInput, setUsuarioInput] = useState('');
  const [passwordInput, setPasswordInput] = useState('');

  // Formulario Cambio Obligatorio
  const [nuevaClaveOblig, setNuevaClaveOblig] = useState('');
  const [confirmClaveOblig, setConfirmClaveOblig] = useState('');

  // Formulario Instalación Inicial
  const [instNombre, setInstNombre] = useState('');
  const [instUsername, setInstUsername] = useState('');
  const [instPassword, setInstPassword] = useState('');
  const [instConfirmPassword, setInstConfirmPassword] = useState('');
  const [codigoGenerado, setCodigoGenerado] = useState<string | null>(null);
  const [codigoGuardadoConfirmado, setCodigoGuardadoConfirmado] = useState(false);
  const [copiado, setCopiado] = useState(false);

  // Formulario Recuperación Admin
  const [recUsername, setRecUsername] = useState('');
  const [recCodigo, setRecCodigo] = useState('');
  const [recNuevaPassword, setRecNuevaPassword] = useState('');
  const [recConfirmNuevaPassword, setRecConfirmNuevaPassword] = useState('');
  const [nuevoCodigoTrasRecuperar, setNuevoCodigoTrasRecuperar] = useState<string | null>(null);

  // Notificaciones y spinners
  const [errorMsg, setErrorMsg] = useState<string | null>(null);
  const [exitoMsg, setExitoMsg] = useState<string | null>(null);
  const [procesando, setProcesando] = useState(false);

  const cambiarModo = (nuevoModo: ModoPantalla) => {
    setErrorMsg(null);
    setExitoMsg(null);
    setMostrarPassword(false);

    setUsuarioInput('');
    setPasswordInput('');
    setNuevaClaveOblig('');
    setConfirmClaveOblig('');
    setInstNombre('');
    setInstUsername('');
    setInstPassword('');
    setInstConfirmPassword('');
    setCodigoGenerado(null);
    setCodigoGuardadoConfirmado(false);
    setRecUsername('');
    setRecCodigo('');
    setRecNuevaPassword('');
    setRecConfirmNuevaPassword('');
    setNuevoCodigoTrasRecuperar(null);

    setModo(nuevoModo);
  };

  useEffect(() => {
    if (estadoSesion === 'CAMBIO_OBLIGATORIO') {
      setModo('CAMBIO_OBLIGATORIO');
      return;
    }

    if (estadoSesion === 'SIN_SESION') {
      cambiarModo('LOGIN');

      const verificarArranque = async () => {
        try {
          const necesitaInstalacion = await apiVerificarInstalacion();
          if (necesitaInstalacion) {
            setModo('INSTALACION');
          }
        } catch {
          // Se conserva modo LOGIN
        }
      };

      verificarArranque();
    }
  }, [estadoSesion]);

  useEffect(() => {
    if (modo === 'LOGIN' && usuarioInputRef.current) {
      usuarioInputRef.current.focus();
    }
  }, [modo]);

  const handleLoginSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setErrorMsg(null);
    setExitoMsg(null);
    limpiarMensajeInactividad();

    const userClean = usuarioInput.trim();
    if (!userClean || !passwordInput) {
      setErrorMsg('Por favor, ingresa tu usuario y contraseña.');
      return;
    }

    if (passwordInput.length > 128) {
      setErrorMsg('La contraseña no puede exceder los 128 caracteres.');
      return;
    }

    try {
      const datos = await iniciarSesion(userClean, passwordInput);
      if (datos.estado_sesion === 'CAMBIO_OBLIGATORIO') {
        setModo('CAMBIO_OBLIGATORIO');
      }
    } catch (err: any) {
      const mensaje = typeof err === 'string' ? err : err?.message || 'Credenciales inválidas.';
      setErrorMsg(mensaje);
    }
  };

  const handleCambioObligatorioSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setErrorMsg(null);
    setExitoMsg(null);

    if (!nuevaClaveOblig.trim() || !confirmClaveOblig.trim()) {
      setErrorMsg('Todos los campos son obligatorios.');
      return;
    }

    if (nuevaClaveOblig !== confirmClaveOblig) {
      setErrorMsg('La confirmación de la nueva contraseña no coincide.');
      return;
    }

    if (nuevaClaveOblig.length < 8 || nuevaClaveOblig.length > 128) {
      setErrorMsg('La nueva contraseña debe tener entre 8 y 128 caracteres.');
      return;
    }

    const tieneLetra = /[a-zA-Z]/.test(nuevaClaveOblig);
    const tieneNumero = /[0-9]/.test(nuevaClaveOblig);
    if (!tieneLetra || !tieneNumero) {
      setErrorMsg('La nueva contraseña debe incluir al menos una letra y un número.');
      return;
    }

    setProcesando(true);
    try {
      await apiCambiarClaveObligatoria({ nueva_password: nuevaClaveOblig });
      completarCambioObligatorio();
    } catch (err: any) {
      const mensaje =
        typeof err === 'string' ? err : err?.message || 'Error al actualizar la contraseña.';
      setErrorMsg(mensaje);
    } finally {
      setProcesando(false);
    }
  };

  const handleInstalacionSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setErrorMsg(null);
    setExitoMsg(null);

    if (
      !instNombre.trim() ||
      !instUsername.trim() ||
      !instPassword.trim() ||
      !instConfirmPassword.trim()
    ) {
      setErrorMsg('Todos los campos son obligatorios.');
      return;
    }

    if (instPassword !== instConfirmPassword) {
      setErrorMsg('Las contraseñas ingresadas no coinciden.');
      return;
    }

    if (instPassword.length < 8 || instPassword.length > 128) {
      setErrorMsg('La contraseña debe tener entre 8 y 128 caracteres.');
      return;
    }

    const tieneLetra = /[a-zA-Z]/.test(instPassword);
    const tieneNumero = /[0-9]/.test(instPassword);
    if (!tieneLetra || !tieneNumero) {
      setErrorMsg('La contraseña debe incluir al menos una letra y un número.');
      return;
    }

    setProcesando(true);
    try {
      const res = await apiInstalacionInicial({
        nombre_completo: instNombre.trim(),
        username: instUsername.trim(),
        password: instPassword,
      });
      setCodigoGenerado(res.codigo_recuperacion);
      setExitoMsg('Instalación realizada con éxito. Guarda bien tu código de recuperación.');
    } catch (err: any) {
      const mensaje =
        typeof err === 'string' ? err : err?.message || 'Error al realizar la instalación inicial.';
      setErrorMsg(mensaje);
    } finally {
      setProcesando(false);
    }
  };

  const copiarCodigo = (codigo: string) => {
    navigator.clipboard.writeText(codigo);
    setCopiado(true);
    setTimeout(() => setCopiado(false), 3000);
  };

  const handleRecuperarSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setErrorMsg(null);
    setExitoMsg(null);

    if (
      !recUsername.trim() ||
      !recCodigo.trim() ||
      !recNuevaPassword.trim() ||
      !recConfirmNuevaPassword.trim()
    ) {
      setErrorMsg('Todos los campos son obligatorios.');
      return;
    }

    if (recNuevaPassword !== recConfirmNuevaPassword) {
      setErrorMsg('La confirmación de la nueva contraseña no coincide.');
      return;
    }

    setProcesando(true);
    try {
      const res = await apiRecuperarAccesoAdmin({
        username: recUsername.trim(),
        codigo_recuperacion: recCodigo.trim(),
        nueva_password: recNuevaPassword,
      });
      setNuevoCodigoTrasRecuperar(res.codigo_recuperacion);
      setExitoMsg(
        'Acceso restablecido exitosamente. Tu nuevo código de recuperación ha sido generado.'
      );
    } catch (err: any) {
      const mensaje =
        typeof err === 'string'
          ? err
          : err?.message || 'No se pudo recuperar el acceso con los datos ingresados.';
      setErrorMsg(mensaje);
    } finally {
      setProcesando(false);
    }
  };

  return (
    <div style={s.container}>
      <LoginBrandPanel appVersion={appVersion} />

      <div style={s.rightPanel}>
        <div style={s.card}>
          {mensajeInactividad && (
            <div
              style={{
                display: 'flex',
                alignItems: 'center',
                gap: '12px',
                padding: '12px 14px',
                backgroundColor: 'rgba(234, 179, 8, 0.15)',
                border: '1px solid rgba(234, 179, 8, 0.4)',
                color: '#FDE047',
                borderRadius: '12px',
                fontSize: '13px',
                marginBottom: '20px',
                fontWeight: '600',
              }}
            >
              <ShieldAlert size={18} style={{ flexShrink: 0 }} />
              <span>{mensajeInactividad}</span>
            </div>
          )}

          {errorMsg && (
            <div
              style={{
                display: 'flex',
                alignItems: 'center',
                gap: '12px',
                padding: '12px 14px',
                backgroundColor: 'rgba(220, 38, 38, 0.15)',
                border: '1px solid rgba(239, 68, 68, 0.4)',
                color: '#FCA5A5',
                borderRadius: '12px',
                fontSize: '13px',
                marginBottom: '20px',
                fontWeight: '600',
              }}
            >
              <ShieldAlert size={18} style={{ flexShrink: 0 }} />
              <span>{errorMsg}</span>
            </div>
          )}

          {exitoMsg && modo !== 'LOGIN' && (
            <div
              style={{
                ...s.warningCard,
                backgroundColor: 'rgba(34, 197, 94, 0.15)',
                borderColor: 'rgba(34, 197, 94, 0.4)',
                color: '#86EFAC',
                marginBottom: '20px',
              }}
            >
              {exitoMsg}
            </div>
          )}

          {modo === 'LOGIN' && (
            <LoginForm
              usuarioInput={usuarioInput}
              setUsuarioInput={setUsuarioInput}
              passwordInput={passwordInput}
              setPasswordInput={setPasswordInput}
              isLoggingIn={isLoggingIn}
              mostrarPassword={mostrarPassword}
              setMostrarPassword={setMostrarPassword}
              usuarioInputRef={usuarioInputRef}
              onSubmit={handleLoginSubmit}
              onIrARecuperacion={() => cambiarModo('RECUPERACION')}
            />
          )}

          {modo === 'CAMBIO_OBLIGATORIO' && (
            <LoginPasswordChange
              nuevaClaveOblig={nuevaClaveOblig}
              setNuevaClaveOblig={setNuevaClaveOblig}
              confirmClaveOblig={confirmClaveOblig}
              setConfirmClaveOblig={setConfirmClaveOblig}
              procesando={procesando}
              onSubmit={handleCambioObligatorioSubmit}
              onCerrarSesion={cerrarSesion}
            />
          )}

          {modo === 'INSTALACION' && (
            <LoginInstallForm
              instNombre={instNombre}
              setInstNombre={setInstNombre}
              instUsername={instUsername}
              setInstUsername={setInstUsername}
              instPassword={instPassword}
              setInstPassword={setInstPassword}
              instConfirmPassword={instConfirmPassword}
              setInstConfirmPassword={setInstConfirmPassword}
              procesando={procesando}
              codigoGenerado={codigoGenerado}
              codigoGuardadoConfirmado={codigoGuardadoConfirmado}
              setCodigoGuardadoConfirmado={setCodigoGuardadoConfirmado}
              copiado={copiado}
              onCopiarCodigo={copiarCodigo}
              onSubmit={handleInstalacionSubmit}
              onVolverLogin={() => cambiarModo('LOGIN')}
            />
          )}

          {modo === 'RECUPERACION' && (
            <LoginRecoverForm
              recUsername={recUsername}
              setRecUsername={setRecUsername}
              recCodigo={recCodigo}
              setRecCodigo={setRecCodigo}
              recNuevaPassword={recNuevaPassword}
              setRecNuevaPassword={setRecNuevaPassword}
              recConfirmNuevaPassword={recConfirmNuevaPassword}
              setRecConfirmNuevaPassword={setRecConfirmNuevaPassword}
              procesando={procesando}
              nuevoCodigoTrasRecuperar={nuevoCodigoTrasRecuperar}
              copiado={copiado}
              onCopiarCodigo={copiarCodigo}
              onSubmit={handleRecuperarSubmit}
              onVolverLogin={() => cambiarModo('LOGIN')}
            />
          )}
        </div>
      </div>
    </div>
  );
};
