import { useCallback, useState } from 'react';
import { admin, type DatosEmpresa, type DatosTaller } from './adminApi';
import {
  Botones,
  Campo,
  EstadoAdmin,
  useConsulta,
  useOperacion,
  type EdicionLocal,
} from './AdminComun';

const campos = [
  ['ruc', 'RUC', 11, true],
  ['razon_social', 'Razón social', 150, true],
  ['nombre_comercial', 'Nombre comercial', 100, true],
  ['direccion', 'Dirección fiscal', 200, true],
  ['ubigeo', 'Ubigeo', 6, true],
  ['telefono', 'Teléfono', 18, false],
  ['correo', 'Correo', 150, false],
] as const;
type CampoEmpresa = (typeof campos)[number][0];
type EmpresaForm = Record<CampoEmpresa, string>;
function datosIniciales(d: DatosEmpresa) {
  return Object.fromEntries(campos.map(([k]) => [k, d[k] ?? ''])) as EmpresaForm;
}
export function EmpresaAdmin({ marcar }: EdicionLocal) {
  const consulta = useConsulta(useCallback(() => admin<DatosEmpresa>('empresa'), []));
  const op = useOperacion();
  const [cambios, setCambios] = useState<EmpresaForm | null>(null);
  const [motivo, setMotivo] = useState('');
  const [errorFormatoUbigeo, setErrorFormatoUbigeo] = useState('');
  const errorUbigeoBackend = /^(?:VALIDATION_ERROR:\s*)?(Ubigeo: .+)$/.exec(op.error)?.[1];
  const errorUbigeo = errorUbigeoBackend || errorFormatoUbigeo;
  const [versionEdicion, setVersionEdicion] = useState<number | null>(null);
  const d = consulta.datos;
  const form = cambios ?? (d ? datosIniciales(d) : null);
  const corregirRuc = !!d?.ruc && form?.ruc.trim() !== d.ruc;
  const limpiar = () => {
    setCambios(null);
    setMotivo('');
    setErrorFormatoUbigeo('');
    setVersionEdicion(null);
    marcar(false);
    op.setError('');
  };
  return (
    <section className="adm-panel">
      <h2>Datos de la empresa</h2>
      <p>Se usan en los nuevos comprobantes. Los anteriores conservan sus datos originales.</p>
      <EstadoAdmin
        cargando={consulta.cargando}
        error={consulta.error}
        reintentar={consulta.recargar}
      />
      <EstadoAdmin error={errorUbigeoBackend ? undefined : op.error} aviso={op.aviso} />
      {d && form && (
        <form
          onSubmit={(e) => {
            e.preventDefault();
            setErrorFormatoUbigeo('');
            void op.ejecutar(async () => {
              await admin('guardar_empresa', {
                datos: {
                  ...form,
                  version: versionEdicion ?? d.version,
                  motivo: corregirRuc ? motivo : null,
                },
              });
              await consulta.recargar();
              limpiar();
            }, 'Datos de la empresa guardados.');
          }}
        >
          <div className="adm-campos">
            {campos.map(([k, etiqueta, max, obligatorio]) => (
              <div key={k}>
                <Campo etiqueta={etiqueta}>
                  <input
                    className="gt-input"
                    required={obligatorio}
                    maxLength={max}
                    type={k === 'correo' ? 'email' : 'text'}
                    inputMode={
                      k === 'ruc' || k === 'ubigeo' ? 'numeric' : k === 'telefono' ? 'tel' : undefined
                    }
                    pattern={k === 'ruc' ? '[0-9]{11}' : k === 'ubigeo' ? '[0-9]{6}' : undefined}
                    readOnly={k === 'ruc' && d.ruc_fijo}
                    aria-invalid={k === 'ubigeo' && !!errorUbigeo ? true : undefined}
                    aria-describedby={k === 'ubigeo' && errorUbigeo ? 'adm-ubigeo-error' : undefined}
                    onInvalid={k === 'ubigeo' ? () => setErrorFormatoUbigeo('Escribe seis dígitos del distrito.') : undefined}
                    value={form[k]}
                    onChange={(e) => {
                      if (k === 'ubigeo') {
                        setErrorFormatoUbigeo('');
                        if (errorUbigeoBackend) op.setError('');
                      }
                      setVersionEdicion(versionEdicion ?? d.version);
                      setCambios({ ...form, [k]: e.target.value });
                      marcar(true);
                    }}
                  />
                </Campo>
                {k === 'ubigeo' && errorUbigeo && (
                  <p id="adm-ubigeo-error" className="adm-campo-error" role="alert">{errorUbigeo}</p>
                )}
              </div>
            ))}
            {corregirRuc && (
              <Campo etiqueta="Motivo de la corrección del RUC">
                <input
                  className="gt-input"
                  required
                  minLength={5}
                  maxLength={250}
                  value={motivo}
                  onChange={(e) => setMotivo(e.target.value)}
                />
              </Campo>
            )}
          </div>
          <aside className="adm-vista-previa" aria-label="Vista previa del emisor">
            <strong>Vista previa del emisor</strong>
            <span>
              {form.razon_social || 'Razón social'}
              {form.nombre_comercial ? ` · ${form.nombre_comercial}` : ''}
            </span>
            <span>
              RUC {form.ruc || 'sin completar'} · {form.direccion || 'Dirección fiscal'}
              {form.ubigeo ? ` · Ubigeo ${form.ubigeo}` : ''}
            </span>
            {(form.telefono || form.correo) && (
              <span>{[form.telefono, form.correo].filter(Boolean).join(' · ')}</span>
            )}
          </aside>
          {d.ruc_fijo && (
            <p className="adm-estado">El RUC está fijo porque ya se utilizó en Producción.</p>
          )}
          <Botones
            cancelar={limpiar}
            guardando={op.guardando}
            deshabilitado={!cambios || consulta.cargando || !!consulta.error}
          />
        </form>
      )}
    </section>
  );
}

