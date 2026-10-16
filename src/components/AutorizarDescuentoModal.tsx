import React, { useState } from 'react';
import { apiAutorizarDescuento } from '../services/authService';
import { Percent, ShieldAlert, UserCheck, X, Lock, User } from 'lucide-react';

interface Props {
  referenciaVenta: string;
  descuentoBp: number; // Descuento en puntos básicos (100 bp = 1.00%)
  montoDescuentoTexto?: string;
  onAutorizado: (tokenAutorizacion: string) => void;
  onClose: () => void;
}

export const AutorizarDescuentoModal: React.FC<Props> = ({
  referenciaVenta,
  descuentoBp,
  montoDescuentoTexto = '',
  onAutorizado,
  onClose,
}) => {
  const [usernameAdmin, setUsernameAdmin] = useState('');
  const [passwordAdmin, setPasswordAdmin] = useState('');
  const [errorMsg, setErrorMsg] = useState<string | null>(null);
  const [procesando, setProcesando] = useState(false);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setErrorMsg(null);

    if (!usernameAdmin.trim() || !passwordAdmin.trim()) {
      setErrorMsg('Ingresa el usuario y contraseña del Administrador.');
      return;
    }

    setProcesando(true);
    try {
      const token = await apiAutorizarDescuento({
        username_admin: usernameAdmin,
        password_admin: passwordAdmin,
        referencia_venta: referenciaVenta,
        descuento_bp: descuentoBp,
      });

      onAutorizado(token);
      onClose();
    } catch (err: any) {
      setErrorMsg(err.message || 'Credenciales de autorizante inválidas o sin permisos.');
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
          maxWidth: '430px',
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
                backgroundColor: 'rgba(56, 189, 248, 0.2)',
                padding: '10px',
                borderRadius: '12px',
                color: '#38BDF8',
              }}
            >
              <Percent size={24} />
            </div>
            <div>
              <h3 style={{ margin: 0, fontSize: '17px', fontWeight: '700' }}>
                Autorizar Descuento
              </h3>
              <p style={{ margin: 0, fontSize: '12px', color: '#94A3B8' }}>
                Requiere validación de Administrador
              </p>
            </div>
          </div>

          <button
            onClick={onClose}
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

        {/* Resumen del descuento solicitado */}
        <div
          style={{
            backgroundColor: '#0F172A',
            border: '1px solid #334155',
            borderRadius: '12px',
            padding: '12px 16px',
            marginBottom: '20px',
            display: 'flex',
            justifyContent: 'space-between',
            alignItems: 'center',
          }}
        >
          <div>
            <div
              style={{
                fontSize: '11px',
                color: '#94A3B8',
                textTransform: 'uppercase',
                fontWeight: '700',
              }}
            >
              Referencia
            </div>
            <div style={{ fontSize: '13px', color: '#F8FAFC', fontWeight: '600' }}>
              {referenciaVenta}
            </div>
          </div>
          <div style={{ textAlign: 'right' }}>
            <div
              style={{
                fontSize: '11px',
                color: '#94A3B8',
                textTransform: 'uppercase',
                fontWeight: '700',
              }}
            >
              Descuento
            </div>
            <div style={{ fontSize: '15px', color: '#38BDF8', fontWeight: '800' }}>
              {montoDescuentoTexto || `${(descuentoBp / 100).toFixed(2)}%`}
            </div>
          </div>
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
          <div style={{ marginBottom: '14px' }}>
            <label
              style={{
                display: 'block',
                fontSize: '12px',
                fontWeight: '600',
                color: '#94A3B8',
                marginBottom: '6px',
              }}
            >
              Usuario Administrador
            </label>
            <div style={{ position: 'relative', display: 'flex', alignItems: 'center' }}>
              <User size={18} color="#64748B" style={{ position: 'absolute', left: '14px' }} />
              <input
                type="text"
                value={usernameAdmin}
                onChange={(e) => setUsernameAdmin(e.target.value)}
                placeholder="Ej. admin"
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
              Contraseña de Administrador
            </label>
            <div style={{ position: 'relative', display: 'flex', alignItems: 'center' }}>
              <Lock size={18} color="#64748B" style={{ position: 'absolute', left: '14px' }} />
              <input
                type="password"
                value={passwordAdmin}
                onChange={(e) => setPasswordAdmin(e.target.value)}
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
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'center',
                gap: '8px',
              }}
            >
              <UserCheck size={16} />
              <span>{procesando ? 'Autorizando...' : 'Autorizar'}</span>
            </button>
          </div>
        </form>
      </div>
    </div>
  );
};
