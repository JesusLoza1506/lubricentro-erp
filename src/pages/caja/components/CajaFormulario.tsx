import { useEffect, useRef, useState, type FormEvent } from 'react';
import { X } from 'lucide-react';
import type { AccionCaja, SolicitudCaja } from '../../../services/cajaService';
import type { CajaDetalle, CajaMovimiento } from '../../../types/caja';
import { decimalBase, soles } from '../../pos/posUtils';

export type FormCaja =
  | { tipo: 'abrir'; fondo: number; titular: string; ambiente: string }
  | { tipo: 'manual' | 'cerrar' | 'aclarar'; detalle: CajaDetalle }
  | { tipo: 'revertir'; movimiento: CajaMovimiento };

const titulos: Record<AccionCaja, string> = {
  abrir: 'Abrir turno',
  manual: 'Registrar movimiento',
  revertir: 'Revertir movimiento',
  cerrar: 'Cerrar turno y guardar arqueo',
  aclarar: 'Aclaración del cierre',
};
const etiquetas = {
  APORTE_FONDO: 'Aporte de fondo',
  GASTO_MENOR: 'Gasto menor',
  RETIRO_PARCIAL: 'Retiro parcial',
  OTRO: 'Otro',
};
const centTexto = (v: number) => (v / 100).toFixed(2);
const validarMotivo = (v: string) => {
  if ([...v].length < 5 || [...v].length > 250)
    throw new Error('Escribe un motivo de 5 a 250 caracteres.');
  return v;
};
function importe(v: string, limite: number, cero = false) {
  const cent = decimalBase(v, 2, cero);
  if (cent > limite) throw new Error(`El importe no puede superar ${soles(limite)}.`);
  return cent;
}

