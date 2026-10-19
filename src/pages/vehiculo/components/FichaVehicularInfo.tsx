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
}

export const FichaVehicularInfo: React.FC<Props> = ({
  vehiculo,
  puedeEditar,
  puedeEliminar,
  onAbrirEditar,
  onAbrirEliminar,
  onAbrirReactivar,
  onAbrirModalCliente,
}) => {
  const tipoDoc = vehiculo.cliente?.tipo_documento || vehiculo.tipo_documento_cliente || 'DNI';
  const numDoc = vehiculo.cliente?.numero_documento || vehiculo.documento_cliente;
  const telefono = vehiculo.cliente?.telefono || vehiculo.telefono_cliente;
  const correo = vehiculo.cliente?.correo || vehiculo.correo_cliente;
  const direccion = vehiculo.cliente?.direccion || vehiculo.direccion_cliente;
  const esActivo = vehiculo.activo === 1;

  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: '16px', width: '100%' }}>
      {/* BANNER DE ESTADO INACTIVO */}
      {!esActivo && (
        <div
          style={{
            display: 'flex',
            alignItems: 'center',
            gap: '12px',
            padding: '12px 18px',
            backgroundColor: 'rgba(239, 68, 68, 0.15)',
            border: '1px solid rgba(239, 68, 68, 0.4)',
            borderRadius: '12px',
            color: '#FCA5A5',
          }}
        >
          <ShieldAlert size={20} color="#EF4444" />
          <span style={{ fontSize: '13px', fontWeight: '600' }}>
            Este vehículo se encuentra actualmente <strong>DESACTIVADO</strong>. No se pueden
            generar nuevas Órdenes de Trabajo hasta que un Administrador lo reactive.
          </span>
        </div>
      )}

      {/* TARJETA UNIFICADA DE PROPIETARIO Y ACCIONES */}
      <div
        style={{
          display: 'flex',
          flexDirection: 'column',
          gap: '14px',
          padding: '18px 22px',
          backgroundColor: 'rgba(15, 23, 42, 0.7)',
          border: '1px solid #334155',
          borderRadius: '16px',
        }}
      >
        <div
          style={{
            display: 'flex',
            justifyContent: 'space-between',
            alignItems: 'center',
            flexWrap: 'wrap',
            gap: '12px',
          }}
        >
          <div style={{ display: 'flex', alignItems: 'center', gap: '10px' }}>
            <div
              style={{
                padding: '8px',
                backgroundColor: 'rgba(56, 189, 248, 0.1)',
                borderRadius: '10px',
              }}
            >
              <User size={18} color="#38BDF8" />
            </div>
            <div>
              <span
                style={{
                  fontSize: '11px',
                  color: '#94A3B8',
                  fontWeight: '600',
                  textTransform: 'uppercase',
                }}
              >
                Propietario Registrado
              </span>
              <h4
                style={{
                  fontSize: '15px',
                  fontWeight: '800',
                  color: '#F8FAFC',
                  margin: '2px 0 0 0',
                }}
              >
                {vehiculo.cliente?.nombre_razon_social ||
                  vehiculo.nombre_cliente ||
                  'Sin Cliente Asociado'}
              </h4>
            </div>
          </div>

          {/* DATOS DE CONTACTO EN FILA LIMPIA */}
          <div style={{ display: 'flex', alignItems: 'center', flexWrap: 'wrap', gap: '16px' }}>
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
                  gap: '6px',
                  fontSize: '13px',
                  color: '#34D399',
                }}
              >
                <Phone size={14} />
                <span>{telefono}</span>
              </div>
            )}
            {correo && (
              <div
                style={{
                  display: 'flex',
                  alignItems: 'center',
                  gap: '6px',
                  fontSize: '13px',
                  color: '#38BDF8',
                }}
              >
                <Mail size={14} />
                <span>{correo}</span>
              </div>
            )}
          </div>
        </div>

        {direccion && (
          <div
            style={{
              display: 'flex',
              alignItems: 'center',
              gap: '6px',
              fontSize: '13px',
              color: '#FBBF24',
              paddingTop: '4px',
              borderTop: '1px dashed rgba(51, 65, 85, 0.6)',
            }}
          >
            <MapPin size={14} />
            <span>Dirección: {direccion}</span>
          </div>
        )}

        {/* BOTONES DE GESTIÓN DEL CLIENTE / FICHA */}
        <div
          style={{
            display: 'flex',
            gap: '10px',
            justifyContent: 'flex-end',
            paddingTop: '8px',
            borderTop: '1px solid rgba(51, 65, 85, 0.5)',
          }}
        >
          {puedeEditar && esActivo && onAbrirModalCliente && (
            <button
              onClick={onAbrirModalCliente}
              style={{
                display: 'flex',
                alignItems: 'center',
                gap: '6px',
                padding: '7px 12px',
                backgroundColor: 'rgba(56, 189, 248, 0.1)',
                color: '#38BDF8',
                border: '1px solid rgba(56, 189, 248, 0.3)',
                borderRadius: '8px',
                fontSize: '12px',
                fontWeight: '700',
                cursor: 'pointer',
              }}
            >
              <UserCheck size={14} /> Gestión Cliente
            </button>
          )}

          {puedeEditar && esActivo && (
            <button
              onClick={onAbrirEditar}
              style={{
                display: 'flex',
                alignItems: 'center',
                gap: '6px',
                padding: '7px 12px',
                backgroundColor: 'rgba(251, 191, 36, 0.1)',
                color: '#FBBF24',
                border: '1px solid rgba(251, 191, 36, 0.3)',
                borderRadius: '8px',
                fontSize: '12px',
                fontWeight: '700',
                cursor: 'pointer',
              }}
            >
              <Pencil size={14} /> Editar Ficha
            </button>
          )}

          {puedeEliminar && esActivo && (
            <button
              onClick={onAbrirEliminar}
              style={{
                display: 'flex',
                alignItems: 'center',
                gap: '6px',
                padding: '7px 12px',
                backgroundColor: 'rgba(239, 68, 68, 0.1)',
                color: '#EF4444',
                border: '1px solid rgba(239, 68, 68, 0.3)',
                borderRadius: '8px',
                fontSize: '12px',
                fontWeight: '700',
                cursor: 'pointer',
              }}
            >
              <Trash2 size={14} /> Desactivar
            </button>
          )}

          {puedeEliminar && !esActivo && (
            <button
              onClick={onAbrirReactivar}
              style={{
                display: 'flex',
                alignItems: 'center',
                gap: '6px',
                padding: '7px 12px',
                backgroundColor: 'rgba(52, 211, 153, 0.1)',
                color: '#34D399',
                border: '1px solid rgba(52, 211, 153, 0.3)',
                borderRadius: '8px',
                fontSize: '12px',
                fontWeight: '700',
                cursor: 'pointer',
              }}
            >
              <RotateCcw size={14} /> Reactivar Vehículo
            </button>
          )}
        </div>
      </div>

      {/* TARJETAS TÉCNICAS DEL VEHÍCULO */}
      <div
        style={{
          display: 'grid',
          gridTemplateColumns: 'repeat(auto-fit, minmax(220px, 1fr))',
          gap: '16px',
          padding: '20px 24px',
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
            <Car size={22} color="#38BDF8" />
          </div>
          <div>
            <span style={{ fontSize: '12px', color: '#94A3B8', fontWeight: '600' }}>Vehículo</span>
            <h4
              style={{ fontSize: '16px', fontWeight: '800', color: '#F8FAFC', margin: '2px 0 0 0' }}
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
            <Gauge size={22} color="#FBBF24" />
          </div>
          <div>
            <span style={{ fontSize: '12px', color: '#94A3B8', fontWeight: '600' }}>
              Kilometraje Actual
            </span>
            <h4
              style={{ fontSize: '16px', fontWeight: '800', color: '#F8FAFC', margin: '2px 0 0 0' }}
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
            <Wrench size={22} color="#34D399" />
          </div>
          <div>
            <span style={{ fontSize: '12px', color: '#94A3B8', fontWeight: '600' }}>Motor</span>
            <h4
              style={{ fontSize: '16px', fontWeight: '800', color: '#F8FAFC', margin: '2px 0 0 0' }}
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
            <Calendar size={22} color="#A855F7" />
          </div>
          <div>
            <span style={{ fontSize: '12px', color: '#94A3B8', fontWeight: '600' }}>Año</span>
            <h4
              style={{ fontSize: '16px', fontWeight: '800', color: '#F8FAFC', margin: '2px 0 0 0' }}
            >
              {vehiculo.anio || 'N/A'}
            </h4>
          </div>
        </div>
      </div>
    </div>
  );
};
