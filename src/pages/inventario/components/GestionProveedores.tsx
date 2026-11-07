import { useNotificarInventario } from '../AvisosInventario';
import { useEffect, useState } from 'react';
import { inventarioService } from '../../../services/inventarioService';
import { ActualizarProveedorDTO, CrearProveedorDTO, ProveedorDTO } from '../../../types/inventario';
import { ModalProveedor } from './ModalProveedor';
import { Search } from 'lucide-react';

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
    <details className="inv-providers">
      <summary>Proveedores</summary>
      <div className="inv-providers-toolbar">
        <div className="inv-search">
          <Search size={16} aria-hidden="true" />
          <input
            aria-label="Buscar proveedores"
            placeholder="Buscar por nombre o documento"
            value={busqueda}
            onChange={(e) => setBusqueda(e.target.value)}
          />
        </div>
        <button className="inv-button" disabled={cargando} onClick={() => void cargar()}>
          Actualizar
        </button>
        <button
          className="inv-button"
          onClick={() => setEditor({ abierto: true, proveedor: null })}
        >
          Nuevo proveedor
        </button>
      </div>
      {error && (
        <p role="alert" className="inv-form-error">
          {error}
        </p>
      )}
      <div className="inv-providers-list">
        {lista
          .filter((p) =>
            `${p.nombre_razon_social} ${p.numero_documento}`.toLocaleLowerCase().includes(filtro)
          )
          .map((p) => (
            <div
              key={p.id_proveedor}
              className="inv-provider-row"
            >
              <span>
                <strong>{p.nombre_razon_social}</strong> · {p.numero_documento}
              </span>
              <button
                className="inv-button"
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
