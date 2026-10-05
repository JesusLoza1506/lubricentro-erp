import React, { useState, useEffect } from 'react';
import { CrearVehiculoDTO, EditarVehiculoDTO, VehiculoCompletoDTO } from '../../../types/vehiculo';

export type ModoModalVehiculo = 'CREAR' | 'EDITAR';

interface ModalVehiculoProps {
  modalAbierto: boolean;
  setModalAbierto: (abierto: boolean) => void;
  modo: ModoModalVehiculo;
  vehiculoEditar?: VehiculoCompletoDTO | null;
  handleGuardar: (data: CrearVehiculoDTO | EditarVehiculoDTO) => Promise<void>;
}

// Helper para extraer de forma segura el mensaje de error sin usar 'any'
const extraerMensajeError = (err: unknown): string => {
  if (typeof err === 'string') return err;
  if (err && typeof err === 'object' && 'message' in err) {
    return String((err as { message: unknown }).message);
  }
  return 'Ocurrió un error inesperado al procesar la solicitud.';
};

export const ModalVehiculo: React.FC<ModalVehiculoProps> = ({
  modalAbierto,
  setModalAbierto,
  modo,
  vehiculoEditar,
  handleGuardar,
}) => {
  const [placa, setPlaca] = useState('');
  const [marca, setMarca] = useState('');
  const [modelo, setModelo] = useState('');
  const [anio, setAnio] = useState<number | ''>('');
  const [tipoMotor, setTipoMotor] = useState('');
  const [kilometraje, setKilometraje] = useState<number | ''>('');
  const [nombreCliente, setNombreCliente] = useState('');
  const [tipoDoc, setTipoDoc] = useState('DNI');
  const [numDoc, setNumDoc] = useState('');
  const [telefono, setTelefono] = useState('');
  const [direccion, setDireccion] = useState('');
  const [guardando, setGuardando] = useState(false);
  const [errorModal, setErrorModal] = useState<string | null>(null);

  // Cálculo dinámico del año máximo permitido (Año actual + 1)
  const anioMaximo = new Date().getFullYear() + 1;

  useEffect(() => {
    setErrorModal(null);
    if (modo === 'EDITAR' && vehiculoEditar) {
      setPlaca(vehiculoEditar.placa || '');
      setMarca(vehiculoEditar.marca || '');
      setModelo(vehiculoEditar.modelo || '');
      setAnio(vehiculoEditar.anio || '');
      setTipoMotor(vehiculoEditar.tipo_motor || '');
      setKilometraje(vehiculoEditar.kilometraje_actual || '');
      setNombreCliente(
        vehiculoEditar.cliente?.nombre_razon_social || vehiculoEditar.nombre_cliente || ''
      );
      setTipoDoc(
        vehiculoEditar.cliente?.tipo_documento || vehiculoEditar.tipo_documento_cliente || 'DNI'
      );
      setNumDoc(vehiculoEditar.cliente?.numero_documento || vehiculoEditar.documento_cliente || '');
      setTelefono(vehiculoEditar.cliente?.telefono || vehiculoEditar.telefono_cliente || '');
      setDireccion(vehiculoEditar.cliente?.direccion || vehiculoEditar.direccion_cliente || '');
    } else {
      setPlaca('');
      setMarca('');
      setModelo('');
      setAnio('');
      setTipoMotor('');
      setKilometraje('');
      setNombreCliente('');
      setTipoDoc('DNI');
      setNumDoc('');
      setTelefono('');
      setDireccion('');
    }
  }, [modo, vehiculoEditar, modalAbierto]);

  if (!modalAbierto) return null;

  // Manejo de cambio en campo Placa (Auto-formato peruano ABC-123)
  const handlePlacaChange = (val: string) => {
    let clean = val.toUpperCase().replace(/[^A-Z0-9]/g, '');
    if (clean.length > 6) clean = clean.slice(0, 6);

    if (clean.length > 3 && /^[A-Z]{3}[0-9]/.test(clean)) {
      setPlaca(`${clean.slice(0, 3)}-${clean.slice(3)}`);
    } else {
      setPlaca(clean);
    }
  };

  // Manejo de cambio en N° Documento según tipo
  const handleNumDocChange = (val: string) => {
    if (tipoDoc === 'DNI') {
      const clean = val.replace(/\D/g, '').slice(0, 8);
      setNumDoc(clean);
    } else if (tipoDoc === 'RUC') {
      const clean = val.replace(/\D/g, '').slice(0, 11);
      setNumDoc(clean);
    } else {
      const clean = val.replace(/[^a-zA-Z0-9]/g, '').slice(0, 12);
      setNumDoc(clean);
    }
  };

  const onSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setGuardando(true);
    setErrorModal(null);

    try {
      if (modo === 'CREAR') {
        const payload: CrearVehiculoDTO = {
          placa: placa.trim().toUpperCase(),
          marca: marca.trim(),
          modelo: modelo.trim(),
          anio: anio ? Number(anio) : null,
          tipo_motor: tipoMotor.trim() || null,
          kilometraje_actual: Number(kilometraje) || 0,
          nombre_razon_social: nombreCliente.trim(),
          tipo_documento: tipoDoc,
          numero_documento: numDoc.trim(),
          telefono: telefono.trim(),
          direccion: direccion.trim(),
        };
        await handleGuardar(payload);
      } else {
        const payload: EditarVehiculoDTO = {
          id_vehiculo: vehiculoEditar?.id_vehiculo,
          placa: placa.trim().toUpperCase(),
          marca: marca.trim(),
          modelo: modelo.trim(),
          anio: anio ? Number(anio) : null,
          tipo_motor: tipoMotor.trim() || null,
          kilometraje_actual: Number(kilometraje) || 0,
          id_cliente: vehiculoEditar?.id_cliente || vehiculoEditar?.cliente?.id_cliente,
          nombre_razon_social: nombreCliente.trim(),
          tipo_documento: tipoDoc,
          numero_documento: numDoc.trim(),
          telefono: telefono.trim(),
          direccion: direccion.trim(),
        };
        await handleGuardar(payload);
      }
      setModalAbierto(false);
    } catch (err: unknown) {
      const mensaje = extraerMensajeError(err);
      setErrorModal(mensaje);
    } finally {
      setGuardando(false);
    }
  };

  return (
    <div
      style={{
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
        zIndex: 1000,
      }}
    >
      <div
        style={{
          width: '560px',
          backgroundColor: '#0F172A',
          border: '1px solid #334155',
          borderRadius: '16px',
          padding: '28px',
          boxShadow: '0 20px 40px rgba(0,0,0,0.5)',
        }}
      >
        <h3
          style={{
            fontSize: '20px',
            fontWeight: '800',
            color: '#F8FAFC',
            marginTop: 0,
            marginBottom: '20px',
          }}
        >
          {modo === 'CREAR' ? 'Registrar Nuevo Vehículo / Cliente' : 'Editar Ficha Vehicular'}
        </h3>

        {errorModal && (
          <div
            style={{
              padding: '12px 16px',
              backgroundColor: 'rgba(239, 68, 68, 0.15)',
              border: '1px solid rgba(239, 68, 68, 0.4)',
              borderRadius: '8px',
              color: '#FCA5A5',
              fontSize: '13px',
              marginBottom: '14px',
            }}
          >
            ⚠️ {errorModal}
          </div>
        )}

        <form onSubmit={onSubmit} style={{ display: 'flex', flexDirection: 'column', gap: '14px' }}>
          {/* DATOS DEL CLIENTE */}
          <div
            style={{
              padding: '12px',
              backgroundColor: 'rgba(30, 41, 59, 0.5)',
              borderRadius: '10px',
              border: '1px solid #334155',
            }}
          >
            <span
              style={{
                fontSize: '11px',
                fontWeight: '700',
                color: '#38BDF8',
                letterSpacing: '0.5px',
                textTransform: 'uppercase',
                display: 'block',
                marginBottom: '10px',
              }}
            >
              Datos del Propietario / Cliente
            </span>
            <div style={{ display: 'flex', flexDirection: 'column', gap: '10px' }}>
              <div>
                <label style={labelStyle}>Cliente / Razón Social *</label>
                <input
                  required
                  type="text"
                  placeholder="Ej. Juan Pérez / Lubricentro S.A.C."
                  value={nombreCliente}
                  onChange={(e) => setNombreCliente(e.target.value)}
                  style={inputStyle}
                />
              </div>
              <div style={{ display: 'grid', gridTemplateColumns: '1fr 1.5fr 1fr', gap: '10px' }}>
                <div>
                  <label style={labelStyle}>Tipo Doc. *</label>
                  <select
                    value={tipoDoc}
                    onChange={(e) => {
                      setTipoDoc(e.target.value);
                      setNumDoc('');
                    }}
                    style={{ ...inputStyle, backgroundColor: '#1E293B', cursor: 'pointer' }}
                  >
                    <option value="DNI" style={optionStyle}>
                      DNI
                    </option>
                    <option value="RUC" style={optionStyle}>
                      RUC
                    </option>
                    <option value="CE" style={optionStyle}>
                      CE
                    </option>
                    <option value="PASAPORTE" style={optionStyle}>
                      PASAPORTE
                    </option>
                  </select>
                </div>
                <div>
                  <label style={labelStyle}>N° Documento *</label>
                  <input
                    required
                    type="text"
                    placeholder={
                      tipoDoc === 'DNI'
                        ? '8 dígitos'
                        : tipoDoc === 'RUC'
                          ? '11 dígitos'
                          : 'N° Documento'
                    }
                    value={numDoc}
                    onChange={(e) => handleNumDocChange(e.target.value)}
                    style={inputStyle}
                  />
                </div>
                <div>
                  <label style={labelStyle}>Teléfono</label>
                  <input
                    type="text"
                    placeholder="987654321"
                    value={telefono}
                    onChange={(e) => setTelefono(e.target.value.replace(/\D/g, '').slice(0, 9))}
                    style={inputStyle}
                  />
                </div>
              </div>
              <div>
                <label style={labelStyle}>Dirección</label>
                <input
                  type="text"
                  placeholder="Ej. Av. Larco 123, Miraflores"
                  value={direccion}
                  onChange={(e) => setDireccion(e.target.value)}
                  style={inputStyle}
                />
              </div>
            </div>
          </div>

          {/* DATOS DEL VEHÍCULO */}
          <div
            style={{
              padding: '12px',
              backgroundColor: 'rgba(30, 41, 59, 0.5)',
              borderRadius: '10px',
              border: '1px solid #334155',
            }}
          >
            <span
              style={{
                fontSize: '11px',
                fontWeight: '700',
                color: '#C084FC',
                letterSpacing: '0.5px',
                textTransform: 'uppercase',
                display: 'block',
                marginBottom: '10px',
              }}
            >
              Datos del Vehículo
            </span>
            <div style={{ display: 'flex', flexDirection: 'column', gap: '10px' }}>
              <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr 1fr', gap: '10px' }}>
                <div>
                  <label style={labelStyle}>Placa *</label>
                  <input
                    required
                    type="text"
                    placeholder="ABC-123"
                    value={placa}
                    onChange={(e) => handlePlacaChange(e.target.value)}
                    style={{ ...inputStyle, fontFamily: 'monospace', fontWeight: '700' }}
                  />
                </div>
                <div>
                  <label style={labelStyle}>Marca *</label>
                  <input
                    required
                    type="text"
                    placeholder="Toyota"
                    value={marca}
                    onChange={(e) => setMarca(e.target.value)}
                    style={inputStyle}
                  />
                </div>
                <div>
                  <label style={labelStyle}>Modelo *</label>
                  <input
                    required
                    type="text"
                    placeholder="Yaris"
                    value={modelo}
                    onChange={(e) => setModelo(e.target.value)}
                    style={inputStyle}
                  />
                </div>
              </div>

              <div style={{ display: 'grid', gridTemplateColumns: '1fr 1.2fr 1fr', gap: '10px' }}>
                <div>
                  <label style={labelStyle}>Año</label>
                  <input
                    type="number"
                    min="1980"
                    max={anioMaximo}
                    placeholder="2020"
                    value={anio}
                    onChange={(e) => setAnio(e.target.value === '' ? '' : Number(e.target.value))}
                    style={inputStyle}
                  />
                </div>
                <div>
                  <label style={labelStyle}>Motor</label>
                  <input
                    type="text"
                    placeholder="1.5L 1NZ-FE"
                    value={tipoMotor}
                    onChange={(e) => setTipoMotor(e.target.value)}
                    style={inputStyle}
                  />
                </div>
                <div>
                  <label style={labelStyle}>KM Actual *</label>
                  <input
                    required
                    type="number"
                    min="0"
                    placeholder="68500"
                    value={kilometraje}
                    onChange={(e) =>
                      setKilometraje(e.target.value === '' ? '' : Number(e.target.value))
                    }
                    style={inputStyle}
                  />
                </div>
              </div>
            </div>
          </div>

          <div
            style={{
              display: 'flex',
              justifyContent: 'flex-end',
              gap: '12px',
              marginTop: '10px',
            }}
          >
            <button
              type="button"
              onClick={() => setModalAbierto(false)}
              style={{
                padding: '10px 16px',
                backgroundColor: 'transparent',
                border: '1px solid #334155',
                color: '#CBD5E1',
                borderRadius: '8px',
                cursor: 'pointer',
              }}
            >
              Cancelar
            </button>
            <button
              type="submit"
              disabled={guardando}
              style={{
                padding: '10px 20px',
                backgroundColor: '#2563EB',
                color: '#FFFFFF',
                border: 'none',
                borderRadius: '8px',
                fontWeight: '700',
                cursor: 'pointer',
              }}
            >
              {guardando ? 'Guardando...' : 'Guardar'}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
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

const optionStyle: React.CSSProperties = {
  backgroundColor: '#0F172A',
  color: '#F8FAFC',
};
