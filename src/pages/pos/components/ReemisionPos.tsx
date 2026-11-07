import { useRef, useState } from 'react';
import type { DocumentoPos, ReceptorPos } from '../../../types/pos';
import { mensajePos } from '../../../services/posService';
import { ModalPos } from './ModalPos';
export function ReemisionPos({
  documento,
  receptor,
  onCerrar,
  onConfirmar,
}: {
  documento: DocumentoPos;
  receptor: ReceptorPos;
  onCerrar: () => void;
  onConfirmar: (r: ReceptorPos, m: string) => Promise<void>;
}) {
  const [r, Sr] = useState(receptor),
    [motivo, Smotivo] = useState(''),
    [error, Serror] = useState(''),
    [ocupado, Socupado] = useState(false);
  const lock = useRef(false);
  const esNota = documento.tipo.startsWith('NC_');
  const cerrar = () => {
    if (
      !ocupado &&
      ((JSON.stringify(r) === JSON.stringify(receptor) && !motivo) ||
        window.confirm('¿Descartar la corrección documental?'))
    )
      onCerrar();
  };
  async function guardar() {
    if (lock.current) return;
    lock.current = true;
    Socupado(true);
    try {
      await onConfirmar(r, motivo);
    } catch (e) {
      Serror(mensajePos(e));
    } finally {
      lock.current = false;
      Socupado(false);
    }
  }
  return (
    <ModalPos captura titulo="Corregir receptor y reemitir" onCerrar={cerrar} ocupado={ocupado}>
      <p className="pos-hint">
        Se conservará {documento.serie}-{documento.numero} rechazado y se reservará otro número. La
        venta, el cobro y el stock permanecen iguales. El tipo y las líneas no se editan.
      </p>
      <form
        onSubmit={(e) => {
          e.preventDefault();
          void guardar();
        }}
      >
        <div className="pos-fields">
          <label>
            Tipo de documento
            <select
              disabled={esNota}
              value={r.tipo ?? ''}
              onChange={(e) => Sr({ ...r, tipo: e.target.value || null })}
            >
              <option value="">Sin identificación</option>
              <option>DNI</option>
              <option>RUC</option>
              <option>CE</option>
            </select>
          </label>
          <label>
            Número de documento
            <input
              readOnly={esNota}
              value={r.documento ?? ''}
              onChange={(e) => Sr({ ...r, documento: e.target.value })}
            />
          </label>
        </div>
        <label>
          Nombre o razón social
          <input
            required
            minLength={3}
            maxLength={150}
            readOnly={esNota}
            value={r.nombre}
            onChange={(e) => Sr({ ...r, nombre: e.target.value })}
          />
        </label>
        <label>
          Dirección
          <input
            maxLength={200}
            readOnly={esNota}
            value={r.direccion ?? ''}
            onChange={(e) => Sr({ ...r, direccion: e.target.value })}
          />
        </label>
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
        {error && (
          <p className="pos-error" role="alert">
            {error}
          </p>
        )}
        <footer className="pos-modal-footer">
          <button type="button" disabled={ocupado} onClick={cerrar}>
            Volver
          </button>
          <button type="submit" className="pos-primary" disabled={ocupado}>
            {ocupado ? 'Creando documento…' : 'Confirmar reemisión'}
          </button>
        </footer>
      </form>
    </ModalPos>
  );
}
