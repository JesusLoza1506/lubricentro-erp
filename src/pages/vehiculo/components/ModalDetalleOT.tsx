import React from 'react';
import { OrdenTrabajoHistorialDTO } from '../../../types/vehiculo';
import { Wrench, Calendar, Gauge, FileText, X, Droplet } from 'lucide-react';

interface ModalDetalleOTProps {
  modalAbierto: boolean;
  setModalAbierto: (abierto: boolean) => void;
  ordenTrabajo: OrdenTrabajoHistorialDTO | null;
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
                  ordenTrabajo.estado === 'FINALIZADO'
                    ? 'rgba(52, 211, 153, 0.2)'
                    : 'rgba(251, 191, 36, 0.2)',
                color: ordenTrabajo.estado === 'FINALIZADO' ? '#34D399' : '#FBBF24',
              }}
            >
              {ordenTrabajo.estado}
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

        {/* METADATOS DE LA OT */}
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
            <span>Fecha: {formatearFecha(ordenTrabajo.fecha_ingreso)}</span>
          </div>
          <div style={{ display: 'flex', alignItems: 'center', gap: '8px', color: '#CBD5E1' }}>
            <Gauge size={16} color="#FBBF24" />
            <span>Ingreso: {ordenTrabajo.kilometraje_ingreso.toLocaleString()} km</span>
          </div>
          <div style={{ display: 'flex', alignItems: 'center', gap: '8px', color: '#CBD5E1' }}>
            <Wrench size={16} color="#34D399" />
            <span>Próximo: {ordenTrabajo.proximo_kilometraje.toLocaleString()} km</span>
          </div>
          <div style={{ display: 'flex', alignItems: 'center', gap: '8px', color: '#CBD5E1' }}>
            <FileText size={16} color="#A855F7" />
            <span>Placa: {ordenTrabajo.placa}</span>
          </div>
        </div>

        {/* TIPO DE ACEITE / LUBRICANTE APLICADO */}
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

        {/* DETALLE DE PRODUCTOS Y SERVICIOS */}
        <div>
          <h4
            style={{
              fontSize: '14px',
              fontWeight: '700',
              color: '#F8FAFC',
              margin: '0 0 10px 0',
            }}
          >
            Insumos y Servicios Aplicados
          </h4>
          {!ordenTrabajo.detalles || ordenTrabajo.detalles.length === 0 ? (
            <p style={{ fontSize: '13px', color: '#94A3B8', fontStyle: 'italic', margin: 0 }}>
              No hay detalle estructurado registrado para esta orden.
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
                  <th style={{ padding: '8px', textAlign: 'center' }}>Cant.</th>
                  <th style={{ padding: '8px', textAlign: 'right' }}>P. Unit</th>
                  <th style={{ padding: '8px', textAlign: 'right' }}>Subtotal</th>
                </tr>
              </thead>
              <tbody>
                {ordenTrabajo.detalles.map((item) => (
                  <tr
                    key={item.id_detalle}
                    style={{ borderBottom: '1px solid rgba(51, 65, 85, 0.3)' }}
                  >
                    <td style={{ padding: '8px', color: '#F8FAFC' }}>{item.descripcion}</td>
                    <td style={{ padding: '8px', textAlign: 'center', color: '#CBD5E1' }}>
                      {item.cantidad}
                    </td>
                    <td style={{ padding: '8px', textAlign: 'right', color: '#CBD5E1' }}>
                      S/ {item.precio_unitario.toFixed(2)}
                    </td>
                    <td
                      style={{
                        padding: '8px',
                        textAlign: 'right',
                        fontWeight: '700',
                        color: '#34D399',
                      }}
                    >
                      S/ {item.subtotal.toFixed(2)}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          )}
        </div>

        {/* OBSERVACIONES TÉCNICAS */}
        {ordenTrabajo.observaciones && (
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
              Observaciones del Técnico
            </span>
            <p style={{ fontSize: '13px', color: '#E2E8F0', margin: 0 }}>
              {ordenTrabajo.observaciones}
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
