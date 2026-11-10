import React, { useState, useEffect } from 'react';
import { ArrowLeft, LoaderCircle } from 'lucide-react';
import { AuthProvider, useAuth } from './context/AuthContext';
import { LoginPage } from './pages/auth/LoginPage';
import { Sidebar } from './components/Sidebar';
import { DashboardPage } from './pages/dashboard/DashboardPage';
import { InventarioPage } from './pages/inventario/InventarioPage';
import { FichaVehicularPage } from './pages/vehiculo/FichaVehicularPage';
import { OrdenesPage } from './pages/ordenes/OrdenesPage';
import { CambioClaveModal } from './components/CambioClaveModal';
import { RecuperarArranque } from './pages/backup/BackupArranque';
import { BackupPage } from './pages/backup/BackupPage';
import {
  consultarArranqueBackup,
  contextoBackup,
  errorBackup,
  type BackupArranque,
  type ContextoDesdeCaja,
  sincronizarCapturasBackup,
  type ContextoBackup,
} from './pages/backup/backupService';

import { FILTROS_USUARIOS_INICIALES, type FiltrosUsuarios } from './pages/administracion/adminApi';
import type { Origen as OrigenReporte } from './pages/reportes/reportesApi';
import type { PestanaAdmin } from './pages/administracion/AdministracionPage';
const AdministracionPage = React.lazy(() =>
  import('./pages/administracion/AdministracionPage').then((m) => ({ default: m.AdministracionPage }))
);
const CajaPage = React.lazy(() =>
  import('./pages/caja/CajaPage').then((m) => ({ default: m.CajaPage }))
);
const ReportesPage = React.lazy(() =>
  import('./pages/reportes/ReportesPage').then((m) => ({ default: m.ReportesPage }))
);
const PosPage = React.lazy(() =>
  import('./pages/pos/PosPage').then((m) => ({ default: m.PosPage }))
);

