import { useEffect, useRef, useState } from 'react';
import type {
  CredencialesPos,
  DetalleVentaPos,
  DevolucionSolicitud,
  MedioPago,
} from '../../../types/pos';
import { mensajePos, posApi } from '../../../services/posService';
import { cantidad, decimalBase, MEDIOS, soles } from '../posUtils';
import { ModalPos } from './ModalPos';
import { AutorizacionPos } from './AutorizacionPos';
export function DevolucionPos({
  venta,
  cajero,
  onCerrar,
  onConfirmar,
}: {
  venta: DetalleVentaPos;
  cajero: boolean;
  onCerrar: () => void;
  onConfirmar: (p: DevolucionSolicitud, c?: CredencialesPos) => Promise<void>;
}) {
  const [capturas, Scapturas] = useState<Record<number, { cantidad: string; recuperada: string }>>(
      {}
    ),
    [motivo, Smotivo] = useState(''),
    [medio, Smedio] = useState<MedioPago>('EFECTIVO'),
    [referencia, Sreferencia] = useState(''),
    [cred, Scred] = useState({ username: '', password: '' }),
    [error, Serror] = useState(''),
    [ocupado, Socupado] = useState(false);
  const lock = useRef(false);
  const lineas = venta.lineas.filter((l) => l.cantidad > (l.devuelta ?? 0));
  const [importe, Simporte] = useState<number | null>(null);
  useEffect(() => {
    let vivo = true;
    Simporte(null);
    const t = setTimeout(() => {
      try {
        const ls = venta.lineas.flatMap((l) => {
          const c = capturas[l.id_detalle!];
          if (!c?.cantidad || Number(c.cantidad.replace(',', '.')) === 0) return [];
          return [
            {
              id_detalle: l.id_detalle!,
              cantidad: decimalBase(c.cantidad, l.unidad === 'ML' ? 3 : 0),
              recuperada: decimalBase(c.recuperada || '0', l.unidad === 'ML' ? 3 : 0, true),
            },
          ];
        });
        if (!ls.length) return;
        void posApi<{ total_cent: number }>('cotizar_devolucion', {
          id_operacion: 'cotizacion-retorno',
          id_venta: venta.id,
          motivo,
          medio,
          referencia: referencia || null,
          lineas: ls,
        })
          .then((r) => {
            if (vivo) {
              Simporte(r.total_cent);
              Serror('');
            }
          })
          .catch((e) => {
            if (vivo) Serror(mensajePos(e));
          });
      } catch (e) {
        if (vivo) Serror(mensajePos(e));
      }
    }, 180);
    return () => {
      vivo = false;
      clearTimeout(t);
    };
  }, [capturas, venta, motivo, medio, referencia]);
  const formularioModificado =
    motivo !== '' || Object.keys(capturas).length > 0 || referencia !== '';
  const cerrar = () => {
    if (
      !ocupado &&
      (!formularioModificado || window.confirm('¿Descartar los datos de esta devolución?'))
    )
      onCerrar();
  };
  function seleccionarTodo() {
    const p: typeof capturas = {};
    for (const l of lineas)
      p[l.id_detalle!] = {
        cantidad: String(
          l.unidad === 'ML'
            ? (l.cantidad - (l.devuelta ?? 0)) / 1000
            : l.cantidad - (l.devuelta ?? 0)
        ),
        recuperada: '0',
      };
    Scapturas(p);
  }
  async function confirmar() {
    if (lock.current) return;
    try {
      const ls = lineas.flatMap((l) => {
        const c = capturas[l.id_detalle!];
        if (!c?.cantidad || Number(c.cantidad.replace(',', '.')) === 0) return [];
        const n = decimalBase(c.cantidad, l.unidad === 'ML' ? 3 : 0);
        const rec = decimalBase(c.recuperada || '0', l.unidad === 'ML' ? 3 : 0, true);
        if (n > l.cantidad - (l.devuelta ?? 0) || rec > n || (!l.id_producto && rec > 0))
          throw new Error('Revisa cantidades pendientes y material recuperado.');
        return [{ id_detalle: l.id_detalle!, cantidad: n, recuperada: rec }];
      });
      if (!ls.length) throw new Error('Selecciona al menos una cantidad para devolver.');
      if (motivo.trim().length < 5)
        throw new Error('Escribe un motivo de al menos cinco caracteres.');
      lock.current = true;
      Socupado(true);
      await onConfirmar(
        {
          id_operacion: crypto.randomUUID(),
          id_venta: venta.id,
          motivo: motivo.trim(),
          medio,
          referencia: referencia.trim() || null,
          lineas: ls,
        },
        cajero ? cred : undefined
      );
    } catch (e) {
      Serror(mensajePos(e));
    } finally {
      lock.current = false;
      Socupado(false);
      Scred((c) => ({ ...c, password: '' }));
    }
  }
  return (
    <ModalPos
      captura
      titulo={`Devolver · venta V-${venta.id}`}
      onCerrar={cerrar}
      ocupado={ocupado}
      ancho
    >
      <p className="pos-hint">
        El importe definitivo se calcula con los precios y descuentos originales. Recupera stock
        solo si el material regresó físicamente y está apto.
      </p>
      <button onClick={seleccionarTodo} disabled={ocupado}>
        Seleccionar todo lo pendiente
      </button>
      <div className="pos-table-wrap">
        <table>
          <thead>
            <tr>
              <th>Producto / servicio</th>
              <th>Pendiente</th>
              <th>Devolver</th>
              <th>Recuperado apto</th>
            </tr>
          </thead>
          <tbody>
            {lineas.map((l) => (
              <tr key={l.id_detalle}>
                <td>
                  <strong>{l.nombre}</strong>
                  <small>
                    Original: {soles(l.total_cent)} · {l.unidad === 'ML' ? 'litros' : 'unidades'}
                  </small>
                </td>
                <td>{cantidad(l.cantidad - (l.devuelta ?? 0), l.unidad)}</td>
                {(['cantidad', 'recuperada'] as const).map((k) => (
                  <td key={k}>
                    <input
                      aria-label={`${k === 'cantidad' ? 'Devolver' : 'Recuperar'} ${l.nombre}`}
                      disabled={ocupado || (k === 'recuperada' && !l.id_producto)}
                      inputMode="decimal"
                      value={capturas[l.id_detalle!]?.[k] ?? (k === 'recuperada' ? '0' : '')}
                      onChange={(e) =>
                        Scapturas((p) => ({
                          ...p,
                          [l.id_detalle!]: {
                            cantidad: p[l.id_detalle!]?.cantidad ?? '',
                            recuperada: p[l.id_detalle!]?.recuperada ?? '0',
                            [k]: e.target.value,
                          },
                        }))
                      }
                    />
                  </td>
                ))}
              </tr>
            ))}
          </tbody>
        </table>
      </div>
      <div className="pos-fields">
        <label>
          Medio de reintegro
          <select value={medio} onChange={(e) => Smedio(e.target.value as MedioPago)}>
            {MEDIOS.map((m) => (
              <option key={m}>{m}</option>
            ))}
          </select>
        </label>
        <label>
          Referencia · opcional
          <input maxLength={100} value={referencia} onChange={(e) => Sreferencia(e.target.value)} />
        </label>
      </div>
      <label>
        Motivo
        <textarea
          required
          minLength={5}
          maxLength={250}
          value={motivo}
          onChange={(e) => Smotivo(e.target.value)}
        />
      </label>
      <AutorizacionPos necesaria={cajero} valor={cred} onChange={Scred} />
      {error && (
        <p className="pos-error" role="alert">
          {error}
        </p>
      )}
      <div className="pos-amount">
        <span>Importe a reintegrar</span>
        <strong>{importe !== null ? soles(importe) : 'Selecciona cantidades'}</strong>
      </div>
      <footer className="pos-modal-footer">
        <button disabled={ocupado} onClick={cerrar}>
          Volver
        </button>
        <button
          className="pos-primary"
          disabled={ocupado || importe === null}
          onClick={() => void confirmar()}
        >
          {ocupado ? 'Registrando devolución…' : 'Confirmar devolución'}
        </button>
      </footer>
    </ModalPos>
  );
}
