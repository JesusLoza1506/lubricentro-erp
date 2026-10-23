import React, { useState, useEffect } from 'react';
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
      <div style={overlayStyle}>
        <div style={modalBoxStyle}>
          <div
            style={{
              display: 'flex',
              justifyContent: 'space-between',
              alignItems: 'center',
              marginBottom: '16px',
            }}
          >
            <h3 style={{ fontSize: '18px', fontWeight: '800', color: '#F8FAFC', margin: 0 }}>
              🛠️ Corregir Mantenimiento Finalizado (Administrador)
            </h3>
            <button
              type="button"
              disabled={cargando}
              onClick={handleCerrar}
              style={{
                background: 'none',
                border: 'none',
                color: '#94A3B8',
                fontSize: '18px',
                cursor: cargando ? 'not-allowed' : 'pointer',
              }}
            >
              ✕
            </button>
          </div>

          {errorModal && <div style={errorBoxStyle}>⚠️ {errorModal}</div>}

          <form
            onSubmit={onSubmit}
            style={{ display: 'flex', flexDirection: 'column', gap: '12px' }}
          >
            <div>
              <label style={labelStyle}>Resumen Público de Servicios *</label>
              <textarea
                required
                disabled={cargando}
                rows={3}
                value={resumen}
                onChange={(e) => {
                  setIsDirty(true);
                  setResumen(e.target.value);
                }}
                style={{ ...inputStyle, fontFamily: 'inherit', resize: 'vertical' }}
              />
            </div>

            <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '10px' }}>
              <div>
                <label style={labelStyle}>Kilometraje de Salida *</label>
                <input
                  required
                  disabled={cargando}
                  type="number"
                  value={kilometrajeSalida}
                  onChange={(e) => {
                    setIsDirty(true);
                    setKilometrajeSalida(Number(e.target.value));
                  }}
                  style={inputStyle}
                />
              </div>
              <div>
                <label style={labelStyle}>Tipo de Aceite</label>
                <select
                  disabled={!tieneServicioCambioAceite || cargando}
                  value={tipoAceite}
                  onChange={(e) => {
                    setIsDirty(true);
                    setTipoAceite(e.target.value);
                  }}
                  style={{
                    ...inputStyle,
                    backgroundColor: !tieneServicioCambioAceite ? '#0F172A' : '#1E293B',
                    cursor: !tieneServicioCambioAceite || cargando ? 'not-allowed' : 'pointer',
                    opacity: !tieneServicioCambioAceite || cargando ? 0.6 : 1,
                  }}
                >
                  {!tieneServicioCambioAceite ? (
                    <option value="SIN_CAMBIO">SIN CAMBIO DE ACEITE</option>
                  ) : (
                    <>
                      <option value="MINERAL">MINERAL (5,000 km)</option>
                      <option value="SINTETICO">SINTÉTICO (10,000 km)</option>
                    </>
                  )}
                </select>
              </div>
            </div>

            {!tieneServicioCambioAceite && (
              <div style={warningBoxStyle}>
                ℹ️ Esta OT no incluye servicio/producto de cambio de aceite. El próximo kilometraje
                no será calculado automáticamente.
              </div>
            )}

            <div>
              <label style={{ ...labelStyle, color: '#EF4444', fontWeight: '700' }}>
                Motivo de Corrección Administrativa (Obligatorio, 5-250 chars) *
              </label>
              <input
                required
                disabled={cargando}
                type="text"
                placeholder="Ej. Corrección por error de digitación en resumen público"
                value={motivo}
                onChange={(e) => {
                  setIsDirty(true);
                  setMotivo(e.target.value);
                }}
                style={{ ...inputStyle, borderColor: '#EF4444' }}
              />
            </div>

            <div
              style={{
                display: 'flex',
                justifyContent: 'flex-end',
                gap: '10px',
                marginTop: '12px',
              }}
            >
              <button
                type="button"
                disabled={cargando}
                onClick={handleCerrar}
                style={{
                  ...btnSecondaryStyle,
                  opacity: cargando ? 0.6 : 1,
                  cursor: cargando ? 'not-allowed' : 'pointer',
                }}
              >
                Cancelar
              </button>
              <button
                type="submit"
                disabled={cargando}
                style={{
                  ...btnPrimaryStyle,
                  opacity: cargando ? 0.6 : 1,
                  cursor: cargando ? 'not-allowed' : 'pointer',
                }}
              >
                {cargando ? 'Guardando...' : 'Aplicar Corrección'}
              </button>
            </div>
          </form>
        </div>
      </div>

      {confirmModalOpen && (
        <div style={overlayStyle}>
          <div style={{ ...modalBoxStyle, width: '420px', textAlign: 'center' }}>
            <h3
              style={{
                fontSize: '18px',
                fontWeight: '800',
                color: '#F8FAFC',
                marginBottom: '12px',
              }}
            >
              Confirmar Corrección de Historial
            </h3>
            <p
              style={{
                fontSize: '13px',
                color: '#94A3B8',
                marginBottom: '20px',
                lineHeight: '1.5',
              }}
            >
              ¿Está seguro de corregir la Orden de Trabajo {ordenTrabajo.codigo_ot}? Esta acción
              modificará el historial público y registrará el motivo en los logs de auditoría.
            </p>
            <div style={{ display: 'flex', justifyContent: 'center', gap: '12px' }}>
              <button
                type="button"
                disabled={cargando}
                onClick={() => setConfirmModalOpen(false)}
                style={{
                  ...btnSecondaryStyle,
                  opacity: cargando ? 0.6 : 1,
                  cursor: cargando ? 'not-allowed' : 'pointer',
                }}
              >
                Cancelar
              </button>
              <button
                type="button"
                disabled={cargando}
                onClick={ejecutarGuardado}
                style={{
                  ...btnPrimaryStyle,
                  backgroundColor: '#D97706',
                  opacity: cargando ? 0.6 : 1,
                  cursor: cargando ? 'not-allowed' : 'pointer',
                }}
              >
                {cargando ? 'Guardando...' : 'Sí, Corregir OT'}
              </button>
            </div>
          </div>
        </div>
      )}
    </>
  );
};