export function CajaFormulario({
  form,
  ocupado,
  pendiente,
  error,
  onCancelar,
  onDirty,
  onEnviar,
  onReintentar,
}: {
  form: FormCaja;
  ocupado: boolean;
  pendiente: boolean;
  error: string;
  onCancelar: () => void;
  onDirty: (v: boolean) => void;
  onEnviar: (accion: AccionCaja, payload: SolicitudCaja['payload']) => Promise<void>;
  onReintentar: () => Promise<void>;
}) {
  const dialogo = useRef<HTMLDialogElement>(null);
  const [cantidad, setCantidad] = useState(form.tipo === 'abrir' ? centTexto(form.fondo) : '');
  const [fondo, setFondo] = useState('0');
  const [texto, setTexto] = useState(
    form.tipo === 'aclarar' ? (form.detalle.turno.aclaracion ?? '') : ''
  );
  const [tipo, setTipo] = useState<'INGRESO_MANUAL' | 'EGRESO_MANUAL'>('INGRESO_MANUAL');
  const [categoria, setCategoria] = useState<keyof typeof etiquetas>('APORTE_FONDO');
  const [validacion, setValidacion] = useState('');
  const [dirty, setDirty] = useState(false);
  const marcar = () => {
    setDirty(true);
    onDirty(true);
  };
  const cerrar = () => {
    if (ocupado) return;
    if (!dirty || window.confirm('¿Descartar los cambios de este formulario?')) onCancelar();
  };
  useEffect(() => {
    const d = dialogo.current;
    const foco = document.activeElement as HTMLElement | null;
    d?.showModal?.();
    return () => {
      d?.close?.();
      foco?.focus();
    };
  }, []);
  const arqueo = form.tipo === 'cerrar' ? form.detalle : null;
  const esperado = arqueo?.resumen.efectivo_esperado_cent ?? 0;
  let contado: number | null = null;
  let retiro: number | null = null;
  if (arqueo) {
    try {
      contado = importe(cantidad, 9_999_999_999, true);
      const fondoNum = importe(fondo || '0', 1_000_000, true);
      if (fondoNum <= contado) retiro = contado - fondoNum;
    } catch {
      /* El mensaje de validación se muestra al enviar. */
    }
  }
  const motivoObligatorio =
    arqueo != null &&
    (arqueo.modo_cierre !== 'NORMAL' || (contado != null && contado !== esperado));
  const enviar = async (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    if (ocupado) return;
    setValidacion('');
    if (pendiente) {
      await onReintentar();
      return;
    }
    const limpio = texto.trim();
    try {
      switch (form.tipo) {
        case 'abrir':
          await onEnviar(form.tipo, { monto_apertura_cent: importe(cantidad, 1_000_000, true) });
          break;
        case 'manual':
          if ([...limpio].length < 3 || [...limpio].length > 120)
            throw new Error('Escribe un concepto de 3 a 120 caracteres.');
          await onEnviar(form.tipo, {
            id_caja: form.detalle.turno.id_caja,
            tipo,
            categoria,
            concepto: limpio,
            monto_cent: importe(cantidad, 1_000_000),
          });
          break;
        case 'revertir':
          await onEnviar(form.tipo, {
            id_movimiento: form.movimiento.id_movimiento,
            motivo: validarMotivo(limpio),
          });
          break;
        case 'cerrar': {
          const num = importe(cantidad, 9_999_999_999, true);
          const siguiente = importe(fondo || '0', 1_000_000, true);
          if (siguiente > num)
            throw new Error('El fondo siguiente no puede superar el efectivo contado.');
          await onEnviar(form.tipo, {
            id_caja: form.detalle.turno.id_caja,
            version_esperada: form.detalle.turno.version,
            efectivo_contado_cent: num,
            fondo_siguiente_cent: siguiente,
            motivo: motivoObligatorio || limpio ? validarMotivo(limpio) : null,
          });
          break;
        }
        case 'aclarar':
          await onEnviar(form.tipo, { id_caja: form.detalle.turno.id_caja, aclaracion: limpio });
          break;
      }
    } catch (err) {
      setValidacion(err instanceof Error ? err.message : 'Revisa los datos ingresados.');
    }
  };
  return (
    <dialog
      ref={dialogo}
      className="gloria-theme caja-dialog"
      aria-labelledby="caja-form-titulo"
      onCancel={(e) => {
        e.preventDefault();
        cerrar();
      }}
    >
      <header className="caja-linea">
        <h2 id="caja-form-titulo">{titulos[form.tipo]}</h2>
        <button type="button" aria-label="Cerrar ventana" onClick={cerrar} disabled={ocupado}>
          <X size={18} />
        </button>
      </header>
      <form onSubmit={enviar} onChange={marcar}>
        {form.tipo === 'abrir' && (
          <p>
            <strong>
              {form.titular} · {form.ambiente}
            </strong>
            <br />
            Confirma cuánto efectivo recibiste para empezar el turno. Puedes ajustar el fondo
            sugerido.
          </p>
        )}
        {form.tipo === 'manual' && (
          <p>
            Medio de pago: <strong>Efectivo</strong>
          </p>
        )}
        {form.tipo === 'revertir' && (
          <p>
            Se registrará el movimiento inverso de {soles(form.movimiento.monto_cent)}. El original
            se conserva.
          </p>
        )}
        {form.tipo === 'aclarar' && (
          <p>
            La aclaración se guarda con tu nombre y fecha. No cambia las cifras del cierre. Déjala
            vacía para retirarla.
          </p>
        )}
        {form.tipo === 'cerrar' && form.detalle.modo_cierre !== 'NORMAL' && (
          <p className="caja-alerta">
            {form.detalle.modo_cierre === 'AUSENCIA'
              ? `Cierre por ausencia de ${form.detalle.turno.titular}. Se requiere un motivo.`
              : 'Cierre de turno recuperado. Se requiere un motivo.'}
          </p>
        )}
        {pendiente && (
          <p className="caja-alerta">
            La respuesta anterior no se pudo confirmar. Al reintentar se envía la misma solicitud.
          </p>
        )}
        <fieldset disabled={ocupado || pendiente}>
          {form.tipo === 'manual' && (
            <div className="caja-campos">
              <label>
                Tipo
                <select
                  value={tipo}
                  onChange={(e) => {
                    const nuevo = e.target.value as typeof tipo;
                    setTipo(nuevo);
                    setCategoria(nuevo === 'INGRESO_MANUAL' ? 'APORTE_FONDO' : 'GASTO_MENOR');
                  }}
                >
                  <option value="INGRESO_MANUAL">Ingreso</option>
                  <option value="EGRESO_MANUAL">Egreso</option>
                </select>
              </label>
              <label>
                Categoría
                <select
                  value={categoria}
                  onChange={(e) => setCategoria(e.target.value as keyof typeof etiquetas)}
                >
                  {(tipo === 'INGRESO_MANUAL'
                    ? (['APORTE_FONDO', 'OTRO'] as const)
                    : (['GASTO_MENOR', 'RETIRO_PARCIAL', 'OTRO'] as const)
                  ).map((k) => (
                    <option key={k} value={k}>
                      {etiquetas[k]}
                    </option>
                  ))}
                </select>
              </label>
            </div>
          )}
          {(form.tipo === 'abrir' || form.tipo === 'manual' || form.tipo === 'cerrar') && (
            <label>
              {form.tipo === 'abrir'
                ? 'Fondo de apertura (S/)'
                : form.tipo === 'manual'
                  ? 'Importe (S/)'
                  : 'Efectivo contado (S/)'}
              <input
                required
                inputMode="decimal"
                value={cantidad}
                onChange={(e) => setCantidad(e.target.value)}
              />
            </label>
          )}
          {arqueo && (
            <>
              <label>
                Fondo para el siguiente turno (S/)
                <input
                  inputMode="decimal"
                  value={fondo}
                  onChange={(e) => setFondo(e.target.value)}
                />
              </label>
              <dl className="caja-cifras caja-arqueo">
                <div>
                  <dt>Efectivo esperado</dt>
                  <dd>{soles(esperado)}</dd>
                </div>
                <div>
                  <dt>Diferencia</dt>
                  <dd>{contado === null ? '—' : soles(contado - esperado)}</dd>
                </div>
                <div>
                  <dt>Retiro de cierre</dt>
                  <dd>{retiro === null ? '—' : soles(retiro)}</dd>
                </div>
              </dl>
              <details>
                <summary>Resumen digital registrado</summary>
                <dl className="caja-desglose">
                  <div>
                    <dt>Tarjeta</dt>
                    <dd>{soles(arqueo.resumen.tarjeta.neto_cent)}</dd>
                  </div>
                  <div>
                    <dt>Yape</dt>
                    <dd>{soles(arqueo.resumen.yape.neto_cent)}</dd>
                  </div>
                  <div>
                    <dt>Plin</dt>
                    <dd>{soles(arqueo.resumen.plin.neto_cent)}</dd>
                  </div>
                  <div>
                    <dt>Transferencia</dt>
                    <dd>{soles(arqueo.resumen.transferencia.neto_cent)}</dd>
                  </div>
                </dl>
              </details>
            </>
          )}
          {form.tipo !== 'abrir' && (
            <label>
              {form.tipo === 'aclarar'
                ? 'Aclaración'
                : form.tipo === 'manual'
                  ? 'Concepto'
                  : motivoObligatorio || form.tipo === 'revertir'
                    ? 'Motivo obligatorio'
                    : 'Motivo (opcional)'}
              <textarea
                rows={3}
                maxLength={form.tipo === 'aclarar' ? 500 : form.tipo === 'manual' ? 120 : 250}
                value={texto}
                onChange={(e) => setTexto(e.target.value)}
              />
            </label>
          )}
        </fieldset>
        {(error || validacion) && (
          <p className="caja-alerta" role="alert">
            {validacion || error}
          </p>
        )}
        <footer className="caja-acciones">
          <button type="button" onClick={cerrar} disabled={ocupado}>
            Cancelar
          </button>
          <button
            type="submit"
            className={
              form.tipo === 'cerrar' || form.tipo === 'revertir' ? 'caja-peligro' : 'caja-principal'
            }
            disabled={ocupado}
          >
            {ocupado
              ? 'Guardando…'
              : pendiente
                ? 'Reintentar operación'
                : form.tipo === 'cerrar'
                  ? 'Confirmar cierre'
                  : form.tipo === 'revertir'
                    ? 'Confirmar reversión'
                    : form.tipo === 'abrir'
                      ? 'Abrir turno'
                      : 'Guardar'}
          </button>
        </footer>
      </form>
    </dialog>
  );
}
