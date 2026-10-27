import React from 'react';
import { TableroZanjasDTO, OrdenTrabajoHistorialDTO } from '../../../types/ordenTrabajo';
import { Wrench, Clock3, ArrowRight, Car, User, Gauge } from 'lucide-react';

interface TableroZanjasProps {
  tablero: TableroZanjasDTO;
  onVerDetalle: (ot: OrdenTrabajoHistorialDTO) => void;
  onIniciarEnZanja: (idOt: number, idZanja: number) => void;
}

type ZanjaItemDTO = TableroZanjasDTO['zanjas'][number];

export const TableroZanjas: React.FC<TableroZanjasProps> = ({
  tablero,
  onVerDetalle,
  onIniciarEnZanja,
}) => {
  // Conservar únicamente las zanjas habilitadas por la configuración existente.
  const zanjasHabilitadas = tablero.zanjas.filter((z: ZanjaItemDTO) => z.activa === 1);

  return (
    <div className="ot-workshop">
      <section className="ot-bays" aria-labelledby="ot-bays-title">
        <div className="ot-section-heading"><div><h2 id="ot-bays-title"><Wrench size={18} />Zanjas de trabajo <span>({zanjasHabilitadas.length})</span></h2><p>Consulta las atenciones en ejecución y los espacios disponibles.</p></div><span className="ot-muted">{zanjasHabilitadas.length} habilitadas</span></div>
        <div className="ot-bays-grid">
          {zanjasHabilitadas.map((z: ZanjaItemDTO) => {
            const ot = z.ot_actual;
            const estaOcupada = z.ocupada || Boolean(ot);
            return (
              <article key={z.id_zanja} className="ot-bay" data-occupied={estaOcupada ? 'true' : 'false'}>
                <header className="ot-bay-heading"><h3>Zanja {z.id_zanja}</h3><span className="ot-status" data-state={estaOcupada ? 'EN_PROCESO' : 'LIBRE'}>{estaOcupada ? 'Ocupada' : 'Disponible'}</span></header>
                {estaOcupada && ot ? (
                  <div className="ot-bay-content">
                    <div className="ot-bay-vehicle"><span><Car size={18} /><strong>{ot.placa}</strong></span><span className="ot-order-code">{ot.codigo_ot}</span></div>
                    <dl className="ot-bay-data"><div><dt><User size={14} />Cliente</dt><dd>{ot.cliente_nombre || 'No especificado'}</dd></div><div><dt><Wrench size={14} />Mecánico</dt><dd>{ot.nombre_mecanico || 'En ejecución'}</dd></div><div><dt><Gauge size={14} />Ingreso</dt><dd>{ot.kilometraje_ingreso.toLocaleString()} km</dd></div></dl>
                    <button className="ot-button ot-open ot-bay-open" onClick={() => onVerDetalle(ot)}>Abrir atención<ArrowRight size={15} /></button>
                  </div>
                ) : (
                  <div className="ot-bay-empty"><span className="ot-free-icon"><Car size={23} /></span><strong>Zanja disponible</strong><p>Lista para iniciar una atención.</p>
                    {tablero.cola_espera.length > 0 && <label className="ot-field"><span>Iniciar con un vehículo de la cola</span><select onChange={(e) => {
                      const val = Number(e.target.value);
                      if (val > 0) onIniciarEnZanja(val, z.id_zanja);
                    }} defaultValue={0}><option value={0}>Seleccionar vehículo…</option>{tablero.cola_espera.map((colaOt: OrdenTrabajoHistorialDTO) => <option key={colaOt.id_ot} value={colaOt.id_ot}>{colaOt.placa} ({colaOt.codigo_ot})</option>)}</select></label>}
                  </div>
                )}
              </article>
            );
          })}
        </div>
        {zanjasHabilitadas.length === 0 && <div className="ot-empty">No hay zanjas habilitadas.</div>}
      </section>
      <section className="ot-queue" aria-labelledby="ot-queue-title">
        <div className="ot-section-heading"><div><h2 id="ot-queue-title"><Clock3 size={18} />Cola de espera <span>({tablero.cola_espera.length})</span></h2><p>Vehículos pendientes de iniciar su atención.</p></div></div>
        {tablero.cola_espera.length === 0 ? <div className="ot-empty">No hay vehículos esperando atención en cola.</div> : (
          <div className="ot-table-scroll" tabIndex={0} role="region" aria-label="Vehículos en espera">
            <table className="ot-table ot-queue-table">
              <thead><tr><th scope="col">Vehículo</th><th scope="col">Código OT</th><th scope="col">Cliente</th><th scope="col">Mecánico asignado</th><th scope="col" className="ot-numeric">Km de ingreso</th><th scope="col">Atención</th></tr></thead>
              <tbody>{tablero.cola_espera.map((ot: OrdenTrabajoHistorialDTO) => <tr key={ot.id_ot}><td><strong className="ot-table-plate">{ot.placa}</strong></td><td><span className="ot-order-code">{ot.codigo_ot}</span></td><td>{ot.cliente_nombre || 'No especificado'}</td><td><span className={ot.nombre_mecanico ? undefined : 'ot-unassigned'}>{ot.nombre_mecanico || 'Sin asignar'}</span></td><td className="ot-numeric">{ot.kilometraje_ingreso.toLocaleString()} km</td><td><button className="ot-button ot-small ot-open" onClick={() => onVerDetalle(ot)}>Abrir atención<ArrowRight size={14} /></button></td></tr>)}</tbody>
            </table>
          </div>
        )}
      </section>
    </div>
  );
};
