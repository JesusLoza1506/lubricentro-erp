import { useEffect, useRef, useState } from 'react';
import { openUrl } from '@tauri-apps/plugin-opener';
import type {
  CredencialesPos,
  DetalleVentaPos as Detalle,
  DevolucionSolicitud,
  DocumentoPos,
  ReceptorPos,
  ResultadoPos,
} from '../../../types/pos';
import { documentoPos, ejecutarPos, mensajePos, posApi } from '../../../services/posService';
import { cantidad, soles } from '../posUtils';
import { ModalPos } from './ModalPos';
import { DevolucionPos } from './DevolucionPos';
import { ReemisionPos } from './ReemisionPos';
import { TicketPos } from './TicketPos';
export function DetalleVentaPos({
  id,
  revisionExterna,
  usuario,
  ambiente,
  admin,
  cajaPropia,
  empresa,
  onCerrar,
  onOperacion,
  onAviso,
}: {
  id: number;
  revisionExterna?: number;
  usuario: number;
  ambiente: string;
  admin: boolean;
  cajaPropia: boolean;
  empresa: string;
  onCerrar: () => void;
  onOperacion: () => void;
  onAviso: (s: string, t: 'EXITO' | 'ERROR') => void;
}) {
  const [v, Sv] = useState<Detalle | null>(null),
    [error, Serror] = useState(''),
    [revision, Srevision] = useState(0),
    [ocupado, Socupado] = useState(false),
    [devolver, Sdevolver] = useState(false),
    [reemision, Sreemision] = useState<DocumentoPos | null>(null);
  const seq = useRef(0);
  const lock = useRef(false);
  useEffect(() => {
    const n = ++seq.current;
    Sv(null);
    posApi<Detalle>('detalle', { id })
      .then((r) => {
        if (n === seq.current) {
          Sv(r);
          Serror('');
        }
      })
      .catch((e) => {
        if (n === seq.current) Serror(mensajePos(e));
      });
    return () => {
      seq.current++;
    };
  }, [id, revision, revisionExterna]);
  const refrescar = () => {
    Srevision((n) => n + 1);
    onOperacion();
  };
  async function documento(d: DocumentoPos, consulta: boolean) {
    if (lock.current) return;
    lock.current = true;
    Socupado(true);
    try {
      const r = await documentoPos(d.id, consulta);
      onAviso(r.mensaje, r.estado === 'ACEPTADO' ? 'EXITO' : 'ERROR');
      refrescar();
    } catch (e) {
      Serror(mensajePos(e));
    } finally {
      lock.current = false;
      Socupado(false);
    }
  }
  async function despues(r: ResultadoPos, mensaje: string) {
    onAviso(mensaje, 'EXITO');
    refrescar();
    try {
      await documentoPos(r.id_documento, false, true,r.token_envio);
      refrescar();
    } catch (e) {
      onAviso(`Operación guardada. El documento sigue pendiente: ${mensajePos(e)}`, 'ERROR');
    }
  }
  async function devolverVenta(p: DevolucionSolicitud, c?: CredencialesPos) {
    const r = await ejecutarPos(
      usuario,
      ambiente,
      'devolver',
      p as unknown as Record<string, unknown>,
      c
    );
    Sdevolver(false);
    await despues(r, 'Devolución registrada correctamente.');
  }
  async function reemitir(r: ReceptorPos, motivo: string) {
    if (!reemision) return;
    const resultado = await ejecutarPos(usuario, ambiente, 'reemitir', {
      id_operacion: crypto.randomUUID(),
      id_documento: reemision.id,
      receptor: r,
      motivo,
    });
    Sreemision(null);
    await despues(resultado, 'Nuevo documento creado sin repetir el cobro.');
  }
  const vigentes = v?.documentos.filter((d) => !d.reemplazado) ?? [];
  const original = vigentes.find((d) => d.relacion === 'ORIGINAL' || d.relacion === 'REEMISION');
  const habilitada =
    !!v &&
    cajaPropia &&
    v.devuelto_cent < v.total_cent &&
    original?.estado === 'ACEPTADO' &&
    vigentes
      .filter((d) => d.relacion === 'NOTA_CREDITO' || d.relacion === 'REEMISION_NC')
      .every((d) => d.estado === 'ACEPTADO');
  return (
    <>
      <ModalPos titulo={`Venta V-${id}`} onCerrar={onCerrar} ocupado={ocupado} ancho>
        {error && (
          <div className="pos-error" role="alert">
            {error}
            <button onClick={() => Srevision((n) => n + 1)}>Actualizar detalle</button>
          </div>
        )}
        {!v ? (
          <p className="pos-empty">
            {error ? 'No se pudo cargar la venta.' : 'Consultando venta…'}
          </p>
        ) : (
          <>
            <div className="pos-detail-summary">
              <div>
                <span className="pos-badge" data-state={v.devuelto_cent ? 'DEVUELTA' : 'VIGENTE'}>
                  {v.devuelto_cent === v.total_cent
                    ? 'Devuelta total'
                    : v.devuelto_cent
                      ? 'Devuelta parcial'
                      : 'Vigente'}
                </span>
                <h3>{v.receptor.nombre}</h3>
                <small>
                  {v.fecha} · {v.emisor} · {v.caja}
                </small>
              </div>
              <strong>{soles(v.total_cent)}</strong>
            </div>
            <div className="pos-table-wrap" role="region" aria-label="Productos y servicios de la venta" tabIndex={0}>
              <table>
                <thead>
                  <tr>
                    <th>Detalle</th>
                    <th>Origen</th>
                    <th>Cantidad</th>
                    <th className="pos-money">Importe</th>
                  </tr>
                </thead>
                <tbody>
                  {v.lineas.map((l) => (
                    <tr key={l.id_detalle}>
                      <td>
                        {l.nombre}
                        <small>{l.codigo}</small>
                      </td>
                      <td>{l.origen}</td>
                      <td>{cantidad(l.cantidad, l.unidad)}</td>
                      <td className="pos-money">{soles(l.total_cent)}</td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
            <div className="pos-payment-summary">
              {v.pagos.map((p, i) => (
                <div key={i}>
                  <strong>
                    {p.medio} · {soles(p.monto_cent)}
                  </strong>
                  {p.vuelto_cent != null && (
                    <small>
                      Recibido {soles(p.recibido_cent ?? 0)} · Vuelto {soles(p.vuelto_cent)}
                    </small>
                  )}
                </div>
              ))}
              <div>
                <strong>Devuelto: {soles(v.devuelto_cent)}</strong>
                <small>Pendiente: {soles(v.total_cent - v.devuelto_cent)}</small>
              </div>
            </div>
            <div className="pos-panel-heading">
              <h3>Documentos vinculados</h3>
              <button disabled={ocupado} onClick={() => Srevision((n) => n + 1)}>
                Actualizar
              </button>
            </div>
            {v.documentos.map((d) => (
              <article className="pos-document" key={d.id}>
                <div>
                  <strong>
                    {d.serie}-{d.numero} <span className="pos-badge" data-state={d.estado}>{d.estado}</span>
                  </strong>
                  <small>
                    {d.tipo} · {d.relacion}
                    {d.reemplazado ? ' · Reemplazado' : ''}
                  </small>
                  <p>{d.mensaje ?? 'Documento guardado localmente; pendiente de envío.'}</p>
                </div>
                <div className="pos-document-actions">
                  {(['pdf', 'xml', 'cdr'] as const).map(
                    (k) =>
                      d[k] && (
                        <button
                          key={k}
                          onClick={() => {
                            const url = d[k];
                            if (url?.startsWith('https://'))
                              void openUrl(url).catch((e) => Serror(mensajePos(e)));
                          }}
                        >
                          {k.toUpperCase()} oficial
                        </button>
                      )
                  )}
                  {admin && !d.reemplazado && !['ACEPTADO', 'RECHAZADO'].includes(d.estado) && (
                    <button
                      disabled={ocupado}
                      onClick={() =>
                        void documento(
                          d,
                          !!(d.requiere_consulta || d.intento || d.incierto) ||
                            d.estado === 'ENVIANDO'
                        )
                      }
                    >
                      {d.requiere_consulta || d.intento || d.incierto || d.estado === 'ENVIANDO'
                        ? 'Consultar estado'
                        : 'Enviar'}
                    </button>
                  )}
                  {admin && !d.reemplazado && d.estado === 'RECHAZADO' && (
                    <button disabled={ocupado} onClick={() => Sreemision(d)}>
                      Corregir y reemitir
                    </button>
                  )}
                </div>
              </article>
            ))}
            {v.devoluciones.length > 0 && (
              <>
                <h3>Devoluciones</h3>
                {v.devoluciones.map((d) => (
                  <p key={d.id}>
                    {d.fecha} · {soles(d.total_cent)} · {d.motivo}
                  </p>
                ))}
              </>
            )}
            <footer className="pos-modal-footer">
              <button onClick={() => window.print()}>Reimprimir ticket</button>
              <button disabled={!habilitada || ocupado} onClick={() => Sdevolver(true)}>
                Devolver
              </button>
              <button className="pos-primary" onClick={onCerrar} disabled={ocupado}>
                Listo
              </button>
            </footer>
            {!habilitada && v.devuelto_cent < v.total_cent && (
              <p className="pos-hint">
                Para devolver necesitas caja propia, original aceptado y notas anteriores aceptadas.
              </p>
            )}
            <TicketPos venta={v} empresa={empresa} />
          </>
        )}
      </ModalPos>
      {devolver && v && (
        <DevolucionPos
          venta={v}
          cajero={!admin}
          onCerrar={() => Sdevolver(false)}
          onConfirmar={devolverVenta}
        />
      )}{' '}
      {reemision && v && (
        <ReemisionPos
          documento={reemision}
          receptor={v.receptor}
          onCerrar={() => Sreemision(null)}
          onConfirmar={reemitir}
        />
      )}
    </>
  );
}
