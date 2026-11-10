import React, { useState } from 'react';
import {
  Shield,
  FileText,
  Wrench,
  Package,
  ShoppingCart,
  Receipt,
  Database,
  BarChart2,
  Home,
  LogOut,
  KeyRound,
  Lock,
  UserRound,
} from 'lucide-react';
import { useAuth } from '../context/AuthContext';
import { ConfirmModal } from './ConfirmModal';
import { ToastNotification } from './ToastNotification';
import logo from '../assets/logo-gloria-login.png';
import '../styles/gloriaTaller.css';
import './sidebarVisual.css';

interface SidebarProps {
  moduloActual: string;
  setModuloActual: (modulo: string) => void;
  onAbrirCambioClave?: () => void;
}

interface MenuItem {
  id: string;
  moduloKey: string;
  label: string;
  icon: React.ElementType;
  siempreVisible: boolean;
  enDesarrollo: boolean;
}

export const Sidebar: React.FC<SidebarProps> = ({
  moduloActual,
  setModuloActual,
  onAbrirCambioClave,
}) => {
  const {
    usuario,
    ambienteActual,
    estadoSesion,
    tienePermiso,
    cerrarSesion,
    hayCambiosSinGuardar,
    setHayCambiosSinGuardar,
  } = useAuth();

  const [mostrarConfirmacion, setMostrarConfirmacion] = useState<boolean>(false);
  const [destinoPendiente, setDestinoPendiente] = useState<string | null>(null);
  const [notificacionDev, setNotificacionDev] = useState<string | null>(null);

  if (estadoSesion !== 'NORMAL' || !usuario) {
    return null;
  }

  const esProdBD = ambienteActual.toUpperCase() === 'PRODUCCION';

  // Módulos organizados según la arquitectura oficial
  const menuItems: MenuItem[] = [
    {
      id: 'dashboard',
      moduloKey: 'DASHBOARD',
      label: 'Inicio',
      icon: Home,
      siempreVisible: true,
      enDesarrollo: false,
    },
    {
      id: 'ficha_vehicular',
      moduloKey: 'FICHA_VEHICULAR',
      label: 'Ficha Vehicular',
      icon: FileText,
      siempreVisible: false,
      enDesarrollo: false,
    },
    {
      id: 'taller',
      moduloKey: 'OT',
      label: 'Órdenes de Trabajo',
      icon: Wrench,
      siempreVisible: false,
      enDesarrollo: false,
    },
    {
      id: 'inventario',
      moduloKey: 'INVENTARIO',
      label: 'Inventario & Stock',
      icon: Package,
      siempreVisible: false,
      enDesarrollo: false,
    },
    {
      id: 'pos',
      moduloKey: 'POS',
      label: 'Punto de Venta / POS',
      icon: ShoppingCart,
      siempreVisible: false,
      enDesarrollo: false,
    },
    {
      id: 'caja',
      moduloKey: 'CAJA',
      label: 'Caja Chica',
      icon: Receipt,
      siempreVisible: false,
      enDesarrollo: false,
    },
    {
      id: 'backup',
      moduloKey: 'BACKUP',
      label: 'Backup y Copias',
      icon: Database,
      siempreVisible: false,
      enDesarrollo: false,
    },
    {
      id: 'administracion',
      moduloKey: 'ADMIN',
      label: 'Administración',
      icon: Shield,
      siempreVisible: false,
      enDesarrollo: false,
    },
    {
      id: 'reportes',
      moduloKey: 'REPORTES',
      label: 'Reportes',
      icon: BarChart2,
      siempreVisible: false,
      enDesarrollo: false,
    },
  ];

  const opcionesPermitidas = menuItems.filter(
    (item) => (item.id !== 'reportes' || usuario?.rol === 'ADMINISTRADOR') &&
      (item.siempreVisible || tienePermiso(item.moduloKey, 'ver'))
  );

  const handleNavegarModulo = (item: MenuItem) => {
    if (item.id === moduloActual) return;

    // Interceptar módulos en construcción
    if (item.enDesarrollo) {
      setNotificacionDev(
        `🚧 El módulo de ${item.label} se encuentra en desarrollo y estará disponible próximamente.`
      );
      return;
    }

    if (hayCambiosSinGuardar) {
      setDestinoPendiente(item.id);
      setMostrarConfirmacion(true);
    } else {
      setModuloActual(item.id);
    }
  };

  const handleAbrirCambioClaveModal = () => {
    if (hayCambiosSinGuardar) {
      setDestinoPendiente('MODAL_CAMBIO_CLAVE');
      setMostrarConfirmacion(true);
    } else if (onAbrirCambioClave) {
      onAbrirCambioClave();
    }
  };

  const handleIntentarCerrarSesion = () => {
    if (hayCambiosSinGuardar) {
      setDestinoPendiente('LOGOUT');
      setMostrarConfirmacion(true);
    } else {
      cerrarSesion();
    }
  };

  const confirmacionSesion =
    destinoPendiente === 'LOGOUT' || destinoPendiente === 'MODAL_CAMBIO_CLAVE';

  const confirmarAccionNavegacion = () => {
    setHayCambiosSinGuardar(false);
    setMostrarConfirmacion(false);

    if (destinoPendiente === 'LOGOUT') {
      cerrarSesion();
    } else if (destinoPendiente === 'MODAL_CAMBIO_CLAVE') {
      if (onAbrirCambioClave) onAbrirCambioClave();
    } else if (destinoPendiente) {
      setModuloActual(destinoPendiente);
    }
    setDestinoPendiente(null);
  };

  return (
    <>
      {notificacionDev && (
        <ToastNotification
          mensaje={notificacionDev}
          tipo="ERROR"
          onCerrar={() => setNotificacionDev(null)}
        />
      )}

      <aside className="gloria-theme gloria-sidebar" aria-label="Panel lateral">
        <header className="sidebar-marca">
          <img className="sidebar-logo" src={logo} alt="Lubricentro Gloria" />
          <div className="sidebar-marca-meta">
            <span className="sidebar-marca-caption">ERP Desktop v7.0</span>
            <span className="sidebar-entorno" data-prueba={!esProdBD}>
              {ambienteActual.toUpperCase()}
            </span>
          </div>
        </header>

        <nav className="sidebar-nav" aria-label="Módulos">
          <p className="sidebar-etiqueta">Módulos</p>
          {opcionesPermitidas.map((item) => {
            const Icon = item.icon;
            const esActivo = moduloActual === item.id;
            return (
              <button
                key={item.id}
                type="button"
                className="sidebar-nav-item"
                aria-current={esActivo ? 'page' : undefined}
                data-pendiente={item.enDesarrollo}
                onClick={() => handleNavegarModulo(item)}
              >
                <Icon size={17} aria-hidden="true" />
                <span className="sidebar-nav-texto">{item.label}</span>
                {item.enDesarrollo && (
                  <span className="sidebar-proximo" title="Próximamente">
                    <Lock size={10} aria-hidden="true" />
                    <span aria-hidden="true">Próx.</span>
                    <span className="gt-sr-only">Próximamente</span>
                  </span>
                )}
              </button>
            );
          })}
        </nav>

        <section className="sidebar-sesion" aria-label="Cuenta y sesión">
          <div className="sidebar-cuenta">
            <span className="sidebar-avatar">
              <UserRound size={17} aria-hidden="true" />
            </span>
            <div className="sidebar-identidad">
              <p className="sidebar-nombre" title={usuario.nombre_completo}>
                {usuario.nombre_completo}
              </p>
              <p className="sidebar-rol">{usuario.rol}</p>
            </div>
          </div>

          {onAbrirCambioClave && (
            <button type="button" className="sidebar-accion" onClick={handleAbrirCambioClaveModal}>
              <KeyRound size={16} aria-hidden="true" />
              Cambiar contraseña
            </button>
          )}

          <button
            type="button"
            className="sidebar-accion sidebar-accion-salir"
            onClick={handleIntentarCerrarSesion}
          >
            <LogOut size={16} aria-hidden="true" />
            Cerrar sesión
          </button>
        </section>
      </aside>

      <ConfirmModal
        abierto={mostrarConfirmacion}
        varianteVisual={confirmacionSesion ? 'gloria' : undefined}
        textoCancelar={confirmacionSesion ? 'Seguir editando' : undefined}
        titulo={confirmacionSesion ? 'Cambios sin guardar' : 'Cambios Sin Guardar'}
        mensaje={
          destinoPendiente === 'LOGOUT'
            ? 'Si cierras sesión, se perderán los cambios que aún no has guardado.'
            : destinoPendiente === 'MODAL_CAMBIO_CLAVE'
              ? 'Si abres el cambio de contraseña, se perderán los cambios del formulario actual.'
              : 'Tienes información o un formulario editado sin guardar. Si navegas fuera o cierras sesión, perderás todos los datos ingresados.'
        }
        textoConfirmar="Sí, Descartar Cambios"
        onConfirmar={confirmarAccionNavegacion}
        onCancelar={() => {
          setMostrarConfirmacion(false);
          setDestinoPendiente(null);
        }}
      />
    </>
  );
};
