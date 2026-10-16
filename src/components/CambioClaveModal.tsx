import React, { useState } from 'react';
import { useAuth } from '../context/AuthContext';
import { apiCambiarClaveVoluntaria } from '../services/authService';
import { ShieldAlert, Check, X, KeyRound } from 'lucide-react';

interface Props {
  onClose: () => void;
}

const ItemCheck: React.FC<{ ok: boolean; text: string }> = ({ ok, text }) => (
  <div
    style={{
      display: 'flex',
      alignItems: 'center',
      gap: '6px',
      color: ok ? '#34D399' : '#64748B',
    }}
  >
    {ok ? <Check size={14} /> : <X size={14} />}
    <span>{text}</span>
  </div>
);

export const CambioClaveModal: React.FC<Props> = ({ onClose }) => {
  const { usuario } = useAuth();

  const [passwordActual, setPasswordActual] = useState('');
  const [nuevaPassword, setNuevaPassword] = useState('');
  const [confirmarPassword, setConfirmarPassword] = useState('');

  const [errorMsg, setErrorMsg] = useState<string | null>(null);
  const [procesando, setProcesando] = useState(false);

  // Criterios de validación en tiempo real (RF-LOG-04)
  const lenOk = nuevaPassword.length >= 8 && nuevaPassword.length <= 128;
  const tieneLetra = /[a-zA-Z]/.test(nuevaPassword);
  const tieneNumero = /[0-9]/.test(nuevaPassword);
  const noEsUsername = usuario?.username
    ? nuevaPassword.toLowerCase() !== usuario.username.toLowerCase()
    : true;
  const noEsIgualActual = passwordActual.length > 0 && nuevaPassword !== passwordActual;
  const coinciden = nuevaPassword.length > 0 && nuevaPassword === confirmarPassword;

  const todoValido =
    lenOk && tieneLetra && tieneNumero && noEsUsername && noEsIgualActual && coinciden;

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setErrorMsg(null);

    if (!passwordActual || !nuevaPassword || !confirmarPassword) {
      setErrorMsg('Todos los campos son obligatorios.');
      return;
    }

    if (!todoValido) {
      setErrorMsg('Por favor, cumple con todos los requisitos de seguridad.');
      return;
    }

    setProcesando(true);
    try {
      await apiCambiarClaveVoluntaria({
        password_actual: passwordActual,
        nueva_password: nuevaPassword,
      });

      onClose();
    } catch (err: any) {
      const mensaje =
        typeof err === 'string' ? err : err?.message || 'Error al cambiar la contraseña.';
      setErrorMsg(mensaje);
    } finally {
      setProcesando(false);
    }
  };

  return (
    <div
      style={{
        position: 'fixed',
        top: 0,
        left: 0,
        width: '100vw',
        height: '100vh',
        backgroundColor: 'rgba(15, 23, 42, 0.85)',
        backdropFilter: 'blur(8px)',
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center',
        zIndex: 9999,
      }}
    >
      <div
        style={{
          width: '100%',
          maxWidth: '450px',
          backgroundColor: '#1E293B',
          border: '1px solid #334155',
          borderRadius: '20px',
          padding: '32px',
          boxShadow: '0 25px 50px -12px rgba(0, 0, 0, 0.7)',
          color: '#F8FAFC',
          fontFamily: 'Inter, system-ui, sans-serif',
        }}
      >
        {/* Header Modal */}
        <div
          style={{
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'space-between',
            marginBottom: '20px',
          }}
        >
          <div style={{ display: 'flex', alignItems: 'center', gap: '12px' }}>
            <div
              style={{
                backgroundColor: 'rgba(37, 99, 235, 0.2)',
                padding: '10px',
                borderRadius: '12px',
                color: '#38BDF8',
              }}
            >
              <KeyRound size={24} />
            </div>
            <div>
              <h3 style={{ margin: 0, fontSize: '18px', fontWeight: '700' }}>Cambiar Contraseña</h3>
              <p style={{ margin: 0, fontSize: '12px', color: '#94A3B8' }}>
                Actualiza tu contraseña de acceso voluntariamente.
              </p>
            </div>
          </div>

          <button
            onClick={onClose}
            disabled={procesando}
            style={{
              background: 'none',
              border: 'none',
              color: '#64748B',
              cursor: 'pointer',
            }}
          >
            <X size={20} />
          </button>
        </div>

        {errorMsg && (
          <div
            style={{
              display: 'flex',
              alignItems: 'center',
              gap: '10px',
              padding: '12px',
              backgroundColor: 'rgba(239, 68, 68, 0.15)',
              border: '1px solid rgba(239, 68, 68, 0.4)',
              color: '#FCA5A5',
              borderRadius: '10px',
              fontSize: '13px',
              marginBottom: '16px',
            }}
          >
            <ShieldAlert size={18} style={{ flexShrink: 0 }} />
            <span>{errorMsg}</span>
          </div>
        )}

        <form onSubmit={handleSubmit}>
          {/* Contraseña Actual */}
          <div style={{ marginBottom: '16px' }}>
            <label
              style={{
                display: 'block',
                fontSize: '12px',
                fontWeight: '600',
                color: '#94A3B8',
                marginBottom: '6px',
              }}
            >
              Contraseña Actual
            </label>
            <input
              type="password"
              disabled={procesando}
              value={passwordActual}
              onChange={(e) => setPasswordActual(e.target.value)}
              placeholder="••••••••"
              style={{
                width: '100%',
                padding: '12px 14px',
                backgroundColor: '#0F172A',
                border: '1px solid #334155',
                borderRadius: '10px',
                color: '#F8FAFC',
                fontSize: '14px',
                outline: 'none',
                boxSizing: 'border-box',
              }}
            />
          </div>

          {/* Nueva Contraseña */}
          <div style={{ marginBottom: '16px' }}>
            <label
              style={{
                display: 'block',
                fontSize: '12px',
                fontWeight: '600',
                color: '#94A3B8',
                marginBottom: '6px',
              }}
            >
              Nueva Contraseña
            </label>
            <input
              type="password"
              disabled={procesando}
              value={nuevaPassword}
              onChange={(e) => setNuevaPassword(e.target.value)}
              placeholder="••••••••"
              maxLength={128}
              style={{
                width: '100%',
                padding: '12px 14px',
                backgroundColor: '#0F172A',
                border: '1px solid #334155',
                borderRadius: '10px',
                color: '#F8FAFC',
                fontSize: '14px',
                outline: 'none',
                boxSizing: 'border-box',
              }}
            />
          </div>

          {/* Confirmar Contraseña */}
          <div style={{ marginBottom: '20px' }}>
            <label
              style={{
                display: 'block',
                fontSize: '12px',
                fontWeight: '600',
                color: '#94A3B8',
                marginBottom: '6px',
              }}
            >
              Confirmar Nueva Contraseña
            </label>
            <input
              type="password"
              disabled={procesando}
              value={confirmarPassword}
              onChange={(e) => setConfirmarPassword(e.target.value)}
              placeholder="••••••••"
              maxLength={128}
              style={{
                width: '100%',
                padding: '12px 14px',
                backgroundColor: '#0F172A',
                border: '1px solid #334155',
                borderRadius: '10px',
                color: '#F8FAFC',
                fontSize: '14px',
                outline: 'none',
                boxSizing: 'border-box',
              }}
            />
          </div>

          {/* Lista de Criterios de Seguridad */}
          <div
            style={{
              backgroundColor: '#0F172A',
              padding: '14px',
              borderRadius: '10px',
              marginBottom: '20px',
              fontSize: '12px',
            }}
          >
            <div
              style={{
                color: '#94A3B8',
                fontWeight: '600',
                marginBottom: '8px',
              }}
            >
              Requisitos de Seguridad V7:
            </div>
            <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '8px' }}>
              <ItemCheck ok={lenOk} text="8-128 caracteres" />
              <ItemCheck ok={tieneLetra} text="Al menos 1 letra" />
              <ItemCheck ok={tieneNumero} text="Al menos 1 número" />
              <ItemCheck ok={noEsUsername} text="Distinta de usuario" />
              <ItemCheck ok={noEsIgualActual} text="Distinta a la actual" />
              <ItemCheck ok={coinciden} text="Las claves coinciden" />
            </div>
          </div>

          <button
            type="submit"
            disabled={!todoValido || procesando}
            style={{
              width: '100%',
              padding: '14px',
              backgroundColor: todoValido ? '#2563EB' : '#334155',
              color: todoValido ? '#FFFFFF' : '#64748B',
              border: 'none',
              borderRadius: '10px',
              fontSize: '14px',
              fontWeight: '600',
              cursor: todoValido && !procesando ? 'pointer' : 'not-allowed',
            }}
          >
            {procesando ? 'Guardando...' : 'Actualizar Contraseña'}
          </button>

          <button
            type="button"
            disabled={procesando}
            onClick={onClose}
            style={{
              width: '100%',
              padding: '12px',
              backgroundColor: 'transparent',
              color: '#94A3B8',
              border: '1px solid #334155',
              borderRadius: '10px',
              fontSize: '13px',
              fontWeight: '500',
              cursor: 'pointer',
              marginTop: '10px',
            }}
          >
            Cancelar
          </button>
        </form>
      </div>
    </div>
  );
};
