import { useRef, useState } from 'react';
import { clienteService } from '../../../services/clienteService';
import type { ClienteDTO } from '../../../types/cliente';
import { ModalCliente } from '../../vehiculo/components/ModalCliente';
import { useCapturaPos } from '../CapturasPos';
import { mensajePos } from '../../../services/posService';
export function ReceptorPos({
  cliente,
  onSeleccionar,
  tipo,
  onTipo,
}: {
  cliente: ClienteDTO | null;
  onSeleccionar: (c: ClienteDTO | null) => void;
  tipo: 'BOLETA' | 'FACTURA';
  onTipo: (s: 'BOLETA' | 'FACTURA') => void;
}) {
  const marcar = useCapturaPos();
  const [q, Sq] = useState(''),
    [resultados, Sresultados] = useState<ClienteDTO[]>([]),
    [error, Serror] = useState(''),
    [nuevo, Snuevo] = useState(false),
    [buscando, Sbuscando] = useState(false);
  const seq = useRef(0);
  async function buscar() {
    const id = ++seq.current;
    Sbuscando(true);
    Serror('');
    try {
      const r = await clienteService.buscarClientesPorTermino(q);
      if (id === seq.current) Sresultados(r);
    } catch (e) {
      if (id === seq.current) Serror(mensajePos(e));
    } finally {
      if (id === seq.current) Sbuscando(false);
    }
  }
  return (
    <div className="pos-receptor">
      <div className="pos-panel-heading">
        <h3>Comprobante y receptor</h3>
        <button type="button" className="pos-link" onClick={() => Snuevo(true)}>
          Nuevo cliente
        </button>
      </div>
      <select
        aria-label="Tipo de comprobante"
        value={tipo}
        onChange={(e) => onTipo(e.target.value as 'BOLETA' | 'FACTURA')}
      >
        <option value="BOLETA">Boleta</option>
        <option value="FACTURA">Factura</option>
      </select>
      <div className="pos-search-row">
        <input
          aria-label="Buscar receptor"
          placeholder="Nombre, DNI o RUC"
          value={q}
          onChange={(e) => Sq(e.target.value)}
          onKeyDown={(e) => {
            if (e.key === 'Enter') {
              e.preventDefault();
              void buscar();
            }
          }}
        />
        <button
          type="button"
          disabled={buscando || q.trim().length < 2}
          onClick={() => void buscar()}
        >
          {buscando ? '…' : 'Buscar'}
        </button>
      </div>
      {resultados.length > 0 && (
        <div className="pos-clientes">
          {resultados.map((c) => (
            <button
              type="button"
              key={c.id_cliente}
              onClick={() => {
                seq.current++;
                onSeleccionar(c);
                Sresultados([]);
              }}
            >
              <strong>{c.nombre_razon_social}</strong>
              <small>
                {c.tipo_documento} {c.numero_documento}
              </small>
            </button>
          ))}
        </div>
      )}
      <div className="pos-receptor-elegido">
        <strong>{cliente?.nombre_razon_social ?? 'Cliente varios'}</strong>
        <small>
          {cliente
            ? `${cliente.tipo_documento} ${cliente.numero_documento}`
            : 'Boleta sin identificación, cuando corresponde.'}
        </small>
        {cliente && (
          <button type="button" className="pos-link" onClick={() => onSeleccionar(null)}>
            Usar Cliente varios
          </button>
        )}
      </div>
      {tipo === 'FACTURA' && (
        <p className="pos-hint">Requiere receptor con RUC, razón social y dirección.</p>
      )}
      {error && (
        <p role="alert" className="pos-error">
          {error}
        </p>
      )}
      <div onChangeCapture={() => marcar(true)}>
        <ModalCliente
          modalAbierto={nuevo}
          setModalAbierto={(abierto) => {
            Snuevo(abierto);
            if (!abierto) marcar(false);
          }}
          modo="CREAR"
          onClienteGuardado={async (c) => {
            await clienteService.registrarCliente(c);
            const creado = await clienteService.buscarClientePorDocumento(
              c.tipo_documento,
              c.numero_documento
            );
            onSeleccionar(creado);
            Snuevo(false);
            marcar(false);
          }}
        />
      </div>
    </div>
  );
}
