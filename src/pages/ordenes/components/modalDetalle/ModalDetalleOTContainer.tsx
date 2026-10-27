import React from 'react';
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
    <div
      style={{
        position: 'fixed',
        top: 0,
        left: 0,
        right: 0,
        bottom: 0,
        backgroundColor: 'rgba(15, 23, 42, 0.85)',
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center',
        zIndex: 1000,
      }}
    >
      <div
        style={{
          backgroundColor: '#1E293B',
          borderRadius: '12px',
          width: '100%',
          maxWidth: '820px',
          padding: '24px',
          border: '1px solid #334155',
          color: '#F8FAFC',
          maxHeight: '90vh',
          overflowY: 'auto',
        }}
      >
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

        <div
          style={{
            display: 'flex',
            justifyContent: 'space-between',
            alignItems: 'center',
            marginTop: '20px',
            paddingTop: '12px',
            borderTop: '1px solid #334155',
          }}
        >
          <strong style={{ fontSize: '16px', color: '#10B981' }}>
            Total General: S/ {(state.ordenActual.total_general_soles ?? 0).toFixed(2)}
          </strong>
          <button
            onClick={onClose}
            style={{
              padding: '8px 16px',
              backgroundColor: '#475569',
              color: '#FFF',
              border: 'none',
              borderRadius: '6px',
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
