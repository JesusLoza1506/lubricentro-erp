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
} from 'lucide-react';
import { useAuth } from '../context/AuthContext';
import { ConfirmModal } from './ConfirmModal';
import { ToastNotification } from './ToastNotification';

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
      enDesarrollo: true,
    },
    {
      id: 'caja',
      moduloKey: 'CAJA',
      label: 'Caja Chica',
      icon: Receipt,
      siempreVisible: false,
      enDesarrollo: true,
    },
    {
      id: 'backup',
      moduloKey: 'BACKUP',
      label: 'Backup & Copias',
      icon: Database,
      siempreVisible: false,
      enDesarrollo: true,
    },
    {
      id: 'administracion',
      moduloKey: 'ADMINISTRACION',
      label: 'Administración',
      icon: Shield,
      siempreVisible: false,
      enDesarrollo: true,
    },
    {
      id: 'reportes',
      moduloKey: 'REPORTES',
      label: 'Reportes',
      icon: BarChart2,
      siempreVisible: false,
      enDesarrollo: true,
    },
  ];

  const opcionesPermitidas = menuItems.filter(
    (item) => item.siempreVisible || tienePermiso(item.moduloKey, 'ver')
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

      <aside
        style={{
          width: '260px',
          backgroundColor: 'rgba(15, 23, 42, 0.8)',
          backdropFilter: 'blur(16px)',
          borderRight: '1px solid #334155',
          display: 'flex',
          flexDirection: 'column',
          justifyContent: 'space-between',
          padding: '24px 16px',
          boxSizing: 'border-box',
        }}
      >
        <div>
          <div
            style={{
              display: 'flex',
              alignItems: 'center',
              gap: '12px',
              padding: '0 12px 24px 12px',
              borderBottom: '1px solid #334155',
              marginBottom: '20px',
            }}
          >
            <div
              style={{
                backgroundColor: '#2563EB',
                padding: '8px',
                borderRadius: '10px',
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'center',
              }}
            >
              <Shield size={20} color="#FFFFFF" />
            </div>
            <div style={{ flex: 1 }}>
              <div style={{ display: 'flex', alignItems: 'center', gap: '6px' }}>
                <h1 style={{ fontSize: '15px', fontWeight: '800', color: '#F8FAFC', margin: 0 }}>
                  Lubricentro Gloria
                </h1>
                <span
                  style={{
                    fontSize: '9px',
                    fontWeight: '800',
                    padding: '2px 5px',
                    borderRadius: '4px',
                    backgroundColor: !esProdBD
                      ? 'rgba(234, 179, 8, 0.2)'
                      : 'rgba(34, 197, 94, 0.2)',
                    color: !esProdBD ? '#FDE047' : '#86EFAC',
                    border: `1px solid ${!esProdBD ? 'rgba(234, 179, 8, 0.4)' : 'rgba(34, 197, 94, 0.4)'}`,
                  }}
                >
                  {ambienteActual.toUpperCase()}
                </span>
              </div>
              <span style={{ fontSize: '11px', color: '#38BDF8', fontWeight: '600' }}>
                ERP Desktop v7.0
              </span>
            </div>
          </div>

          <nav style={{ display: 'flex', flexDirection: 'column', gap: '6px' }}>
            {opcionesPermitidas.map((item) => {
              const Icon = item.icon;
              const esActivo = moduloActual === item.id;
              return (
                <button
                  key={item.id}
                  onClick={() => handleNavegarModulo(item)}
                  style={{
                    display: 'flex',
                    alignItems: 'center',
                    justifyContent: 'space-between',
                    padding: '12px 16px',
                    borderRadius: '10px',
                    border: 'none',
                    backgroundColor: esActivo ? '#2563EB' : 'transparent',
                    color: esActivo ? '#FFFFFF' : item.enDesarrollo ? '#64748B' : '#94A3B8',
                    fontSize: '14px',
                    fontWeight: esActivo ? '700' : '500',
                    cursor: 'pointer',
                    textAlign: 'left',
                    transition: 'all 0.2s ease',
                    opacity: item.enDesarrollo ? 0.75 : 1,
                  }}
                >
                  <div style={{ display: 'flex', alignItems: 'center', gap: '12px' }}>
                    <Icon
                      size={18}
                      color={esActivo ? '#FFFFFF' : item.enDesarrollo ? '#64748B' : '#94A3B8'}
                    />
                    <span>{item.label}</span>
                  </div>
                  {item.enDesarrollo && (
                    <span
                      style={{
                        fontSize: '9px',
                        fontWeight: '700',
                        color: '#94A3B8',
                        backgroundColor: 'rgba(51, 65, 85, 0.6)',
                        padding: '2px 6px',
                        borderRadius: '4px',
                        display: 'flex',
                        alignItems: 'center',
                        gap: '3px',
                      }}
                    >
                      <Lock size={10} /> Próx.
                    </span>
                  )}
                </button>
              );
            })}
          </nav>
        </div>

        <div
          style={{
            borderTop: '1px solid #334155',
            paddingTop: '16px',
            display: 'flex',
            flexDirection: 'column',
            gap: '12px',
          }}
        >
          <div
            style={{
              padding: '10px 12px',
              backgroundColor: 'rgba(30, 41, 59, 0.5)',
              borderRadius: '8px',
              display: 'flex',
              flexDirection: 'column',
              gap: '6px',
            }}
          >
            <div>
              <div
                style={{
                  fontSize: '13px',
                  fontWeight: '700',
                  color: '#F8FAFC',
                  overflow: 'hidden',
                  textOverflow: 'ellipsis',
                  whiteSpace: 'nowrap',
                }}
              >
                {usuario?.nombre_completo}
              </div>
              <div style={{ fontSize: '11px', color: '#38BDF8', fontWeight: '600' }}>
                {usuario?.rol}
              </div>
            </div>

            {onAbrirCambioClave && (
              <button
                onClick={handleAbrirCambioClaveModal}
                style={{
                  display: 'flex',
                  alignItems: 'center',
                  gap: '6px',
                  padding: '6px 8px',
                  marginTop: '4px',
                  backgroundColor: 'rgba(37, 99, 235, 0.15)',
                  color: '#38BDF8',
                  border: '1px solid rgba(56, 189, 248, 0.3)',
                  borderRadius: '6px',
                  cursor: 'pointer',
                  fontSize: '11px',
                  fontWeight: '600',
                  transition: 'all 0.2s ease',
                }}
              >
                <KeyRound size={13} />
                <span>Cambiar Contraseña</span>
              </button>
            )}
          </div>

          <button
            onClick={handleIntentarCerrarSesion}
            style={{
              display: 'flex',
              alignItems: 'center',
              gap: '8px',
              padding: '10px 12px',
              backgroundColor: 'rgba(220, 38, 38, 0.15)',
              color: '#FCA5A5',
              border: '1px solid rgba(239, 68, 68, 0.3)',
              borderRadius: '8px',
              cursor: 'pointer',
              fontSize: '13px',
              fontWeight: '600',
            }}
          >
            <LogOut size={16} />
            <span>Cerrar Sesión</span>
          </button>
        </div>
      </aside>

      <ConfirmModal
        abierto={mostrarConfirmacion}
        titulo="Cambios Sin Guardar"
        mensaje="Tienes información o un formulario editado sin guardar. Si navegas fuera o cierras sesión, perderás todos los datos ingresados."
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
