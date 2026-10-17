import React from 'react';
import { User, Lock, Eye, EyeOff, ArrowRight } from 'lucide-react';
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
            fontWeight: '800',
            color: '#F8FAFC',
            marginBottom: '6px',
            letterSpacing: '-0.5px',
          }}
        >
          Bienvenido de nuevo
        </h2>
        <p style={{ fontSize: '14px', color: '#94A3B8', margin: 0 }}>
          Introduce tus credenciales para iniciar sesión.
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
              maxLength={128}
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
          <span>{isLoggingIn ? 'Iniciando sesión...' : 'Ingresar al ERP'}</span>
          <ArrowRight size={18} />
        </button>
      </form>

      <p
        style={{
          fontSize: '12px',
          color: '#64748B',
          textAlign: 'center',
          marginTop: '16px',
          marginBottom: '8px',
        }}
      >
        Consulte con el administrador si no puede ingresar.
      </p>

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
          style={{
            background: 'none',
            border: 'none',
            color: '#38BDF8',
            fontSize: '13px',
            cursor: isLoggingIn ? 'not-allowed' : 'pointer',
            fontWeight: '500',
            outline: '2px solid transparent',
            outlineOffset: '2px',
          }}
        >
          ¿Olvidaste la clave de Administrador?
        </button>
      </div>
    </>
  );
};
