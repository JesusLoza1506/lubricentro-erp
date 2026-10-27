import React, { useState, useEffect, useRef } from 'react';
import { CrearOrdenTrabajoPayload, ServicioInicialDTO } from '../../../types/ordenTrabajo';
import { vehiculoService } from '../../../services/vehiculoService';
import { ordenesService, MecanicoDTO, ServicioResumenDTO } from '../../../services/ordenesService';
import { VehiculoCompletoDTO } from '../../../types/vehiculo';

interface ModalCrearOTProps {
  idMecanicoDefault: number;
  placaInicial?: string;
  vehiculoDirecto?: VehiculoCompletoDTO;
  onClose: () => void;
  onSubmit: (payload: CrearOrdenTrabajoPayload) => Promise<void>;
  onIrARegistrarVehiculo?: () => void;
}

export const ModalCrearOT: React.FC<ModalCrearOTProps> = ({
  idMecanicoDefault,
  placaInicial,
  vehiculoDirecto,
  onClose,
  onSubmit,
  onIrARegistrarVehiculo,
}) => {
  const [listaVehiculosBD, setListaVehiculosBD] = useState<VehiculoCompletoDTO[]>([]);
  const [busqueda, setBusqueda] = useState(
    vehiculoDirecto ? vehiculoDirecto.placa : placaInicial ? placaInicial.toUpperCase() : ''
  );
  const [mostrarDropdown, setMostrarDropdown] = useState(false);

  const [mecanicos, setMecanicos] = useState<MecanicoDTO[]>([]);
  const [serviciosDisponibles, setServiciosDisponibles] = useState<ServicioResumenDTO[]>([]);

  const [idMecanico, setIdMecanico] = useState<number>(idMecanicoDefault || 0);
  const [kilometrajeIngreso, setKilometrajeIngreso] = useState<string>(
    vehiculoDirecto?.kilometraje_actual ? String(vehiculoDirecto.kilometraje_actual) : ''
  );
  const [pedidoCliente, setPedidoCliente] = useState('');
  const [idServicioInicial, setIdServicioInicial] = useState<number>(0);

  const [vehiculoSeleccionado, setVehiculoSeleccionado] = useState<VehiculoCompletoDTO | null>(
    vehiculoDirecto || null
  );
  const [loading, setLoading] = useState(false);
  const [errorMsg, setErrorMsg] = useState('');

  const dropdownRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const init = async () => {
      try {
        const [mecanicosList, serviciosList] = await Promise.all([
          ordenesService.listarMecanicos(),
          ordenesService.listarServicios(),
        ]);
        setMecanicos(mecanicosList);
        setServiciosDisponibles(serviciosList);
        if (serviciosList.length > 0) setIdServicioInicial(serviciosList[0].id_servicio);

        if (vehiculoDirecto) {
          seleccionarVehiculoReal(vehiculoDirecto);
        } else if (placaInicial) {
          const vehs = await vehiculoService.buscarVehiculosCandidatos(placaInicial);
          if (vehs.length > 0) seleccionarVehiculoReal(vehs[0]);
        }
      } catch (err) {
        console.error('Error inicializando modal crear OT:', err);
      }
    };
    init();
  }, [placaInicial, vehiculoDirecto]);

  useEffect(() => {
    if (busqueda.trim().length >= 2) {
      vehiculoService
        .buscarVehiculosCandidatos(busqueda)
        .then(setListaVehiculosBD)
        .catch(console.error);
    } else {
      setListaVehiculosBD([]);
    }
  }, [busqueda]);

  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (dropdownRef.current && !dropdownRef.current.contains(event.target as Node)) {
        setMostrarDropdown(false);
      }
    };
    document.addEventListener('mousedown', handleClickOutside);
    return () => document.removeEventListener('mousedown', handleClickOutside);
  }, []);

  const seleccionarVehiculoReal = (v: VehiculoCompletoDTO) => {
    setVehiculoSeleccionado(v);
    setBusqueda(v.placa);
    setKilometrajeIngreso(v.kilometraje_actual ? String(v.kilometraje_actual) : '');
    setMostrarDropdown(false);
    setErrorMsg('');
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    if (!vehiculoSeleccionado) {
      setErrorMsg('Debe seleccionar un vehículo válido registrado.');
      return;
    }

    const kmNum = Number(kilometrajeIngreso);
    // Validación estricta: Rechazar kilometraje cero o negativo
    if (isNaN(kmNum) || kmNum <= 0) {
      setErrorMsg('El kilometraje de ingreso es obligatorio y debe ser estrictamente mayor a 0.');
      return;
    }

    if (!idServicioInicial) {
      setErrorMsg('Debe seleccionar obligatoriamente un servicio inicial.');
      return;
    }

    try {
      setLoading(true);
      setErrorMsg('');

      const serviciosIniciales: ServicioInicialDTO[] = [
        { id_servicio: idServicioInicial, cantidad: 1 },
      ];

      await onSubmit({
        id_vehiculo: vehiculoSeleccionado.id_vehiculo,
        id_cliente_atendido: vehiculoSeleccionado.id_cliente,
        kilometraje_ingreso: kmNum,
        id_mecanico: idMecanico > 0 ? idMecanico : undefined,
        pedido_cliente: pedidoCliente.trim() || undefined,
        servicios_iniciales: serviciosIniciales,
      });

      onClose();
    } catch (err: any) {
      setErrorMsg(typeof err === 'string' ? err : err?.message || 'Error al recepcionar OT.');
    } finally {
      setLoading(false);
    }
  };

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
          Nueva Orden de Trabajo (Recepción)
        </h2>

        {errorMsg && (
          <div
            style={{
              padding: '10px',
              backgroundColor: '#7F1D1D',
              color: '#FCA5A5',
              borderRadius: '6px',
              marginBottom: '16px',
              fontSize: '13px',
            }}
          >
            {errorMsg}
          </div>
        )}

        <form onSubmit={handleSubmit}>
          <div style={{ marginBottom: '16px', position: 'relative' }} ref={dropdownRef}>
            <label
              style={{ display: 'block', fontSize: '13px', color: '#94A3B8', marginBottom: '6px' }}
            >
              Placa Vehículo *
            </label>
            <input
              type="text"
              required
              placeholder="Buscar por placa..."
              value={busqueda}
              onFocus={() => setMostrarDropdown(true)}
              onChange={(e) => {
                setBusqueda(e.target.value.toUpperCase());
                setMostrarDropdown(true);
                setVehiculoSeleccionado(null);
              }}
              style={{
                width: '100%',
                padding: '10px',
                backgroundColor: '#0F172A',
                border: '1px solid #334155',
                borderRadius: '6px',
                color: '#F8FAFC',
                textTransform: 'uppercase',
              }}
            />
            {mostrarDropdown && (
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
                  maxHeight: '180px',
                  overflowY: 'auto',
                  zIndex: 1050,
                }}
              >
                {listaVehiculosBD.length > 0 ? (
                  listaVehiculosBD.map((v) => (
                    <div
                      key={v.id_vehiculo}
                      onClick={() => seleccionarVehiculoReal(v)}
                      style={{
                        padding: '8px 12px',
                        cursor: 'pointer',
                        borderBottom: '1px solid #1E293B',
                        fontSize: '13px',
                      }}
                    >
                      🚘 <strong>{v.placa}</strong> - {v.marca} {v.modelo}
                    </div>
                  ))
                ) : (
                  <div
                    style={{
                      padding: '10px',
                      fontSize: '12px',
                      color: '#94A3B8',
                      textAlign: 'center',
                    }}
                  >
                    No se encontraron vehículos.{' '}
                    {onIrARegistrarVehiculo && (
                      <button
                        type="button"
                        onClick={onIrARegistrarVehiculo}
                        style={{
                          background: 'none',
                          border: 'none',
                          color: '#38BDF8',
                          cursor: 'pointer',
                          textDecoration: 'underline',
                          padding: 0,
                          fontSize: '12px',
                        }}
                      >
                        Registrar uno nuevo
                      </button>
                    )}
                  </div>
                )}
              </div>
            )}
          </div>

          <div style={{ marginBottom: '16px' }}>
            <label
              style={{ display: 'block', fontSize: '13px', color: '#94A3B8', marginBottom: '6px' }}
            >
              Servicio Inicial Único * (Cantidad: 1)
            </label>
            <select
              value={idServicioInicial}
              onChange={(e) => setIdServicioInicial(Number(e.target.value))}
              style={{
                width: '100%',
                padding: '10px',
                backgroundColor: '#0F172A',
                border: '1px solid #334155',
                borderRadius: '6px',
                color: '#F8FAFC',
              }}
            >
              {serviciosDisponibles.map((s) => (
                <option key={s.id_servicio} value={s.id_servicio}>
                  {s.nombre} (S/ {s.precio_venta_soles.toFixed(2)})
                </option>
              ))}
            </select>
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
                Km Ingreso * (Mayor a 0)
              </label>
              <input
                type="number"
                min={1}
                required
                value={kilometrajeIngreso}
                onChange={(e) => setKilometrajeIngreso(e.target.value)}
                style={{
                  width: '100%',
                  padding: '10px',
                  backgroundColor: '#0F172A',
                  border: '1px solid #334155',
                  borderRadius: '6px',
                  color: '#F8FAFC',
                }}
              />
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
                Mecánico Predeterminado
              </label>
              <select
                value={idMecanico}
                onChange={(e) => setIdMecanico(Number(e.target.value))}
                style={{
                  width: '100%',
                  padding: '10px',
                  backgroundColor: '#0F172A',
                  border: '1px solid #334155',
                  borderRadius: '6px',
                  color: '#F8FAFC',
                }}
              >
                <option value={0}>Sin Asignar (A cola común)</option>
                {mecanicos.map((m) => (
                  <option key={m.id_usuario} value={m.id_usuario}>
                    {m.nombre_completo}
                  </option>
                ))}
              </select>
            </div>
          </div>

          <div style={{ marginBottom: '20px' }}>
            <label
              style={{ display: 'block', fontSize: '13px', color: '#94A3B8', marginBottom: '6px' }}
            >
              Pedido del Cliente
            </label>
            <textarea
              rows={2}
              value={pedidoCliente}
              onChange={(e) => setPedidoCliente(e.target.value)}
              placeholder="Detalle de fallas o pedido especial..."
              style={{
                width: '100%',
                padding: '10px',
                backgroundColor: '#0F172A',
                border: '1px solid #334155',
                borderRadius: '6px',
                color: '#F8FAFC',
              }}
            />
          </div>

          <div style={{ display: 'flex', justifyContent: 'flex-end', gap: '12px' }}>
            <button
              type="button"
              disabled={loading}
              onClick={onClose}
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
              disabled={loading || !vehiculoSeleccionado}
              style={{
                padding: '10px 18px',
                backgroundColor: '#2563EB',
                color: '#FFF',
                border: 'none',
                borderRadius: '6px',
                fontWeight: '600',
                cursor: loading ? 'not-allowed' : 'pointer',
                opacity: loading ? 0.6 : 1,
              }}
            >
              {loading ? 'Recepcionando...' : 'Recepcionar OT'}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
};
