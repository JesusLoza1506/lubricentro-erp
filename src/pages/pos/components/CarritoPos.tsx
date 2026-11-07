import { Trash2, FileCheck2, CreditCard } from 'lucide-react';
import type { LineaPos, OtPos, TotalesPos } from '../../../types/pos';
import { cantidad, soles } from '../posUtils';
export function CarritoPos({
  lineas,
  totales,
  cargando,
  error,
  ot,
  otContexto,
  descuento,
  motivo,
  onDescuento,
  onMotivo,
  onCantidad,
  onQuitar,
  onVaciar,
  onCobrar,
  puedeCobrar,
  bloqueoCobro,
  children,
}: {
  lineas: LineaPos[];
  totales: TotalesPos | null;
  cargando: boolean;
  error: string;
  ot: number | null;
  otContexto: OtPos | null;
  descuento: string;
  motivo: string;
  onDescuento: (s: string) => void;
  onMotivo: (s: string) => void;
  onCantidad: (indice: number, s: string) => void;
  onQuitar: (indice: number) => void;
  onVaciar: () => void;
  onCobrar: () => void;
  puedeCobrar: boolean;
  bloqueoCobro?: string;
  children: React.ReactNode;
}) {
  return (
    <aside className="pos-panel pos-cart" aria-labelledby="pos-cart-title">
      <div className="pos-panel-heading">
        <div>
          <h2 id="pos-cart-title" tabIndex={-1}>Venta actual <span className="pos-count" title="Líneas de venta">{lineas.length}</span></h2>
          <p>
            {otContexto
              ? `${otContexto.codigo} · ${otContexto.placa}`
              : ot ? `OT #${ot} + adicionales` : 'Venta directa'}
          </p>
        </div>
        <button
          className="pos-icon"
          aria-label="Vaciar venta"
          disabled={!lineas.length}
          onClick={onVaciar}
        >
          <Trash2 size={17} />
        </button>
      </div>
      <div className="pos-cart-scroll" role="region" aria-label="Detalle de la venta actual" tabIndex={0}>
        <div className="pos-cart-lines">
          {!lineas.length ? (
            <div className="pos-empty">
              <FileCheck2 size={24} aria-hidden="true" />
              <p>Tu venta comienza aquí</p>
              <small>Agrega un producto o carga una orden finalizada.</small>
            </div>
          ) : (
            lineas.map((l, i) => {
              const historica = l.id_consumo_ot != null || l.id_servicio_ot != null;
              return (
                <div className="pos-cart-line" key={`${l.codigo}-${i}`}>
                  <div>
                    <strong>{l.nombre}</strong>
                    <small>
                      {historica ? 'OT · precio histórico' : l.codigo} · {soles(l.precio_cent)} /{' '}
                      {l.unidad === 'ML' ? 'L' : 'UND'}
                    </small>
                  </div>
                  <div className="pos-line-controls">
                    {historica ? (
                      <span className="pos-badge">{cantidad(l.cantidad, l.unidad)}</span>
                    ) : (
                      <input
                        key={`${l.codigo}-${l.cantidad}`}
                        aria-label={`Cantidad ${l.nombre}`}
                        type="text"
                        inputMode="decimal"
                        defaultValue={String(l.unidad === 'ML' ? l.cantidad / 1000 : l.cantidad)}
                        onBlur={(e) => onCantidad(i, e.target.value)}
                        onKeyDown={(e) => {
                          if (e.key === 'Enter') e.currentTarget.blur();
                        }}
                      />
                    )}
                    <strong>{soles(l.total_cent)}</strong>
                    {!historica && (
                      <button
                        className="pos-icon"
                        aria-label={`Quitar ${l.nombre}`}
                        onClick={() => onQuitar(i)}
                      >
                        <Trash2 size={14} />
                      </button>
                    )}
                  </div>
                </div>
              );
            })
          )}
        </div>
        {children}
        <div className="pos-discount">
          <label>
            Descuento (%)
            <input
              inputMode="decimal"
              value={descuento}
              onChange={(e) => onDescuento(e.target.value)}
            />
          </label>
          {Number(descuento.replace(',', '.')) > 5 && (
            <label>
              Motivo del descuento
              <textarea
                minLength={5}
                maxLength={250}
                value={motivo}
                onChange={(e) => onMotivo(e.target.value)}
              />
            </label>
          )}
        </div>
        {error && (
          <p className="pos-error" role="alert">
            {error}
          </p>
        )}
        <details className="pos-breakdown">
          <summary>Ver desglose del total</summary>
          <div className="pos-totals">
            <div>
              <span>Bruto</span>
              <span>{soles(totales?.bruto_cent ?? 0)}</span>
            </div>
            <div>
              <span>Descuento</span>
              <span>− {soles(totales?.descuento_cent ?? 0)}</span>
            </div>
            <div>
              <span>Base gravada</span>
              <span>{soles(totales?.base_cent ?? 0)}</span>
            </div>
            <div>
              <span>Impuestos incluidos</span>
              <span>{soles(totales?.igv_cent ?? 0)}</span>
            </div>
          </div>
        </details>
      </div>
      <footer className="pos-checkout">
        <div className="pos-total">
          <span>Total a cobrar</span>
          <strong>{soles(totales?.total_cent ?? 0)}</strong>
        </div>
        {bloqueoCobro && <p className="pos-checkout-hint" id="pos-checkout-reason">{bloqueoCobro}</p>}
        <button
          type="button"
          className="pos-primary pos-pay"
          disabled={!puedeCobrar || cargando || !totales || !!error}
          aria-describedby={bloqueoCobro ? 'pos-checkout-reason' : undefined}
          aria-busy={cargando}
          onClick={onCobrar}
        >
          <CreditCard size={16} aria-hidden="true" />
          {cargando ? 'Calculando…' : 'Cobrar venta'}
        </button>
      </footer>
    </aside>
  );
}
