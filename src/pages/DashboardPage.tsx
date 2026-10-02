import React, { useEffect, useState } from 'react';
import { useAuth } from '../context/AuthContext';
import { obtenerDatosDashboard, DashboardDTO } from '../services/dashboardService';
import {
  Wrench,
  ShoppingCart,
  Package,
  BarChart3,
  CheckCircle2,
  Users,
  Clock,
  Car,
} from 'lucide-react';

interface DashboardPageProps {
  setModuloActual: (modulo: string) => void;
}

export const DashboardPage: React.FC<DashboardPageProps> = ({ setModuloActual }) => {
  const { usuario, tienePermiso } = useAuth();
  const [metrics, setMetrics] = useState<DashboardDTO | null>(null);
  const [cargandoMetrics, setCargandoMetrics] = useState<boolean>(false);

  // Evaluaciones basadas estrictamente en la matriz de permisos_rol
  const puedeVerKPIsGlobales = tienePermiso('BACKUP', 'ver') && tienePermiso('COLA_SUNAT', 'ver');
  const esOperativoTallerOnly = tienePermiso('OT', 'ver') && !tienePermiso('POS', 'ver');

  useEffect(() => {
    if (usuario) {
      setCargandoMetrics(true);
      obtenerDatosDashboard(usuario.id_usuario)
        .then((data) => setMetrics(data))
        .catch((err) => console.error('Error al cargar métricas del dashboard:', err))
        .finally(() => setCargandoMetrics(false));
    }
  }, [usuario]);

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
      {/* BANNER DE BIENVENIDA */}
      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
        <div>
          <div style={{ display: 'flex', alignItems: 'center', gap: '10px', marginBottom: '4px' }}>
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
            <span
              style={{
                fontSize: '11px',
                fontWeight: '700',
                padding: '4px 10px',
                borderRadius: '12px',
                backgroundColor: 'rgba(37, 99, 235, 0.2)',
                color: '#38BDF8',
                border: '1px solid rgba(37, 99, 235, 0.4)',
              }}
            >
              {usuario?.rol}
            </span>
          </div>
          <p style={{ color: '#94A3B8', fontSize: '14px', margin: 0 }}>
            {puedeVerKPIsGlobales
              ? 'Resumen general de operaciones, ventas y estado del taller.'
              : esOperativoTallerOnly
                ? 'Listado de tus órdenes de trabajo asignadas y estado de zanjas.'
                : 'Estado de turno de caja y acceso rápido a ventas en mostrador.'}
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
            Sistema Operativo
          </span>
        </div>
      </div>

      {/* METRICAS GLOBALES REALES */}
      {puedeVerKPIsGlobales && (
        <div style={{ display: 'grid', gridTemplateColumns: 'repeat(4, 1fr)', gap: '16px' }}>
          <div style={kpiCardStyle}>
            <span style={{ fontSize: '12px', color: '#94A3B8', fontWeight: '600' }}>
              Ventas del Día
            </span>
            <h3 style={{ fontSize: '24px', fontWeight: '800', color: '#F8FAFC', margin: '4px 0' }}>
              {cargandoMetrics ? '...' : `S/ ${(metrics?.ventas_del_dia || 0).toFixed(2)}`}
            </h3>
            <span style={{ fontSize: '11px', color: '#34D399' }}>
              {metrics?.comprobantes_emitidos || 0} comprobantes emitidos
            </span>
          </div>

          <div style={kpiCardStyle}>
            <span style={{ fontSize: '12px', color: '#94A3B8', fontWeight: '600' }}>
              OTs Activas
            </span>
            <h3 style={{ fontSize: '24px', fontWeight: '800', color: '#F8FAFC', margin: '4px 0' }}>
              {cargandoMetrics ? '...' : `${metrics?.ots_activas || 0} en taller`}
            </h3>
            <span style={{ fontSize: '11px', color: '#38BDF8' }}>Zanjas 1 y 2 en proceso</span>
          </div>

          <div style={kpiCardStyle}>
            <span style={{ fontSize: '12px', color: '#94A3B8', fontWeight: '600' }}>
              Stock Crítico
            </span>
            <h3 style={{ fontSize: '24px', fontWeight: '800', color: '#FBBF24', margin: '4px 0' }}>
              {cargandoMetrics ? '...' : `${metrics?.productos_stock_critico || 0} productos`}
            </h3>
            <span style={{ fontSize: '11px', color: '#94A3B8' }}>En umbral mínimo</span>
          </div>

          <div style={kpiCardStyle}>
            <span style={{ fontSize: '12px', color: '#94A3B8', fontWeight: '600' }}>
              Caja Chica
            </span>
            <h3 style={{ fontSize: '24px', fontWeight: '800', color: '#34D399', margin: '4px 0' }}>
              {cargandoMetrics ? '...' : metrics?.estado_caja || 'Cerrada'}
            </h3>
            <span style={{ fontSize: '11px', color: '#94A3B8' }}>Turno activo</span>
          </div>
        </div>
      )}

      {/* TARJETAS DE ACCESOS RÁPIDOS ADAPTATIVAS */}
      <div style={{ display: 'grid', gridTemplateColumns: 'repeat(3, 1fr)', gap: '20px' }}>
        {tienePermiso('OT', 'ver') && (
          <div style={moduleCardStyle} onClick={() => setModuloActual('taller')}>
            <div style={iconBoxStyle('#2563EB')}>
              <Wrench size={24} color="#38BDF8" />
            </div>
            <h3 style={cardTitleStyle}>Órdenes de Trabajo</h3>
            <p style={cardDescStyle}>
              {esOperativoTallerOnly
                ? 'Revisa tus servicios asignados y actualiza el estado de tu zanja.'
                : 'Gestiona servicios de mantenimiento, cambios de aceite y diagnósticos.'}
            </p>
            <span style={cardLinkStyle('#38BDF8')}>Ver módulo de Taller →</span>
          </div>
        )}

        {tienePermiso('POS', 'ver') && (
          <div style={moduleCardStyle} onClick={() => setModuloActual('pos')}>
            <div style={iconBoxStyle('#059669')}>
              <ShoppingCart size={24} color="#34D399" />
            </div>
            <h3 style={cardTitleStyle}>Punto de Venta / POS</h3>
            <p style={cardDescStyle}>
              Emisión de boletas, facturas y cobros rápidos para la atención en mostrador.
            </p>
            <span style={cardLinkStyle('#34D399')}>Abrir caja y POS →</span>
          </div>
        )}

        {tienePermiso('FICHA_VEHICULAR', 'ver') && (
          <div style={moduleCardStyle} onClick={() => setModuloActual('ficha_vehicular')}>
            <div style={iconBoxStyle('#7C3AED')}>
              <Car size={24} color="#C084FC" />
            </div>
            <h3 style={cardTitleStyle}>Ficha Vehicular</h3>
            <p style={cardDescStyle}>
              Consulta el historial cronológico de cambios de aceite por número de placa.
            </p>
            <span style={cardLinkStyle('#C084FC')}>Buscar vehículo →</span>
          </div>
        )}

        {tienePermiso('INVENTARIO', 'ver') && (
          <div style={moduleCardStyle} onClick={() => setModuloActual('inventario')}>
            <div style={iconBoxStyle('#D97706')}>
              <Package size={24} color="#FBBF24" />
            </div>
            <h3 style={cardTitleStyle}>Inventario & Stock</h3>
            <p style={cardDescStyle}>
              Consulta disponibilidad de lubricantes, repuestos y aceites a granel.
            </p>
            <span style={cardLinkStyle('#FBBF24')}>Ver catálogo de stock →</span>
          </div>
        )}

        {tienePermiso('FIDELIZACION', 'ver') && (
          <div style={moduleCardStyle} onClick={() => setModuloActual('fidelizacion')}>
            <div style={iconBoxStyle('#2563EB')}>
              <Users size={24} color="#38BDF8" />
            </div>
            <h3 style={cardTitleStyle}>Fidelización</h3>
            <p style={cardDescStyle}>
              Recordatorios preventivos de mantenimiento por kilometraje proyectado.
            </p>
            <span style={cardLinkStyle('#38BDF8')}>Ver clientes candidatos →</span>
          </div>
        )}

        {tienePermiso('CAJA', 'ver') && (
          <div style={moduleCardStyle} onClick={() => setModuloActual('caja')}>
            <div style={iconBoxStyle('#059669')}>
              <BarChart3 size={24} color="#34D399" />
            </div>
            <h3 style={cardTitleStyle}>Caja Chica</h3>
            <p style={cardDescStyle}>
              Apertura de turno, arqueo y reporte de cierre de caja diaria.
            </p>
            <span style={cardLinkStyle('#34D399')}>Gestionar mi caja →</span>
          </div>
        )}
      </div>

      {/* SECCIÓN OPERATIVA DE TALLER */}
      {esOperativoTallerOnly && (
        <div
          style={{
            padding: '24px',
            backgroundColor: 'rgba(15, 23, 42, 0.5)',
            border: '1px solid #334155',
            borderRadius: '16px',
            display: 'flex',
            flexDirection: 'column',
            gap: '16px',
          }}
        >
          <div style={{ display: 'flex', alignItems: 'center', gap: '12px' }}>
            <Clock size={20} color="#FBBF24" />
            <h4 style={{ fontSize: '16px', fontWeight: '700', color: '#F8FAFC', margin: 0 }}>
              Mis Órdenes de Trabajo Pendientes ({metrics?.ots_activas || 0})
            </h4>
          </div>
          <p style={{ fontSize: '13px', color: '#94A3B8', margin: 0 }}>
            {metrics?.ots_activas && metrics.ots_activas > 0
              ? `Tienes ${metrics.ots_activas} orden(es) de trabajo activas en taller.`
              : 'No tienes vehículos asignados en este momento. Ingresa al módulo de Órdenes de Trabajo para revisar el tablero de Zanjas.'}
          </p>
        </div>
      )}
    </div>
  );
};

