import { useEffect, useState } from 'react';
import { createPortal } from 'react-dom';
import { Printer } from 'lucide-react';
import type { CajaDetalle } from '../../../types/caja';
import { soles } from '../../pos/posUtils';
import { fechaCaja } from './CajaConsultas';

const medios = [
  ['tarjeta', 'Tarjeta'],
  ['yape', 'Yape'],
  ['plin', 'Plin'],
  ['transferencia', 'Transferencia'],
] as const;
const tipoCierre = { NORMAL: 'Normal', AUSENCIA: 'Por ausencia', RECUPERACION: 'Por recuperación' };
const monto = (n: number | null) => (n == null ? '—' : soles(n));

export function CajaResumenTurno({
  detalle,
  onAclarar,
}: {
  detalle: CajaDetalle;
  onAclarar: () => void;
}) {
  const { turno, resumen } = detalle;
  const [imprimir, setImprimir] = useState(false);
  const [error, setError] = useState('');
  useEffect(() => {
    if (!imprimir) return;
    const anterior = document.activeElement as HTMLElement | null;
    document.body.classList.add('caja-imprimiendo');
    const finalizar = () => {
      document.body.classList.remove('caja-imprimiendo');
      setImprimir(false);
      anterior?.focus();
    };
    window.addEventListener('afterprint', finalizar);
    const cuadro = requestAnimationFrame(() => {
      try {
        window.print();
      } catch {
        setError('No se pudo abrir la impresión. Intenta imprimir el cierre nuevamente.');
        finalizar();
      }
    });
    return () => {
      cancelAnimationFrame(cuadro);
      window.removeEventListener('afterprint', finalizar);
      document.body.classList.remove('caja-imprimiendo');
    };
  }, [imprimir]);

  const cifrasCierre = turno.estado === 'CERRADA' && (
    <>
      <dl className="caja-cifras">
        <div>
          <dt>Esperado</dt>
          <dd>{monto(turno.efectivo_esperado_cent)}</dd>
        </div>
        <div>
          <dt>Contado</dt>
          <dd>{monto(turno.efectivo_contado_cent)}</dd>
        </div>
        <div>
          <dt>Diferencia</dt>
          <dd>{monto(turno.diferencia_efectivo_cent)}</dd>
        </div>
        <div>
          <dt>Fondo siguiente</dt>
          <dd>{monto(turno.fondo_siguiente_cent)}</dd>
        </div>
        <div>
          <dt>Retiro de cierre</dt>
          <dd>{monto(turno.retiro_cierre_cent)}</dd>
        </div>
      </dl>
      <p>
        {turno.tipo_cierre && tipoCierre[turno.tipo_cierre]} · Cerró {turno.quien_cerro}{' '}
        {turno.fecha_cierre && `· ${fechaCaja(turno.fecha_cierre)}`}
      </p>
      {turno.motivo_cierre && (
        <p>
          <strong>Motivo:</strong> {turno.motivo_cierre}
        </p>
      )}
      {turno.aclaracion && (
        <p>
          <strong>Aclaración:</strong> {turno.aclaracion}
          <br />
          {turno.autor_aclaracion}{' '}
          {turno.fecha_aclaracion && `· ${fechaCaja(turno.fecha_aclaracion)}`}
        </p>
      )}
    </>
  );
  const digitales = (
    <table>
      <thead>
        <tr>
          <th>Medio</th>
          <th>Cobros</th>
          <th>Reintegros</th>
          <th>Neto</th>
        </tr>
      </thead>
      <tbody>
        {medios.map(([k, nombre]) => (
          <tr key={k}>
            <th scope="row">{nombre}</th>
            <td>{soles(resumen[k].cobros_cent)}</td>
            <td>{soles(resumen[k].reintegros_cent)}</td>
            <td>{soles(resumen[k].neto_cent)}</td>
          </tr>
        ))}
      </tbody>
    </table>
  );
  return (
    <>
      <div className="caja-resumen-grid">
        <section className="caja-panel">
          <h2>Efectivo del turno</h2>
          <strong className="caja-total">{soles(resumen.efectivo_esperado_cent)}</strong>
          <p>Efectivo esperado</p>
          <dl className="caja-desglose">
            <div>
              <dt>Fondo de apertura</dt>
              <dd>{soles(turno.monto_apertura_cent)}</dd>
            </div>
            <div>
              <dt>Cobros</dt>
              <dd>{soles(resumen.efectivo.cobros_cent)}</dd>
            </div>
            <div>
              <dt>Reintegros</dt>
              <dd>{soles(resumen.efectivo.reintegros_cent)}</dd>
            </div>
            <div>
              <dt>Ingresos manuales</dt>
              <dd>{soles(resumen.ingresos_manuales_cent)}</dd>
            </div>
            <div>
              <dt>Egresos manuales</dt>
              <dd>{soles(resumen.egresos_manuales_cent)}</dd>
            </div>
          </dl>
        </section>
        <section className="caja-panel">
          <h2>Pagos digitales</h2>
          <p>No forman parte del efectivo que se cuenta al cerrar.</p>
          <div className="caja-tabla">{digitales}</div>
        </section>
      </div>
      {cifrasCierre && (
        <section className="caja-panel">
          <div className="caja-linea">
            <h2>Arqueo guardado</h2>
            <div className="caja-acciones">
              {detalle.puede_aclarar && (
                <button onClick={onAclarar}>
                  {turno.aclaracion ? 'Editar aclaración' : 'Agregar aclaración'}
                </button>
              )}
              <button
                disabled={imprimir}
                onClick={() => {
                  setError('');
                  setImprimir(true);
                }}
              >
                <Printer size={16} /> Imprimir cierre
              </button>
            </div>
          </div>
          {cifrasCierre}
          {error && (
            <p className="caja-alerta" role="alert">
              {error}
            </p>
          )}
        </section>
      )}
      {imprimir &&
        createPortal(
          <article className="caja-impresion">
            <h1>Lubricentro Gloria · Cierre de caja</h1>
            <p>
              {turno.codigo_caja} · {turno.ambiente}
            </p>
            <p>
              {turno.titular} · Apertura {fechaCaja(turno.fecha_apertura)} · Fondo inicial{' '}
              {soles(turno.monto_apertura_cent)}
            </p>
            {cifrasCierre}
            <h2>Medios digitales</h2>
            {digitales}
          </article>,
          document.body
        )}
    </>
  );
}
