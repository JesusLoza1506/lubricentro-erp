import React from 'react';
import { LogOut } from 'lucide-react';
import { AccessField, AccessHeading, AccessSubmit } from './AccessUi';

interface LoginPasswordChangeProps {
  nuevaClaveOblig: string;
  setNuevaClaveOblig: (val: string) => void;
  confirmClaveOblig: string;
  setConfirmClaveOblig: (val: string) => void;
  procesando: boolean;
  onSubmit: (e: React.FormEvent) => void;
  onCerrarSesion: () => void;
}

export const LoginPasswordChange: React.FC<LoginPasswordChangeProps> = ({
  nuevaClaveOblig,
  setNuevaClaveOblig,
  confirmClaveOblig,
  setConfirmClaveOblig,
  procesando,
  onSubmit,
  onCerrarSesion,
}) => {
  const handleCerrarSesionLimpio = () => {
    // Purga de campos al cancelar/cerrar sesión
    setNuevaClaveOblig('');
    setConfirmClaveOblig('');
    onCerrarSesion();
  };

  return (
    <>
      <AccessHeading label="Seguridad de la cuenta" title="Actualiza tu contraseña" focus
        description="Ingresaste con una contraseña temporal. Elige una nueva para continuar." />
      <form className="gloria-form" onSubmit={onSubmit}>
          <AccessField id="oblig-nueva-password" label="Nueva contraseña" type="password" disabled={procesando}
            value={nuevaClaveOblig} onChange={(e) => setNuevaClaveOblig(e.target.value)} autoComplete="new-password" maxLength={256} hint="De 8 a 128 caracteres, con al menos una letra y un número." />
          <AccessField id="oblig-confirm-password" label="Confirmar nueva contraseña" type="password" disabled={procesando}
            value={confirmClaveOblig} onChange={(e) => setConfirmClaveOblig(e.target.value)} autoComplete="new-password" maxLength={256} />
        <div className="gloria-form-actions">
          <AccessSubmit busy={procesando} busyLabel="Guardando…">Guardar contraseña y continuar</AccessSubmit>
          <button type="button" className="gt-button gt-button--quiet" disabled={procesando}
            onClick={handleCerrarSesionLimpio}>
            <LogOut size={16} aria-hidden="true" /> Cerrar sesión
          </button>
        </div>
      </form>
    </>
  );
};
