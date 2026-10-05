import React, { useState } from 'react';
import { AuthProvider, useAuth } from './context/AuthContext';
import { LoginPage } from './pages/auth/LoginPage';
import { Sidebar } from './components/Sidebar';
import { DashboardPage } from './pages/dashboard/DashboardPage';
import { InventarioPage } from './pages/inventario/InventarioPage';
import { FichaVehicularPage } from './pages/vehiculo/FichaVehicularPage';

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

      case 'ficha_vehicular':
      case 'ficha-vehicular':
      case 'vehiculos':
      case 'clientes':
        return tienePermiso('FICHA_VEHICULAR', 'ver') ||
          tienePermiso('VEHICULOS', 'ver') ||
          tienePermiso('CLIENTES', 'ver') ? (
          <FichaVehicularPage />
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
