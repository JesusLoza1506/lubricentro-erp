import { mensajeErrorOT } from '../AvisosOT';
import '../ordenesVisual.css';
import React, { useState, useEffect, useRef } from 'react';
import { Car, ClipboardPlus, AlertCircle, X } from 'lucide-react';
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
        setErrorMsg(mensajeErrorOT(err, 'No se pudieron cargar los datos de recepción. Cierra y vuelve a abrir el formulario para reintentar.'));
      }
    };
    init();
  }, [placaInicial, vehiculoDirecto]);

  useEffect(() => {
    if (busqueda.trim().length >= 2) {
      vehiculoService
        .buscarVehiculosCandidatos(busqueda)
        .then(setListaVehiculosBD)
        .catch((err: unknown) => setErrorMsg(mensajeErrorOT(err, 'No se pudo buscar el vehículo. Intenta nuevamente.')));
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
    <div className="gloria-theme gt-overlay ot-overlay">
      <div className="gt-dialog ot-dialog ot-reception-dialog" role="dialog" aria-modal="true"
        aria-labelledby="ot-reception-title" aria-describedby="ot-reception-description">
        <header className="ot-dialog-heading">
          <span className="ot-dialog-icon"><ClipboardPlus size={21} aria-hidden="true" /></span>
          <div>
            <span className="ot-eyebrow">Ingreso al taller</span>
            <h2 id="ot-reception-title">Recepcionar vehículo</h2>
            <p id="ot-reception-description">Registra el ingreso y deja la orden en la cola de espera.</p>
          </div>
          <button type="button" className="ot-button ot-close" aria-label="Cerrar recepción" disabled={loading} onClick={onClose}>
            <X size={18} aria-hidden="true" />
          </button>
        </header>

        {errorMsg && (
          <div className="gt-notice gt-notice--error ot-form-error" role="alert">
            <AlertCircle size={17} aria-hidden="true" /> <span>{errorMsg}</span>
          </div>
        )}

        <form onSubmit={handleSubmit} className="ot-reception-form">
          <div className="ot-field ot-reception-search" ref={dropdownRef}>
            <label htmlFor="ot-reception-plate">Placa del vehículo *</label>
            <input id="ot-reception-plate" type="text" required placeholder="Buscar por placa..." value={busqueda}
              onFocus={() => setMostrarDropdown(true)}
              onChange={(e) => {
                setBusqueda(e.target.value.toUpperCase());
                setMostrarDropdown(true);
                setVehiculoSeleccionado(null);
              }}
            />
            {mostrarDropdown && (
              <div className="ot-reception-results">
                {listaVehiculosBD.length > 0 ? listaVehiculosBD.map((v) => (
                  <button type="button" key={v.id_vehiculo} onClick={() => seleccionarVehiculoReal(v)}>
                    <Car size={16} aria-hidden="true" /> <strong>{v.placa}</strong>
                    <span>{v.marca} {v.modelo}</span>
                  </button>
                )) : (
                  <div className="ot-reception-no-results">
                    No se encontraron vehículos.{' '}
                    {onIrARegistrarVehiculo && (
                      <button type="button" className="ot-register-link" onClick={onIrARegistrarVehiculo}>
                        Registrar uno nuevo
                      </button>
                    )}
                  </div>
                )}
              </div>
            )}
            {vehiculoSeleccionado && !mostrarDropdown && (
              <span className="ot-reception-selected">Vehículo seleccionado</span>
            )}
          </div>

          <div className="ot-field">
            <label htmlFor="ot-reception-service">Servicio inicial * <small>· Cantidad: 1</small></label>
            <select id="ot-reception-service" value={idServicioInicial} onChange={(e) => setIdServicioInicial(Number(e.target.value))}>
              {serviciosDisponibles.map((s) => (
                <option key={s.id_servicio} value={s.id_servicio}>
                  {s.nombre} (S/ {s.precio_venta_soles.toFixed(2)})
                </option>
              ))}
            </select>
          </div>

          <div className="ot-reception-grid">
            <div className="ot-field">
              <label htmlFor="ot-reception-mileage">Kilometraje de ingreso *</label>
              <input id="ot-reception-mileage" type="number" min={1} required value={kilometrajeIngreso}
                onChange={(e) => setKilometrajeIngreso(e.target.value)} />
            </div>
            <div className="ot-field">
              <label htmlFor="ot-reception-mechanic">Mecánico asignado <small>(opcional)</small></label>
              <select id="ot-reception-mechanic" value={idMecanico} onChange={(e) => setIdMecanico(Number(e.target.value))}>
                <option value={0}>Sin asignar · Cola común</option>
                {mecanicos.map((m) => (
                  <option key={m.id_usuario} value={m.id_usuario}>{m.nombre_completo}</option>
                ))}
              </select>
            </div>
          </div>

          <div className="ot-field">
            <label htmlFor="ot-reception-request">Pedido del cliente</label>
            <textarea id="ot-reception-request" rows={2} value={pedidoCliente}
              onChange={(e) => setPedidoCliente(e.target.value)} placeholder="Detalle de fallas o pedido especial..." />
          </div>

          <footer className="ot-dialog-footer ot-reception-footer">
            <button type="button" className="ot-button" disabled={loading} onClick={onClose}>Cancelar</button>
            <button type="submit" className="ot-button ot-primary" disabled={loading || !vehiculoSeleccionado} aria-busy={loading}>
              {loading && <span className="gt-spinner" aria-hidden="true" />}
              {loading ? 'Recepcionando...' : 'Recepcionar OT'}
            </button>
          </footer>
        </form>
      </div>
    </div>
  );
};
