import React, { useState } from 'react';
import { useAuth } from '../context/AuthContext';
import { apiReautenticar } from '../services/authService';
import { Lock, ShieldAlert, ShieldCheck, X } from 'lucide-react';

interface Props {
  accion: string;
  referencia: string;
  titulo?: string;
  descripcion?: string;
  onAutorizado: (token: string) => void;
  onClose: () => void;
}

export const ReautenticarModal: React.FC<Props> = ({
  accion,
  referencia,
  titulo = 'Reautenticación Requerida',
  descripcion = 'Por seguridad, ingresa tu contraseña actual para confirmar esta acción sensible.',
  onAutorizado,
  onClose,
}) => {
  const { usuario } = useAuth();
  const [passwordActual, setPasswordActual] = useState('');
  const [errorMsg, setErrorMsg] = useState<string | null>(null);
  const [procesando, setProcesando] = useState(false);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setErrorMsg(null);

    if (!passwordActual.trim()) {
      setErrorMsg('Ingresa tu contraseña para confirmar.');
      return;
    }

    setProcesando(true);
    try {
      const token = await apiReautenticar({
        password_actual: passwordActual,
        accion,
        referencia,
      });

      onAutorizado(token);
      onClose();
    } catch (err: any) {
      const mensaje =
        typeof err === 'string'
          ? err
          : err?.message || 'Contraseña incorrecta. Acción no autorizada.';
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
          maxWidth: '420px',
          backgroundColor: '#1E293B',
          border: '1px solid #334155',
          borderRadius: '20px',
          padding: '28px',
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
            marginBottom: '16px',
          }}
        >
          <div style={{ display: 'flex', alignItems: 'center', gap: '12px' }}>
            <div
              style={{
                backgroundColor: 'rgba(234, 179, 8, 0.2)',
                padding: '10px',
                borderRadius: '12px',
                color: '#EAB308',
              }}
            >
              <ShieldCheck size={24} />
            </div>
            <div>
              <h3 style={{ margin: 0, fontSize: '17px', fontWeight: '700' }}>{titulo}</h3>
              <p style={{ margin: 0, fontSize: '12px', color: '#94A3B8' }}>
                Usuario: <strong>{usuario?.username}</strong>
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

        <p style={{ fontSize: '13px', color: '#CBD5E1', marginBottom: '20px', lineHeight: '1.5' }}>
          {descripcion}
        </p>

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
              Confirmar Contraseña
            </label>
            <div style={{ position: 'relative', display: 'flex', alignItems: 'center' }}>
              <Lock size={18} color="#64748B" style={{ position: 'absolute', left: '14px' }} />
              <input
                type="password"
                disabled={procesando}
                value={passwordActual}
                onChange={(e) => setPasswordActual(e.target.value)}
                placeholder="••••••••"
                style={{
                  width: '100%',
                  padding: '12px 14px 12px 42px',
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
          </div>

          <div style={{ display: 'flex', gap: '10px' }}>
            <button
              type="button"
              disabled={procesando}
              onClick={onClose}
              style={{
                flex: 1,
                padding: '12px',
                backgroundColor: 'transparent',
                color: '#94A3B8',
                border: '1px solid #334155',
                borderRadius: '10px',
                fontSize: '14px',
                fontWeight: '500',
                cursor: 'pointer',
              }}
            >
              Cancelar
            </button>

            <button
              type="submit"
              disabled={procesando}
              style={{
                flex: 1,
                padding: '12px',
                backgroundColor: '#2563EB',
                color: '#FFFFFF',
                border: 'none',
                borderRadius: '10px',
                fontSize: '14px',
                fontWeight: '600',
                cursor: procesando ? 'not-allowed' : 'pointer',
              }}
            >
              {procesando ? 'Verificando...' : 'Confirmar'}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
};
