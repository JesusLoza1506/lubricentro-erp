import React, { useState, useEffect } from 'react';
import { Pencil, X, Car, Info, AlertCircle, ShieldCheck } from 'lucide-react';
import { HistorialOTPublicoDTO, CorregirHistorialOtPayloadDTO } from '../../../types/vehiculo';

interface Props {
  ordenTrabajo: HistorialOTPublicoDTO | any;
  modalAbierto: boolean;
  setModalAbierto: (abierto: boolean) => void;
  handleGuardar: (payload: CorregirHistorialOtPayloadDTO) => Promise<void>;
}

export const ModalCorregirHistorialOT: React.FC<Props> = ({
  ordenTrabajo,
  modalAbierto,
  setModalAbierto,
  handleGuardar,
}) => {
  const tieneServicioCambioAceite = Boolean(
    Boolean(ordenTrabajo?.tipo_aceite) ||
    ordenTrabajo?.servicios?.some((s: any) => s.es_cambio_aceite === 1) ||
    (ordenTrabajo?.km_proximo !== null && ordenTrabajo?.km_proximo !== undefined) ||
    (ordenTrabajo?.proximo_kilometraje !== null && ordenTrabajo?.proximo_kilometraje !== undefined)
  );

  const [resumen, setResumen] = useState('');
  const [kilometrajeSalida, setKilometrajeSalida] = useState<number>(0);
  const [tipoAceite, setTipoAceite] = useState<string>('MINERAL');
  const [motivo, setMotivo] = useState('');
  const [cargando, setCargando] = useState(false);
  const [isDirty, setIsDirty] = useState(false);
  const [errorModal, setErrorModal] = useState<string | null>(null);

  const [confirmModalOpen, setConfirmModalOpen] = useState(false);

  useEffect(() => {
    if (modalAbierto && ordenTrabajo) {
      setResumen(ordenTrabajo.resumen_publico || ordenTrabajo.observaciones || '');
      setKilometrajeSalida(
        ordenTrabajo.kilometraje_salida ?? ordenTrabajo.kilometraje_ingreso ?? 0
      );

      if (tieneServicioCambioAceite) {
        setTipoAceite(ordenTrabajo.tipo_aceite || 'SINTETICO');
      } else {
        setTipoAceite('SIN_CAMBIO');
      }
      setMotivo('');
      setErrorModal(null);
      setIsDirty(false);
    }
  }, [modalAbierto, ordenTrabajo, tieneServicioCambioAceite]);

  if (!modalAbierto || !ordenTrabajo) return null;

  const handleCerrar = () => {
    if (cargando) return;
    if (isDirty) {
      const confirmar = window.confirm('¿Desea descartar los cambios no guardados?');
      if (!confirmar) return;
    }
    setModalAbierto(false);
  };

  const ejecutarGuardado = async () => {
    setCargando(true);
    setErrorModal(null);

    try {
      await handleGuardar({
        id_ot: ordenTrabajo.id_ot,
        resumen_publico: resumen.trim(),
        kilometraje_salida: Number(kilometrajeSalida),
        tipo_aceite:
          tieneServicioCambioAceite && tipoAceite !== 'SIN_CAMBIO'
            ? (tipoAceite as any)
            : undefined,
        motivo: motivo.trim(),
      });
      setIsDirty(false);
      setModalAbierto(false);
    } catch (err: unknown) {
      if (typeof err === 'string') setErrorModal(err);
      else if (err && typeof err === 'object' && 'message' in err) {
        setErrorModal(String((err as { message: unknown }).message));
      } else {
        setErrorModal('Error al corregir el historial de la OT.');
      }
    } finally {
      setCargando(false);
      setConfirmModalOpen(false);
    }
  };

  const onSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!motivo.trim() || motivo.trim().length < 5) {
      setErrorModal('El motivo es obligatorio (mínimo 5 caracteres).');
      return;
    }

    setConfirmModalOpen(true);
  };

  return (
    <>
      <div className="fv-modal-overlay fv-correction-overlay">
        <div className="fv-dialog fv-correction-dialog" role="dialog" aria-modal={!confirmModalOpen} aria-labelledby="fv-correction-title">
          <header className="fv-dialog-header">
            <div className="fv-dialog-title-group">
              <span className="fv-dialog-icon"><Pencil size={21} /></span>
              <div><span className="fv-eyebrow">Historial · Administración</span><h3 id="fv-correction-title">Corregir atención</h3></div>
            </div>
            <button type="button" disabled={cargando} onClick={handleCerrar} className="fv-icon-button" aria-label="Cerrar corrección"><X size={20} /></button>
          </header>
          <form onSubmit={onSubmit} className="fv-correction-form">
            <div className="fv-dialog-body">
              <div className="fv-correction-context">
                <span className="fv-plate-tag"><Car size={16} />{ordenTrabajo.placa_snapshot || ordenTrabajo.placa || 'Vehículo'}</span>
                <strong>{ordenTrabajo.codigo_ot}</strong>
                <span className="fv-detail-muted">Atención finalizada</span>
              </div>
              <p className="fv-form-intro">Ajusta los datos registrados e indica el motivo de la corrección.</p>
              {errorModal && <div className="fv-form-error" role="alert"><AlertCircle size={17} /><span>{errorModal}</span></div>}
              <div className="fv-field">
                <label htmlFor="fv-correction-summary">Resumen del trabajo <span>*</span></label>
                <textarea id="fv-correction-summary" required disabled={cargando} rows={3} value={resumen} onChange={(e) => {
                  setIsDirty(true);
                  setResumen(e.target.value);
                }} />
              </div>
              <div className="fv-form-grid">
                <div className="fv-field">
                  <label htmlFor="fv-correction-km">Kilometraje de salida <span>*</span></label>
                  <input id="fv-correction-km" required disabled={cargando} type="number" value={kilometrajeSalida} onChange={(e) => {
                    setIsDirty(true);
                    setKilometrajeSalida(Number(e.target.value));
                  }} />
                </div>
                <div className="fv-field">
                  <label htmlFor="fv-correction-oil">Tipo de aceite</label>
                  <select id="fv-correction-oil" disabled={!tieneServicioCambioAceite || cargando} value={tipoAceite} onChange={(e) => {
                    setIsDirty(true);
                    setTipoAceite(e.target.value);
                  }}>
                    {!tieneServicioCambioAceite ? <option value="SIN_CAMBIO">Sin cambio de aceite</option> : <><option value="MINERAL">Mineral (5,000 km)</option><option value="SINTETICO">Sintético (10,000 km)</option></>}
                  </select>
                </div>
              </div>
              {!tieneServicioCambioAceite && <div className="fv-form-note"><Info size={16} /><span>Esta atención no incluye cambio de aceite. No se calculará un próximo kilometraje.</span></div>}
              <div className="fv-field fv-reason-field">
                <label htmlFor="fv-correction-reason">Motivo de la corrección <span>*</span></label>
                <input id="fv-correction-reason" required disabled={cargando} type="text" placeholder="Ej. Se digitó incorrectamente el kilometraje de salida" value={motivo} onChange={(e) => {
                  setIsDirty(true);
                  setMotivo(e.target.value);
                }} aria-describedby="fv-correction-reason-help" />
                <small id="fv-correction-reason-help">Describe el motivo en 5 a 250 caracteres. Quedará registrado en la auditoría.</small>
              </div>
            </div>
            <footer className="fv-dialog-footer">
              <span className="fv-detail-muted"><ShieldCheck size={14} />Corrección administrativa</span>
              <div className="fv-footer-actions"><button type="button" disabled={cargando} onClick={handleCerrar} className="fv-button">Cancelar</button><button type="submit" disabled={cargando} className="fv-button fv-primary">{cargando ? 'Guardando...' : 'Revisar corrección'}</button></div>
            </footer>
          </form>
        </div>
      </div>
      {confirmModalOpen && (
        <div className="fv-modal-overlay fv-correction-confirm-overlay">
          <div className="fv-dialog fv-confirm-dialog" role="dialog" aria-modal="true" aria-labelledby="fv-confirm-correction-title">
            <header className="fv-dialog-header"><div className="fv-dialog-title-group"><span className="fv-dialog-icon"><Pencil size={20} /></span><div><span className="fv-eyebrow">{ordenTrabajo.codigo_ot}</span><h3 id="fv-confirm-correction-title">Confirmar corrección</h3></div></div></header>
            <div className="fv-dialog-body"><p className="fv-form-intro">Se actualizarán los datos de esta atención y se registrará el motivo en la auditoría.</p><div className="fv-work-note"><h4>Motivo indicado</h4><p>{motivo}</p></div></div>
            <footer className="fv-dialog-footer"><button type="button" disabled={cargando} onClick={() => setConfirmModalOpen(false)} className="fv-button">Volver</button><button type="button" disabled={cargando} onClick={ejecutarGuardado} className="fv-button fv-primary">{cargando ? 'Guardando...' : 'Sí, guardar corrección'}</button></footer>
          </div>
        </div>
      )}
    </>
  );
};
