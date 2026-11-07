import React, { useEffect, useRef, useState } from 'react';
import { OrigenKardexDTO } from '../../../types/inventario';
import { inventarioService } from '../../../services/inventarioService';

// Seis decimales conservan los microsoles originales, sin redondear a céntimos.
export const dineroMicro = (valor: number) => {
  const micros = BigInt(valor);
  const absoluto = micros < 0n ? -micros : micros;
  return `${micros < 0n ? '-' : ''}S/ ${(absoluto / 1_000_000n).toLocaleString('es-PE')}.${(absoluto % 1_000_000n).toString().padStart(6, '0')}`;
};
export const cantidadKardex = (valor: number, unidad: string) =>
  unidad === 'ML'
    ? `${(valor / 1000).toLocaleString('es-PE', { minimumFractionDigits: 3, maximumFractionDigits: 3 })} L`
    : `${valor} UND`;

export const ModalOrigenKardex: React.FC<{ idMovimiento: number; onClose: () => void }> = ({
  idMovimiento,
  onClose,
}) => {
  const [origen, setOrigen] = useState<OrigenKardexDTO | null>(null);
  const [error, setError] = useState('');
  const [cargando, setCargando] = useState(false);
  const [intento, setIntento] = useState(0);
  const consulta = useRef(0);
  useEffect(() => {
    const actual = ++consulta.current;
    setOrigen(null);
    setError('');
    setCargando(true);
    inventarioService
      .obtenerOrigenKardex(idMovimiento)
      .then((data) => {
        if (actual === consulta.current) setOrigen(data);
      })
      .catch((err: unknown) => {
        if (actual === consulta.current)
          setError(
            typeof err === 'string'
              ? err
              : err instanceof Error
                ? err.message
                : 'No se pudo consultar el origen.'
          );
      })
      .finally(() => {
        if (actual === consulta.current) setCargando(false);
      });
    return () => {
      consulta.current += 1;
    };
  }, [idMovimiento, intento]);
  return (
    <div
      className="gloria-theme inv-overlay"
      style={{
        position: 'fixed',
        inset: 0,
        background: 'rgba(0,0,0,.75)',
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center',
        zIndex: 1250,
        padding: 16,
      }}
    >
      <section
        className="inv-dialog inv-dialog-wide inv-origin-dialog"
        role="dialog"
        aria-modal="true"
        aria-label="Origen del movimiento"
        style={{
          background: '#0F172A',
          color: '#F8FAFC',
          border: '1px solid #334155',
          borderRadius: 12,
          padding: 24,
          width: 700,
          maxWidth: '100%',
          maxHeight: '85vh',
          overflow: 'auto',
        }}
      >
        <header className="inv-dialog-header">
          <div><span className="inv-dialog-kicker">Kardex · Consulta</span>
            <h3>Origen del movimiento{origen ? ` · ${origen.codigo}` : ''}</h3></div>
        </header>
        {cargando && <p role="status">Consultando origen…</p>}
        {error && (
          <div role="alert" className="inv-form-error">
            <p style={{ color: '#F87171' }}>{error}</p>
            <button className="inv-button" onClick={() => setIntento((n) => n + 1)}>Reintentar</button>
          </div>
        )}
        {origen && (
          <>
            <p>
              {origen.tipo === 'OT' ? 'Orden de trabajo' : origen.tipo} · {origen.estado}
            </p>
            <p style={{ color: '#CBD5E1' }}>
              Fecha: {origen.fecha}
              <br />
              Referencia: {origen.documento || 'Sin referencia adicional'}
            </p>
            {origen.resumen && <p style={{ whiteSpace: 'pre-wrap' }}>{origen.resumen}</p>}
            <p style={{ color: '#94A3B8', fontSize: 12 }}>
              Consulta del registro de origen y sus materiales históricos. No modifica la operación.
            </p>
            <div className="inv-table-scroll inv-origin-scroll" tabIndex={0} role="region" aria-label="Materiales del movimiento">
            <table className="inv-table">
              <thead>
                <tr>
                  <th style={{ textAlign: 'left' }}>Producto / servicio</th>
                  <th>Cantidad registrada</th>
                  <th>Costo histórico</th>
                </tr>
              </thead>
              <tbody>
                {origen.lineas.map((l, i) => (
                  <tr key={i} style={{ borderTop: '1px solid #334155' }}>
                    <td style={{ padding: '10px 0' }}>{l.nombre}</td>
                    <td style={{ textAlign: 'center' }}>
                      {cantidadKardex(l.cantidad, l.unidad_medida)}
                    </td>
                    <td style={{ textAlign: 'right' }}>
                      {l.valor_micro == null ? 'No aplica' : dineroMicro(l.valor_micro)}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
            </div>
            {origen.comprobantes.length > 0 && (
              <div>
                <h4>Comprobantes vinculados</h4>
                {origen.comprobantes.map((c) => (
                  <details key={c.id_comprobante} style={{ marginBottom: 10 }}>
                    <summary style={{ cursor: 'pointer', color: '#38BDF8' }}>
                      {c.tipo_comprobante} {c.numero}
                    </summary>
                    <p>
                      Estado de envío: {c.estado_envio}
                      <br />
                      Total: S/ {(c.total_cent / 100).toFixed(2)}
                    </p>
                  </details>
                ))}
              </div>
            )}
          </>
        )}
        <footer className="inv-form-footer">
          <button className="inv-button" type="button" onClick={onClose}>
            Cerrar origen
          </button>
        </footer>
      </section>
    </div>
  );
};