export function TallerAdmin({ marcar, navegar }: EdicionLocal) {
  const consulta = useConsulta(useCallback(() => admin<DatosTaller>('taller'), []));
  const op = useOperacion();
  const [edicion, setEdicion] = useState<{ valor: string; anterior: number } | null>(null);
  const d = consulta.datos;
  const valor = edicion?.valor ?? String(d?.cantidad ?? '');
  const cantidad = Number(valor);
  const retiradas = d?.zanjas.filter((z) => z.activa && z.id > cantidad) ?? [];
  const ocupadas = retiradas.filter((z) => z.ot);
  const limpiar = () => {
    setEdicion(null);
    marcar(false);
    op.setError('');
  };
  return (
    <section className="adm-panel">
      <h2>Capacidad del taller</h2>
      <p>Configura entre 1 y 6 zanjas. Cambiar la capacidad no mueve las órdenes en espera.</p>
      <EstadoAdmin
        cargando={consulta.cargando}
        error={consulta.error}
        reintentar={consulta.recargar}
      />
      <EstadoAdmin error={op.error} aviso={op.aviso} />
      {d && (
        <form
          onSubmit={(e) => {
            e.preventDefault();
            void op.ejecutar(async () => {
              await admin('guardar_taller', {
                cantidad,
                anterior: edicion?.anterior ?? d.cantidad,
              });
              await consulta.recargar();
              limpiar();
            }, 'Capacidad del taller actualizada.');
          }}
        >
          <div className="adm-campos">
            <Campo etiqueta="Cantidad de zanjas">
              <input
                className="gt-input"
                required
                type="number"
                min={1}
                max={6}
                step={1}
                value={valor}
                onChange={(e) => {
                  setEdicion({ valor: e.target.value, anterior: edicion?.anterior ?? d.cantidad });
                  marcar(true);
                }}
              />
            </Campo>
          </div>
          <ul className="adm-zanjas">
            {d.zanjas.map((z) => (
              <li key={z.id}>
                <strong>Zanja {z.id}</strong>
                <span>
                  {z.ot ? `Ocupada · ${z.ot}` : z.activa ? 'Disponible' : 'Deshabilitada'}
                </span>
              </li>
            ))}
          </ul>
          {retiradas.length > 0 && (
            <p>Se deshabilitarán las zanjas {retiradas.map((z) => z.id).join(', ')}.</p>
          )}
          {ocupadas.length > 0 && (
            <div className="gt-notice gt-notice--error adm-estado adm-error" role="alert">
              <span>Resuelve primero las órdenes {ocupadas.map((z) => z.ot).join(', ')}.</span>
              <button type="button" className="gt-button" onClick={() => navegar('ordenes')}>
                Ver en Taller
              </button>
            </div>
          )}
          <Botones
            cancelar={limpiar}
            guardando={op.guardando}
            deshabilitado={
              cantidad === d.cantidad ||
              ocupadas.length > 0 ||
              consulta.cargando ||
              !!consulta.error
            }
          />
        </form>
      )}
    </section>
  );
}
