import React, { useState, useEffect } from 'react';
import {
  CrearVehiculoDTO,
  EditarVehiculoDTO,
  VehiculoCompletoDTO,
  TipoMotor,
} from '../../../types/vehiculo';
import { ClienteDTO } from '../../../types/cliente';
import { clienteService } from '../../../services/clienteService';

export type ModoModalVehiculo = 'CREAR' | 'EDITAR';

interface ModalVehiculoProps {
  modalAbierto: boolean;
  setModalAbierto: (abierto: boolean) => void;
  modo: ModoModalVehiculo;
  vehiculoEditar?: VehiculoCompletoDTO | null;
  handleGuardar: (data: CrearVehiculoDTO | EditarVehiculoDTO) => Promise<void>;
  esAdmin?: boolean;
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
  esAdmin = false,
}) => {
  const [placa, setPlaca] = useState('');
  const [marca, setMarca] = useState('');
  const [modelo, setModelo] = useState('');
  const [anio, setAnio] = useState<number | ''>('');
  const [tipoMotor, setTipoMotor] = useState<string>('');
  const [kilometraje, setKilometraje] = useState<number | ''>('');

  // Datos del Cliente
  const [idClienteSeleccionado, setIdClienteSeleccionado] = useState<number | null>(null);
  const [nombreCliente, setNombreCliente] = useState('');
  const [tipoDoc, setTipoDoc] = useState('DNI');
  const [numDoc, setNumDoc] = useState('');
  const [telefono, setTelefono] = useState('');
  const [correo, setCorreo] = useState('');
  const [direccion, setDireccion] = useState('');
  const [motivo, setMotivo] = useState('');

  // Estados del Minibuscador Predictivo
  const [sugerenciasClientes, setSugerenciasClientes] = useState<ClienteDTO[]>([]);
  const [mostrarSugerencias, setMostrarSugerencias] = useState(false);

  const [guardando, setGuardando] = useState(false);
  const [errorModal, setErrorModal] = useState<string | null>(null);

  // Estado para el modal de confirmación
  const [confirmModalOpen, setConfirmModalOpen] = useState(false);

  const anioMaximo = new Date().getFullYear() + 1;

  useEffect(() => {
    setErrorModal(null);
    setMotivo('');
    setConfirmModalOpen(false);
    setSugerenciasClientes([]);
    setMostrarSugerencias(false);

    if (modo === 'EDITAR' && vehiculoEditar) {
      setPlaca(vehiculoEditar.placa || '');
      setMarca(vehiculoEditar.marca || '');
      setModelo(vehiculoEditar.modelo || '');
      setAnio(vehiculoEditar.anio || '');
      setTipoMotor(vehiculoEditar.tipo_motor || '');
      setKilometraje(vehiculoEditar.kilometraje_actual || '');

      setIdClienteSeleccionado(vehiculoEditar.id_cliente || null);
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

      setIdClienteSeleccionado(null);
      setNombreCliente('');
      setTipoDoc('DNI');
      setNumDoc('');
      setTelefono('');
      setCorreo('');
      setDireccion('');
    }
  }, [modo, vehiculoEditar, modalAbierto]);

  if (!modalAbierto) return null;

  // Manejador de entrada para el Minibuscador Predictivo de Clientes
  const handleNombreClienteChange = async (val: string) => {
    setNombreCliente(val);
    setIdClienteSeleccionado(null); // Al editar el texto libre, liberamos el ID previo

    if (val.trim().length >= 2) {
      try {
        const resultados = await clienteService.buscarClientesPorTermino(val);
        setSugerenciasClientes(resultados);
        setMostrarSugerencias(true);
      } catch (err) {
        console.error('Error al buscar clientes:', err);
      }
    } else {
      setSugerenciasClientes([]);
      setMostrarSugerencias(false);
    }
  };

  // Al seleccionar un cliente sugerido de la lista flotante
  const seleccionarClienteExistente = (cliente: ClienteDTO) => {
    setIdClienteSeleccionado(cliente.id_cliente);
    setNombreCliente(cliente.nombre_razon_social);
    setTipoDoc(cliente.tipo_documento);
    setNumDoc(cliente.numero_documento);
    setTelefono(cliente.telefono || '');
    setCorreo(cliente.correo || '');
    setDireccion(cliente.direccion || '');
    setMostrarSugerencias(false);
  };

  const handlePlacaChange = (val: string) => {
    const clean = val
      .toUpperCase()
      .replace(/[^A-Z0-9]/g, '')
      .slice(0, 6);
    setPlaca(clean);
  };

  const handleTelefonoChange = (val: string) => {
    let clean = val.trim().replace(/[\s-]/g, '');
    if (clean.startsWith('+51')) {
      clean = clean.slice(3);
    } else if (clean.startsWith('51') && clean.length > 9) {
      clean = clean.slice(2);
    }
    setTelefono(clean.replace(/\D/g, '').slice(0, 9));
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
  const tipoDocAnterior =
    vehiculoEditar?.cliente?.tipo_documento || vehiculoEditar?.tipo_documento_cliente || 'DNI';
  const numDocAnterior =
    vehiculoEditar?.cliente?.numero_documento || vehiculoEditar?.documento_cliente || '';

  // Detección de cambios sensibles que requieren motivo y confirmación explícita
  const disminuyeKm = modo === 'EDITAR' && kmActualVal < kmAnteriorVal;
  const cambiaPlaca = modo === 'EDITAR' && vehiculoEditar && placa !== vehiculoEditar.placa;
  const cambiaTipoDoc = modo === 'EDITAR' && vehiculoEditar && tipoDoc !== tipoDocAnterior;
  const cambiaNumDoc =
    modo === 'EDITAR' && vehiculoEditar && numDoc.trim() !== numDocAnterior.trim();

  const requiereMotivo = disminuyeKm || cambiaPlaca || cambiaTipoDoc || cambiaNumDoc;

  const ejecutarGuardado = async () => {
    setGuardando(true);
    setErrorModal(null);

    try {
      const motorFinal = (tipoMotor.trim() as TipoMotor) || null;

      if (modo === 'CREAR') {
        const payload: CrearVehiculoDTO = {
          placa: placa.trim().toUpperCase(),
          marca: marca.trim(),
          modelo: modelo.trim(),
          anio: anio ? Number(anio) : null,
          tipo_motor: motorFinal,
          kilometraje_actual: kmActualVal,
          id_cliente: idClienteSeleccionado || undefined,
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
          id_cliente: idClienteSeleccionado || vehiculoEditar?.id_cliente,
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
      setConfirmModalOpen(false);
    }
  };

  const onSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    setErrorModal(null);

    if (requiereMotivo && (!motivo.trim() || motivo.trim().length < 5)) {
      setErrorModal(
        'Para realizar este cambio sensible (placa, kilometraje o documento del propietario), debe ingresar un motivo obligatorio (mínimo 5 caracteres).'
      );
      return;
    }

    if (requiereMotivo) {
      setConfirmModalOpen(true);
    } else {
      ejecutarGuardado();
    }
  };

  const placaDeshabilitada = modo === 'EDITAR' && !esAdmin;

  return (
    <>
      <div style={overlayStyle}>
        <div style={modalBoxStyle}>
          <h3
            style={{ fontSize: '20px', fontWeight: '800', color: '#F8FAFC', margin: '0 0 20px 0' }}
          >
            {modo === 'CREAR' ? 'Registrar Nuevo Vehículo / Cliente' : 'Editar Ficha Vehicular'}
          </h3>

          {errorModal && <div style={errorBoxStyle}>⚠️ {errorModal}</div>}

          <form
            onSubmit={onSubmit}
            style={{ display: 'flex', flexDirection: 'column', gap: '14px' }}
          >
            {/* DATOS DEL CLIENTE */}
            <div style={sectionBoxStyle}>
              <span style={sectionTitleStyle}>Datos del Propietario / Cliente</span>
              <div style={{ display: 'flex', flexDirection: 'column', gap: '10px' }}>
                {/* Campo con Minibuscador Predictivo */}
                <div style={{ position: 'relative' }}>
                  <label style={labelStyle}>
                    Cliente / Razón Social * (Escriba para buscar existente)
                  </label>
                  <input
                    required
                    type="text"
                    placeholder="Ej. Juan Pérez / Buscar por RUC o DNI..."
                    value={nombreCliente}
                    onChange={(e) => handleNombreClienteChange(e.target.value)}
                    onFocus={() => {
                      if (sugerenciasClientes.length > 0) setMostrarSugerencias(true);
                    }}
                    style={inputStyle}
                  />

                  {/* Lista desplegable flotante del Minibuscador */}
                  {mostrarSugerencias && sugerenciasClientes.length > 0 && (
                    <div style={dropdownContainerStyle}>
                      {sugerenciasClientes.map((c) => (
                        <div
                          key={c.id_cliente}
                          onClick={() => seleccionarClienteExistente(c)}
                          style={dropdownItemStyle}
                          onMouseEnter={(e) => (e.currentTarget.style.backgroundColor = '#334155')}
                          onMouseLeave={(e) =>
                            (e.currentTarget.style.backgroundColor = 'transparent')
                          }
                        >
                          <div style={{ fontWeight: '700', color: '#F8FAFC', fontSize: '13px' }}>
                            {c.nombre_razon_social}
                          </div>
                          <div style={{ fontSize: '11px', color: '#38BDF8' }}>
                            {c.tipo_documento}: {c.numero_documento}{' '}
                            {c.telefono ? `| Tel: ${c.telefono}` : ''}
                          </div>
                        </div>
                      ))}
                    </div>
                  )}
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
                      onChange={(e) => handleTelefonoChange(e.target.value)}
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
                      disabled={placaDeshabilitada}
                      type="text"
                      placeholder="ABC123"
                      value={placa}
                      onChange={(e) => handlePlacaChange(e.target.value)}
                      style={{
                        ...inputStyle,
                        fontFamily: 'monospace',
                        fontWeight: '700',
                        opacity: placaDeshabilitada ? 0.6 : 1,
                        cursor: placaDeshabilitada ? 'not-allowed' : 'text',
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

                {requiereMotivo && (
                  <div style={{ marginTop: '4px' }}>
                    <label style={{ ...labelStyle, color: '#EF4444', fontWeight: '700' }}>
                      ⚠️ Motivo Administrativo Obligatorio (5-250 chars) *
                    </label>
                    <input
                      required
                      type="text"
                      placeholder="Ej. Corrección por cambio de placa, documento o ajuste de kilometraje"
                      value={motivo}
                      onChange={(e) => setMotivo(e.target.value)}
                      style={{ ...inputStyle, borderColor: '#EF4444' }}
                    />
                  </div>
                )}
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
                style={btnSecondaryStyle}
              >
                Cancelar
              </button>
              <button type="submit" disabled={guardando} style={btnPrimaryStyle}>
                {guardando ? 'Guardando...' : 'Guardar Ficha'}
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
              Confirmar Modificación Sensible
            </h3>
            <p
              style={{
                fontSize: '13px',
                color: '#94A3B8',
                marginBottom: '20px',
                lineHeight: '1.5',
              }}
            >
              Está a punto de guardar cambios de carácter administrativo (
              {[
                cambiaPlaca ? 'Cambio de Placa' : null,
                cambiaTipoDoc || cambiaNumDoc ? 'Documento del Cliente' : null,
                disminuyeKm ? 'Reducción de Kilometraje' : null,
              ]
                .filter(Boolean)
                .join(', ')}
              ). ¿Desea proceder y registrar este motivo en auditoría?
            </p>
            <div style={{ display: 'flex', justifyContent: 'center', gap: '12px' }}>
              <button
                type="button"
                onClick={() => setConfirmModalOpen(false)}
                style={btnSecondaryStyle}
              >
                Revisar
              </button>
              <button
                type="button"
                onClick={ejecutarGuardado}
                style={{ ...btnPrimaryStyle, backgroundColor: '#D97706' }}
              >
                Sí, Aplicar Cambios
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

const dropdownContainerStyle: React.CSSProperties = {
  position: 'absolute',
  top: '100%',
  left: 0,
  right: 0,
  backgroundColor: '#1E293B',
  border: '1px solid #334155',
  borderRadius: '8px',
  marginTop: '4px',
  maxHeight: '180px',
  overflowY: 'auto',
  zIndex: 1100,
  boxShadow: '0 10px 25px rgba(0,0,0,0.5)',
};

const dropdownItemStyle: React.CSSProperties = {
  padding: '8px 12px',
  cursor: 'pointer',
  borderBottom: '1px solid #334155',
  transition: 'background-color 0.15s ease',
};
