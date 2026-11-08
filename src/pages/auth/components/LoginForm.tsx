import React from 'react';
import { User, Lock, Eye, EyeOff } from 'lucide-react';
import { AccessField, AccessHeading, AccessSubmit } from './AccessUi';

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
      <AccessHeading
        label="Acceso al sistema"
        title="Iniciar sesión"
        description="Ingresa con tu cuenta del taller."
      />
      <form className="gloria-form" onSubmit={onSubmit}>
        <AccessField
          id="login-username"
          label="Usuario"
          type="text"
          disabled={isLoggingIn}
          inputRef={usuarioInputRef}
          value={usuarioInput}
          onChange={(e) => setUsuarioInput(e.target.value)}
          placeholder="Ej. admin"
          autoComplete="username"
          icon={<User size={16} />}
        />
        <AccessField
          id="login-password"
          label="Contraseña"
          type={mostrarPassword ? 'text' : 'password'}
          disabled={isLoggingIn}
          value={passwordInput}
          onChange={(e) => setPasswordInput(e.target.value)}
          placeholder="••••••••"
          maxLength={256}
          autoComplete="current-password"
          icon={<Lock size={16} />}
          action={
            <button
              type="button"
              className="gt-button gt-button--icon gt-field-action"
              disabled={isLoggingIn}
              onClick={() => setMostrarPassword(!mostrarPassword)}
              title={mostrarPassword ? 'Ocultar contraseña' : 'Mostrar contraseña'}
              aria-label={mostrarPassword ? 'Ocultar contraseña' : 'Mostrar contraseña'}
            >
              {mostrarPassword ? (
                <EyeOff size={16} aria-hidden="true" />
              ) : (
                <Eye size={16} aria-hidden="true" />
              )}
            </button>
          }
        />
        <div className="gloria-form-actions">
          <AccessSubmit busy={isLoggingIn} busyLabel="Iniciando sesión…">
            Ingresar
          </AccessSubmit>
          <button
            type="button"
            className="gt-button gt-button--quiet"
            disabled={isLoggingIn}
            onClick={onIrARecuperacion}
          >
            ¿No puedes ingresar?
          </button>
        </div>
      </form>
    </>
  );
};
