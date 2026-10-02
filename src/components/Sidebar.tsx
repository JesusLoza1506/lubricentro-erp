import React from 'react';
import {
  Shield,
  FileText,
  Wrench,
  Package,
  ShoppingCart,
  Receipt,
  Users,
  Activity,
  Database,
  LogOut,
} from 'lucide-react';
import { useAuth } from '../context/AuthContext';

interface SidebarProps {
  moduloActual: string;
  setModuloActual: (modulo: string) => void;
}

export const Sidebar: React.FC<SidebarProps> = ({ moduloActual, setModuloActual }) => {
  const { usuario, tienePermiso, cerrarSesion } = useAuth();

  // Catálogo oficial de los 9 módulos según la especificación
  const menuItems = [
    {
      id: 'dashboard',
      moduloKey: 'DASHBOARD',
      label: 'Inicio',
      icon: Shield,
      siempreVisible: true, // Accesible para todos tras login
    },
    {
      id: 'ficha_vehicular',
      moduloKey: 'FICHA_VEHICULAR',
      label: 'Ficha Vehicular',
      icon: FileText,
      siempreVisible: false,
    },
    {
      id: 'taller',
      moduloKey: 'OT',
      label: 'Órdenes de Trabajo',
      icon: Wrench,
      siempreVisible: false,
    },
    {
      id: 'inventario',
      moduloKey: 'INVENTARIO',
      label: 'Inventario & Stock',
      icon: Package,
      siempreVisible: false,
    },
    {
      id: 'pos',
      moduloKey: 'POS',
      label: 'Punto de Venta / POS',
      icon: ShoppingCart,
      siempreVisible: false,
    },
    {
      id: 'caja',
      moduloKey: 'CAJA',
      label: 'Caja Chica',
      icon: Receipt,
      siempreVisible: false,
    },
    {
      id: 'fidelizacion',
      moduloKey: 'FIDELIZACION',
      label: 'Fidelización',
      icon: Users,
      siempreVisible: false,
    },
    {
      id: 'monitoreo',
      moduloKey: 'COLA_SUNAT',
      label: 'Cola SUNAT',
      icon: Activity,
      siempreVisible: false,
    },
    {
      id: 'backup',
      moduloKey: 'BACKUP',
      label: 'Backup & Copias',
      icon: Database,
      siempreVisible: false,
    },
  ];

  // Ocultamiento estricto: solo se muestran los módulos con puede_ver = 1 en permisos_rol
  const opcionesPermitidas = menuItems.filter(
    (item) => item.siempreVisible || tienePermiso(item.moduloKey, 'ver')
  );

  return (
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
        {/* Header Marca */}
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
          <div>
            <h1 style={{ fontSize: '15px', fontWeight: '800', color: '#F8FAFC', margin: 0 }}>
              Lubricentro Gloria
            </h1>
            <span style={{ fontSize: '11px', color: '#38BDF8', fontWeight: '600' }}>
              ERP Desktop
            </span>
          </div>
        </div>

        {/* Opciones de Navegación */}
        <nav style={{ display: 'flex', flexDirection: 'column', gap: '6px' }}>
          {opcionesPermitidas.map((item) => {
            const Icon = item.icon;
            const esActivo = moduloActual === item.id;
            return (
              <button
                key={item.id}
                onClick={() => setModuloActual(item.id)}
                style={{
                  display: 'flex',
                  alignItems: 'center',
                  gap: '12px',
                  padding: '12px 16px',
                  borderRadius: '10px',
                  border: 'none',
                  backgroundColor: esActivo ? '#2563EB' : 'transparent',
                  color: esActivo ? '#FFFFFF' : '#94A3B8',
                  fontSize: '14px',
                  fontWeight: esActivo ? '700' : '500',
                  cursor: 'pointer',
                  textAlign: 'left',
                  transition: 'all 0.2s ease',
                }}
              >
                <Icon size={18} color={esActivo ? '#FFFFFF' : '#94A3B8'} />
                <span>{item.label}</span>
              </button>
            );
          })}
        </nav>
      </div>

      {/* Footer Usuario Logueado */}
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
            padding: '8px 12px',
            backgroundColor: 'rgba(30, 41, 59, 0.5)',
            borderRadius: '8px',
          }}
        >
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

        <button
          onClick={cerrarSesion}
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
  );
};