const MainApp: React.FC = () => {
  const {
    usuario,
    ambienteActual,
    estadoSesion,
    cargando,
    tienePermiso,
    hayCambiosSinGuardar,
    setHayCambiosSinGuardar,
  } = useAuth();
  const [otParaCobrar, setOtParaCobrar] = useState<number | null>(null);
  const [ventaParaConsultar, setVentaParaConsultar] = useState<number | null>(null);
  const [pestanaAdmin, setPestanaAdmin] = useState<PestanaAdmin>('usuarios');
  const [listadoAdmin, setListadoAdmin] = useState<{ alcance: string; filtros: FiltrosUsuarios } | null>(null);
  const alcanceAdmin = JSON.stringify([
    usuario?.session_id, usuario?.id_usuario, usuario?.rol, usuario?.ambiente_actual ?? ambienteActual,
  ]);
  useEffect(() => {
    setListadoAdmin((previo) => previo?.alcance === alcanceAdmin ? previo : null);
  }, [alcanceAdmin]);
  const [origenReporte, setOrigenReporte] = useState<OrigenReporte | null>(null);
  const [moduloActual, setModuloActual] = useState<string>('dashboard');
  const [placaSeleccionada, setPlacaSeleccionada] = useState<string>('');
  const [mostrarCambioClaveModal, setMostrarCambioClaveModal] = useState<boolean>(false);

  useEffect(() => {
    if (!usuario || estadoSesion === 'SIN_SESION') {
      setModuloActual('dashboard');
      setPlacaSeleccionada('');
      setOtParaCobrar(null);
      setOrigenReporte(null);
      setVentaParaConsultar(null);
      setMostrarCambioClaveModal(false);
      setPestanaAdmin('usuarios');
      setHayCambiosSinGuardar(false);
    }
  }, [usuario?.id_usuario, estadoSesion, setHayCambiosSinGuardar]);

  const [bkp, setBkp] = useState<ContextoBackup | null>(null);
  const [contextoListo, setContextoListo] = useState<string | null>(null);
  const identidadSesion = usuario?.session_id ?? 'sin-sesion';
  const [regresoBackup, setRegresoBackup] = useState<{ sesion: string; datos: ContextoDesdeCaja } | null>(null);
  const volverABackup = regresoBackup?.sesion === identidadSesion;
  const [restauracionInicial, setRestauracionInicial] = useState(false);
  useEffect(() => {
    if (cargando || estadoSesion === 'CAMBIO_OBLIGATORIO') return;
    let viva = true;
    void contextoBackup()
      .then((datos) => {
        if (!viva) return;
        sincronizarCapturasBackup(datos);
        setBkp(datos);
        if (datos.pendiente) setModuloActual('backup');
      })
      .catch(() => {
        if (viva) setBkp(null);
      })
      .finally(() => {
        if (viva) setContextoListo(identidadSesion);
      });
    return () => {
      viva = false;
    };
  }, [cargando, usuario?.session_id, usuario?.rol, estadoSesion, identidadSesion]);

  if (cargando || (estadoSesion === 'NORMAL' && contextoListo !== identidadSesion)) {
    return (
      <div className="gloria-theme gt-boot" role="status">
        <LoaderCircle className="gt-boot-spinner" size={18} aria-hidden="true" />
        Comprobando sesión…
      </div>
    );
  }

  if (estadoSesion === 'SIN_SESION' || estadoSesion === 'CAMBIO_OBLIGATORIO' || !usuario) {
    if (restauracionInicial && !usuario && bkp?.inicial) {
      return (
        <div className="bkp-login">
          <BackupPage inicial onVolver={() => setRestauracionInicial(false)} />
        </div>
      );
    }
    return (
      <>
        <LoginPage />
        {!usuario && bkp?.inicial && (
          <button className="gloria-theme gt-button gt-button--primary bkp-entrada" onClick={() => setRestauracionInicial(true)}>
            Restaurar una copia
          </button>
        )}
      </>
    );
  }

  const navegarAFichaVehicular = (placa?: string) => {
    if (placa) {
      setPlacaSeleccionada(placa.toUpperCase().trim());
    }
    setModuloActual('ficha_vehicular');
  };

  const puedeReportes = usuario.rol === 'ADMINISTRADOR' && !bkp?.pendiente;
  const abrirOrigenReporte = (origen: OrigenReporte) => {
    if (!puedeReportes) return;
    if (origen.destino === 'pos') {
      setOtParaCobrar(null);
      setVentaParaConsultar(origen.id);
    }
    if (origen.destino === 'administracion') setPestanaAdmin('usuarios');
    setOrigenReporte(origen);
    setModuloActual(origen.destino);
  };
  const regresarAReportes = () => {
    if (hayCambiosSinGuardar && !window.confirm('¿Descartar los cambios y volver a Reportes?')) return;
    setHayCambiosSinGuardar(false);
    setOrigenReporte(null);
    setVentaParaConsultar(null);
    setModuloActual('reportes');
    requestAnimationFrame(() => document.getElementById('host-reportes')?.focus());
  };
  const renderModulo = () => {
    switch (moduloActual) {
      case 'reportes':
        return puedeReportes ? null : <DashboardPage setModuloActual={setModuloActual} />;
      case 'ordenes':
      case 'taller':
      case 'ot':
        return tienePermiso('OT', 'ver') || tienePermiso('ORDENES', 'ver') ? (
          <OrdenesPage
            idOrigenReporte={origenReporte?.destino === 'taller' ? origenReporte.id : undefined}
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
      case 'administracion':
        return usuario.rol === 'ADMINISTRADOR' && !bkp?.pendiente ? (
          <React.Suspense fallback={<p role="status">Cargando Administración…</p>}>
            <AdministracionPage
              key={alcanceAdmin}
              idOrigenReporte={origenReporte?.destino === 'administracion' ? origenReporte.id : undefined}
              pestana={pestanaAdmin}
              onPestana={setPestanaAdmin}
              onNavegar={setModuloActual}
              filtrosUsuarios={listadoAdmin?.alcance === alcanceAdmin ? listadoAdmin.filtros : FILTROS_USUARIOS_INICIALES}
              onFiltrosUsuarios={(filtros) => setListadoAdmin({ alcance: alcanceAdmin, filtros })}
            />
          </React.Suspense>
        ) : <DashboardPage setModuloActual={setModuloActual} />;
      case 'backup':
        return tienePermiso('BACKUP', 'ver') ? (
          <BackupPage
            idOrigenReporte={origenReporte?.destino === 'backup' ? origenReporte.id : undefined}
            contextoCaja={volverABackup ? regresoBackup?.datos : undefined}
            onCaja={(datos) => {
              setRegresoBackup({ sesion: identidadSesion, datos });
              setModuloActual('caja');
            }}
            onFinalizar={() => setBkp((prev) => ({ ...prev, pendiente: false }))}
          />
        ) : (
          <DashboardPage setModuloActual={setModuloActual} />
        );
      case 'caja':
        return tienePermiso('CAJA', 'ver') ? (
          <React.Suspense fallback={<p role="status">Cargando Caja Chica…</p>}>
            <CajaPage
              idOrigenReporte={origenReporte?.destino === 'caja' ? origenReporte.id : undefined}
              onVenta={(id) => {
                if (bkp?.pendiente || !tienePermiso('POS', 'ver')) return;
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
          <InventarioPage origenReporte={origenReporte?.destino === 'inventario' ? origenReporte.producto : undefined} />
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
        setModuloActual={(destino) => {
          if (bkp?.pendiente && destino !== 'backup' && destino !== 'caja') return;
          if (destino !== 'backup' && destino !== 'caja') setRegresoBackup(null);
          setOrigenReporte(null);
          setVentaParaConsultar(null);
          setModuloActual(destino);
        }}
        onAbrirCambioClave={() => setMostrarCambioClaveModal(true)}
      />
      <main style={{ flex: 1, display: 'flex', flexDirection: 'column', overflow: 'hidden' }}>
        {(bkp?.pendiente || volverABackup) && moduloActual === 'caja' && (
          <div className="gloria-theme bkp-aviso">
            <button className="gt-button gt-button--quiet" onClick={() => setModuloActual('backup')}>
              {bkp?.pendiente ? 'Volver a completar recuperación' : 'Volver a Backup y Copias'}
            </button>
          </div>
        )}
        {origenReporte && moduloActual !== 'reportes' && puedeReportes && (
          <div className="gloria-theme rep-retorno">
            <button className="gt-button gt-button--quiet" type="button" onClick={regresarAReportes}>
              <ArrowLeft size={16} aria-hidden="true" /> Volver a Reportes
            </button>
            <span>Registro vigente · tu reporte conserva el corte anterior</span>
          </div>
        )}
        {puedeReportes && (moduloActual === 'reportes' || origenReporte) && (
          <div className="rep-host" id="host-reportes" tabIndex={-1} hidden={moduloActual !== 'reportes'}>
            <React.Suspense fallback={<p role="status">Cargando Reportes…</p>}>
              <ReportesPage key={alcanceAdmin} onOrigen={abrirOrigenReporte} />
            </React.Suspense>
          </div>
        )}
        {renderModulo()}
      </main>

      {mostrarCambioClaveModal && (
        <CambioClaveModal onClose={() => setMostrarCambioClaveModal(false)} />
      )}
    </div>
  );
};

export const App: React.FC = () => {
  const [arranque, setArranque] = useState<BackupArranque | null>(null);
  const [error, setError] = useState('');
  const [intento, setIntento] = useState(0);
  useEffect(() => {
    let vigente = true;
    consultarArranqueBackup()
      .then((datos) => { if (vigente) setArranque(datos); })
      .catch((e) => { if (vigente) setError(errorBackup(e)); });
    return () => { vigente = false; };
  }, [intento]);

  if (!arranque) return (
    <div className="gloria-theme gt-boot">
      {error ? <div className="gt-notice gt-notice--error" role="alert">
        No se pudo comprobar el arranque: {error}
        <button className="gt-button" onClick={() => { setError(''); setIntento((i) => i + 1); }}>Reintentar</button>
      </div> : <p role="status">Comprobando la base activa…</p>}
    </div>
  );
  if (arranque.bloqueado) return <RecuperarArranque datos={arranque} />;
  return <AuthProvider><MainApp /></AuthProvider>;
};

export default App;
