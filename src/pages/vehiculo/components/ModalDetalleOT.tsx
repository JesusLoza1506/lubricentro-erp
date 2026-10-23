import React from 'react';
import {
  HistorialOTPublicoDTO,
  ServicioOTPublicoDTO,
  InsumoOTPublicoDTO,
} from '../../../types/vehiculo';
import { Wrench, Calendar, Gauge, X, Droplet, UserCheck, User, Car, Package, FilePenLine } from 'lucide-react';

interface ModalDetalleOTProps {
  modalAbierto: boolean;
  setModalAbierto: (abierto: boolean) => void;
  ordenTrabajo: HistorialOTPublicoDTO | null;
}

export const ModalDetalleOT: React.FC<ModalDetalleOTProps> = ({
  modalAbierto,
  setModalAbierto,
  ordenTrabajo,
}) => {
  if (!modalAbierto || !ordenTrabajo) return null;

  const formatearFecha = (fechaStr?: string) => {
    if (!fechaStr) return 'N/A';
    const fecha = new Date(fechaStr);
    return isNaN(fecha.getTime()) ? 'N/A' : fecha.toLocaleDateString('es-PE');
  };

  const nombreClienteAtendido =
    ordenTrabajo.nombre_cliente ||
    ordenTrabajo.cliente_nombre ||
    'Cliente Varios / No especificado';

  const docClienteAtendido = ordenTrabajo.documento_cliente || ordenTrabajo.cliente_documento;
  const tipoDocClienteAtendido = ordenTrabajo.tipo_documento_cliente || 'DOC';

  return (
    <div className="fv-modal-overlay">
      <div className="fv-dialog fv-attention-dialog" role="dialog" aria-modal="true" aria-labelledby="fv-attention-title">
        <header className="fv-dialog-header">
          <div className="fv-dialog-title-group">
            <span className="fv-dialog-icon"><Wrench size={21} /></span>
            <div>
              <span className="fv-eyebrow">Historial de mantenimiento</span>
              <h3 id="fv-attention-title">Atención {ordenTrabajo.codigo_ot}</h3>
            </div>
          </div>
          <button type="button" className="fv-icon-button" aria-label="Cerrar detalle de atención" onClick={() => setModalAbierto(false)}><X size={20} /></button>
        </header>

        <div className="fv-dialog-body">
          <div className="fv-attention-identity">
            <span className="fv-plate-tag"><Car size={16} />{ordenTrabajo.placa_snapshot || ordenTrabajo.placa}</span>
            <span className="fv-status" data-state={ordenTrabajo.estado || 'FINALIZADO'}>{ordenTrabajo.estado || 'FINALIZADO'}</span>
            {ordenTrabajo.corregido === 1 && <span className="fv-corrected"><FilePenLine size={13} />Corregido</span>}
          </div>

          <div className="fv-attention-metrics">
            <div><Calendar size={17} /><span>Fecha de finalización</span><strong>{formatearFecha(ordenTrabajo.fecha_fin || ordenTrabajo.fecha_ingreso)}</strong></div>
            <div><Gauge size={17} /><span>Kilometraje de salida</span><strong>{(ordenTrabajo.kilometraje_salida ?? ordenTrabajo.kilometraje_ingreso ?? 0).toLocaleString()} <small>km</small></strong></div>
            <div><Droplet size={17} /><span>Próximo cambio de aceite</span><strong>{ordenTrabajo.km_proximo !== null && ordenTrabajo.km_proximo !== undefined
              ? `${ordenTrabajo.km_proximo.toLocaleString()} km`
              : ordenTrabajo.proximo_kilometraje
                ? `${ordenTrabajo.proximo_kilometraje.toLocaleString()} km`
                : 'No registrado'}</strong></div>
          </div>

          <div className="fv-attention-people">
            <div><User size={17} /><div><span className="fv-eyebrow">Cliente atendido</span><strong>{nombreClienteAtendido}</strong>{docClienteAtendido && <span className="fv-detail-muted">{tipoDocClienteAtendido}: {docClienteAtendido}</span>}</div></div>
            {ordenTrabajo.nombre_mecanico && <div><UserCheck size={17} /><div><span className="fv-eyebrow">Mecánico a cargo</span><strong>{ordenTrabajo.nombre_mecanico}</strong></div></div>}
          </div>

          {ordenTrabajo.tipo_aceite && <div className="fv-oil-note"><Droplet size={18} /><span>Aceite aplicado <strong>{ordenTrabajo.tipo_aceite}</strong></span></div>}

          <section className="fv-detail-section">
            <h4>Servicios y productos utilizados</h4>
            {(!ordenTrabajo.servicios || ordenTrabajo.servicios.length === 0) && (!ordenTrabajo.insumos || ordenTrabajo.insumos.length === 0) ? (
              <p className="fv-detail-muted">No hay detalle público registrado para esta orden.</p>
            ) : (
              <div className="fv-detail-table-scroll">
                <table className="fv-detail-table">
                  <thead><tr><th scope="col">Descripción</th><th scope="col">Cantidad</th></tr></thead>
                  <tbody>
                    {ordenTrabajo.servicios?.map((s: ServicioOTPublicoDTO) => (
                      <tr key={`srv-${s.id_detalle}`}><td><div className="fv-detail-item"><Wrench size={15} /><div><span>{s.nombre_snapshot}</span><small>Servicio</small></div></div></td><td>{s.cantidad}</td></tr>
                    ))}
                    {ordenTrabajo.insumos?.map((i: InsumoOTPublicoDTO) => (
                      <tr key={`ins-${i.id_detalle}`}><td><div className="fv-detail-item"><Package size={15} /><div><span>{i.nombre_snapshot}</span><small>Producto / insumo</small></div></div></td><td>{i.cantidad_formateada} {i.unidad_medida || ''}</td></tr>
                    ))}
                  </tbody>
                </table>
              </div>
            )}
          </section>

          {(ordenTrabajo.resumen_publico || ordenTrabajo.observaciones) && <section className="fv-work-note"><h4>Resumen del trabajo</h4><p>{ordenTrabajo.resumen_publico || ordenTrabajo.observaciones}</p></section>}
        </div>

        <footer className="fv-dialog-footer"><span className="fv-detail-muted">Información registrada en esta atención.</span><button type="button" onClick={() => setModalAbierto(false)} className="fv-button">Cerrar</button></footer>
      </div>
    </div>
  );
};
