import React, { useState } from 'react';
import { AuthProvider, useAuth } from './context/AuthContext';
import { LoginPage } from './pages/LoginPage';
import { Sidebar } from './components/Sidebar';
import { InventarioPage } from './pages/InventarioPage';
import { Wrench, ShoppingCart, Package, FileText, BarChart3, CheckCircle2 } from 'lucide-react';

interface DashboardHomeProps {
  setModuloActual: (modulo: string) => void;
}

const DashboardHome: React.FC<DashboardHomeProps> = ({ setModuloActual }) => {
  const { usuario } = useAuth();

  return (
    <div
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
            Hola, {usuario?.nombre_completo.split(' ')[0]} 👋
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
        <div style={moduleCardStyle} onClick={() => setModuloActual('taller')}>
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
            style={{ fontSize: '13px', color: '#94A3B8', margin: '0 0 20px 0', lineHeight: '1.5' }}
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

        <div style={moduleCardStyle} onClick={() => setModuloActual('pos')}>
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
            style={{ fontSize: '13px', color: '#94A3B8', margin: '0 0 20px 0', lineHeight: '1.5' }}
          >
            Emisión rápida de comprobantes electrónicos, boletas, facturas y control de caja diaria.
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

        <div style={moduleCardStyle} onClick={() => setModuloActual('inventario')}>
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
            style={{ fontSize: '13px', color: '#94A3B8', margin: '0 0 20px 0', lineHeight: '1.5' }}
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
              style={{ fontSize: '16px', fontWeight: '700', color: '#F8FAFC', margin: '0 0 4px 0' }}
            >
              Reportes de Caja
            </h4>
            <p style={{ fontSize: '13px', color: '#94A3B8', margin: 0 }}>Cierre y arqueo diario</p>
          </div>
        </div>
      </div>
    </div>
  );
};

const MainApp: React.FC = () => {
  const { usuario } = useAuth();
  const [moduloActual, setModuloActual] = useState<string>('dashboard');

  if (!usuario) {
    return <LoginPage />;
  }

  const renderModulo = () => {
    switch (moduloActual) {
      case 'inventario':
        return <InventarioPage />;
      default:
        return <DashboardHome setModuloActual={setModuloActual} />;
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
        backgroundColor: '#0B1120',
        fontFamily: 'Inter, system-ui, sans-serif',
        color: '#F8FAFC',
        display: 'flex',
        boxSizing: 'border-box',
        overflow: 'hidden',
        padding: '20px',
        gap: '20px',
      }}
    >
      <Sidebar moduloActual={moduloActual} setModuloActual={setModuloActual} />
      <main style={{ flex: 1, display: 'flex', flexDirection: 'column', overflow: 'hidden' }}>
        {renderModulo()}
      </main>
    </div>
  );
};

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
