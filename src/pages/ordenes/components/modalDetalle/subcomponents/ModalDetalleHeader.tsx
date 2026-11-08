import { useNotificarOT, mensajeErrorOT } from '../../../AvisosOT';
import { LockKeyhole, AlertCircle, Pencil } from 'lucide-react';
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
    <section className="ot-detail-header-section" aria-labelledby="ot-private-note-title">
      {(errorMsg || errLocal) && (
        <div className="gt-notice gt-notice--error ot-form-error" role="alert">
          <AlertCircle size={17} aria-hidden="true" /> <span>{errLocal || errorMsg}</span>
        </div>
      )}

      <div className="ot-private-note">
        <div className="ot-private-note-heading">
          <h3 id="ot-private-note-title">
            <LockKeyhole size={15} aria-hidden="true" /> Nota interna
          </h3>
          {puedeEditarNota && !editandoNota && (
            <button
              type="button"
              className="ot-button ot-small"
              onClick={() => {
                setNuevaNota(ordenActual.nota_interna || '');
                setEditandoNota(true);
              }}
            >
              <Pencil size={13} aria-hidden="true" /> Editar
            </button>
          )}
        </div>
        {editandoNota ? (
          <div className="ot-note-editor">
            <textarea
              rows={2}
              value={nuevaNota}
              aria-label="Editar nota interna"
              onChange={(e) => setNuevaNota(e.target.value)}
            />
            <div className="ot-footer-actions">
              <button
                type="button"
                className="ot-button ot-small"
                disabled={saving}
                onClick={() => {
                  setEditandoNota(false);
                  setErrLocal('');
                }}
              >
                Cancelar
              </button>
              <button
                type="button"
                className="ot-button ot-primary ot-small"
                disabled={saving}
                aria-busy={saving}
                onClick={handleGuardarNota}
              >
                {saving ? 'Guardando...' : 'Guardar'}
              </button>
            </div>
          </div>
        ) : (
          <p>{ordenActual.nota_interna || <span>Sin nota interna registrada</span>}</p>
        )}
      </div>
    </section>
  );
};
