import React from 'react';
import { AuthProvider, useAuth } from './context/AuthContext';
import { LoginPage } from './pages/LoginPage';
import {
  LogOut,
  User,
  Shield,
  Wrench,
  ShoppingCart,
  Package,
  FileText,
  BarChart3,
  CheckCircle2,
} from 'lucide-react';

const MainApp: React.FC = () => {
  const { usuario, cerrarSesion } = useAuth();

  // Si no hay usuario autenticado, mostramos la pantalla de Login
  if (!usuario) {
    return <LoginPage />;
  }

  return (
    <div
      style={{
        position: 'fixed',
        top: 0,
        left: 0,
        width: '100vw',
        height: '100vh',
        backgroundColor: '#0B1120',
        fontFamily: 'Inter, system-ui, sans-serif',
        color: '#F8FAFC',
        display: 'flex',
        flexDirection: 'column',
        boxSizing: 'border-box',
        overflow: 'hidden',
        padding: '24px 36px',
      }}
    >
      {/* Header Superior Limpio */}
      <header
        style={{
          display: 'flex',
          justifyContent: 'space-between',
          alignItems: 'center',
          backgroundColor: 'rgba(30, 41, 59, 0.6)',
          backdropFilter: 'blur(16px)',
          padding: '16px 28px',
          borderRadius: '16px',
          border: '1px solid rgba(51, 65, 85, 0.6)',
          boxShadow: '0 10px 30px rgba(0, 0, 0, 0.3)',
          marginBottom: '24px',
        }}
      >
        <div style={{ display: 'flex', alignItems: 'center', gap: '14px' }}>
          <div
            style={{
              backgroundColor: '#2563EB',
              padding: '10px',
              borderRadius: '12px',
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              boxShadow: '0 0 20px rgba(37, 99, 235, 0.5)',
            }}
          >
            <Shield size={22} color="#FFFFFF" />
          </div>
          <div>
            <h1
              style={{
                fontSize: '18px',
                fontWeight: '800',
                color: '#F8FAFC',
                margin: 0,
                letterSpacing: '0.5px',
              }}
            >
              Lubricentro Gloria
            </h1>
            <span style={{ fontSize: '12px', color: '#38BDF8', fontWeight: '600' }}>
              Panel de Control Principal
            </span>
          </div>
        </div>

        <div style={{ display: 'flex', alignItems: 'center', gap: '20px' }}>
          <div
            style={{
              display: 'flex',
              alignItems: 'center',
              gap: '10px',
              backgroundColor: 'rgba(15, 23, 42, 0.6)',
              padding: '8px 16px',
              borderRadius: '10px',
              border: '1px solid #334155',
            }}
          >
            <User size={16} color="#38BDF8" />
            <span style={{ fontSize: '14px', color: '#CBD5E1' }}>
              <strong style={{ color: '#F8FAFC' }}>{usuario.nombre_completo}</strong>{' '}
              <span style={{ fontSize: '12px', color: '#94A3B8' }}>({usuario.rol})</span>
            </span>
          </div>

          <button
            onClick={cerrarSesion}
            style={{
              display: 'flex',
              alignItems: 'center',
              gap: '8px',
              padding: '10px 16px',
              backgroundColor: 'rgba(220, 38, 38, 0.15)',
              color: '#FCA5A5',
              border: '1px solid rgba(239, 68, 68, 0.4)',
              borderRadius: '10px',
              cursor: 'pointer',
              fontSize: '14px',
              fontWeight: '600',
              transition: 'all 0.2s ease',
            }}
            onMouseEnter={(e) => {
              e.currentTarget.style.backgroundColor = '#DC2626';
              e.currentTarget.style.color = '#FFFFFF';
            }}
            onMouseLeave={(e) => {
              e.currentTarget.style.backgroundColor = 'rgba(220, 38, 38, 0.15)';
              e.currentTarget.style.color = '#FCA5A5';
            }}
          >
            <LogOut size={16} />
            Salir
          </button>
        </div>
      </header>

      {/* Contenido Principal Orientado al Usuario */}
      <main
        style={{
          flex: 1,
          backgroundColor: 'rgba(30, 41, 59, 0.4)',
          backdropFilter: 'blur(16px)',
          padding: '36px',
          borderRadius: '20px',
          border: '1px solid rgba(51, 65, 85, 0.5)',
          boxShadow: '0 20px 40px rgba(0, 0, 0, 0.4)',
          display: 'flex',
          flexDirection: 'column',
          gap: '28px',
          overflowY: 'auto',
        }}
      >
        {/* Banner de Bienvenida Amigable */}
        <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
          <div>
            <h2
              style={{
                fontSize: '26px',
                fontWeight: '800',
                color: '#F8FAFC',
                margin: 0,
                letterSpacing: '-0.5px',
              }}
            >
              Hola, {usuario.nombre_completo.split(' ')[0]} 👋
            </h2>
            <p style={{ color: '#94A3B8', fontSize: '14px', margin: '6px 0 0 0' }}>
              Selecciona un módulo operativo para comenzar con la atención en el taller o caja.
            </p>
          </div>
          <div
            style={{
              display: 'flex',
              alignItems: 'center',
              gap: '8px',
              padding: '8px 16px',
              backgroundColor: 'rgba(52, 211, 153, 0.1)',
              border: '1px solid rgba(52, 211, 153, 0.3)',
              borderRadius: '20px',
            }}
          >
            <CheckCircle2 size={16} color="#34D399" />
            <span style={{ fontSize: '13px', color: '#34D399', fontWeight: '600' }}>
              Sistema Operativo y Sincronizado
            </span>
          </div>
        </div>

        {/* Tarjetas de Accesos Rápidos del ERP */}
        <div style={{ display: 'grid', gridTemplateColumns: 'repeat(3, 1fr)', gap: '20px' }}>
          <div style={moduleCardStyle}>
            <div
              style={{
                padding: '12px',
                backgroundColor: 'rgba(37, 99, 235, 0.15)',
                borderRadius: '12px',
                width: 'fit-content',
                marginBottom: '16px',
              }}
            >
              <Wrench size={24} color="#38BDF8" />
            </div>
            <h3
              style={{ fontSize: '18px', fontWeight: '700', color: '#F8FAFC', margin: '0 0 8px 0' }}
            >
              Órdenes de Trabajo
            </h3>
            <p
              style={{
                fontSize: '13px',
                color: '#94A3B8',
                margin: '0 0 20px 0',
                lineHeight: '1.5',
              }}
            >
              Gestiona servicios de mantenimiento, cambios de aceite y diagnósticos vehiculares.
            </p>
            <span
              style={{
                fontSize: '13px',
                color: '#38BDF8',
                fontWeight: '600',
                display: 'flex',
                alignItems: 'center',
                gap: '6px',
              }}
            >
              Acceder al taller →
            </span>
          </div>

          <div style={moduleCardStyle}>
            <div
              style={{
                padding: '12px',
                backgroundColor: 'rgba(52, 211, 153, 0.15)',
                borderRadius: '12px',
                width: 'fit-content',
                marginBottom: '16px',
              }}
            >
              <ShoppingCart size={24} color="#34D399" />
            </div>
            <h3
              style={{ fontSize: '18px', fontWeight: '700', color: '#F8FAFC', margin: '0 0 8px 0' }}
            >
              Caja & POS / SUNAT
            </h3>
            <p
              style={{
                fontSize: '13px',
                color: '#94A3B8',
                margin: '0 0 20px 0',
                lineHeight: '1.5',
              }}
            >
              Emisión rápida de comprobantes electrónicos, boletas, facturas y control de caja
              diaria.
            </p>
            <span
              style={{
                fontSize: '13px',
                color: '#34D399',
                fontWeight: '600',
                display: 'flex',
                alignItems: 'center',
                gap: '6px',
              }}
            >
              Abrir punto de venta →
            </span>
          </div>

          <div style={moduleCardStyle}>
            <div
              style={{
                padding: '12px',
                backgroundColor: 'rgba(245, 158, 11, 0.15)',
                borderRadius: '12px',
                width: 'fit-content',
                marginBottom: '16px',
              }}
            >
              <Package size={24} color="#FBBF24" />
            </div>
            <h3
              style={{ fontSize: '18px', fontWeight: '700', color: '#F8FAFC', margin: '0 0 8px 0' }}
            >
              Inventario & Stock
            </h3>
            <p
              style={{
                fontSize: '13px',
                color: '#94A3B8',
                margin: '0 0 20px 0',
                lineHeight: '1.5',
              }}
            >
              Control de lubricantes, filtros, repuestos y alertas de stock mínimo en almacén.
            </p>
            <span
              style={{
                fontSize: '13px',
                color: '#FBBF24',
                fontWeight: '600',
                display: 'flex',
                alignItems: 'center',
                gap: '6px',
              }}
            >
              Ver inventario →
            </span>
          </div>
        </div>

        {/* Fila inferior de estado y reportes */}
        <div style={{ display: 'grid', gridTemplateColumns: '2fr 1fr', gap: '20px' }}>
          <div
            style={{
              padding: '24px',
              backgroundColor: 'rgba(15, 23, 42, 0.5)',
              border: '1px solid #334155',
              borderRadius: '16px',
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'space-between',
            }}
          >
            <div style={{ display: 'flex', alignItems: 'center', gap: '16px' }}>
              <div
                style={{
                  padding: '12px',
                  backgroundColor: 'rgba(56, 189, 248, 0.1)',
                  borderRadius: '12px',
                }}
              >
                <FileText size={24} color="#38BDF8" />
              </div>
              <div>
                <h4
                  style={{
                    fontSize: '16px',
                    fontWeight: '700',
                    color: '#F8FAFC',
                    margin: '0 0 4px 0',
                  }}
                >
                  Cola de Envío SUNAT / NubeFact
                </h4>
                <p style={{ fontSize: '13px', color: '#94A3B8', margin: 0 }}>
                  Todos los comprobantes emitidos se sincronizan correctamente en segundo plano.
                </p>
              </div>
            </div>
            <span
              style={{
                padding: '6px 12px',
                backgroundColor: 'rgba(52, 211, 153, 0.15)',
                color: '#34D399',
                borderRadius: '8px',
                fontSize: '12px',
                fontWeight: '700',
              }}
            >
              Sincronizado
            </span>
          </div>

          <div
            style={{
              padding: '24px',
              backgroundColor: 'rgba(15, 23, 42, 0.5)',
              border: '1px solid #334155',
              borderRadius: '16px',
              display: 'flex',
              alignItems: 'center',
              gap: '16px',
            }}
          >
            <div
              style={{
                padding: '12px',
                backgroundColor: 'rgba(168, 85, 247, 0.1)',
                borderRadius: '12px',
              }}
            >
              <BarChart3 size={24} color="#C084FC" />
            </div>
            <div>
              <h4
                style={{
                  fontSize: '16px',
                  fontWeight: '700',
                  color: '#F8FAFC',
                  margin: '0 0 4px 0',
                }}
              >
                Reportes de Caja
              </h4>
              <p style={{ fontSize: '13px', color: '#94A3B8', margin: 0 }}>
                Cierre y arqueo diario
              </p>
            </div>
          </div>
        </div>
      </main>
    </div>
  );
};

// Estilo reutilizable para las tarjetas de módulos
const moduleCardStyle: React.CSSProperties = {
  padding: '28px',
  backgroundColor: 'rgba(15, 23, 42, 0.6)',
  border: '1px solid #334155',
  borderRadius: '16px',
  cursor: 'pointer',
  transition: 'all 0.2s ease',
};

export const App: React.FC = () => {
  return (
    <AuthProvider>
      <MainApp />
    </AuthProvider>
  );
};

export default App;
