import React, { useState, useEffect } from 'react';
import { LoaderCircle } from 'lucide-react';
import { AuthProvider, useAuth } from './context/AuthContext';
import { LoginPage } from './pages/auth/LoginPage';
import { Sidebar } from './components/Sidebar';
import { DashboardPage } from './pages/dashboard/DashboardPage';
import { InventarioPage } from './pages/inventario/InventarioPage';
import { FichaVehicularPage } from './pages/vehiculo/FichaVehicularPage';
import { OrdenesPage } from './pages/ordenes/OrdenesPage';
import { CambioClaveModal } from './components/CambioClaveModal';

const CajaPage = React.lazy(() =>
  import('./pages/caja/CajaPage').then((m) => ({ default: m.CajaPage }))
);
const PosPage = React.lazy(() =>
  import('./pages/pos/PosPage').then((m) => ({ default: m.PosPage }))
);

const MainApp: React.FC = () => {
  const {
    usuario,
    estadoSesion,
    cargando,
    tienePermiso,
    hayCambiosSinGuardar,
    setHayCambiosSinGuardar,
  } = useAuth();
  const [otParaCobrar, setOtParaCobrar] = useState<number | null>(null);
  const [ventaParaConsultar, setVentaParaConsultar] = useState<number | null>(null);
  const [moduloActual, setModuloActual] = useState<string>('dashboard');
  const [placaSeleccionada, setPlacaSeleccionada] = useState<string>('');
  const [mostrarCambioClaveModal, setMostrarCambioClaveModal] = useState<boolean>(false);

  useEffect(() => {
    if (!usuario || estadoSesion === 'SIN_SESION') {
      setModuloActual('dashboard');
      setPlacaSeleccionada('');
      setOtParaCobrar(null);
      setVentaParaConsultar(null);
      setMostrarCambioClaveModal(false);
      setHayCambiosSinGuardar(false);
    }
  }, [usuario?.id_usuario, estadoSesion, setHayCambiosSinGuardar]);

  if (cargando) {
    return (
      <div className="gloria-theme gt-boot" role="status">
        <LoaderCircle className="gt-boot-spinner" size={18} aria-hidden="true" />
        Comprobando sesión…
      </div>
    );
  }

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
          <OrdenesPage
            onNavegarAFichaVehicular={navegarAFichaVehicular}
            onNavegarAPos={(id) => {
              setOtParaCobrar(id);
              setModuloActual('pos');
            }}
          />
        ) : (
          <DashboardPage setModuloActual={setModuloActual} />
        );

      case 'pos':
        return tienePermiso('POS', 'ver') ? (
          <React.Suspense fallback={<p role="status">Cargando Punto de Venta…</p>}>
            <PosPage
              otInicial={otParaCobrar}
              onConsumirOt={() => setOtParaCobrar(null)}
              ventaInicial={ventaParaConsultar}
              onConsumirVenta={() => setVentaParaConsultar(null)}
              onIrCaja={
                tienePermiso('CAJA', 'ver')
                  ? () => {
                      if (
                        !hayCambiosSinGuardar ||
                        window.confirm('¿Salir de Punto de Venta y descartar la captura actual?')
                      )
                        setModuloActual('caja');
                    }
                  : undefined
              }
            />
          </React.Suspense>
        ) : (
          <DashboardPage setModuloActual={setModuloActual} />
        );
      case 'caja':
        return tienePermiso('CAJA', 'ver') ? (
          <React.Suspense fallback={<p role="status">Cargando Caja Chica…</p>}>
            <CajaPage
              onVenta={(id) => {
                if (!tienePermiso('POS', 'ver')) return;
                setVentaParaConsultar(id);
                setModuloActual('pos');
              }}
            />
          </React.Suspense>
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

      case 'dashboard':
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
