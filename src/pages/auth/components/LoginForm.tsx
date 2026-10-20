import React from 'react';
import { User, Lock, Eye, EyeOff } from 'lucide-react';
import { loginStyles as s } from '../../../styles/loginStyles';

interface LoginFormProps {
  usuarioInput: string;
  setUsuarioInput: (val: string) => void;
  passwordInput: string;
  setPasswordInput: (val: string) => void;
  isLoggingIn: boolean;
  mostrarPassword: boolean;
  setMostrarPassword: (val: boolean) => void;
  usuarioInputRef: React.RefObject<HTMLInputElement | null>;
  onSubmit: (e: React.FormEvent) => void;
  onIrARecuperacion: () => void;
}

export const LoginForm: React.FC<LoginFormProps> = ({
  usuarioInput,
  setUsuarioInput,
  passwordInput,
  setPasswordInput,
  isLoggingIn,
  mostrarPassword,
  setMostrarPassword,
  usuarioInputRef,
  onSubmit,
  onIrARecuperacion,
}) => {
  return (
    <>
      <div style={{ marginBottom: '24px' }}>
        <h2
          style={{
            fontSize: '24px',
            fontWeight: '600',
            color: '#F8FAFC',
            marginBottom: '6px',
            letterSpacing: '-0.5px',
          }}
        >
          Iniciar sesión
        </h2>
        <p style={{ fontSize: '14px', color: '#94A3B8', margin: 0 }}>
          Ingresa tu usuario y contraseña.
        </p>
      </div>

      <form onSubmit={onSubmit}>
        <div style={{ marginBottom: '18px' }}>
          <label htmlFor="login-username" style={s.label}>
            Usuario
          </label>
          <div style={{ position: 'relative', display: 'flex', alignItems: 'center' }}>
            <User
              size={18}
              color="#64748B"
              style={{ position: 'absolute', left: '14px', pointerEvents: 'none' }}
            />
            <input
              id="login-username"
              ref={usuarioInputRef}
              type="text"
              disabled={isLoggingIn}
              value={usuarioInput}
              onChange={(e) => setUsuarioInput(e.target.value)}
              placeholder="Ej. admin"
              style={s.inputWithIcon}
              autoComplete="username"
            />
          </div>
        </div>

        <div style={{ marginBottom: '22px' }}>
          <label htmlFor="login-password" style={s.label}>
            Contraseña
          </label>
          <div style={{ position: 'relative', display: 'flex', alignItems: 'center' }}>
            <Lock
              size={18}
              color="#64748B"
              style={{ position: 'absolute', left: '14px', pointerEvents: 'none' }}
            />
            <input
              id="login-password"
              type={mostrarPassword ? 'text' : 'password'}
              disabled={isLoggingIn}
              value={passwordInput}
              onChange={(e) => setPasswordInput(e.target.value)}
              placeholder="••••••••"
              maxLength={256}
              style={s.inputWithIcon}
              autoComplete="current-password"
            />
            <button
              type="button"
              disabled={isLoggingIn}
              onClick={() => setMostrarPassword(!mostrarPassword)}
              style={s.toggleEyeBtn}
              title={mostrarPassword ? 'Ocultar contraseña' : 'Mostrar contraseña'}
              aria-label={mostrarPassword ? 'Ocultar contraseña' : 'Mostrar contraseña'}
            >
              {mostrarPassword ? <EyeOff size={18} /> : <Eye size={18} />}
            </button>
          </div>
        </div>

        <button type="submit" disabled={isLoggingIn} style={s.submitButton}>
          <span>{isLoggingIn ? 'Iniciando sesión...' : 'Ingresar'}</span>
        </button>
      </form>

      <div
        style={{
          marginTop: '12px',
          textAlign: 'center',
          display: 'flex',
          flexDirection: 'column',
          gap: '8px',
        }}
      >
        <button
          type="button"
          disabled={isLoggingIn}
          onClick={onIrARecuperacion}
          title="Recuperar acceso de administrador con el código de recuperación"
          style={{
            background: 'none',
            border: 'none',
            color: '#B5C4D8',
            fontSize: '13px',
            cursor: isLoggingIn ? 'not-allowed' : 'pointer',
            fontWeight: '500',
            outline: '2px solid transparent',
            outlineOffset: '2px',
          }}
        >
          ¿No puedes ingresar?
        </button>
      </div>
    </>
  );
};
