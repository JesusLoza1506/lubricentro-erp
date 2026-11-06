import React from 'react';
import { ServicioDTO } from '../../../types/inventario';

interface ServiciosTablaProps {
  cargando: boolean;
  servicios: ServicioDTO[];
  esAdmin: boolean;
  onEditar: (servicio: ServicioDTO) => void;
  onToggleEstado: (servicio: ServicioDTO) => void;
  onNuevoServicio: () => void;
}

export const ServiciosTabla: React.FC<ServiciosTablaProps> = ({
  cargando,
  servicios,
  esAdmin,
  onEditar,
  onToggleEstado,
  onNuevoServicio,
}) => {
  return (
    <div style={{ flex: 1, display: 'flex', flexDirection: 'column', gap: '12px' }}>
      {esAdmin && (
        <div style={{ display: 'flex', justifyContent: 'flex-end' }}>
          <button
            onClick={onNuevoServicio}
            style={{
              padding: '8px 16px',
              backgroundColor: '#10B981',
              color: '#FFFFFF',
              border: 'none',
              borderRadius: '8px',
              fontWeight: '700',
              cursor: 'pointer',
            }}
          >
            + Nuevo Servicio
          </button>
        </div>
      )}

      <div
        style={{
          flex: 1,
          backgroundColor: 'rgba(15, 23, 42, 0.6)',
          border: '1px solid #334155',
          borderRadius: '16px',
          overflowY: 'auto',
        }}
      >
        <table
          style={{
            width: '100%',
            borderCollapse: 'collapse',
            textAlign: 'left',
            fontSize: '14px',
          }}
        >
          <thead>
            <tr
              style={{
                borderBottom: '1px solid #334155',
                color: '#94A3B8',
                backgroundColor: '#1E293B',
              }}
            >
              <th style={{ padding: '16px' }}>CÓDIGO</th>
              <th style={{ padding: '16px' }}>NOMBRE DEL SERVICIO</th>
              <th style={{ padding: '16px' }}>DESCRIPCIÓN</th>
              <th style={{ padding: '16px' }}>PRECIO VENTA</th>
              <th style={{ padding: '16px' }}>TIPO</th>
              <th style={{ padding: '16px' }}>ESTADO</th>
              {esAdmin && <th style={{ padding: '16px', textAlign: 'center' }}>ACCIONES</th>}
            </tr>
          </thead>
          <tbody>
            {cargando ? (
              <tr>
                <td colSpan={7} style={{ padding: '32px', textAlign: 'center', color: '#94A3B8' }}>
                  Cargando servicios...
                </td>
              </tr>
            ) : servicios.length === 0 ? (
              <tr>
                <td colSpan={7} style={{ padding: '32px', textAlign: 'center', color: '#94A3B8' }}>
                  No hay servicios de mantenimiento registrados.
                </td>
              </tr>
            ) : (
              servicios.map((s) => (
                <tr
                  key={s.id_servicio}
                  style={{
                    borderBottom: '1px solid rgba(51, 65, 85, 0.4)',
                    opacity: s.activo ? 1 : 0.6,
                  }}
                >
                  <td style={{ padding: '16px', color: '#38BDF8', fontFamily: 'monospace' }}>
                    {s.codigo}
                  </td>
                  <td style={{ padding: '16px', fontWeight: '600', color: '#F8FAFC' }}>
                    {s.nombre}
                  </td>
                  <td style={{ padding: '16px', color: '#94A3B8' }}>{s.descripcion || '-'}</td>
                  <td style={{ padding: '16px', fontWeight: '700', color: '#34D399' }}>
                    S/ {(s.precio_venta_cent / 100).toFixed(2)}
                  </td>
                  <td style={{ padding: '16px' }}>
                    {s.es_cambio_aceite ? (
                      <span
                        style={{
                          padding: '4px 8px',
                          borderRadius: '6px',
                          fontSize: '11px',
                          fontWeight: '700',
                          backgroundColor: 'rgba(251, 191, 36, 0.15)',
                          color: '#FBBF24',
                          border: '1px solid #FBBF24',
                        }}
                      >
                        CAMBIO ACEITE
                      </span>
                    ) : (
                      <span style={{ color: '#94A3B8' }}>GENERAL</span>
                    )}
                  </td>
                  <td style={{ padding: '16px' }}>
                    <span
                      style={{
                        padding: '4px 8px',
                        borderRadius: '6px',
                        fontSize: '11px',
                        fontWeight: '700',
                        backgroundColor: s.activo
                          ? 'rgba(16, 185, 129, 0.15)'
                          : 'rgba(239, 68, 68, 0.15)',
                        color: s.activo ? '#10B981' : '#EF4444',
                      }}
                    >
                      {s.activo ? 'ACTIVO' : 'INACTIVO'}
                    </span>
                  </td>
                  {esAdmin && (
                    <td style={{ padding: '16px', textAlign: 'center' }}>
                      <div style={{ display: 'flex', justifyContent: 'center', gap: '8px' }}>
                        <button
                          onClick={() => onEditar(s)}
                          style={{
                            padding: '6px 10px',
                            backgroundColor: 'rgba(30, 41, 59, 0.8)',
                            border: '1px solid #334155',
                            borderRadius: '8px',
                            cursor: 'pointer',
                          }}
                        >
                          Editar
                        </button>
                        <button
                          onClick={() => onToggleEstado(s)}
                          style={{
                            padding: '6px 10px',
                            backgroundColor: s.activo
                              ? 'rgba(239, 68, 68, 0.15)'
                              : 'rgba(16, 185, 129, 0.15)',
                            border: `1px solid ${s.activo ? '#EF4444' : '#10B981'}`,
                            color: s.activo ? '#EF4444' : '#10B981',
                            borderRadius: '8px',
                            cursor: 'pointer',
                          }}
                        >
                          {s.activo ? 'Desactivar' : 'Activar'}
                        </button>
                      </div>
                    </td>
                  )}
                </tr>
              ))
            )}
          </tbody>
        </table>
      </div>
    </div>
  );
};
