import { useEffect, useRef, useState } from 'react';
import type { CredencialesPos, MedioPago, PagoPos, TotalesPos } from '../../../types/pos';
import { mensajePos } from '../../../services/posService';
import { AutorizacionPos } from './AutorizacionPos';
import { ModalPos } from './ModalPos';
import { MEDIOS, construirPagos, soles } from '../posUtils';
export function CobroPos({
  totales,
  requiereAdmin,
  onCerrar,
  onConfirmar,
}: {
  totales: TotalesPos;
  requiereAdmin: boolean;
  onCerrar: () => void;
  onConfirmar: (p: PagoPos[], c?: CredencialesPos) => Promise<void>;
}) {
  const [medio, Smedio] = useState<MedioPago>('EFECTIVO'),
    [segundo, Ssegundo] = useState<MedioPago | null>(null),
    [importe, Simporte] = useState(''),
    [recibido, Srecibido] = useState((totales.total_cent / 100).toFixed(2)),
    [referencia, Sreferencia] = useState(''),
    [cred, Scred] = useState({ username: '', password: '' }),
    [ocupado, Socupado] = useState(false),
    [error, Serror] = useState('');
  const lock = useRef(false);
  const recibidoEditado = useRef(false);
  const parte = Number(importe.replace(',', '.')) * 100 || 0;
  const principal = totales.total_cent - (segundo ? Math.round(parte) : 0);
  const efectivo =
    medio === 'EFECTIVO' ? principal : segundo === 'EFECTIVO' ? Math.round(parte) : 0;
  useEffect(() => {
    if (!recibidoEditado.current) Srecibido((Math.max(0, efectivo) / 100).toFixed(2));
  }, [efectivo]);
  const cerrar = () => {
    const modificado =
      medio !== 'EFECTIVO' ||
      segundo !== null ||
      referencia !== '' ||
      cred.username !== '' ||
      cred.password !== '' ||
      recibido !== (totales.total_cent / 100).toFixed(2);
    if (
      !ocupado &&
      (!modificado ||
        window.confirm('¿Descartar los datos de este cobro? La venta permanecerá en el carrito.'))
    )
      onCerrar();
  };
  async function confirmar() {
    if (lock.current) return;
    try {
      const p = construirPagos(totales.total_cent, medio, segundo, importe, recibido, referencia);
      lock.current = true;
      Socupado(true);
      Serror('');
      await onConfirmar(p, requiereAdmin ? cred : undefined);
    } catch (e) {
      Serror(mensajePos(e));
    } finally {
      lock.current = false;
      Socupado(false);
      Scred((c) => ({ ...c, password: '' }));
    }
  }
  return (
    <ModalPos captura titulo="Confirmar cobro" onCerrar={cerrar} ocupado={ocupado}>
      <form
        onSubmit={(e) => {
          e.preventDefault();
          void confirmar();
        }}
      >
        <div className="pos-amount">
          <span>Total a cobrar</span>
          <strong>{soles(totales.total_cent)}</strong>
          <small>Incluye impuestos · PEN</small>
        </div>
        <div className="pos-fields">
          <label>
            Medio principal
            <select value={medio} onChange={(e) => Smedio(e.target.value as MedioPago)}>
              {MEDIOS.map((m) => (
                <option key={m}>{m}</option>
              ))}
            </select>
          </label>
          <label>
            Importe aplicado
            <input readOnly value={soles(principal)} />
          </label>
        </div>
        <label className="pos-check">
          <input
            type="checkbox"
            checked={segundo !== null}
            onChange={(e) => {
              Ssegundo(e.target.checked ? (medio === 'YAPE' ? 'TARJETA' : 'YAPE') : null);
              Simporte('');
            }}
          />
          Dividir el pago entre dos medios
        </label>
        {segundo && (
          <div className="pos-fields">
            <label>
              Segundo medio
              <select value={segundo} onChange={(e) => Ssegundo(e.target.value as MedioPago)}>
                {MEDIOS.filter((m) => m !== medio).map((m) => (
                  <option key={m}>{m}</option>
                ))}
              </select>
            </label>
            <label>
              Importe del segundo medio (S/)
              <input
                required
                inputMode="decimal"
                value={importe}
                onChange={(e) => Simporte(e.target.value)}
              />
            </label>
          </div>
        )}
        {efectivo > 0 && (
          <div className="pos-fields">
            <label>
              Efectivo recibido (S/)
              <input
                required
                inputMode="decimal"
                value={recibido}
                onChange={(e) => {
                  recibidoEditado.current = true;
                  Srecibido(e.target.value);
                }}
              />
            </label>
            <label>
              Vuelto
              <input
                readOnly
                value={soles(
                  Math.max(0, Math.round(Number(recibido.replace(',', '.')) * 100) - efectivo)
                )}
              />
            </label>
          </div>
        )}
        <label>
          Referencia de pago · opcional
          <input maxLength={100} value={referencia} onChange={(e) => Sreferencia(e.target.value)} />
        </label>
        <p className="pos-hint">
          Comprueba los pagos digitales antes de confirmar. El sistema no consulta bancos ni
          billeteras.
        </p>
        <AutorizacionPos necesaria={requiereAdmin} valor={cred} onChange={Scred} />
        {error && (
          <p role="alert" className="pos-error">
            {error}
          </p>
        )}
        <footer className="pos-modal-footer">
          <button type="button" disabled={ocupado} onClick={cerrar}>
            Volver
          </button>
          <button className="pos-primary" disabled={ocupado} type="submit">
            {ocupado ? 'Registrando cobro…' : 'Confirmar cobro'}
          </button>
        </footer>
      </form>
    </ModalPos>
  );
}
