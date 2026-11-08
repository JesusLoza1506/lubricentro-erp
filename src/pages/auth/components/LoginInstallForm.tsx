import React from 'react';
import { AccessField, AccessHeading, AccessSubmit, RecoveryCode } from './AccessUi';

interface LoginInstallFormProps {
  instNombre: string;
  setInstNombre: (val: string) => void;
  instUsername: string;
  setInstUsername: (val: string) => void;
  instPassword: string;
  setInstPassword: (val: string) => void;
  instConfirmPassword: string;
  setInstConfirmPassword: (val: string) => void;
  procesando: boolean;
  codigoGenerado: string | null;
  codigoGuardadoConfirmado: boolean;
  setCodigoGuardadoConfirmado: (val: boolean) => void;
  copiado: boolean;
  onCopiarCodigo: (codigo: string) => void;
  onSubmit: (e: React.FormEvent) => void;
  onVolverLogin: () => void;
}

export const LoginInstallForm: React.FC<LoginInstallFormProps> = ({
  instNombre,
  setInstNombre,
  instUsername,
  setInstUsername,
  instPassword,
  setInstPassword,
  instConfirmPassword,
  setInstConfirmPassword,
  procesando,
  codigoGenerado,
  codigoGuardadoConfirmado,
  setCodigoGuardadoConfirmado,
  copiado,
  onCopiarCodigo,
  onSubmit,
  onVolverLogin,
}) => {
  return codigoGenerado ? (
    <>
      <AccessHeading
        label="Instalación inicial"
        title="Guarda tu código"
        focus
        description="Solo se muestra esta vez. Te servirá para recuperar el acceso de administrador."
      />
      <RecoveryCode
        code={codigoGenerado}
        copied={copiado}
        onCopy={onCopiarCodigo}
        onContinue={onVolverLogin}
        confirmation={{ checked: codigoGuardadoConfirmado, onChange: setCodigoGuardadoConfirmado }}
      />
    </>
  ) : (
    <>
      <AccessHeading
        label="Instalación inicial"
        title="Crear administrador"
        focus
        description="Configura la cuenta principal de tu taller."
      />
      <form className="gloria-form" onSubmit={onSubmit}>
        <AccessField
          id="inst-nombre"
          label="Nombre completo"
          type="text"
          disabled={procesando}
          value={instNombre}
          onChange={(e) => setInstNombre(e.target.value)}
          autoComplete="name"
          placeholder="Ej. Administrador principal"
        />
        <AccessField
          id="inst-username"
          label="Usuario"
          type="text"
          disabled={procesando}
          value={instUsername}
          onChange={(e) => setInstUsername(e.target.value)}
          autoComplete="username"
          placeholder="Ej. admin"
        />
        <AccessField
          id="inst-password"
          label="Contraseña"
          type="password"
          disabled={procesando}
          value={instPassword}
          onChange={(e) => setInstPassword(e.target.value)}
          autoComplete="new-password"
          maxLength={256}
          hint="De 8 a 128 caracteres, con al menos una letra y un número."
        />
        <AccessField
          id="inst-confirm-password"
          label="Confirmar contraseña"
          type="password"
          disabled={procesando}
          value={instConfirmPassword}
          onChange={(e) => setInstConfirmPassword(e.target.value)}
          autoComplete="new-password"
          maxLength={256}
        />
        <div className="gloria-form-actions">
          <AccessSubmit busy={procesando} busyLabel="Configurando…">
            Crear administrador
          </AccessSubmit>
          <button
            type="button"
            className="gt-button gt-button--quiet"
            disabled={procesando}
            onClick={onVolverLogin}
          >
            Volver al inicio de sesión
          </button>
        </div>
      </form>
    </>
  );
};
