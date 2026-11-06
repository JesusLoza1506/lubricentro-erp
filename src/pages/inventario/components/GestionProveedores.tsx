import { useNotificarInventario } from '../AvisosInventario';
import { useEffect, useState } from 'react';
import { inventarioService } from '../../../services/inventarioService';
import { ActualizarProveedorDTO, CrearProveedorDTO, ProveedorDTO } from '../../../types/inventario';
import { ModalProveedor } from './ModalProveedor';
import { modalInputStyle, botonSecundarioStyle } from '../inventarioStyles';

export function GestionProveedores() {
  const notificar = useNotificarInventario();
  const [lista, setLista] = useState<ProveedorDTO[]>([]);
  const [busqueda, setBusqueda] = useState('');
  const [error, setError] = useState('');
  const [cargando, setCargando] = useState(false);
  const [editor, setEditor] = useState<{ abierto: boolean; proveedor: ProveedorDTO | null }>({
    abierto: false,
    proveedor: null,
  });
  const cargar = async () => {
    setCargando(true);
    setError('');
    try {
      setLista(await inventarioService.listarProveedores());
    } catch (e) {
      setError(String(e));
    } finally {
      setCargando(false);
    }
  };
  useEffect(() => {
    void cargar();
  }, []);
  const crear = async (payload: CrearProveedorDTO) => {
    const id = await inventarioService.crearProveedor(payload);
    setLista((prev) => [...prev, { ...payload, id_proveedor: id }]);
    notificar('Proveedor registrado correctamente', 'EXITO');
  };
  const editar = async (payload: ActualizarProveedorDTO) => {
    await inventarioService.actualizarProveedor(payload);
    setLista((prev) =>
      prev.map((p) => (p.id_proveedor === payload.id_proveedor ? { ...p, ...payload } : p))
    );
    notificar('Proveedor actualizado correctamente', 'EXITO');
  };
  const filtro = busqueda.trim().toLocaleLowerCase();
  return (
    <details
      className="inv-providers"
      style={{ padding: 16, background: '#1E293B', borderRadius: 12, color: '#CBD5E1' }}
    >
      <summary style={{ cursor: 'pointer', fontWeight: 700 }}>Proveedores</summary>
      <div style={{ display: 'flex', gap: 12, marginTop: 12 }}>
        <input
          aria-label="Buscar proveedores"
          style={modalInputStyle}
          placeholder="Buscar por nombre o documento"
          value={busqueda}
          onChange={(e) => setBusqueda(e.target.value)}
        />
        <button style={botonSecundarioStyle} disabled={cargando} onClick={() => void cargar()}>
          Actualizar
        </button>
        <button
          style={botonSecundarioStyle}
          onClick={() => setEditor({ abierto: true, proveedor: null })}
        >
          Nuevo proveedor
        </button>
      </div>
      {error && (
        <p role="alert" style={{ color: '#FCA5A5' }}>
          {error}
        </p>
      )}
      <div style={{ maxHeight: 220, overflow: 'auto', marginTop: 12 }}>
        {lista
          .filter((p) =>
            `${p.nombre_razon_social} ${p.numero_documento}`.toLocaleLowerCase().includes(filtro)
          )
          .map((p) => (
            <div
              key={p.id_proveedor}
              style={{
                display: 'flex',
                justifyContent: 'space-between',
                padding: 10,
                borderBottom: '1px solid #334155',
                gap: 12,
              }}
            >
              <span>
                <strong>{p.nombre_razon_social}</strong> · {p.numero_documento}
              </span>
              <button
                style={botonSecundarioStyle}
                onClick={() => setEditor({ abierto: true, proveedor: p })}
              >
                Editar
              </button>
            </div>
          ))}
        {!cargando && lista.length === 0 && <p>No hay proveedores registrados.</p>}
      </div>
      <ModalProveedor
        abierto={editor.abierto}
        proveedorAEditar={editor.proveedor}
        onClose={() => setEditor({ abierto: false, proveedor: null })}
        onGuardar={crear}
        onEditar={editar}
      />
    </details>
  );
}
