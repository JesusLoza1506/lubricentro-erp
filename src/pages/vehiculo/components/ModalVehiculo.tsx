import React, { useState, useEffect } from 'react';
import {
  CrearVehiculoDTO,
  EditarVehiculoDTO,
  VehiculoCompletoDTO,
  TipoMotor,
} from '../../../types/vehiculo';

export type ModoModalVehiculo = 'CREAR' | 'EDITAR';

interface ModalVehiculoProps {
  modalAbierto: boolean;
  setModalAbierto: (abierto: boolean) => void;
  modo: ModoModalVehiculo;
  vehiculoEditar?: VehiculoCompletoDTO | null;
  handleGuardar: (data: CrearVehiculoDTO | EditarVehiculoDTO) => Promise<void>;
}

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
  const [tipoMotor, setTipoMotor] = useState<string>('');
  const [kilometraje, setKilometraje] = useState<number | ''>('');
  const [nombreCliente, setNombreCliente] = useState('');
  const [tipoDoc, setTipoDoc] = useState('DNI');
  const [numDoc, setNumDoc] = useState('');
  const [telefono, setTelefono] = useState('');
  const [correo, setCorreo] = useState('');
  const [direccion, setDireccion] = useState('');
  const [motivo, setMotivo] = useState('');
  const [guardando, setGuardando] = useState(false);
  const [errorModal, setErrorModal] = useState<string | null>(null);

  const anioMaximo = new Date().getFullYear() + 1;

  useEffect(() => {
    setErrorModal(null);
    setMotivo('');
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
      setCorreo(vehiculoEditar.cliente?.correo || vehiculoEditar.correo_cliente || '');
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
      setCorreo('');
      setDireccion('');
    }
  }, [modo, vehiculoEditar, modalAbierto]);

  if (!modalAbierto) return null;

  const handlePlacaChange = (val: string) => {
    const clean = val
      .toUpperCase()
      .replace(/[^A-Z0-9]/g, '')
      .slice(0, 6);
    setPlaca(clean);
  };

  const handleNumDocChange = (val: string) => {
    if (tipoDoc === 'DNI') {
      setNumDoc(val.replace(/\D/g, '').slice(0, 8));
    } else if (tipoDoc === 'RUC') {
      setNumDoc(val.replace(/\D/g, '').slice(0, 11));
    } else {
      setNumDoc(val.replace(/[^a-zA-Z0-9]/g, '').slice(0, 12));
    }
  };

  const kmActualVal = Number(kilometraje) || 0;
  const kmAnteriorVal = vehiculoEditar?.kilometraje_actual || 0;
  const disminuyeKm = modo === 'EDITAR' && kmActualVal < kmAnteriorVal;

  const onSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setGuardando(true);
    setErrorModal(null);

    try {
      if (disminuyeKm && (!motivo.trim() || motivo.trim().length < 5)) {
        throw new Error(
          'Al disminuir el kilometraje, debe ingresar un motivo obligatorio (mínimo 5 caracteres).'
        );
      }

      const motorFinal = (tipoMotor.trim() as TipoMotor) || null;

      if (modo === 'CREAR') {
        const payload: CrearVehiculoDTO = {
          placa: placa.trim().toUpperCase(),
          marca: marca.trim(),
          modelo: modelo.trim(),
          anio: anio ? Number(anio) : null,
          tipo_motor: motorFinal,
          kilometraje_actual: kmActualVal,
          nombre_razon_social: nombreCliente.trim(),
          tipo_documento: tipoDoc,
          numero_documento: numDoc.trim(),
          telefono: telefono.trim() || undefined,
          correo: correo.trim() || undefined,
          direccion: direccion.trim() || undefined,
        };
        await handleGuardar(payload);
      } else {
        const payload: EditarVehiculoDTO = {
          id_vehiculo: vehiculoEditar?.id_vehiculo,
          placa: placa.trim().toUpperCase(),
          marca: marca.trim(),
          modelo: modelo.trim(),
          anio: anio ? Number(anio) : null,
          tipo_motor: motorFinal,
          kilometraje_actual: kmActualVal,
          id_cliente: vehiculoEditar?.id_cliente,
          nombre_razon_social: nombreCliente.trim(),
          tipo_documento: tipoDoc,
          numero_documento: numDoc.trim(),
          telefono: telefono.trim() || undefined,
          correo: correo.trim() || undefined,
          direccion: direccion.trim() || undefined,
          motivo: motivo.trim() || undefined,
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
    <div style={overlayStyle}>
      <div style={modalBoxStyle}>
        <h3 style={{ fontSize: '20px', fontWeight: '800', color: '#F8FAFC', margin: '0 0 20px 0' }}>
          {modo === 'CREAR' ? 'Registrar Nuevo Vehículo / Cliente' : 'Editar Ficha Vehicular'}
        </h3>

        {errorModal && <div style={errorBoxStyle}>⚠️ {errorModal}</div>}

        <form onSubmit={onSubmit} style={{ display: 'flex', flexDirection: 'column', gap: '14px' }}>
          {/* DATOS DEL CLIENTE */}
          <div style={sectionBoxStyle}>
            <span style={sectionTitleStyle}>Datos del Propietario / Cliente</span>
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
                    <option value="DNI">DNI</option>
                    <option value="RUC">RUC</option>
                    <option value="CE">CE</option>
                  </select>
                </div>
                <div>
                  <label style={labelStyle}>N° Documento *</label>
                  <input
                    required
                    type="text"
                    placeholder={
                      tipoDoc === 'DNI' ? '8 dígitos' : tipoDoc === 'RUC' ? '11 dígitos' : 'CE'
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
                    onChange={(e) => setTelefono(e.target.value)}
                    style={inputStyle}
                  />
                </div>
              </div>
              <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '10px' }}>
                <div>
                  <label style={labelStyle}>Correo Electrónico</label>
                  <input
                    type="email"
                    placeholder="cliente@correo.com"
                    value={correo}
                    onChange={(e) => setCorreo(e.target.value)}
                    style={inputStyle}
                  />
                </div>
                <div>
                  <label style={labelStyle}>Dirección</label>
                  <input
                    type="text"
                    placeholder="Av. Principal 123"
                    value={direccion}
                    onChange={(e) => setDireccion(e.target.value)}
                    style={inputStyle}
                  />
                </div>
              </div>
            </div>
          </div>

          {/* DATOS DEL VEHÍCULO */}
          <div style={sectionBoxStyle}>
            <span style={{ ...sectionTitleStyle, color: '#C084FC' }}>Datos del Vehículo</span>
            <div style={{ display: 'flex', flexDirection: 'column', gap: '10px' }}>
              <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr 1fr', gap: '10px' }}>
                <div>
                  <label style={labelStyle}>Placa * (6 Caracteres)</label>
                  <input
                    required
                    disabled={modo === 'EDITAR'}
                    type="text"
                    placeholder="ABC123"
                    value={placa}
                    onChange={(e) => handlePlacaChange(e.target.value)}
                    style={{
                      ...inputStyle,
                      fontFamily: 'monospace',
                      fontWeight: '700',
                      opacity: modo === 'EDITAR' ? 0.6 : 1,
                    }}
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
                    min="1900"
                    max={anioMaximo}
                    placeholder="2020"
                    value={anio}
                    onChange={(e) => setAnio(e.target.value === '' ? '' : Number(e.target.value))}
                    style={inputStyle}
                  />
                </div>
                <div>
                  <label style={labelStyle}>Tipo de Motor</label>
                  <select
                    value={tipoMotor}
                    onChange={(e) => setTipoMotor(e.target.value)}
                    style={{ ...inputStyle, backgroundColor: '#1E293B', cursor: 'pointer' }}
                  >
                    <option value="">Sin especificar</option>
                    <option value="GASOLINA">GASOLINA</option>
                    <option value="DIESEL">DIESEL</option>
                    <option value="GLP">GLP</option>
                    <option value="GNV">GNV</option>
                    <option value="HIBRIDO">HÍBRIDO</option>
                    <option value="ELECTRICO">ELÉCTRICO</option>
                    <option value="OTRO">OTRO</option>
                  </select>
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

              {disminuyeKm && (
                <div style={{ marginTop: '4px' }}>
                  <label style={{ ...labelStyle, color: '#EF4444', fontWeight: '700' }}>
                    ⚠️ Motivo de Disminución de Kilometraje (Obligatorio, 5-250 chars) *
                  </label>
                  <input
                    required
                    type="text"
                    placeholder="Ej. Corrección por error de digitación en tablero"
                    value={motivo}
                    onChange={(e) => setMotivo(e.target.value)}
                    style={{ ...inputStyle, borderColor: '#EF4444' }}
                  />
                </div>
              )}
            </div>
          </div>

          <div
            style={{ display: 'flex', justifyContent: 'flex-end', gap: '12px', marginTop: '10px' }}
          >
            <button type="button" onClick={() => setModalAbierto(false)} style={btnSecondaryStyle}>
              Cancelar
            </button>
            <button type="submit" disabled={guardando} style={btnPrimaryStyle}>
              {guardando ? 'Guardando...' : 'Guardar Ficha'}
            </button>
          </div>
        </form>
      </div>
    </div>
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
  zIndex: 1000,
};

const modalBoxStyle: React.CSSProperties = {
  width: '580px',
  backgroundColor: '#0F172A',
  border: '1px solid #334155',
  borderRadius: '16px',
  padding: '28px',
  boxShadow: '0 20px 40px rgba(0,0,0,0.5)',
  maxHeight: '90vh',
  overflowY: 'auto',
};

const sectionBoxStyle: React.CSSProperties = {
  padding: '12px',
  backgroundColor: 'rgba(30, 41, 59, 0.5)',
  borderRadius: '10px',
  border: '1px solid #334155',
};

const sectionTitleStyle: React.CSSProperties = {
  fontSize: '11px',
  fontWeight: '700',
  color: '#38BDF8',
  letterSpacing: '0.5px',
  textTransform: 'uppercase',
  display: 'block',
  marginBottom: '10px',
};

const errorBoxStyle: React.CSSProperties = {
  padding: '12px 16px',
  backgroundColor: 'rgba(239, 68, 68, 0.15)',
  border: '1px solid rgba(239, 68, 68, 0.4)',
  borderRadius: '8px',
  color: '#FCA5A5',
  fontSize: '13px',
  marginBottom: '14px',
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
  padding: '10px 20px',
  backgroundColor: '#2563EB',
  color: '#FFFFFF',
  border: 'none',
  borderRadius: '8px',
  fontWeight: '700',
  cursor: 'pointer',
};

const btnSecondaryStyle: React.CSSProperties = {
  padding: '10px 16px',
  backgroundColor: 'transparent',
  border: '1px solid #334155',
  color: '#CBD5E1',
  borderRadius: '8px',
  cursor: 'pointer',
};
