import React from 'react';
import {
  Package,
  Wrench,
  ShoppingCart,
  Receipt,
  Users,
  Activity,
  Database,
  Shield,
  LogOut,
} from 'lucide-react';
import { useAuth } from '../context/AuthContext';

interface SidebarProps {
  moduloActual: string;
  setModuloActual: (modulo: string) => void;
}

export const Sidebar: React.FC<SidebarProps> = ({ moduloActual, setModuloActual }) => {
  const { usuario, cerrarSesion } = useAuth();

  const menuItems = [
    {
      id: 'dashboard',
      label: 'Inicio',
      icon: Shield,
      roles: ['ADMINISTRADOR', 'CAJERO', 'MECANICO'],
    },
    { id: 'inventario', label: 'Inventario & Stock', icon: Package, roles: ['ADMINISTRADOR'] },
    {
      id: 'taller',
      label: 'Órdenes de Trabajo',
      icon: Wrench,
      roles: ['ADMINISTRADOR', 'MECANICO'],
    },
    {
      id: 'pos',
      label: 'Punto de Venta / POS',
      icon: ShoppingCart,
      roles: ['ADMINISTRADOR', 'CAJERO'],
    },
    { id: 'caja', label: 'Caja Chica', icon: Receipt, roles: ['ADMINISTRADOR', 'CAJERO'] },
    { id: 'fidelizacion', label: 'Fidelización', icon: Users, roles: ['ADMINISTRADOR'] },
    { id: 'monitoreo', label: 'Cola SUNAT', icon: Activity, roles: ['ADMINISTRADOR'] },
    { id: 'backup', label: 'Backup & Copias', icon: Database, roles: ['ADMINISTRADOR'] },
  ];

  // Filtrar según el rol activo del usuario
  const opcionesPermitidas = menuItems.filter(
    (item) => usuario?.rol && item.roles.includes(usuario.rol)
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
