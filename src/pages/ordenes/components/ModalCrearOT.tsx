import React, { useState, useEffect, useRef } from 'react';
import {
  CrearOrdenTrabajoPayload,
  TipoAceiteOT,
  OrdenTrabajoHistorialDTO,
} from '../../../types/ordenTrabajo';
import { vehiculoService } from '../../../services/vehiculoService';
import { ordenesService, MecanicoDTO } from '../../../services/ordenesService';
import { VehiculoCompletoDTO } from '../../../types/vehiculo';

interface ModalCrearOTProps {
  ordenesExistentes?: OrdenTrabajoHistorialDTO[];
  zanjaInicial?: number;
  zanjasOcupadas?: number[];
  idMecanicoDefault: number;
  placaInicial?: string;
  onClose: () => void;
  onSubmit: (payload: CrearOrdenTrabajoPayload) => Promise<void>;
  onIrARegistrarVehiculo?: () => void;
}

export const ModalCrearOT: React.FC<ModalCrearOTProps> = ({
  zanjaInicial,
  zanjasOcupadas = [],
  idMecanicoDefault,
  placaInicial,
  onClose,
  onSubmit,
  onIrARegistrarVehiculo,
}) => {
  const [listaVehiculosBD, setListaVehiculosBD] = useState<VehiculoCompletoDTO[]>([]);
  const [cargandoVehiculos, setCargandoVehiculos] = useState(false);
  const [busqueda, setBusqueda] = useState(placaInicial ? placaInicial.toUpperCase() : '');
  const [mostrarDropdown, setMostrarDropdown] = useState(false);

  // Carga de mecánicos dinámicos
  const [mecanicos, setMecanicos] = useState<MecanicoDTO[]>([]);
  const [cargandoMecanicos, setCargandoMecanicos] = useState(false);
  const [idMecanico, setIdMecanico] = useState<number>(0);

  // Verificación de ocupación de zanjas (1 y 2)
  const zanja1Ocupada = zanjasOcupadas.includes(1);
  const zanja2Ocupada = zanjasOcupadas.includes(2);
  const sinZanjasDisponibles = zanja1Ocupada && zanja2Ocupada;

  const obtenerZanjaInicialValida = (): number => {
    if (zanjaInicial && !zanjasOcupadas.includes(zanjaInicial)) {
      return zanjaInicial;
    }
    if (!zanja1Ocupada) return 1;
    if (!zanja2Ocupada) return 2;
    return 1;
  };

  const [zanja, setZanja] = useState<number>(obtenerZanjaInicialValida());
  const [kilometrajeIngreso, setKilometrajeIngreso] = useState<string>('');
  const [tipoAceite, setTipoAceite] = useState<TipoAceiteOT>('MINERAL');
  const [observaciones, setObservaciones] = useState('');

  // Vehículo seleccionado / Estado
  const [vehiculoSeleccionado, setVehiculoSeleccionado] = useState<VehiculoCompletoDTO | null>(
    null
  );
  const [loading, setLoading] = useState(false);
  const [errorMsg, setErrorMsg] = useState('');

  const dropdownRef = useRef<HTMLDivElement>(null);

  // 1. Cargar mecánicos dinámicamente desde la BD
  useEffect(() => {
    const cargarMecanicos = async () => {
      try {
        setCargandoMecanicos(true);
        const lista = await ordenesService.listarMecanicos();
        setMecanicos(lista);
        if (lista.length > 0) {
          setIdMecanico(lista[0].id_usuario);
        }
      } catch (err) {
        console.error('Error al cargar la lista de mecánicos:', err);
      } finally {
        setCargandoMecanicos(false);
      }
    };

    cargarMecanicos();
  }, []);

  // 2. Cargar TODOS los vehículos directamente de la BD SQLite
  useEffect(() => {
    const cargarVehiculosDeBD = async () => {
      try {
        setCargandoVehiculos(true);
        const listaBD = await vehiculoService.listarVehiculos(idMecanicoDefault);
        if (Array.isArray(listaBD)) {
          setListaVehiculosBD(listaBD);
          if (placaInicial) {
            const encontrado = listaBD.find(
              (v) => v.placa.toUpperCase() === placaInicial.toUpperCase()
            );
            if (encontrado) {
              seleccionarVehiculoReal(encontrado);
            } else {
              consultarPlacaEspecificaBD(placaInicial);
            }
          }
        }
      } catch (err) {
        console.error('Error al obtener los vehículos:', err);
      } finally {
        setCargandoVehiculos(false);
      }
    };

    cargarVehiculosDeBD();
  }, [idMecanicoDefault, placaInicial]);

  useEffect(() => {
    setZanja(obtenerZanjaInicialValida());
  }, [zanjaInicial, zanjasOcupadas]);

  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (dropdownRef.current && !dropdownRef.current.contains(event.target as Node)) {
        setMostrarDropdown(false);
      }
    };
    document.addEventListener('mousedown', handleClickOutside);
    return () => document.removeEventListener('mousedown', handleClickOutside);
  }, []);

  const seleccionarVehiculoReal = (vehiculo: VehiculoCompletoDTO) => {
    setVehiculoSeleccionado(vehiculo);
    setBusqueda(vehiculo.placa);
    setKilometrajeIngreso(''); // Sin auto-relleno
    setMostrarDropdown(false);
    setErrorMsg('');
  };

  const consultarPlacaEspecificaBD = async (placaATipear: string) => {
    const placaLimpia = placaATipear.toUpperCase().trim();
    if (!placaLimpia) return;

    try {
      setLoading(true);
      setErrorMsg('');
      const data = await vehiculoService.obtenerVehiculoPorPlaca(placaLimpia, idMecanicoDefault);
      if (data && data.placa) {
        seleccionarVehiculoReal(data);
      } else {
        setVehiculoSeleccionado(null);
        setErrorMsg(
          `El vehículo con placa "${placaLimpia}" no está registrado en la base de datos.`
        );
      }
    } catch {
      setVehiculoSeleccionado(null);
      setErrorMsg(`No se encontró el vehículo con placa "${placaLimpia}".`);
    } finally {
      setLoading(false);
      setMostrarDropdown(false);
    }
  };

  const opcionesFiltradas = listaVehiculosBD.filter(
    (v) =>
      v.placa.toLowerCase().includes(busqueda.toLowerCase()) ||
      (v.marca && v.marca.toLowerCase().includes(busqueda.toLowerCase())) ||
      (v.modelo && v.modelo.toLowerCase().includes(busqueda.toLowerCase()))
  );

  const handleIrARegistrar = () => {
    onClose();
    if (onIrARegistrarVehiculo) {
      onIrARegistrarVehiculo();
    }
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    if (sinZanjasDisponibles) {
      setErrorMsg('No hay zanjas disponibles en este momento.');
      return;
    }

    if (!vehiculoSeleccionado) {
      setErrorMsg('Debe seleccionar un vehículo válido registrado en la base de datos.');
      return;
    }

    if (!idMecanico || idMecanico === 0) {
      setErrorMsg('Debe seleccionar un mecánico válido.');
      return;
    }

    const kmIngresadoNum = Number(kilometrajeIngreso);
    if (isNaN(kmIngresadoNum) || kmIngresadoNum <= 0) {
      setErrorMsg('Debe ingresar un kilometraje válido.');
      return;
    }

    // Permite mayor o igual para mantener consistencia estricta con la validación de Backend
    const ultimoKmBD = vehiculoSeleccionado.kilometraje_actual || 0;
    if (kmIngresadoNum < ultimoKmBD) {
      setErrorMsg(
        `El kilometraje de ingreso (${kmIngresadoNum.toLocaleString()} km) debe ser mayor o igual al último registrado (${ultimoKmBD.toLocaleString()} km).`
      );
      return;
    }

    try {
      setLoading(true);
      setErrorMsg('');
      await onSubmit({
        placa: vehiculoSeleccionado.placa,
        id_mecanico: Number(idMecanico),
        zanja: Number(zanja),
        kilometraje_ingreso: kmIngresadoNum,
        tipo_aceite: tipoAceite,
        observaciones: observaciones.trim() || undefined,
      });
      onClose();
    } catch (err: any) {
      setErrorMsg(typeof err === 'string' ? err : err?.message || 'Error al crear la OT.');
    } finally {
      setLoading(false);
    }
  };

  // Cálculo para ayuda informativa
  const ultimoKm = vehiculoSeleccionado?.kilometraje_actual || 0;
  const estimadoProximo = ultimoKm + (tipoAceite === 'SINTETICO' ? 10000 : 5000);

  return (
    <div
      style={{
        position: 'fixed',
        top: 0,
        left: 0,
        right: 0,
        bottom: 0,
        backgroundColor: 'rgba(15, 23, 42, 0.8)',
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
          maxWidth: '520px',
          padding: '24px',
          border: '1px solid #334155',
          color: '#F8FAFC',
        }}
      >
        <h2 style={{ marginTop: 0, fontSize: '20px', color: '#F8FAFC' }}>
          Nueva Orden de Trabajo {zanjaInicial ? `- Zanja ${zanjaInicial}` : ''}
        </h2>

        {sinZanjasDisponibles && (
          <div
            style={{
              padding: '12px',
              backgroundColor: '#7F1D1D',
              border: '1px solid #EF4444',
              color: '#FCA5A5',
              borderRadius: '6px',
              marginBottom: '16px',
              fontSize: '14px',
              fontWeight: 600,
              textAlign: 'center',
            }}
          >
            🚫 No hay zanjas disponibles. Libere una zanja finalizando o cancelando una OT activa.
          </div>
        )}

        {errorMsg && !sinZanjasDisponibles && (
          <div
            style={{
              padding: '10px',
              backgroundColor: '#7F1D1D',
              color: '#FCA5A5',
              borderRadius: '6px',
              marginBottom: '16px',
              fontSize: '14px',
              display: 'flex',
              justifyContent: 'space-between',
              alignItems: 'center',
            }}
          >
            <span>{errorMsg}</span>
            {!vehiculoSeleccionado && busqueda.trim() && (
              <button
                type="button"
                onClick={handleIrARegistrar}
                style={{
                  backgroundColor: '#D97706',
                  color: '#FFF',
                  border: 'none',
                  padding: '4px 8px',
                  borderRadius: '4px',
                  cursor: 'pointer',
                  fontWeight: 600,
                  fontSize: '11px',
                }}
              >
                Registrar ↗
              </button>
            )}
          </div>
        )}

        <form onSubmit={handleSubmit}>
          <div style={{ marginBottom: '16px', position: 'relative' }} ref={dropdownRef}>
            <label
              style={{ display: 'block', fontSize: '13px', color: '#94A3B8', marginBottom: '6px' }}
            >
              Seleccionar Placa *
            </label>
            <div style={{ position: 'relative' }}>
              <input
                type="text"
                required
                disabled={sinZanjasDisponibles}
                placeholder={
                  cargandoVehiculos
                    ? 'Cargando vehículos...'
                    : 'Seleccione o escriba la placa a buscar...'
                }
                value={busqueda}
                onFocus={() => setMostrarDropdown(true)}
                onChange={(e) => {
                  setBusqueda(e.target.value.toUpperCase());
                  setMostrarDropdown(true);
                  setVehiculoSeleccionado(null);
                  setErrorMsg('');
                }}
                onKeyDown={(e) => {
                  if (e.key === 'Enter') {
                    e.preventDefault();
                    if (busqueda.trim() && !vehiculoSeleccionado) {
                      consultarPlacaEspecificaBD(busqueda);
                    }
                  }
                }}
                style={{
                  width: '100%',
                  padding: '10px 36px 10px 12px',
                  backgroundColor: sinZanjasDisponibles ? '#1E293B' : '#0F172A',
                  border: vehiculoSeleccionado ? '1px solid #10B981' : '1px solid #334155',
                  borderRadius: '6px',
                  color: '#F8FAFC',
                  textTransform: 'uppercase',
                  boxSizing: 'border-box',
                }}
              />
              <span
                onClick={() => !sinZanjasDisponibles && setMostrarDropdown(!mostrarDropdown)}
                style={{
                  position: 'absolute',
                  right: '12px',
                  top: '50%',
                  transform: 'translateY(-50%)',
                  color: '#94A3B8',
                  cursor: 'pointer',
                  fontSize: '12px',
                }}
              >
                ▼
              </span>
            </div>

            {mostrarDropdown && !sinZanjasDisponibles && (
              <div
                style={{
                  position: 'absolute',
                  top: '100%',
                  left: 0,
                  right: 0,
                  backgroundColor: '#0F172A',
                  border: '1px solid #334155',
                  borderRadius: '6px',
                  marginTop: '4px',
                  maxHeight: '200px',
                  overflowY: 'auto',
                  zIndex: 1050,
                  boxShadow: '0 10px 15px -3px rgba(0, 0, 0, 0.5)',
                }}
              >
                {cargandoVehiculos ? (
                  <div
                    style={{
                      padding: '12px',
                      fontSize: '13px',
                      color: '#94A3B8',
                      textAlign: 'center',
                    }}
                  >
                    Cargando todos los vehículos de la base de datos...
                  </div>
                ) : opcionesFiltradas.length > 0 ? (
                  opcionesFiltradas.map((v) => (
                    <div
                      key={v.placa}
                      onClick={() => seleccionarVehiculoReal(v)}
                      style={{
                        padding: '10px 12px',
                        cursor: 'pointer',
                        fontSize: '13px',
                        borderBottom: '1px solid #1E293B',
                        color: '#F8FAFC',
                        display: 'flex',
                        justifyContent: 'space-between',
                      }}
                      onMouseEnter={(e) => (e.currentTarget.style.backgroundColor = '#1E293B')}
                      onMouseLeave={(e) => (e.currentTarget.style.backgroundColor = 'transparent')}
                    >
                      <span>
                        🚘 <strong>{v.placa}</strong>{' '}
                        {v.marca ? `— ${v.marca} ${v.modelo || ''}` : ''}
                      </span>
                      <span style={{ color: '#38BDF8', fontSize: '12px' }}>
                        {v.kilometraje_actual ? `${v.kilometraje_actual.toLocaleString()} km` : ''}
                      </span>
                    </div>
                  ))
                ) : (
                  <div
                    style={{
                      padding: '12px',
                      fontSize: '13px',
                      color: '#94A3B8',
                      textAlign: 'center',
                    }}
                  >
                    No hay coincidencia local para "{busqueda}"
                  </div>
                )}

                {busqueda.trim() && (
                  <div
                    onClick={() => consultarPlacaEspecificaBD(busqueda)}
                    style={{
                      padding: '10px 12px',
                      cursor: 'pointer',
                      fontSize: '13px',
                      backgroundColor: '#1E293B',
                      color: '#38BDF8',
                      fontWeight: 600,
                      textAlign: 'center',
                      borderTop: '1px solid #334155',
                    }}
                  >
                    🔍 Consultar "{busqueda}" directamente en la BD
                  </div>
                )}
              </div>
            )}

            {vehiculoSeleccionado && (
              <div
                style={{
                  marginTop: '8px',
                  padding: '8px 12px',
                  backgroundColor: '#064E3B',
                  border: '1px solid #059669',
                  borderRadius: '6px',
                  fontSize: '12px',
                  color: '#A7F3D0',
                }}
              >
                ✓ <strong>{vehiculoSeleccionado.placa}</strong> —{' '}
                {vehiculoSeleccionado.marca || 'Vehículo'} {vehiculoSeleccionado.modelo || ''} (Km
                BD: {ultimoKm.toLocaleString()})
              </div>
            )}
          </div>

          <div
            style={{
              display: 'grid',
              gridTemplateColumns: '1fr 1fr',
              gap: '12px',
              marginBottom: '16px',
            }}
          >
            <div>
              <label
                style={{
                  display: 'block',
                  fontSize: '13px',
                  color: '#94A3B8',
                  marginBottom: '6px',
                }}
              >
                Asignar Zanja *
              </label>
              <select
                disabled={sinZanjasDisponibles}
                value={zanja}
                onChange={(e) => setZanja(Number(e.target.value))}
                style={{
                  width: '100%',
                  padding: '10px',
                  backgroundColor: sinZanjasDisponibles ? '#1E293B' : '#0F172A',
                  border: '1px solid #334155',
                  borderRadius: '6px',
                  color: '#F8FAFC',
                  boxSizing: 'border-box',
                }}
              >
                <option value={1} disabled={zanja1Ocupada}>
                  Zanja 1 {zanja1Ocupada ? '(Ocupada)' : ''}
                </option>
                <option value={2} disabled={zanja2Ocupada}>
                  Zanja 2 {zanja2Ocupada ? '(Ocupada)' : ''}
                </option>
              </select>
            </div>

            <div>
              <label
                style={{
                  display: 'block',
                  fontSize: '13px',
                  color: '#94A3B8',
                  marginBottom: '6px',
                }}
              >
                Mecánico Asignado *
              </label>
              <select
                disabled={sinZanjasDisponibles || cargandoMecanicos || mecanicos.length === 0}
                value={idMecanico}
                onChange={(e) => setIdMecanico(Number(e.target.value))}
                style={{
                  width: '100%',
                  padding: '10px',
                  backgroundColor: sinZanjasDisponibles ? '#1E293B' : '#0F172A',
                  border: '1px solid #334155',
                  borderRadius: '6px',
                  color: '#F8FAFC',
                  boxSizing: 'border-box',
                }}
              >
                {cargandoMecanicos ? (
                  <option value={0}>Cargando mecánicos...</option>
                ) : mecanicos.length > 0 ? (
                  mecanicos.map((m) => (
                    <option key={m.id_usuario} value={m.id_usuario}>
                      {m.nombre_completo}
                    </option>
                  ))
                ) : (
                  <option value={0}>No hay mecánicos registrados</option>
                )}
              </select>
            </div>
          </div>

          <div style={{ marginBottom: '16px' }}>
            <label
              style={{ display: 'block', fontSize: '13px', color: '#94A3B8', marginBottom: '6px' }}
            >
              Kilometraje de Ingreso (km) *
            </label>
            <input
              type="number"
              required
              min={0}
              disabled={sinZanjasDisponibles}
              placeholder="Ej. 65200"
              value={kilometrajeIngreso}
              onChange={(e) => {
                setKilometrajeIngreso(e.target.value);
                setErrorMsg('');
              }}
              style={{
                width: '100%',
                padding: '10px',
                backgroundColor: sinZanjasDisponibles ? '#1E293B' : '#0F172A',
                border: '1px solid #334155',
                borderRadius: '6px',
                color: '#F8FAFC',
                boxSizing: 'border-box',
              }}
            />
            {/* Texto de Ayuda / Placeholder Informativo */}
            {vehiculoSeleccionado && (
              <div
                style={{
                  marginTop: '6px',
                  fontSize: '12px',
                  color: '#38BDF8',
                  display: 'flex',
                  alignItems: 'center',
                  gap: '4px',
                }}
              >
                <span>💡</span>
                <span>
                  Último ingreso: <strong>{ultimoKm.toLocaleString()} km</strong> | Estimado actual:{' '}
                  <strong>{estimadoProximo.toLocaleString()} km</strong>
                </span>
              </div>
            )}
          </div>

          <div style={{ marginBottom: '16px' }}>
            <label
              style={{ display: 'block', fontSize: '13px', color: '#94A3B8', marginBottom: '6px' }}
            >
              Tipo de Aceite a Aplicar *
            </label>
            <select
              disabled={sinZanjasDisponibles}
              value={tipoAceite}
              onChange={(e) => setTipoAceite(e.target.value as TipoAceiteOT)}
              style={{
                width: '100%',
                padding: '10px',
                backgroundColor: sinZanjasDisponibles ? '#1E293B' : '#0F172A',
                border: '1px solid #334155',
                borderRadius: '6px',
                color: '#F8FAFC',
                boxSizing: 'border-box',
              }}
            >
              <option value="MINERAL">Mineral (+5,000 km próx. mantenimiento)</option>
              <option value="SINTETICO">Sintético (+10,000 km próx. mantenimiento)</option>
            </select>
          </div>

          <div style={{ marginBottom: '20px' }}>
            <label
              style={{ display: 'block', fontSize: '13px', color: '#94A3B8', marginBottom: '6px' }}
            >
              Observaciones
            </label>
            <textarea
              rows={3}
              disabled={sinZanjasDisponibles}
              placeholder="Detalle los requerimientos del cliente..."
              value={observaciones}
              onChange={(e) => setObservaciones(e.target.value)}
              style={{
                width: '100%',
                padding: '10px',
                backgroundColor: sinZanjasDisponibles ? '#1E293B' : '#0F172A',
                border: '1px solid #334155',
                borderRadius: '6px',
                color: '#F8FAFC',
                boxSizing: 'border-box',
              }}
            />
          </div>

          <div style={{ display: 'flex', justifyContent: 'flex-end', gap: '12px' }}>
            <button
              type="button"
              onClick={onClose}
              disabled={loading}
              style={{
                padding: '10px 16px',
                backgroundColor: 'transparent',
                color: '#94A3B8',
                border: '1px solid #334155',
                borderRadius: '6px',
                cursor: 'pointer',
              }}
            >
              Cancelar
            </button>
            <button
              type="submit"
              disabled={loading || sinZanjasDisponibles || !vehiculoSeleccionado || !idMecanico}
              style={{
                padding: '10px 18px',
                backgroundColor:
                  sinZanjasDisponibles || !vehiculoSeleccionado || !idMecanico
                    ? '#475569'
                    : '#2563EB',
                color: '#FFFFFF',
                border: 'none',
                borderRadius: '6px',
                fontWeight: '600',
                cursor:
                  sinZanjasDisponibles || !vehiculoSeleccionado || !idMecanico
                    ? 'not-allowed'
                    : 'pointer',
              }}
            >
              {loading ? 'Guardando...' : 'Crear OT'}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
};