const overlayStyle: React.CSSProperties = {
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
  zIndex: 1100,
};

const modalBoxStyle: React.CSSProperties = {
  width: '500px',
  backgroundColor: '#0F172A',
  border: '1px solid #334155',
  borderRadius: '16px',
  padding: '24px',
  boxShadow: '0 20px 40px rgba(0,0,0,0.5)',
};

const errorBoxStyle: React.CSSProperties = {
  padding: '10px 14px',
  backgroundColor: 'rgba(239, 68, 68, 0.15)',
  border: '1px solid rgba(239, 68, 68, 0.4)',
  borderRadius: '8px',
  color: '#FCA5A5',
  fontSize: '12px',
};

const warningBoxStyle: React.CSSProperties = {
  padding: '8px 12px',
  backgroundColor: 'rgba(234, 179, 8, 0.12)',
  border: '1px solid rgba(234, 179, 8, 0.3)',
  borderRadius: '8px',
  color: '#FDE047',
  fontSize: '11px',
};

const labelStyle: React.CSSProperties = {
  display: 'block',
  fontSize: '11px',
  color: '#94A3B8',
  marginBottom: '4px',
};

const inputStyle: React.CSSProperties = {
  width: '100%',
  padding: '8px 12px',
  backgroundColor: '#0F172A',
  border: '1px solid #334155',
  borderRadius: '8px',
  color: '#F8FAFC',
  fontSize: '13px',
  outline: 'none',
  boxSizing: 'border-box',
};

const btnPrimaryStyle: React.CSSProperties = {
  padding: '8px 16px',
  backgroundColor: '#2563EB',
  color: '#FFFFFF',
  border: 'none',
  borderRadius: '8px',
  fontWeight: '700',
  cursor: 'pointer',
};

const btnSecondaryStyle: React.CSSProperties = {
  padding: '8px 14px',
  backgroundColor: 'transparent',
  border: '1px solid #334155',
  color: '#CBD5E1',
  borderRadius: '8px',
  cursor: 'pointer',
};
