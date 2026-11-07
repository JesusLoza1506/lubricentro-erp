import React from 'react';
import { KeyRound } from 'lucide-react';
import { AccessField, AccessHeading, AccessSubmit, RecoveryCode } from './AccessUi';

interface LoginRecoverFormProps {
  recUsername: string;
  setRecUsername: (val: string) => void;
  recCodigo: string;
  setRecCodigo: (val: string) => void;
  recNuevaPassword: string;
  setRecNuevaPassword: (val: string) => void;
  recConfirmNuevaPassword: string;
  setRecConfirmNuevaPassword: (val: string) => void;
  procesando: boolean;
  nuevoCodigoTrasRecuperar: string | null;
  copiado: boolean;
  onCopiarCodigo: (codigo: string) => void;
  onSubmit: (e: React.FormEvent) => void;
  onVolverLogin: () => void;
}

export const LoginRecoverForm: React.FC<LoginRecoverFormProps> = ({
  recUsername,
  setRecUsername,
  recCodigo,
  setRecCodigo,
  recNuevaPassword,
  setRecNuevaPassword,
  recConfirmNuevaPassword,
  setRecConfirmNuevaPassword,
  procesando,
  nuevoCodigoTrasRecuperar,
  copiado,
  onCopiarCodigo,
  onSubmit,
  onVolverLogin,
}) => {
  return (
    nuevoCodigoTrasRecuperar ? (
      <>
        <AccessHeading label="Recuperación" title="Acceso restablecido" focus
          description="El código anterior ya no es válido. Guarda este nuevo código en un lugar seguro." />
        <RecoveryCode code={nuevoCodigoTrasRecuperar} copied={copiado} onCopy={onCopiarCodigo}
          onContinue={onVolverLogin} />
      </>
    ) : (
      <>
        <AccessHeading label="Recuperación" title="Recuperar acceso" focus
          description="Usa el código de recuperación de la cuenta de administrador." />
        <form className="gloria-form" onSubmit={onSubmit}>
          <AccessField id="rec-username" label="Usuario administrador" type="text" disabled={procesando}
            value={recUsername} onChange={(e) => setRecUsername(e.target.value)} autoComplete="username" placeholder="Ej. admin" />
          <AccessField id="rec-codigo" label="Código de recuperación" type="text" disabled={procesando}
            value={recCodigo} onChange={(e) => setRecCodigo(e.target.value)} autoComplete="off" placeholder="Pega tu código completo" icon={<KeyRound size={16} />} />
          <AccessField id="rec-nueva-password" label="Nueva contraseña" type="password" disabled={procesando}
            value={recNuevaPassword} onChange={(e) => setRecNuevaPassword(e.target.value)} autoComplete="new-password" maxLength={256} hint="De 8 a 128 caracteres, con al menos una letra y un número." />
          <AccessField id="rec-confirm-password" label="Confirmar nueva contraseña" type="password" disabled={procesando}
            value={recConfirmNuevaPassword} onChange={(e) => setRecConfirmNuevaPassword(e.target.value)} autoComplete="new-password" maxLength={256} />
          <div className="gloria-form-actions">
            <AccessSubmit busy={procesando} busyLabel="Restableciendo…">Restablecer acceso</AccessSubmit>
            <button type="button" className="gt-button gt-button--quiet" disabled={procesando}
              onClick={onVolverLogin}>Volver al inicio de sesión</button>
          </div>
        </form>
      </>
    )
  );
};
