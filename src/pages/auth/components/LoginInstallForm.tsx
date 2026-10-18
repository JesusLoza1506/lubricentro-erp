import React from 'react';
import { Copy, CheckCircle2 } from 'lucide-react';
import { loginStyles as s } from '../../../styles/loginStyles';

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
          Instalación Inicial
        </h2>
        <p style={{ fontSize: '13px', color: '#94A3B8', margin: 0 }}>
          Configura la cuenta principal de Administrador para tu sistema.
        </p>
      </div>

      {!codigoGenerado ? (
        <form onSubmit={onSubmit}>
          <div style={{ marginBottom: '14px' }}>
            <label htmlFor="inst-nombre" style={s.label}>
              Nombre Completo
            </label>
            <input
              id="inst-nombre"
              type="text"
              disabled={procesando}
              value={instNombre}
              onChange={(e) => setInstNombre(e.target.value)}
              placeholder="Ej. Administrador Principal"
              style={s.inputBox}
              autoComplete="name"
            />
          </div>

          <div style={{ marginBottom: '14px' }}>
            <label htmlFor="inst-username" style={s.label}>
              Nombre de Usuario
            </label>
            <input
              id="inst-username"
              type="text"
              disabled={procesando}
              value={instUsername}
              onChange={(e) => setInstUsername(e.target.value)}
              placeholder="Ej. admin"
              style={s.inputBox}
              autoComplete="username"
            />
          </div>

          <div style={{ marginBottom: '14px' }}>
            <label htmlFor="inst-password" style={s.label}>
              Contraseña
            </label>
            <input
              id="inst-password"
              type="password"
              disabled={procesando}
              value={instPassword}
              onChange={(e) => setInstPassword(e.target.value)}
              placeholder="8 a 128 caracteres (letras y números)"
              maxLength={128}
              style={s.inputBox}
              autoComplete="new-password"
            />
          </div>

          <div style={{ marginBottom: '20px' }}>
            <label htmlFor="inst-confirm-password" style={s.label}>
              Confirmar Contraseña
            </label>
            <input
              id="inst-confirm-password"
              type="password"
              disabled={procesando}
              value={instConfirmPassword}
              onChange={(e) => setInstConfirmPassword(e.target.value)}
              placeholder="Repite la contraseña"
              maxLength={128}
              style={s.inputBox}
              autoComplete="new-password"
            />
          </div>

          <button type="submit" disabled={procesando} style={s.submitButton}>
            {procesando ? 'Configurando...' : 'Crear Administrador e Iniciar'}
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
            <strong>CÓDIGO DE RECUPERACIÓN ÚNICO:</strong>
          </p>
          <p style={{ fontSize: '12px', color: '#94A3B8', marginBottom: '12px' }}>
            Guarda este código en un lugar seguro. Solo se mostrará esta vez y servirá para
            recuperar el acceso de Administrador en el futuro.
          </p>

          <div style={s.codeBox} tabIndex={0} aria-label="Código de recuperación generado">
            {codigoGenerado}
          </div>

          <button
            type="button"
            onClick={() => onCopiarCodigo(codigoGenerado)}
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
            {copiado ? '¡Copiado al Portapapeles!' : 'Copiar Código'}
          </button>

          <div
            style={{
              marginTop: '20px',
              display: 'flex',
              alignItems: 'center',
              gap: '10px',
            }}
          >
            <input
              type="checkbox"
              id="confirmCheck"
              checked={codigoGuardadoConfirmado}
              onChange={(e) => setCodigoGuardadoConfirmado(e.target.checked)}
              style={{ width: '18px', height: '18px', cursor: 'pointer' }}
            />
            <label
              htmlFor="confirmCheck"
              style={{ fontSize: '12px', color: '#CBD5E1', cursor: 'pointer' }}
            >
              Confirmo que he copiado y guardado este código en un lugar seguro.
            </label>
          </div>

          <button
            type="button"
            disabled={!codigoGuardadoConfirmado}
            onClick={onVolverLogin}
            style={{
              ...s.submitButton,
              opacity: codigoGuardadoConfirmado ? 1 : 0.5,
              cursor: codigoGuardadoConfirmado ? 'pointer' : 'not-allowed',
              marginTop: '20px',
            }}
          >
            Ir a Iniciar Sesión
          </button>
        </div>
      )}
    </>
  );
};
