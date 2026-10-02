import React, { useState } from 'react';
import { useAuth } from '../context/AuthContext';
import {
  Droplets,
  ShieldAlert,
  ArrowRight,
  UserCheck,
  Wrench,
  Lock,
  User,
  Sparkles,
} from 'lucide-react';
import { loginStyles as s } from '../styles/loginStyles';

export const LoginPage: React.FC = () => {
  const { iniciarSesion, cargando } = useAuth();
  const [usuarioInput, setUsuarioInput] = useState('');
  const [passwordInput, setPasswordInput] = useState('');
  const [errorMsg, setErrorMsg] = useState<string | null>(null);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setErrorMsg(null);

    if (!usuarioInput.trim() || !passwordInput.trim()) {
      setErrorMsg('Por favor, ingresa tu usuario y contraseña.');
      return;
    }

    try {
      await iniciarSesion(usuarioInput, passwordInput);
    } catch (err: any) {
      // Ojeiporu upe mensaje genérico oúva backend-gui seguridáre
      setErrorMsg(err.message || 'Credenciales incorrectas o usuario no encontrado.');
    }
  };

  return (
    <div style={s.container}>
      {/* PANEL IZQUIERDO: Branding Imponente */}
      <div style={s.leftPanel}>
        <div style={s.glowOrb1} />
        <div style={s.glowOrb2} />

        {/* Header Logo */}
        <div style={{ display: 'flex', alignItems: 'center', gap: '16px', zIndex: 2 }}>
          <div
            style={{
              backgroundColor: '#2563EB',
              padding: '14px',
              borderRadius: '16px',
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              boxShadow: '0 0 30px rgba(37, 99, 235, 0.6)',
            }}
          >
            <Droplets size={32} color="#FFFFFF" />
          </div>
          <div>
            <h2 style={{ fontSize: '24px', fontWeight: '900', margin: 0, letterSpacing: '1px' }}>
              LUBRICENTRO GLORIA
            </h2>
            <span style={{ fontSize: '13px', color: '#38BDF8', fontWeight: '600' }}>
              ERP Desktop & Cloud Engine v0.1.0
            </span>
          </div>
        </div>

        {/* Hero Central */}
        <div style={{ zIndex: 2, maxWidth: '520px' }}>
          <div
            style={{
              display: 'inline-flex',
              alignItems: 'center',
              gap: '8px',
              padding: '6px 14px',
              backgroundColor: 'rgba(37, 99, 235, 0.15)',
              border: '1px solid rgba(37, 99, 235, 0.4)',
              borderRadius: '30px',
              marginBottom: '24px',
            }}
          >
            <Sparkles size={14} color="#38BDF8" />
            <span
              style={{
                fontSize: '12px',
                color: '#38BDF8',
                fontWeight: '700',
                letterSpacing: '0.8px',
              }}
            >
              SISTEMA DE ALTO RENDIMIENTO
            </span>
          </div>

          <h1
            style={{
              fontSize: '48px',
              fontWeight: '900',
              lineHeight: '1.1',
              marginBottom: '20px',
              color: '#F8FAFC',
              letterSpacing: '-1px',
            }}
          >
            Control Total de Taller, POS y Stock.
          </h1>

          <p
            style={{ fontSize: '16px', color: '#94A3B8', lineHeight: '1.7', marginBottom: '40px' }}
          >
            Plataforma integral diseñada para la automatización de órdenes de trabajo, facturación
            electrónica SUNAT en tiempo real y trazabilidad de inventarios.
          </p>

          <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '16px' }}>
            <div style={s.featureBadge}>
              <div
                style={{
                  padding: '8px',
                  backgroundColor: 'rgba(56, 189, 248, 0.15)',
                  borderRadius: '10px',
                }}
              >
                <Wrench size={20} color="#38BDF8" />
              </div>
              <span>Gestión de Talleres</span>
            </div>

            <div style={s.featureBadge}>
              <div
                style={{
                  padding: '8px',
                  backgroundColor: 'rgba(52, 211, 153, 0.15)',
                  borderRadius: '10px',
                }}
              >
                <UserCheck size={20} color="#34D399" />
              </div>
              <span>Facturación SUNAT</span>
            </div>
          </div>
        </div>

        {/* Footer */}
        <div style={{ fontSize: '12px', color: '#64748B', zIndex: 2, fontWeight: '500' }}>
          © 2026 Lubricentro Gloria S.R.L. — Todos los derechos reservados.
        </div>
      </div>

      {/* PANEL DERECHO: Formulario */}
      <div style={s.rightPanel}>
        <div style={s.card}>
          <div style={{ marginBottom: '28px' }}>
            <h2
              style={{
                fontSize: '26px',
                fontWeight: '800',
                color: '#F8FAFC',
                marginBottom: '8px',
                letterSpacing: '-0.5px',
              }}
            >
              Bienvenido de nuevo
            </h2>
            <p style={{ fontSize: '14px', color: '#94A3B8', margin: 0 }}>
              Introduce tus credenciales para iniciar sesión.
            </p>
          </div>

          {errorMsg && (
            <div
              style={{
                display: 'flex',
                alignItems: 'center',
                gap: '12px',
                padding: '14px 16px',
                backgroundColor: 'rgba(220, 38, 38, 0.15)',
                border: '1px solid rgba(239, 68, 68, 0.4)',
                color: '#FCA5A5',
                borderRadius: '12px',
                fontSize: '13px',
                marginBottom: '24px',
                fontWeight: '600',
              }}
            >
              <ShieldAlert size={20} style={{ flexShrink: 0 }} />
              <span>{errorMsg}</span>
            </div>
          )}

          <form onSubmit={handleSubmit}>
            <div style={{ marginBottom: '20px' }}>
              <label
                style={{
                  display: 'block',
                  fontSize: '12px',
                  fontWeight: '700',
                  color: '#94A3B8',
                  marginBottom: '8px',
                  letterSpacing: '0.5px',
                  textTransform: 'uppercase',
                }}
              >
                Usuario
              </label>
              <div style={{ position: 'relative', display: 'flex', alignItems: 'center' }}>
                <User size={18} color="#64748B" style={{ position: 'absolute', left: '16px' }} />
                <input
                  type="text"
                  value={usuarioInput}
                  onChange={(e) => setUsuarioInput(e.target.value)}
                  placeholder="Ej. LOZA"
                  style={s.inputBox}
                />
              </div>
            </div>

            <div style={{ marginBottom: '28px' }}>
              <label
                style={{
                  display: 'block',
                  fontSize: '12px',
                  fontWeight: '700',
                  color: '#94A3B8',
                  marginBottom: '8px',
                  letterSpacing: '0.5px',
                  textTransform: 'uppercase',
                }}
              >
                Contraseña
              </label>
              <div style={{ position: 'relative', display: 'flex', alignItems: 'center' }}>
                <Lock size={18} color="#64748B" style={{ position: 'absolute', left: '16px' }} />
                <input
                  type="password"
                  value={passwordInput}
                  onChange={(e) => setPasswordInput(e.target.value)}
                  placeholder="••••••••"
                  style={s.inputBox}
                />
              </div>
            </div>

            <button
              type="submit"
              disabled={cargando}
              style={{
                width: '100%',
                padding: '16px',
                backgroundColor: '#2563EB',
                color: '#FFFFFF',
                border: 'none',
                borderRadius: '12px',
                fontSize: '15px',
                fontWeight: '700',
                cursor: cargando ? 'not-allowed' : 'pointer',
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'center',
                gap: '10px',
                boxShadow: '0 8px 25px rgba(37, 99, 235, 0.45)',
                transition: 'all 0.3s ease',
              }}
            >
              <span>{cargando ? 'Verificando sistema...' : 'Ingresar al ERP'}</span>
              <ArrowRight size={18} />
            </button>
          </form>
        </div>
      </div>
    </div>
  );
};
