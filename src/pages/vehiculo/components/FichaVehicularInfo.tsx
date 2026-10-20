import React from 'react';
import { VehiculoCompletoDTO } from '../../../types/vehiculo';
import {
  User,
  Phone,
  Pencil,
  Trash2,
  RotateCcw,
  Car,
  Gauge,
  Wrench,
  Calendar,
  MapPin,
  FileText,
  Mail,
  ShieldAlert,
  UserCheck,
} from 'lucide-react';

interface Props {
  vehiculo: VehiculoCompletoDTO;
  puedeEditar: boolean;
  puedeEliminar: boolean;
  onAbrirEditar: () => void;
  onAbrirEliminar: () => void;
  onAbrirReactivar: () => void;
  onAbrirModalCliente?: () => void;
  accionesVehiculo?: React.ReactNode;
}

export const FichaVehicularInfo: React.FC<Props> = ({
  vehiculo,
  puedeEditar,
  puedeEliminar,
  onAbrirEditar,
  onAbrirEliminar,
  onAbrirReactivar,
  onAbrirModalCliente,
  accionesVehiculo,
}) => {
  const tipoDoc = vehiculo.cliente?.tipo_documento || vehiculo.tipo_documento_cliente || 'DNI';
  const numDoc = vehiculo.cliente?.numero_documento || vehiculo.documento_cliente;
  const telefono = vehiculo.cliente?.telefono || vehiculo.telefono_cliente;
  const correo = vehiculo.cliente?.correo || vehiculo.correo_cliente;
  const direccion = vehiculo.cliente?.direccion || vehiculo.direccion_cliente;
  const esActivo = vehiculo.activo === 1;

  return (
    <section className="fv-info">
      {!esActivo && (
        <div className="fv-inactive" role="status">
          <ShieldAlert size={20} />
          <span>Este vehículo se encuentra actualmente <strong>DESACTIVADO</strong>. No se pueden
            generar nuevas Órdenes de Trabajo hasta que un Administrador lo reactive.</span>
        </div>
      )}
      <div className="fv-summary">
        <div className="fv-vehicle-heading">
          <div className="fv-vehicle-identity">
            <span className="fv-vehicle-icon"><Car size={24} /></span>
            <div>
              <span className="fv-eyebrow">Vehículo</span>
              <h3><span className="fv-plate">{vehiculo.placa}</span><span className="fv-vehicle-model">{vehiculo.marca} {vehiculo.modelo}</span></h3>
            </div>
          </div>
          {accionesVehiculo}
        </div>
        <div className="fv-specs">
          <div><Gauge size={17} /><span>Kilometraje actual</span><strong>{(vehiculo.kilometraje_actual ?? 0).toLocaleString()} km</strong></div>
          <div><Wrench size={17} /><span>Motor</span><strong>{vehiculo.tipo_motor || 'No especificado'}</strong></div>
          <div><Calendar size={17} /><span>Año</span><strong>{vehiculo.anio || 'N/A'}</strong></div>
        </div>
        <div className="fv-owner">
          <div className="fv-owner-name">
            <User size={18} />
            <div><span className="fv-eyebrow">Propietario registrado</span>
              <h4>{vehiculo.cliente?.nombre_razon_social || vehiculo.nombre_cliente || 'Sin Cliente Asociado'}</h4>
            </div>
          </div>
          <div className="fv-contact">
            {numDoc && (<span><FileText size={14} />{tipoDoc}: <strong>{numDoc}</strong></span>)}
            {telefono && (<span><Phone size={14} />{telefono}</span>)}
            {correo && (<span><Mail size={14} />{correo}</span>)}
          </div>
          {direccion && (<p className="fv-address"><MapPin size={14} /><span>Dirección: {direccion}</span></p>)}
        </div>
        <div className="fv-record-actions">
          {puedeEditar && esActivo && onAbrirModalCliente && (
            <button onClick={onAbrirModalCliente} className="fv-button fv-small"><UserCheck size={14} />Gestionar cliente</button>
          )}
          {puedeEditar && esActivo && (
            <button onClick={onAbrirEditar} className="fv-button fv-small"><Pencil size={14} />Editar ficha</button>
          )}
          {puedeEliminar && esActivo && (
            <button onClick={onAbrirEliminar} className="fv-button fv-small fv-danger"><Trash2 size={14} />Desactivar</button>
          )}
          {puedeEliminar && !esActivo && (
            <button onClick={onAbrirReactivar} className="fv-button fv-small fv-positive"><RotateCcw size={14} />Reactivar vehículo</button>
          )}
        </div>
      </div>
    </section>
  );
};