// ESTILOS DE COMPONENTES REUTILIZABLES
const kpiCardStyle: React.CSSProperties = {
  padding: '20px',
  backgroundColor: 'rgba(15, 23, 42, 0.6)',
  border: '1px solid #334155',
  borderRadius: '14px',
  display: 'flex',
  flexDirection: 'column',
};

const moduleCardStyle: React.CSSProperties = {
  padding: '28px',
  backgroundColor: 'rgba(15, 23, 42, 0.6)',
  border: '1px solid #334155',
  borderRadius: '16px',
  cursor: 'pointer',
  transition: 'all 0.2s ease',
};

const iconBoxStyle = (bgColor: string): React.CSSProperties => ({
  padding: '12px',
  backgroundColor: `${bgColor}25`,
  borderRadius: '12px',
  width: 'fit-content',
  marginBottom: '16px',
});

const cardTitleStyle: React.CSSProperties = {
  fontSize: '18px',
  fontWeight: '700',
  color: '#F8FAFC',
  margin: '0 0 8px 0',
};

const cardDescStyle: React.CSSProperties = {
  fontSize: '13px',
  color: '#94A3B8',
  margin: '0 0 20px 0',
  lineHeight: '1.5',
};

const cardLinkStyle = (color: string): React.CSSProperties => ({
  fontSize: '13px',
  color: color,
  fontWeight: '600',
  display: 'flex',
  alignItems: 'center',
  gap: '6px',
});
