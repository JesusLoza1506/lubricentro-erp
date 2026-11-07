import '../../ordenesVisual.css';
import React from 'react';
import { Car, ClipboardList, X } from 'lucide-react';
import { ModalDetalleOTProps } from '../ModalDetalleOT';
import { useModalDetalleOT } from './useModalDetalleOT';
import { ModalDetalleHeader } from './subcomponents/ModalDetalleHeader';
import { AccionesEstadoBar } from './subcomponents/AccionesEstadoBar';
import { TablaServiciosOT } from './subcomponents/TablaServiciosOT';
import { TablaInsumosOT } from './subcomponents/TablaInsumosOT';
import { useAuth } from '../../../../context/AuthContext';

export const ModalDetalleOTContainer: React.FC<ModalDetalleOTProps> = (props) => {
  const { onClose, onActualizarOT } = props;
  const state = useModalDetalleOT(props);
  const { usuario } = useAuth();
  const idUsuario = (usuario as any)?.id_usuario || (usuario as any)?.id || 0;

  return (
    <div className="gloria-theme gt-overlay ot-overlay">
      <div className="gt-dialog ot-dialog ot-detail-dialog" role="dialog" aria-modal="true"
        aria-label="Detalle de orden de trabajo">
        <header className="ot-dialog-heading">
          <span className="ot-dialog-icon"><ClipboardList size={21} aria-hidden="true" /></span>
          <div>
            <span className="ot-eyebrow">Atención de taller</span>
            <h2>OT: {state.ordenActual.codigo_ot}</h2>
            <p className="ot-detail-plate"><Car size={15} aria-hidden="true" /> {state.ordenActual.placa}</p>
          </div>
          <span className="ot-status ot-detail-state" data-state={state.ordenActual.estado}>
            {state.ordenActual.estado.replace(/_/g, ' ')}
          </span>
          <button type="button" className="ot-button ot-close" aria-label="Cerrar detalle" onClick={onClose}>
            <X size={18} aria-hidden="true" />
          </button>
        </header>

        <div className="ot-detail-body">
          <ModalDetalleHeader
            ordenActual={state.ordenActual}
            errorMsg={state.errorMsg}
            rol={state.rol}
            idUsuario={idUsuario}
            onActualizarOT={onActualizarOT || (async () => {})}
          />

          {!state.esTerminal && (
            <AccionesEstadoBar
              ordenActual={state.ordenActual}
              rol={state.rol}
              loading={state.loading}
              modoAccion={state.modoAccion}
              setModoAccion={state.setModoAccion}
              hasIniciarHandler={state.hasIniciarHandler}
              hasFinalizarHandler={state.hasFinalizarHandler}
              hasCancelarHandler={state.hasCancelarHandler}
              hasReasignarHandler={state.hasReasignarHandler}
              mecanicos={state.mecanicos}
              tablero={state.tablero}
              idZanjaSel={state.idZanjaSel}
              setIdZanjaSel={state.setIdZanjaSel}
              idMecanicoInicio={state.idMecanicoInicio}
              setIdMecanicoInicio={state.setIdMecanicoInicio}
              kmSalida={state.kmSalida}
              setKmSalida={state.setKmSalida}
              tipoAceite={state.tipoAceite}
              setTipoAceite={state.setTipoAceite}
              resumenPublico={state.resumenPublico}
              setResumenPublico={state.setResumenPublico}
              motivoAccion={state.motivoAccion}
              setMotivoAccion={state.setMotivoAccion}
              idMecanicoNuevo={state.idMecanicoNuevo}
              setIdMecanicoNuevo={state.setIdMecanicoNuevo}
              onIniciarSubmit={state.handleIniciarSubmit}
              onFinalizarSubmit={state.handleFinalizarSubmit}
              onCancelarSubmit={state.handleCancelarSubmit}
              onReasignarSubmit={state.handleReasignarSubmit}
            />
          )}

          <TablaServiciosOT
            ordenActual={state.ordenActual}
            puedeAgregarServicios={state.puedeAgregarServicios}
            modoAccion={state.modoAccion}
            setModoAccion={state.setModoAccion}
            serviciosCatalogo={state.serviciosCatalogo}
            idServicioSel={state.idServicioSel}
            setIdServicioSel={state.setIdServicioSel}
            cantServicioSel={state.cantServicioSel}
            setCantServicioSel={state.setCantServicioSel}
            setIdDetalleServicioEdit={state.setIdDetalleServicioEdit}
            onAgregarServicioSubmit={state.handleAgregarServicioSubmit}
            onModificarServicioSubmit={state.handleModificarServicioSubmit}
            onEliminarServicio={state.handleEliminarServicio}
          />

          <TablaInsumosOT
            ordenActual={state.ordenActual}
            esTerminal={state.esTerminal}
            modoAccion={state.modoAccion}
            setModoAccion={state.setModoAccion}
            productosCatalogo={state.productosCatalogo}
            idProductoSel={state.idProductoSel}
            setIdProductoSel={state.setIdProductoSel}
            cantProductoSel={state.cantProductoSel}
            setCantProductoSel={state.setCantProductoSel}
            esGranelSel={state.esGranelSel}
            cantDevolver={state.cantDevolver}
            setCantDevolver={state.setCantDevolver}
            motivoAccion={state.motivoAccion}
            setMotivoAccion={state.setMotivoAccion}
            setIdDetalleDevolver={state.setIdDetalleDevolver}
            onAgregarInsumoSubmit={state.handleAgregarInsumoSubmit}
            onDevolverInsumoSubmit={state.handleDevolverInsumoSubmit}
            procesandoDespacho={state.loading && state.modoAccion === 'AGREGAR_INSUMO'}
            procesandoDevolucion={state.loading && state.modoAccion === 'DEVOLVER'}
          />

        </div>
        <footer className="ot-dialog-footer ot-detail-footer">
          <strong>Total de la orden: S/ {(state.ordenActual.total_general_soles ?? 0).toFixed(2)}</strong>
          <div className="ot-footer-actions">
            <button type="button" className="ot-button" onClick={onClose}>Cerrar</button>
            {state.ordenActual.estado === 'FINALIZADO' && state.rol !== 'MECANICO' && props.onCobrar && (
              <button type="button" className="ot-button ot-primary" onClick={() => props.onCobrar?.(state.ordenActual.id_ot)}>
                Cobrar en POS
              </button>
            )}
          </div>
        </footer>
      </div>
    </div>
  );
};
