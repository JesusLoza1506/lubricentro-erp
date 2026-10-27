import React from 'react';
import { OrdenTrabajoHistorialDTO } from '../../../../../types/ordenTrabajo';
import { ServicioResumenDTO } from '../../../../../services/ordenesService';

interface TablaServiciosOTProps {
  ordenActual: OrdenTrabajoHistorialDTO;
  puedeAgregarServicios: boolean;
  modoAccion: string;
  setModoAccion: (modo: any) => void;
  serviciosCatalogo: ServicioResumenDTO[];
  idServicioSel: number;
  setIdServicioSel: (v: number) => void;
  cantServicioSel: number;
  setCantServicioSel: (v: number) => void;
  setIdDetalleServicioEdit: (v: number) => void;
  onAgregarServicioSubmit: () => void;
  onModificarServicioSubmit: () => void;
  onEliminarServicio: (idDetalle: number) => void;
}

export const TablaServiciosOT: React.FC<TablaServiciosOTProps> = ({
  ordenActual,
  puedeAgregarServicios,
  modoAccion,
  setModoAccion,
  serviciosCatalogo,
  idServicioSel,
  setIdServicioSel,
  cantServicioSel,
  setCantServicioSel,
  setIdDetalleServicioEdit,
  onAgregarServicioSubmit,
  onModificarServicioSubmit,
  onEliminarServicio,
}) => {
  return (
    <>
      <div
        style={{
          display: 'flex',
          justifyContent: 'space-between',
          alignItems: 'center',
          marginBottom: '8px',
        }}
      >
        <h4 style={{ color: '#38BDF8', margin: 0 }}>Servicios Aplicados</h4>
        {puedeAgregarServicios && (
          <button
            onClick={() => setModoAccion('AGREGAR_SERVICIO')}
            style={{
              padding: '4px 8px',
              backgroundColor: '#2563EB',
              color: '#FFF',
              border: 'none',
              borderRadius: '4px',
              fontSize: '12px',
              cursor: 'pointer',
            }}
          >
            + Agregar Servicio
          </button>
        )}
      </div>

      {modoAccion === 'AGREGAR_SERVICIO' && puedeAgregarServicios && (
        <div
          style={{
            padding: '10px',
            backgroundColor: '#0F172A',
            borderRadius: '6px',
            marginBottom: '12px',
          }}
        >
          <label style={{ fontSize: '12px', color: '#94A3B8' }}>Seleccionar Servicio</label>
          <select
            value={idServicioSel}
            onChange={(e) => setIdServicioSel(Number(e.target.value))}
            style={{
              width: '100%',
              padding: '6px',
              marginBottom: '8px',
              backgroundColor: '#1E293B',
              border: '1px solid #334155',
              color: '#FFF',
            }}
          >
            <option value={0}>-- Seleccionar Servicio --</option>
            {(serviciosCatalogo || []).map((s) => (
              <option key={s.id_servicio} value={s.id_servicio}>
                {s.nombre} - S/ {(s.precio_venta_soles ?? 0).toFixed(2)}
              </option>
            ))}
          </select>
          <label style={{ fontSize: '12px', color: '#94A3B8' }}>Cantidad</label>
          <input
            type="number"
            min={1}
            value={cantServicioSel}
            onChange={(e) => setCantServicioSel(Number(e.target.value))}
            style={{
              width: '100%',
              padding: '6px',
              marginBottom: '8px',
              backgroundColor: '#1E293B',
              border: '1px solid #334155',
              color: '#FFF',
            }}
          />
          <div style={{ display: 'flex', gap: '8px', justifyContent: 'flex-end' }}>
            <button
              onClick={() => setModoAccion('NINGUNO')}
              style={{
                padding: '4px 8px',
                backgroundColor: '#475569',
                color: '#FFF',
                border: 'none',
                borderRadius: '4px',
              }}
            >
              Cancelar
            </button>
            <button
              onClick={onAgregarServicioSubmit}
              style={{
                padding: '4px 8px',
                backgroundColor: '#2563EB',
                color: '#FFF',
                border: 'none',
                borderRadius: '4px',
              }}
            >
              Guardar Servicio
            </button>
          </div>
        </div>
      )}

      {modoAccion === 'EDITAR_SERVICIO' && puedeAgregarServicios && (
        <div
          style={{
            padding: '10px',
            backgroundColor: '#0F172A',
            borderRadius: '6px',
            marginBottom: '12px',
          }}
        >
          <label style={{ fontSize: '12px', color: '#94A3B8' }}>Nueva Cantidad de Servicio</label>
          <input
            type="number"
            min={1}
            value={cantServicioSel}
            onChange={(e) => setCantServicioSel(Number(e.target.value))}
            style={{
              width: '100%',
              padding: '6px',
              marginBottom: '8px',
              backgroundColor: '#1E293B',
              border: '1px solid #334155',
              color: '#FFF',
            }}
          />
          <div style={{ display: 'flex', gap: '8px', justifyContent: 'flex-end' }}>
            <button
              onClick={() => setModoAccion('NINGUNO')}
              style={{
                padding: '4px 8px',
                backgroundColor: '#475569',
                color: '#FFF',
                border: 'none',
                borderRadius: '4px',
              }}
            >
              Cancelar
            </button>
            <button
              onClick={onModificarServicioSubmit}
              style={{
                padding: '4px 8px',
                backgroundColor: '#0284C7',
                color: '#FFF',
                border: 'none',
                borderRadius: '4px',
              }}
            >
              Actualizar Cantidad
            </button>
          </div>
        </div>
      )}

      <table
        style={{
          width: '100%',
          borderCollapse: 'collapse',
          marginBottom: '16px',
          fontSize: '13px',
        }}
      >
        <thead>
          <tr style={{ borderBottom: '1px solid #334155', color: '#94A3B8' }}>
            <th style={{ textAlign: 'left', padding: '6px' }}>Servicio</th>
            <th style={{ padding: '6px' }}>Cant.</th>
            <th style={{ padding: '6px' }}>P. Unit</th>
            <th style={{ padding: '6px' }}>Subtotal</th>
            {puedeAgregarServicios && <th style={{ padding: '6px' }}>Acciones</th>}
          </tr>
        </thead>
        <tbody>
          {ordenActual.servicios && ordenActual.servicios.length > 0 ? (
            ordenActual.servicios.map((s) => (
              <tr key={s.id_detalle} style={{ borderBottom: '1px solid #0F172A' }}>
                <td style={{ padding: '6px' }}>{s.nombre_snapshot}</td>
                <td style={{ padding: '6px', textAlign: 'center' }}>{s.cantidad}</td>
                <td style={{ padding: '6px', textAlign: 'right' }}>
                  S/ {(s.precio_unitario_soles ?? 0).toFixed(2)}
                </td>
                <td style={{ padding: '6px', textAlign: 'right', fontWeight: 'bold' }}>
                  S/ {(s.subtotal_soles ?? 0).toFixed(2)}
                </td>
                {puedeAgregarServicios && (
                  <td style={{ padding: '6px', textAlign: 'center' }}>
                    <button
                      onClick={() => {
                        setIdDetalleServicioEdit(s.id_detalle);
                        setCantServicioSel(s.cantidad);
                        setModoAccion('EDITAR_SERVICIO');
                      }}
                      style={{
                        padding: '2px 6px',
                        backgroundColor: '#0284C7',
                        color: '#FFF',
                        border: 'none',
                        borderRadius: '4px',
                        cursor: 'pointer',
                        fontSize: '11px',
                        marginRight: '4px',
                      }}
                    >
                      Editar
                    </button>
                    <button
                      onClick={() => onEliminarServicio(s.id_detalle)}
                      style={{
                        padding: '2px 6px',
                        backgroundColor: '#DC2626',
                        color: '#FFF',
                        border: 'none',
                        borderRadius: '4px',
                        cursor: 'pointer',
                        fontSize: '11px',
                      }}
                    >
                      Retirar
                    </button>
                  </td>
                )}
              </tr>
            ))
          ) : (
            <tr>
              <td
                colSpan={puedeAgregarServicios ? 5 : 4}
                style={{ padding: '8px', textAlign: 'center', color: '#94A3B8' }}
              >
                Sin servicios registrados
              </td>
            </tr>
          )}
        </tbody>
      </table>
    </>
  );
};
