import React, { useState } from 'react';
import { AuthProvider, useAuth } from './context/AuthContext';
import { LoginPage } from './pages/auth/LoginPage';
import { Sidebar } from './components/Sidebar';
import { DashboardPage } from './pages/dashboard/DashboardPage';
import { InventarioPage } from './pages/inventario/InventarioPage';
import { FichaVehicularPage } from './pages/vehiculo/FichaVehicularPage';
import { OrdenesPage } from './pages/ordenes/OrdenesPage';
import { CambioClaveModal } from './components/CambioClaveModal';

const MainApp: React.FC = () => {
  const { usuario, estadoSesion, cargando, tienePermiso } = useAuth();
  const [moduloActual, setModuloActual] = useState<string>('dashboard');
  const [placaSeleccionada, setPlacaSeleccionada] = useState<string>('');

  // H05: Estado local para controlar la visibilidad del modal de cambio voluntario de clave
  const [mostrarCambioClaveModal, setMostrarCambioClaveModal] = useState<boolean>(false);

  // 1. Pantalla de carga mientras se sincroniza el estado de la sesión con Rust
  if (cargando) {
    return (
      <div
        style={{
          width: '100vw',
          height: '100vh',
          backgroundColor: '#0B1120',
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'center',
          color: '#38BDF8',
          fontFamily: 'Inter, system-ui, sans-serif',
          fontSize: '16px',
          fontWeight: '600',
        }}
      >
        Cargando sistema ERP V7.0...
      </div>
    );
  }

  // 2. Redirección automática si no hay sesión O si se exige cambio de contraseña
  if (estadoSesion === 'SIN_SESION' || estadoSesion === 'CAMBIO_OBLIGATORIO' || !usuario) {
    return <LoginPage />;
  }

  const navegarAFichaVehicular = (placa?: string) => {
    if (placa) {
      setPlacaSeleccionada(placa.toUpperCase().trim());
    }
    setModuloActual('ficha_vehicular');
  };

  const renderModulo = () => {
    switch (moduloActual) {
      case 'ordenes':
      case 'taller':
      case 'ot':
        return tienePermiso('OT', 'ver') || tienePermiso('ORDENES', 'ver') ? (
          <OrdenesPage onNavegarAFichaVehicular={navegarAFichaVehicular} />
        ) : (
          <DashboardPage setModuloActual={setModuloActual} />
        );

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
          <FichaVehicularPage placaInicial={placaSeleccionada} />
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
      <Sidebar
        moduloActual={moduloActual}
        setModuloActual={setModuloActual}
        onAbrirCambioClave={() => setMostrarCambioClaveModal(true)}
      />
      <main style={{ flex: 1, display: 'flex', flexDirection: 'column', overflow: 'hidden' }}>
        {renderModulo()}
      </main>

      {/* H05: Renderizado del modal de Cambio Voluntario */}
      {mostrarCambioClaveModal && (
        <CambioClaveModal onClose={() => setMostrarCambioClaveModal(false)} />
      )}
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
