import { useNotificarOT, mensajeErrorOT } from '../../../AvisosOT';
import { LockKeyhole } from 'lucide-react';
import React, { useState } from 'react';
import { OrdenTrabajoHistorialDTO } from '../../../../../types/ordenTrabajo';
import { ordenesService } from '../../../../../services/ordenesService';

interface ModalDetalleHeaderProps {
  ordenActual: OrdenTrabajoHistorialDTO;
  rol: string;
  idUsuario: number;
  errorMsg?: string;
  onActualizarOT?: () => Promise<void>;
}

export const ModalDetalleHeader: React.FC<ModalDetalleHeaderProps> = ({
  ordenActual,
  rol,
  idUsuario,
  errorMsg,
  onActualizarOT,
}) => {
  const notificar = useNotificarOT();
  const [editandoNota, setEditandoNota] = useState(false);
  const [nuevaNota, setNuevaNota] = useState(ordenActual.nota_interna || '');
  const [saving, setSaving] = useState(false);
  const [errLocal, setErrLocal] = useState('');

  const esTerminal = ordenActual.estado === 'FINALIZADO' || ordenActual.estado === 'CANCELADO';
  const rolUpper = rol.toUpperCase();

  // Autoriza al ADMINISTRADOR y al MECANICO asignado propietario
  const esMecanicoPropietario =
    (rolUpper === 'MECANICO' || rolUpper === 'TECNICO') && ordenActual.id_mecanico === idUsuario;

  const puedeEditarNota = !esTerminal && (rolUpper === 'ADMINISTRADOR' || esMecanicoPropietario);

  const handleGuardarNota = async () => {
    try {
      setSaving(true);
      setErrLocal('');
      await ordenesService.modificarNotaInternaOT(
        {
          id_ot: ordenActual.id_ot,
          nota_interna: nuevaNota,
        },
        idUsuario,
        rol
      );
      setEditandoNota(false);
      notificar('¡Nota interna actualizada correctamente!', 'EXITO');
      try {
        if (onActualizarOT) await onActualizarOT();
      } catch (err: unknown) {
        const mensaje = `La nota se guardó, pero no se pudo actualizar la vista. ${mensajeErrorOT(err, 'Vuelve a consultar la orden.')}`;
        setErrLocal(mensaje);
        notificar(mensaje, 'ERROR');
      }
    } catch (err: any) {
      const msg =
        typeof err === 'string' ? err : err?.message || 'Error al actualizar nota interna';
      setErrLocal(msg);
      notificar(msg, 'ERROR');
    } finally {
      setSaving(false);
    }
  };

  return (
    <section className="ot-detail-header-section">
      <div
        style={{
          display: 'flex',
          justifyContent: 'space-between',
          alignItems: 'center',
          marginBottom: '16px',
        }}
      >
        <h2 style={{ margin: 0, fontSize: '18px' }}>
          OT: <span style={{ color: '#38BDF8' }}>{ordenActual.codigo_ot}</span> ({ordenActual.placa}
          )
        </h2>
        <span
          style={{
            padding: '4px 10px',
            borderRadius: '6px',
            fontSize: '12px',
            fontWeight: 'bold',
            backgroundColor: '#334155',
          }}
        >
          <span className="ot-status" data-state={ordenActual.estado}>{ordenActual.estado.replace(/_/g, ' ')}</span>
        </span>
      </div>

      {(errorMsg || errLocal) && (
        <div role="alert"
          style={{
            padding: '10px',
            backgroundColor: '#7F1D1D',
            color: '#FCA5A5',
            borderRadius: '6px',
            marginBottom: '16px',
            fontSize: '13px',
          }}
        >
          {errLocal || errorMsg}
        </div>
      )}

      <div
        style={{
          padding: '10px',
          backgroundColor: '#0F172A',
          borderLeft: '4px solid #F59E0B',
          borderRadius: '4px',
          marginBottom: '16px',
          fontSize: '13px',
        }}
      >
        <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
          <span>
            <LockKeyhole size={14} aria-hidden="true" /> <strong>Nota interna</strong>
          </span>
          {puedeEditarNota && !editandoNota && (
            <button
              onClick={() => {
                setNuevaNota(ordenActual.nota_interna || '');
                setEditandoNota(true);
              }}
              style={{
                background: 'none',
                border: 'none',
                color: '#38BDF8',
                cursor: 'pointer',
                fontSize: '12px',
                textDecoration: 'underline',
              }}
            >
              Editar
            </button>
          )}
        </div>

        {editandoNota ? (
          <div style={{ marginTop: '8px' }}>
            <textarea
              rows={2}
              value={nuevaNota}
              onChange={(e) => setNuevaNota(e.target.value)}
              style={{
                width: '100%',
                padding: '6px',
                backgroundColor: '#1E293B',
                border: '1px solid #334155',
                color: '#FFF',
                borderRadius: '4px',
                marginBottom: '6px',
              }}
            />
            <div style={{ display: 'flex', gap: '6px', justifyContent: 'flex-end' }}>
              <button
                disabled={saving}
                onClick={() => {
                  setEditandoNota(false);
                  setErrLocal('');
                }}
                style={{
                  padding: '2px 8px',
                  backgroundColor: '#475569',
                  color: '#FFF',
                  border: 'none',
                  borderRadius: '4px',
                }}
              >
                Cancelar
              </button>
              <button
                disabled={saving}
                onClick={handleGuardarNota}
                style={{
                  padding: '2px 8px',
                  backgroundColor: '#16A34A',
                  color: '#FFF',
                  border: 'none',
                  borderRadius: '4px',
                }} data-tone="primary"
              >
                {saving ? 'Guardando...' : 'Guardar'}
              </button>
            </div>
          </div>
        ) : (
          <div style={{ marginTop: '4px', color: '#CBD5E1' }}>
            {ordenActual.nota_interna || (
              <span style={{ color: '#64748B', fontStyle: 'italic' }}>
                Sin nota interna registrada
              </span>
            )}
          </div>
        )}
      </div>
    </section>
  );
};
