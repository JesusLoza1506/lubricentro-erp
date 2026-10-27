import React, { useEffect } from 'react';
import {
  OrdenTrabajoHistorialDTO,
  TipoAceiteOT,
  TableroZanjasDTO,
} from '../../../../../types/ordenTrabajo';
import { MecanicoDTO } from '../../../../../services/ordenesService';

interface AccionesEstadoBarProps {
  ordenActual: OrdenTrabajoHistorialDTO;
  rol: string;
  loading: boolean;
  modoAccion: string;
  setModoAccion: (modo: any) => void;
  hasIniciarHandler: boolean;
  hasFinalizarHandler: boolean;
  hasCancelarHandler: boolean;
  hasReasignarHandler: boolean;
  mecanicos: MecanicoDTO[];
  tablero?: TableroZanjasDTO | null;
  idZanjaSel: number;
  setIdZanjaSel: (v: number) => void;
  idMecanicoInicio: number;
  setIdMecanicoInicio: (v: number) => void;
  kmSalida: number;
  setKmSalida: (v: number) => void;
  tipoAceite: TipoAceiteOT | '';
  setTipoAceite: (v: TipoAceiteOT | '') => void;
  resumenPublico: string;
  setResumenPublico: (v: string) => void;
  motivoAccion: string;
  setMotivoAccion: (v: string) => void;
  idMecanicoNuevo: number;
  setIdMecanicoNuevo: (v: number) => void;
  onIniciarSubmit: () => void;
  onFinalizarSubmit: () => void;
  onCancelarSubmit: () => void;
  onReasignarSubmit: () => void;
}

