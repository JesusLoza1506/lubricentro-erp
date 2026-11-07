import type { DetalleVentaPos } from '../../../types/pos';
import { cantidad, soles } from '../posUtils';
export function TicketPos({ venta, empresa }: { venta: DetalleVentaPos; empresa: string }) {
  const original = venta.documentos
    .filter((d) => ['ORIGINAL', 'REEMISION'].includes(d.relacion) && !d.reemplazado)
    .slice(-1)[0];
  return (
    <div className="pos-ticket">
      <h2>{empresa}</h2>
      <p>REGISTRO LOCAL DE VENTA · V-{venta.id}</p>
      <p>
        {venta.fecha} · {venta.emisor}
      </p>
      <p>
        {venta.receptor.nombre}
        <br />
        {venta.receptor.documento}
      </p>
      <p>
        {original?.serie}-{original?.numero} · Estado: {original?.estado}
      </p>
      <hr />
      {venta.lineas.map((l, i) => (
        <p key={i}>
          {l.nombre}
          <br />
          {cantidad(l.cantidad, l.unidad)} · {soles(l.total_cent)}
        </p>
      ))}
      <hr />
      <p>Descuento: {soles(venta.descuento_cent)}</p>
      <strong>TOTAL: {soles(venta.total_cent)}</strong>
      {venta.pagos.map((p, i) => (
        <p key={i}>
          {p.medio}: {soles(p.monto_cent)}
          {p.vuelto_cent != null && ` · Vuelto: ${soles(p.vuelto_cent)}`}
        </p>
      ))}
      <p>Devuelto: {soles(venta.devuelto_cent)}</p>
      <small>
        Este ticket es un registro local. Los archivos oficiales y su aceptación se consultan en el
        comprobante electrónico.
      </small>
    </div>
  );
}
