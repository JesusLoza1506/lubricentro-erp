import React from 'react';
import { VehiculoCompletoDTO } from '../../../types/vehiculo';
import {
  User,
  Phone,
  Pencil,
  Trash2,
  Car,
  Gauge,
  Wrench,
  Calendar,
  MapPin,
  FileText,
} from 'lucide-react';

interface Props {
  vehiculo: VehiculoCompletoDTO;
  puedeEditar: boolean;
  puedeEliminar: boolean;
  onAbrirEditar: () => void;
  onAbrirEliminar: () => void;
}

export const FichaVehicularInfo: React.FC<Props> = ({
  vehiculo,
  puedeEditar,
  puedeEliminar,
  onAbrirEditar,
  onAbrirEliminar,
}) => {
  const tipoDoc = vehiculo.cliente?.tipo_documento || vehiculo.tipo_documento_cliente || 'DNI';
  const numDoc = vehiculo.cliente?.numero_documento || vehiculo.documento_cliente;
  const telefono = vehiculo.cliente?.telefono || vehiculo.telefono_cliente;
  const direccion = vehiculo.cliente?.direccion || vehiculo.direccion_cliente;

  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: '16px' }}>
      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
        {/* TARJETA COMPLETA DE INFORMACIÓN DEL PROPIETARIO */}
        <div
          style={{
            display: 'flex',
            alignItems: 'center',
            flexWrap: 'wrap',
            gap: '16px',
            padding: '10px 18px',
            backgroundColor: 'rgba(15, 23, 42, 0.6)',
            border: '1px solid #334155',
            borderRadius: '12px',
          }}
        >
          <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
            <User size={18} color="#38BDF8" />
            <span style={{ fontSize: '13px', color: '#94A3B8' }}>Propietario:</span>
            <strong style={{ fontSize: '14px', color: '#F8FAFC' }}>
              {vehiculo.cliente?.nombre_razon_social ||
                vehiculo.nombre_cliente ||
                'Sin Cliente Asociado'}
            </strong>
          </div>

          {numDoc && (
            <div
              style={{
                display: 'flex',
                alignItems: 'center',
                gap: '6px',
                fontSize: '13px',
                color: '#CBD5E1',
              }}
            >
              <FileText size={14} color="#C084FC" />
              <span>
                {tipoDoc}: <strong>{numDoc}</strong>
              </span>
            </div>
          )}

          {telefono && (
            <div
              style={{
                display: 'flex',
                alignItems: 'center',
                gap: '4px',
                fontSize: '13px',
                color: '#34D399',
              }}
            >
              <Phone size={14} />
              <span>{telefono}</span>
            </div>
          )}

          {direccion && (
            <div
              style={{
                display: 'flex',
                alignItems: 'center',
                gap: '4px',
                fontSize: '13px',
                color: '#FBBF24',
              }}
            >
              <MapPin size={14} />
              <span>{direccion}</span>
            </div>
          )}
        </div>

        <div style={{ display: 'flex', gap: '10px' }}>
          {puedeEditar && (
            <button
              onClick={onAbrirEditar}
              style={{
                display: 'flex',
                alignItems: 'center',
                gap: '6px',
                padding: '8px 14px',
                backgroundColor: 'rgba(251, 191, 36, 0.15)',
                color: '#FBBF24',
                border: '1px solid rgba(251, 191, 36, 0.4)',
                borderRadius: '10px',
                fontSize: '13px',
                fontWeight: '700',
                cursor: 'pointer',
              }}
            >
              <Pencil size={15} /> Editar Ficha
            </button>
          )}

          {puedeEliminar && (
            <button
              onClick={onAbrirEliminar}
              style={{
                display: 'flex',
                alignItems: 'center',
                gap: '6px',
                padding: '8px 14px',
                backgroundColor: 'rgba(239, 68, 68, 0.15)',
                color: '#EF4444',
                border: '1px solid rgba(239, 68, 68, 0.4)',
                borderRadius: '10px',
                fontSize: '13px',
                fontWeight: '700',
                cursor: 'pointer',
              }}
            >
              <Trash2 size={15} /> Eliminar
            </button>
          )}
        </div>
      </div>

      {/* TARJETAS TÉCNICAS */}
      <div
        style={{
          display: 'grid',
          gridTemplateColumns: 'repeat(4, 1fr)',
          gap: '16px',
          padding: '24px',
          backgroundColor: 'rgba(15, 23, 42, 0.6)',
          border: '1px solid #334155',
          borderRadius: '16px',
        }}
      >
        <div style={{ display: 'flex', alignItems: 'center', gap: '14px' }}>
          <div
            style={{
              padding: '12px',
              backgroundColor: 'rgba(56, 189, 248, 0.1)',
              borderRadius: '12px',
            }}
          >
            <Car size={24} color="#38BDF8" />
          </div>
          <div>
            <span style={{ fontSize: '12px', color: '#94A3B8', fontWeight: '600' }}>Vehículo</span>
            <h4
              style={{ fontSize: '18px', fontWeight: '800', color: '#F8FAFC', margin: '2px 0 0 0' }}
            >
              {vehiculo.marca} {vehiculo.modelo}
            </h4>
            <span style={{ fontSize: '12px', color: '#C084FC', fontWeight: '700' }}>
              Placa: {vehiculo.placa}
            </span>
          </div>
        </div>

        <div style={{ display: 'flex', alignItems: 'center', gap: '14px' }}>
          <div
            style={{
              padding: '12px',
              backgroundColor: 'rgba(251, 191, 36, 0.1)',
              borderRadius: '12px',
            }}
          >
            <Gauge size={24} color="#FBBF24" />
          </div>
          <div>
            <span style={{ fontSize: '12px', color: '#94A3B8', fontWeight: '600' }}>
              Kilometraje Actual
            </span>
            <h4
              style={{ fontSize: '18px', fontWeight: '800', color: '#F8FAFC', margin: '2px 0 0 0' }}
            >
              {(vehiculo.kilometraje_actual ?? 0).toLocaleString()} km
            </h4>
          </div>
        </div>

        <div style={{ display: 'flex', alignItems: 'center', gap: '14px' }}>
          <div
            style={{
              padding: '12px',
              backgroundColor: 'rgba(52, 211, 153, 0.1)',
              borderRadius: '12px',
            }}
          >
            <Wrench size={24} color="#34D399" />
          </div>
          <div>
            <span style={{ fontSize: '12px', color: '#94A3B8', fontWeight: '600' }}>Motor</span>
            <h4
              style={{ fontSize: '18px', fontWeight: '800', color: '#F8FAFC', margin: '2px 0 0 0' }}
            >
              {vehiculo.tipo_motor || 'No especificado'}
            </h4>
          </div>
        </div>

        <div style={{ display: 'flex', alignItems: 'center', gap: '14px' }}>
          <div
            style={{
              padding: '12px',
              backgroundColor: 'rgba(168, 85, 247, 0.1)',
              borderRadius: '12px',
            }}
          >
            <Calendar size={24} color="#A855F7" />
          </div>
          <div>
            <span style={{ fontSize: '12px', color: '#94A3B8', fontWeight: '600' }}>Año</span>
            <h4
              style={{ fontSize: '18px', fontWeight: '800', color: '#F8FAFC', margin: '2px 0 0 0' }}
            >
              {vehiculo.anio || 'N/A'}
            </h4>
          </div>
        </div>
      </div>
    </div>
  );
};