export const AccionesEstadoBar: React.FC<AccionesEstadoBarProps> = ({
  ordenActual,
  rol,
  loading,
  modoAccion,
  setModoAccion,
  hasIniciarHandler,
  hasFinalizarHandler,
  hasCancelarHandler,
  hasReasignarHandler,
  mecanicos,
  tablero,
  idZanjaSel,
  setIdZanjaSel,
  idMecanicoInicio,
  setIdMecanicoInicio,
  kmSalida,
  setKmSalida,
  tipoAceite,
  setTipoAceite,
  resumenPublico,
  setResumenPublico,
  motivoAccion,
  setMotivoAccion,
  idMecanicoNuevo,
  setIdMecanicoNuevo,
  onIniciarSubmit,
  onFinalizarSubmit,
  onCancelarSubmit,
  onReasignarSubmit,
}) => {
  // Selecciona automáticamente la primera zanja libre si existen opciones devueltas
  useEffect(() => {
    if (modoAccion === 'INICIAR' && tablero?.zanjas) {
      const primeraLibre = tablero.zanjas.find((z) => z.activa && !z.ocupada);
      if (primeraLibre) {
        setIdZanjaSel(primeraLibre.id_zanja);
      } else {
        setIdZanjaSel(0);
      }
    }
  }, [modoAccion, tablero]);

  const hayZanjasValidas = Boolean(tablero?.zanjas && tablero.zanjas.length > 0);

  // CORRECCIÓN PROBLEMA #2: Exige que la zanja elegida esté activa y NO ocupada
  const zanjaElegidaObj = tablero?.zanjas?.find((z) => z.id_zanja === idZanjaSel);
  const zanjaElegidaEsValida = Boolean(
    zanjaElegidaObj && zanjaElegidaObj.activa && !zanjaElegidaObj.ocupada
  );

  return (
    <>
      <div
        style={{
          display: 'flex',
          gap: '8px',
          marginBottom: '16px',
          flexWrap: 'wrap',
        }}
      >
        {ordenActual.estado === 'EN_ESPERA' && hasIniciarHandler && (
          <button
            disabled={loading}
            onClick={() => setModoAccion('INICIAR')}
            style={{
              padding: '6px 12px',
              backgroundColor: '#2563EB',
              color: '#FFF',
              border: 'none',
              borderRadius: '6px',
              cursor: loading ? 'not-allowed' : 'pointer',
              fontSize: '12px',
              fontWeight: '600',
              opacity: loading ? 0.6 : 1,
            }}
          >
            ▶ Iniciar Trabajo en Zanja
          </button>
        )}

        {ordenActual.estado === 'EN_PROCESO' && hasFinalizarHandler && (
          <button
            disabled={loading}
            onClick={() => setModoAccion('FINALIZAR')}
            style={{
              padding: '6px 12px',
              backgroundColor: '#16A34A',
              color: '#FFF',
              border: 'none',
              borderRadius: '6px',
              cursor: loading ? 'not-allowed' : 'pointer',
              fontSize: '12px',
              fontWeight: '600',
              opacity: loading ? 0.6 : 1,
            }}
          >
            ✔ Finalizar OT
          </button>
        )}

        {rol === 'ADMINISTRADOR' && hasReasignarHandler && (
          <button
            disabled={loading}
            onClick={() => setModoAccion('REASIGNAR')}
            style={{
              padding: '6px 12px',
              backgroundColor: '#0284C7',
              color: '#FFF',
              border: 'none',
              borderRadius: '6px',
              cursor: loading ? 'not-allowed' : 'pointer',
              fontSize: '12px',
              fontWeight: '600',
              opacity: loading ? 0.6 : 1,
            }}
          >
            👤 Reasignar Mecánico
          </button>
        )}

        {hasCancelarHandler && (
          <button
            disabled={loading}
            onClick={() => setModoAccion('CANCELAR')}
            style={{
              padding: '6px 12px',
              backgroundColor: '#DC2626',
              color: '#FFF',
              border: 'none',
              borderRadius: '6px',
              cursor: loading ? 'not-allowed' : 'pointer',
              fontSize: '12px',
              fontWeight: '600',
              opacity: loading ? 0.6 : 1,
            }}
          >
            ✖ Cancelar OT
          </button>
        )}
      </div>

      {modoAccion === 'INICIAR' && (
        <div
          style={{
            padding: '12px',
            backgroundColor: '#0F172A',
            borderRadius: '6px',
            marginBottom: '16px',
            border: '1px solid #2563EB',
          }}
        >
          <h5 style={{ margin: '0 0 8px 0', color: '#38BDF8' }}>
            Asignar Zanja de Trabajo (Dinámica)
          </h5>
          <select
            disabled={loading || !hayZanjasValidas}
            value={idZanjaSel}
            onChange={(e) => setIdZanjaSel(Number(e.target.value))}
            style={{
              width: '100%',
              padding: '6px',
              marginBottom: '8px',
              backgroundColor: '#1E293B',
              border: '1px solid #334155',
              color: '#FFF',
            }}
          >
            {hayZanjasValidas ? (
              tablero!.zanjas.map((z) => (
                <option key={z.id_zanja} value={z.id_zanja} disabled={!z.activa || z.ocupada}>
                  Zanja N° {z.id_zanja}{' '}
                  {z.ocupada ? '(Ocupada)' : !z.activa ? '(Inhabilitada)' : '(Libre)'}
                </option>
              ))
            ) : (
              <option value={0} disabled>
                -- No hay zanjas activas disponibles --
              </option>
            )}
          </select>

          {rol === 'ADMINISTRADOR' && !ordenActual.id_mecanico && (
            <>
              <label style={{ fontSize: '12px', color: '#94A3B8' }}>
                Seleccionar Mecánico Responsable *
              </label>
              <select
                disabled={loading}
                value={idMecanicoInicio}
                onChange={(e) => setIdMecanicoInicio(Number(e.target.value))}
                style={{
                  width: '100%',
                  padding: '6px',
                  marginBottom: '8px',
                  backgroundColor: '#1E293B',
                  border: '1px solid #334155',
                  color: '#FFF',
                }}
              >
                <option value={0}>-- Seleccionar Mecánico --</option>
                {(mecanicos || []).map((m) => (
                  <option key={m.id_usuario} value={m.id_usuario}>
                    {m.nombre_completo}
                  </option>
                ))}
              </select>
            </>
          )}

          <div style={{ display: 'flex', gap: '8px', justifyContent: 'flex-end' }}>
            <button
              disabled={loading}
              onClick={() => setModoAccion('NINGUNO')}
              style={{
                padding: '4px 10px',
                backgroundColor: '#475569',
                color: '#FFF',
                border: 'none',
                borderRadius: '4px',
                cursor: loading ? 'not-allowed' : 'pointer',
              }}
            >
              Cancelar
            </button>
            <button
              disabled={loading || !zanjaElegidaEsValida}
              onClick={onIniciarSubmit}
              style={{
                padding: '4px 10px',
                backgroundColor: zanjaElegidaEsValida ? '#2563EB' : '#475569',
                color: '#FFF',
                border: 'none',
                borderRadius: '4px',
                cursor: loading || !zanjaElegidaEsValida ? 'not-allowed' : 'pointer',
                opacity: loading || !zanjaElegidaEsValida ? 0.6 : 1,
              }}
            >
              {loading ? 'Procesando...' : 'Confirmar Inicio'}
            </button>
          </div>
        </div>
      )}

      {modoAccion === 'FINALIZAR' && (
        <div
          style={{
            padding: '12px',
            backgroundColor: '#0F172A',
            borderRadius: '6px',
            marginBottom: '16px',
            border: '1px solid #16A34A',
          }}
        >
          <h5 style={{ margin: '0 0 8px 0', color: '#4ADE80' }}>Finalizar Orden de Trabajo</h5>
          <label style={{ fontSize: '12px', color: '#94A3B8' }}>Km Salida *</label>
          <input
            disabled={loading}
            type="number"
            value={kmSalida}
            onChange={(e) => setKmSalida(Number(e.target.value))}
            style={{
              width: '100%',
              padding: '6px',
              marginBottom: '8px',
              backgroundColor: '#1E293B',
              border: '1px solid #334155',
              color: '#FFF',
            }}
          />
          <label style={{ fontSize: '12px', color: '#94A3B8' }}>Tipo de Aceite Utilizado</label>
          <select
            disabled={loading}
            value={tipoAceite}
            onChange={(e) => setTipoAceite(e.target.value as TipoAceiteOT | '')}
            style={{
              width: '100%',
              padding: '6px',
              marginBottom: '8px',
              backgroundColor: '#1E293B',
              border: '1px solid #334155',
              color: '#FFF',
            }}
          >
            <option value="">No aplica / Sin cambio de aceite</option>
            <option value="MINERAL">MINERAL (5,000 km)</option>
            <option value="SINTETICO">SINTETICO (10,000 km)</option>
          </select>
          <label style={{ fontSize: '12px', color: '#94A3B8' }}>Resumen Público *</label>
          <textarea
            disabled={loading}
            rows={2}
            value={resumenPublico}
            onChange={(e) => setResumenPublico(e.target.value)}
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
              disabled={loading}
              onClick={() => setModoAccion('NINGUNO')}
              style={{
                padding: '4px 10px',
                backgroundColor: '#475569',
                color: '#FFF',
                border: 'none',
                borderRadius: '4px',
              }}
            >
              Cancelar
            </button>
            <button
              disabled={loading}
              onClick={onFinalizarSubmit}
              style={{
                padding: '4px 10px',
                backgroundColor: '#16A34A',
                color: '#FFF',
                border: 'none',
                borderRadius: '4px',
                opacity: loading ? 0.6 : 1,
              }}
            >
              {loading ? 'Finalizando...' : 'Confirmar Finalización'}
            </button>
          </div>
        </div>
      )}

      {modoAccion === 'CANCELAR' && (
        <div
          style={{
            padding: '12px',
            backgroundColor: '#0F172A',
            borderRadius: '6px',
            marginBottom: '16px',
            border: '1px solid #DC2626',
          }}
        >
          <h5 style={{ margin: '0 0 8px 0', color: '#FCA5A5' }}>Cancelar Orden de Trabajo</h5>
          <textarea
            disabled={loading}
            rows={2}
            value={motivoAccion}
            onChange={(e) => setMotivoAccion(e.target.value)}
            placeholder="Motivo de cancelación (mínimo 5 caracteres)..."
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
              disabled={loading}
              onClick={() => setModoAccion('NINGUNO')}
              style={{
                padding: '4px 10px',
                backgroundColor: '#475569',
                color: '#FFF',
                border: 'none',
                borderRadius: '4px',
              }}
            >
              Volver
            </button>
            <button
              disabled={loading}
              onClick={onCancelarSubmit}
              style={{
                padding: '4px 10px',
                backgroundColor: '#DC2626',
                color: '#FFF',
                border: 'none',
                borderRadius: '4px',
                opacity: loading ? 0.6 : 1,
              }}
            >
              {loading ? 'Cancelando...' : 'Confirmar Cancelación'}
            </button>
          </div>
        </div>
      )}

      {modoAccion === 'REASIGNAR' && (
        <div
          style={{
            padding: '12px',
            backgroundColor: '#0F172A',
            borderRadius: '6px',
            marginBottom: '16px',
            border: '1px solid #0284C7',
          }}
        >
          <h5 style={{ margin: '0 0 8px 0', color: '#38BDF8' }}>Reasignar Mecánico</h5>
          <select
            disabled={loading}
            value={idMecanicoNuevo}
            onChange={(e) => setIdMecanicoNuevo(Number(e.target.value))}
            style={{
              width: '100%',
              padding: '6px',
              marginBottom: '8px',
              backgroundColor: '#1E293B',
              border: '1px solid #334155',
              color: '#FFF',
            }}
          >
            <option value={0}>Seleccione Nuevo Mecánico...</option>
            {(mecanicos || []).map((m) => (
              <option key={m.id_usuario} value={m.id_usuario}>
                {m.nombre_completo}
              </option>
            ))}
          </select>
          <textarea
            disabled={loading}
            rows={2}
            value={motivoAccion}
            onChange={(e) => setMotivoAccion(e.target.value)}
            placeholder="Motivo de reasignación (mínimo 5 caracteres)..."
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
              disabled={loading}
              onClick={() => setModoAccion('NINGUNO')}
              style={{
                padding: '4px 10px',
                backgroundColor: '#475569',
                color: '#FFF',
                border: 'none',
                borderRadius: '4px',
              }}
            >
              Cancelar
            </button>
            <button
              disabled={loading}
              onClick={onReasignarSubmit}
              style={{
                padding: '4px 10px',
                backgroundColor: '#0284C7',
                color: '#FFF',
                border: 'none',
                borderRadius: '4px',
                opacity: loading ? 0.6 : 1,
              }}
            >
              {loading ? 'Reasignando...' : 'Confirmar Reasignación'}
            </button>
          </div>
        </div>
      )}
    </>
  );
};
