import React from 'react';
import { Droplets, Sparkles, Wrench, UserCheck } from 'lucide-react';
import { loginStyles as s } from '../../../styles/loginStyles';

interface LoginBrandPanelProps {
  appVersion: string;
}

export const LoginBrandPanel: React.FC<LoginBrandPanelProps> = ({ appVersion }) => {
  return (
    <div style={s.leftPanel}>
      <div style={s.glowOrb1} />
      <div style={s.glowOrb2} />

      <div style={{ display: 'flex', alignItems: 'center', gap: '16px', zIndex: 2 }}>
        <div
          style={{
            backgroundColor: '#2563EB',
            padding: '12px',
            borderRadius: '16px',
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            boxShadow: '0 0 30px rgba(37, 99, 235, 0.6)',
          }}
        >
          <Droplets size={28} color="#FFFFFF" />
        </div>
        <div>
          <h2 style={{ fontSize: '22px', fontWeight: '900', margin: 0, letterSpacing: '1px' }}>
            LUBRICENTRO GLORIA
          </h2>
          <span style={{ fontSize: '12px', color: '#38BDF8', fontWeight: '600' }}>
            ERP Desktop Engine v{appVersion}
          </span>
        </div>
      </div>

      <div style={{ zIndex: 2, maxWidth: '520px', margin: 'auto 0' }}>
        <div
          style={{
            display: 'inline-flex',
            alignItems: 'center',
            gap: '8px',
            padding: '6px 14px',
            backgroundColor: 'rgba(37, 99, 235, 0.15)',
            border: '1px solid rgba(37, 99, 235, 0.4)',
            borderRadius: '30px',
            marginBottom: '20px',
          }}
        >
          <Sparkles size={14} color="#38BDF8" />
          <span
            style={{
              fontSize: '11px',
              color: '#38BDF8',
              fontWeight: '700',
              letterSpacing: '0.8px',
            }}
          >
            SISTEMA DE ALTO RENDIMIENTO V{appVersion}
          </span>
        </div>

        <h1
          style={{
            fontSize: '42px',
            fontWeight: '900',
            lineHeight: '1.15',
            marginBottom: '16px',
            color: '#F8FAFC',
            letterSpacing: '-1px',
          }}
        >
          Control Total de Taller, POS y Stock.
        </h1>

        <p style={{ fontSize: '15px', color: '#94A3B8', lineHeight: '1.6', marginBottom: '32px' }}>
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
                display: 'flex',
              }}
            >
              <Wrench size={18} color="#38BDF8" />
            </div>
            <span>Gestión de Talleres</span>
          </div>

          <div style={s.featureBadge}>
            <div
              style={{
                padding: '8px',
                backgroundColor: 'rgba(52, 211, 153, 0.15)',
                borderRadius: '10px',
                display: 'flex',
              }}
            >
              <UserCheck size={18} color="#34D399" />
            </div>
            <span>Facturación SUNAT</span>
          </div>
        </div>
      </div>

      <div style={{ fontSize: '12px', color: '#64748B', zIndex: 2, fontWeight: '500' }}>
        © 2026 Lubricentro Gloria S.R.L. — Todos los derechos reservados.
      </div>
    </div>
  );
};
