import React from 'react';
import { ModalDetalleOTContainer } from './modalDetalle/ModalDetalleOTContainer';
import { OrdenTrabajoHistorialDTO, TipoAceiteOT } from '../../../types/ordenTrabajo';
import { OrdenTrabajoHistorialDTO as VehiculoOTDTO } from '../../../types/vehiculo';

export interface ModalDetalleOTProps {
  ordenTrabajo?: OrdenTrabajoHistorialDTO | VehiculoOTDTO;
  ot?: OrdenTrabajoHistorialDTO | VehiculoOTDTO;
  modalAbierto?: boolean;
  setModalAbierto?: (abierto: boolean) => void;
  onClose: () => void;
  onIniciarZanja?: (idOt: number, idZanja: number, idMecanico?: number) => Promise<void>;
  onFinalizarOT?: (
    idOt: number,
    kmSalida: number,
    resumen: string,
    tipoAceite?: TipoAceiteOT
  ) => Promise<void>;
  onCancelarOT?: (idOt: number, motivo: string) => Promise<void>;
  onReasignarMecanico?: (idOt: number, nuevoIdMecanico: number, motivo: string) => Promise<void>;
  onActualizarOT?: () => Promise<void>;
}

export const ModalDetalleOT: React.FC<ModalDetalleOTProps> = (props) => {
  return <ModalDetalleOTContainer {...props} />;
};
