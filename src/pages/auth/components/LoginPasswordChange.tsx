import React from 'react';
import { LogOut } from 'lucide-react';
import { loginStyles as s } from '../../../styles/loginStyles';

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
      <div style={{ marginBottom: '24px' }}>
        <h2
          style={{
            fontSize: '22px',
            fontWeight: '800',
            color: '#F8FAFC',
            marginBottom: '6px',
          }}
        >
          Cambio Obligatorio de Contraseña
        </h2>
        <p style={{ fontSize: '13px', color: '#94A3B8', margin: 0 }}>
          Has ingresado con una clave temporal. Por seguridad, debes definir una nueva contraseña.
        </p>
      </div>

      <form onSubmit={onSubmit}>
        <div style={{ marginBottom: '14px' }}>
          <label htmlFor="oblig-nueva-password" style={s.label}>
            Nueva Contraseña
          </label>
          <input
            id="oblig-nueva-password"
            type="password"
            disabled={procesando}
            value={nuevaClaveOblig}
            onChange={(e) => setNuevaClaveOblig(e.target.value)}
            placeholder="7 a 128 caracteres (letras y números)"
            maxLength={128}
            style={s.inputBox}
            autoComplete="new-password"
          />
        </div>

        <div style={{ marginBottom: '20px' }}>
          <label htmlFor="oblig-confirm-password" style={s.label}>
            Confirmar Nueva Contraseña
          </label>
          <input
            id="oblig-confirm-password"
            type="password"
            disabled={procesando}
            value={confirmClaveOblig}
            onChange={(e) => setConfirmClaveOblig(e.target.value)}
            placeholder="Repite la nueva contraseña"
            maxLength={128}
            style={s.inputBox}
            autoComplete="new-password"
          />
        </div>

        <button type="submit" disabled={procesando} style={s.submitButton}>
          {procesando ? 'Guardando...' : 'Guardar Contraseña y Continuar'}
        </button>

        <button
          type="button"
          disabled={procesando}
          onClick={handleCerrarSesionLimpio}
          style={{
            ...s.secondaryButton,
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            gap: '8px',
            marginTop: '12px',
          }}
        >
          <LogOut size={16} />
          <span>Cerrar Sesión</span>
        </button>
      </form>
    </>
  );
};
