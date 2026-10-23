import React from 'react';
import {
  HistorialOTPublicoDTO,
  ServicioOTPublicoDTO,
  InsumoOTPublicoDTO,
} from '../../../types/vehiculo';
import { Wrench, Calendar, Gauge, FileText, X, Droplet, UserCheck, User, Car } from 'lucide-react';

interface ModalDetalleOTProps {
  modalAbierto: boolean;
  setModalAbierto: (abierto: boolean) => void;
  ordenTrabajo: HistorialOTPublicoDTO | null;
}

export const ModalDetalleOT: React.FC<ModalDetalleOTProps> = ({
  modalAbierto,
  setModalAbierto,
  ordenTrabajo,
}) => {
  if (!modalAbierto || !ordenTrabajo) return null;

  const formatearFecha = (fechaStr?: string) => {
    if (!fechaStr) return 'N/A';
    const fecha = new Date(fechaStr);
    return isNaN(fecha.getTime()) ? 'N/A' : fecha.toLocaleDateString('es-PE');
  };

  const nombreClienteAtendido =
    ordenTrabajo.nombre_cliente ||
    ordenTrabajo.cliente_nombre ||
    'Cliente Varios / No especificado';

  const docClienteAtendido = ordenTrabajo.documento_cliente || ordenTrabajo.cliente_documento;
  const tipoDocClienteAtendido = ordenTrabajo.tipo_documento_cliente || 'DOC';

  return (
    <div
      style={{
        position: 'fixed',
        top: 0,
        left: 0,
        width: '100vw',
        height: '100vh',
        backgroundColor: 'rgba(0,0,0,0.75)',
        backdropFilter: 'blur(4px)',
        display: 'flex',
        justifyContent: 'center',
        alignItems: 'center',
        zIndex: 1000,
      }}
    >
      <div
        style={{
          width: '560px',
          backgroundColor: '#0F172A',
          border: '1px solid #334155',
          borderRadius: '16px',
          padding: '24px',
          boxShadow: '0 20px 40px rgba(0,0,0,0.5)',
          display: 'flex',
          flexDirection: 'column',
          gap: '20px',
          maxHeight: '90vh',
          overflowY: 'auto',
        }}
      >
        <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
          <div style={{ display: 'flex', alignItems: 'center', gap: '10px' }}>
            <span style={{ fontSize: '20px', fontWeight: '800', color: '#38BDF8' }}>
              {ordenTrabajo.codigo_ot}
            </span>
            <span
              style={{
                fontSize: '11px',
                fontWeight: '700',
                padding: '2px 8px',
                borderRadius: '8px',
                backgroundColor:
                  ordenTrabajo.estado === 'FINALIZADO' || !ordenTrabajo.estado
                    ? 'rgba(52, 211, 153, 0.2)'
                    : ordenTrabajo.estado === 'CANCELADO'
                      ? 'rgba(239, 68, 68, 0.2)'
                      : 'rgba(251, 191, 36, 0.2)',
                color:
                  ordenTrabajo.estado === 'FINALIZADO' || !ordenTrabajo.estado
                    ? '#34D399'
                    : ordenTrabajo.estado === 'CANCELADO'
                      ? '#EF4444'
                      : '#FBBF24',
              }}
            >
              {ordenTrabajo.estado || 'FINALIZADO'}
            </span>
          </div>
          <button
            onClick={() => setModalAbierto(false)}
            style={{
              backgroundColor: 'transparent',
              border: 'none',
              color: '#94A3B8',
              cursor: 'pointer',
            }}
          >
            <X size={20} />
          </button>
        </div>

        {/* IDENTIDAD HISTÓRICA */}
        <div
          style={{
            display: 'grid',
            gridTemplateColumns: '1fr 1fr',
            gap: '12px',
            padding: '12px',
            backgroundColor: 'rgba(30, 41, 59, 0.7)',
            borderRadius: '12px',
            border: '1px solid #334155',
            fontSize: '13px',
          }}
        >
          <div style={{ display: 'flex', alignItems: 'center', gap: '8px', color: '#CBD5E1' }}>
            <Car size={16} color="#38BDF8" />
            <span>
              Placa: <strong>{ordenTrabajo.placa_snapshot || ordenTrabajo.placa}</strong>
            </span>
          </div>
          <div style={{ display: 'flex', alignItems: 'center', gap: '8px', color: '#CBD5E1' }}>
            <User size={16} color="#34D399" />
            <span>
              Cliente: <strong>{nombreClienteAtendido}</strong>
              {docClienteAtendido && ` [${tipoDocClienteAtendido}: ${docClienteAtendido}]`}
            </span>
          </div>
        </div>

        {/* METADATOS DE ATENCIÓN PÚBLICA */}
        <div
          style={{
            display: 'grid',
            gridTemplateColumns: '1fr 1fr',
            gap: '12px',
            padding: '12px',
            backgroundColor: 'rgba(30, 41, 59, 0.5)',
            borderRadius: '12px',
            border: '1px solid #334155',
            fontSize: '13px',
          }}
        >
          <div style={{ display: 'flex', alignItems: 'center', gap: '8px', color: '#CBD5E1' }}>
            <Calendar size={16} color="#38BDF8" />
            <span>
              Fecha Fin: {formatearFecha(ordenTrabajo.fecha_fin || ordenTrabajo.fecha_ingreso)}
            </span>
          </div>
          <div style={{ display: 'flex', alignItems: 'center', gap: '8px', color: '#CBD5E1' }}>
            <Gauge size={16} color="#FBBF24" />
            <span>
              Salida:{' '}
              {(
                ordenTrabajo.kilometraje_salida ??
                ordenTrabajo.kilometraje_ingreso ??
                0
              ).toLocaleString()}{' '}
              km
            </span>
          </div>
          <div style={{ display: 'flex', alignItems: 'center', gap: '8px', color: '#CBD5E1' }}>
            <Wrench size={16} color="#34D399" />
            <span>
              Próximo Cambio:{' '}
              {ordenTrabajo.km_proximo !== null && ordenTrabajo.km_proximo !== undefined
                ? `${ordenTrabajo.km_proximo.toLocaleString()} km`
                : ordenTrabajo.proximo_kilometraje
                  ? `${ordenTrabajo.proximo_kilometraje.toLocaleString()} km`
                  : 'Sin cambio'}
            </span>
          </div>
          <div style={{ display: 'flex', alignItems: 'center', gap: '8px', color: '#CBD5E1' }}>
            <FileText size={16} color="#A855F7" />
            <span>Código: {ordenTrabajo.codigo_ot}</span>
          </div>
          {ordenTrabajo.nombre_mecanico && (
            <div
              style={{
                gridColumn: 'span 2',
                display: 'flex',
                alignItems: 'center',
                gap: '8px',
                color: '#CBD5E1',
                borderTop: '1px solid rgba(51, 65, 85, 0.5)',
                paddingTop: '8px',
              }}
            >
              <UserCheck size={16} color="#38BDF8" />
              <span>
                Mecánico Atendió: <strong>{ordenTrabajo.nombre_mecanico}</strong>
              </span>
            </div>
          )}
        </div>

        {/* TIPO DE ACEITE APLICADO */}
        {ordenTrabajo.tipo_aceite && (
          <div
            style={{
              display: 'flex',
              alignItems: 'center',
              gap: '10px',
              padding: '12px 16px',
              backgroundColor: 'rgba(56, 189, 248, 0.1)',
              borderRadius: '12px',
              border: '1px solid rgba(56, 189, 248, 0.3)',
            }}
          >
            <Droplet size={20} color="#38BDF8" />
            <div>
              <span
                style={{
                  fontSize: '11px',
                  fontWeight: '700',
                  color: '#38BDF8',
                  display: 'block',
                  textTransform: 'uppercase',
                }}
              >
                Aceite / Lubricante Aplicado
              </span>
              <span style={{ fontSize: '14px', fontWeight: '600', color: '#F8FAFC' }}>
                {ordenTrabajo.tipo_aceite}
              </span>
            </div>
          </div>
        )}

        {/* INSUMOS Y SERVICIOS PÚBLICOS (SIN PRECIOS/COSTOS) */}
        <div>
          <h4
            style={{
              fontSize: '14px',
              fontWeight: '700',
              color: '#F8FAFC',
              margin: '0 0 10px 0',
            }}
          >
            Servicios e Insumos Aplicados
          </h4>
          {(!ordenTrabajo.servicios || ordenTrabajo.servicios.length === 0) &&
          (!ordenTrabajo.insumos || ordenTrabajo.insumos.length === 0) ? (
            <p style={{ fontSize: '13px', color: '#94A3B8', fontStyle: 'italic', margin: 0 }}>
              No hay detalle público registrado para esta orden.
            </p>
          ) : (
            <table
              style={{
                width: '100%',
                borderCollapse: 'collapse',
                fontSize: '13px',
                textAlign: 'left',
              }}
            >
              <thead>
                <tr style={{ borderBottom: '1px solid #334155', color: '#94A3B8' }}>
                  <th style={{ padding: '8px' }}>Descripción</th>
                  <th style={{ padding: '8px', textAlign: 'center' }}>Cantidad</th>
                </tr>
              </thead>
              <tbody>
                {ordenTrabajo.servicios?.map((s: ServicioOTPublicoDTO) => (
                  <tr
                    key={`srv-${s.id_detalle}`}
                    style={{ borderBottom: '1px solid rgba(51, 65, 85, 0.3)' }}
                  >
                    <td style={{ padding: '8px', color: '#F8FAFC' }}>🛠️ {s.nombre_snapshot}</td>
                    <td style={{ padding: '8px', textAlign: 'center', color: '#CBD5E1' }}>
                      {s.cantidad}
                    </td>
                  </tr>
                ))}
                {ordenTrabajo.insumos?.map((i: InsumoOTPublicoDTO) => (
                  <tr
                    key={`ins-${i.id_detalle}`}
                    style={{ borderBottom: '1px solid rgba(51, 65, 85, 0.3)' }}
                  >
                    <td style={{ padding: '8px', color: '#F8FAFC' }}>🛢️ {i.nombre_snapshot}</td>
                    <td style={{ padding: '8px', textAlign: 'center', color: '#CBD5E1' }}>
                      {i.cantidad_formateada} {i.unidad_medida || ''}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          )}
        </div>

        {/* RESUMEN PÚBLICO */}
        {(ordenTrabajo.resumen_publico || ordenTrabajo.observaciones) && (
          <div
            style={{
              padding: '12px',
              backgroundColor: 'rgba(15, 23, 42, 0.8)',
              borderRadius: '10px',
              border: '1px solid #334155',
            }}
          >
            <span
              style={{
                fontSize: '11px',
                fontWeight: '700',
                color: '#94A3B8',
                display: 'block',
                marginBottom: '4px',
              }}
            >
              Resumen de Trabajo Realizado
            </span>
            <p style={{ fontSize: '13px', color: '#E2E8F0', margin: 0 }}>
              {ordenTrabajo.resumen_publico || ordenTrabajo.observaciones}
            </p>
          </div>
        )}

        <div style={{ display: 'flex', justifyContent: 'flex-end' }}>
          <button
            onClick={() => setModalAbierto(false)}
            style={{
              padding: '8px 20px',
              backgroundColor: '#1E293B',
              border: '1px solid #334155',
              color: '#F8FAFC',
              borderRadius: '8px',
              fontWeight: '600',
              cursor: 'pointer',
            }}
          >
            Cerrar
          </button>
        </div>
      </div>
    </div>
  );
};
