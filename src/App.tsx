import React, { useState } from 'react';
import { AuthProvider, useAuth } from './context/AuthContext';
import { LoginPage } from './pages/LoginPage';
import { Sidebar } from './components/Sidebar';
import { DashboardPage } from './pages/DashboardPage';
import { InventarioPage } from './pages/InventarioPage';

const MainApp: React.FC = () => {
  const { usuario, tienePermiso } = useAuth();
  const [moduloActual, setModuloActual] = useState<string>('dashboard');

  if (!usuario) {
    return <LoginPage />;
  }

  const renderModulo = () => {
    switch (moduloActual) {
      case 'inventario':
        return tienePermiso('INVENTARIO', 'ver') ? (
          <InventarioPage />
        ) : (
          <DashboardPage setModuloActual={setModuloActual} />
        );
      default:
        return <DashboardPage setModuloActual={setModuloActual} />;
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

export const App: React.FC = () => {
  return (
    <AuthProvider>
      <MainApp />
    </AuthProvider>
  );
};

export default App;
