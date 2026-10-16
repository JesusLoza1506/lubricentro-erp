import React from 'react';
import { KeyRound, Copy, CheckCircle2 } from 'lucide-react';
import { loginStyles as s } from '../../../styles/loginStyles';

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
          Recuperar Administrador
        </h2>
        <p style={{ fontSize: '13px', color: '#94A3B8', margin: 0 }}>
          Restaura el acceso del usuario Administrador utilizando tu código de recuperación.
        </p>
      </div>

      {!nuevoCodigoTrasRecuperar ? (
        <form onSubmit={onSubmit}>
          <div style={{ marginBottom: '14px' }}>
            <label style={s.label}>Usuario Administrador</label>
            <input
              type="text"
              disabled={procesando}
              value={recUsername}
              onChange={(e) => setRecUsername(e.target.value)}
              placeholder="Ej. admin"
              style={s.inputBox}
            />
          </div>

          <div style={{ marginBottom: '14px' }}>
            <label style={s.label}>Código de Recuperación</label>
            <div style={{ position: 'relative', display: 'flex', alignItems: 'center' }}>
              <KeyRound
                size={18}
                color="#64748B"
                style={{ position: 'absolute', left: '14px', pointerEvents: 'none' }}
              />
              <input
                type="text"
                disabled={procesando}
                value={recCodigo}
                onChange={(e) => setRecCodigo(e.target.value)}
                placeholder="XXXX-XXXX-XXXX-XXXX-XXXX-XXXX-XXXX-XXXX"
                style={s.inputWithIcon}
              />
            </div>
          </div>

          <div style={{ marginBottom: '14px' }}>
            <label style={s.label}>Nueva Contraseña</label>
            <input
              type="password"
              disabled={procesando}
              value={recNuevaPassword}
              onChange={(e) => setRecNuevaPassword(e.target.value)}
              placeholder="Mínimo 8 caracteres (letras y números)"
              maxLength={128}
              style={s.inputBox}
            />
          </div>

          <div style={{ marginBottom: '20px' }}>
            <label style={s.label}>Confirmar Nueva Contraseña</label>
            <input
              type="password"
              disabled={procesando}
              value={recConfirmNuevaPassword}
              onChange={(e) => setRecConfirmNuevaPassword(e.target.value)}
              placeholder="Repite la nueva contraseña"
              maxLength={128}
              style={s.inputBox}
            />
          </div>

          <button type="submit" disabled={procesando} style={s.submitButton}>
            {procesando ? 'Restableciendo...' : 'Restablecer Clave de Admin'}
          </button>

          <button
            type="button"
            disabled={procesando}
            onClick={onVolverLogin}
            style={s.secondaryButton}
          >
            Volver al Login
          </button>
        </form>
      ) : (
        <div>
          <p style={{ fontSize: '13px', color: '#E2E8F0', marginBottom: '8px' }}>
            <strong>NUEVO CÓDIGO DE RECUPERACIÓN GENERADO:</strong>
          </p>
          <p style={{ fontSize: '12px', color: '#94A3B8', marginBottom: '12px' }}>
            Al restablecer el acceso, tu código anterior ha quedado invalidado. Guarda este nuevo
            código:
          </p>

          <div style={s.codeBox}>{nuevoCodigoTrasRecuperar}</div>

          <button
            type="button"
            onClick={() => onCopiarCodigo(nuevoCodigoTrasRecuperar)}
            style={{
              ...s.secondaryButton,
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              gap: '8px',
              color: copiado ? '#34D399' : '#94A3B8',
            }}
          >
            {copiado ? <CheckCircle2 size={16} /> : <Copy size={16} />}
            {copiado ? '¡Copiado al Portapapeles!' : 'Copiar Nuevo Código'}
          </button>

          <button
            type="button"
            onClick={onVolverLogin}
            style={{ ...s.submitButton, marginTop: '20px' }}
          >
            Ir a Iniciar Sesión
          </button>
        </div>
      )}
    </>
  );
};
