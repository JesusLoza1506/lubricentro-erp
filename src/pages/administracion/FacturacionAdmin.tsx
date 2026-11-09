import { useCallback, useState } from 'react';
import { admin, type Facturacion, type Serie, type TipoSerie } from './adminApi';
import {
  Botones,
  Campo,
  confirmarDescarteAdmin,
  DialogoAdmin,
  EstadoAdmin,
  EtiquetaAdmin,
  useConsulta,
  useOperacion,
  type EdicionLocal,
} from './AdminComun';
const tipos: Record<TipoSerie, string> = {
  BOLETA: 'Boleta',
  FACTURA: 'Factura',
  NC_BOLETA: 'Nota de crédito de boleta',
  NC_FACTURA: 'Nota de crédito de factura',
};
const LIMITE = 99_999_999;
type FormSerie = {
  serie?: Serie;
  tipo: TipoSerie;
  codigo: string;
  ultimo: string;
  referencia: string;
};
type CambioSerie = { serie: Serie; otra?: Serie };

function serieModificada(form: FormSerie) {
  return form.tipo !== (form.serie?.tipo ?? 'BOLETA')
    || form.codigo !== (form.serie?.codigo ?? '')
    || form.ultimo !== String(form.serie ? form.serie.ultimo + 1 : 0)
    || ((!!form.serie || Number(form.ultimo) > 0) && form.referencia !== '');
}

export function FacturacionAdmin({ marcar }: EdicionLocal) {
  const consulta = useConsulta(useCallback(() => admin<Facturacion>('facturacion'), []));
  const cred = useOperacion();
  const num = useOperacion();
  const [integracion, setIntegracion] = useState<{
    ruta: string;
    token: string;
    version: number;
  } | null>(null);
  const [form, setForm] = useState<FormSerie | null>(null);
  const [cambio, setCambio] = useState<CambioSerie | null>(null);
  const d = consulta.datos;
  const ruta = integracion?.ruta ?? d?.ruta ?? '';
  const token = integracion?.token ?? '';
  const limpiarToken = () => {
    setIntegracion(null);
    marcar(false);
    cred.setError('');
  };
  const limpiarSerie = () => {
    setForm(null);
    setCambio(null);
    marcar(false);
    num.setError('');
  };
  const solicitarCierreSerie = () => {
    if (!num.guardando && confirmarDescarteAdmin(!!form && serieModificada(form))) limpiarSerie();
  };
  const actualizar = (val: Partial<FormSerie>) => {
    if (!form) return;
    const siguiente = { ...form, ...val };
    setForm(siguiente);
    marcar(serieModificada(siguiente));
  };
  return (
    <>
      <EstadoAdmin
        cargando={consulta.cargando}
        error={consulta.error}
        reintentar={consulta.recargar}
      />
      {d && (
        <>
          <section className="adm-panel" aria-labelledby="adm-nubefact">
            <h2 id="adm-nubefact">NubeFact</h2>
            <p>
              La configuración se guarda localmente. Para comprobar la emisión usa el POS en
              ambiente de Prueba.
            </p>
            <EtiquetaAdmin tono={d.configuracion_completa ? 'exito' : 'aviso'}>
              {d.configuracion_completa ? 'Configuración completa' : 'Falta configurar'}
            </EtiquetaAdmin>
            <EstadoAdmin error={cred.error} aviso={cred.aviso} />
            <form
              onSubmit={(e) => {
                e.preventDefault();
                void cred.ejecutar(async () => {
                  await admin('guardar_token', {
                    ruta,
                    token,
                    version: integracion?.version ?? d.version,
                  });
                  await consulta.recargar();
                  limpiarToken();
                }, 'Integración guardada.');
              }}
            >
              <div className="adm-campos">
                <Campo etiqueta="Ruta de NubeFact">
                  <input
                    className="gt-input"
                    type="url"
                    maxLength={500}
                    required
                    value={ruta}
                    disabled={!!form || !!cambio || num.guardando}
                    onChange={(e) => {
                      setIntegracion({
                        ruta: e.target.value,
                        token,
                        version: integracion?.version ?? d.version,
                      });
                      marcar(true);
                    }}
                  />
                </Campo>
                <Campo
                  etiqueta={
                    d.token_configurado ? 'Reemplazar token (opcional)' : 'Token de NubeFact'
                  }
                >
                  <input
                    className="gt-input"
                    type="password"
                    autoComplete="off"
                    required={!d.token_configurado}
                    maxLength={2048}
                    value={token}
                    disabled={!!form || !!cambio || num.guardando}
                    onChange={(e) => {
                      setIntegracion({
                        ruta,
                        token: e.target.value,
                        version: integracion?.version ?? d.version,
                      });
                      marcar(true);
                    }}
                  />
                </Campo>
              </div>
              {d.token_configurado && <p>Si dejas el token vacío, se conserva el vigente.</p>}
              <Botones
                cancelar={limpiarToken}
                guardando={cred.guardando}
                deshabilitado={!integracion || consulta.cargando || !!consulta.error}
              />
            </form>
          </section>
          <section className="adm-panel" aria-labelledby="adm-series">
            <div className="adm-fila">
              <div>
                <h2 id="adm-series">Series de comprobantes</h2>
                <p>Solo puede haber una serie activa por tipo en este ambiente.</p>
              </div>
              <button
                className="gt-button gt-button--primary"
                disabled={!!integracion || cred.guardando || num.guardando}
                onClick={() => {
                  num.setError('');
                  setForm({ tipo: 'BOLETA', codigo: '', ultimo: '0', referencia: '' });
                }}
              >
                Nueva serie
              </button>
            </div>
            {!form && !cambio && <EstadoAdmin aviso={num.aviso} />}
            <div className="adm-tabla">
              <table>
                <thead>
                  <tr>
                    <th scope="col">Tipo</th>
                    <th scope="col">Serie</th>
                    <th scope="col">Estado</th>
                    <th scope="col">Último / siguiente</th>
                    <th scope="col">Acciones</th>
                  </tr>
                </thead>
                <tbody>
                  {d.series.map((s) => (
                    <tr key={s.id}>
                      <td>{tipos[s.tipo]}</td>
                      <td>
                        <strong>{s.codigo}</strong>
                      </td>
                      <td>
                        <div className="adm-etiquetas">
                          <EtiquetaAdmin tono={s.activa ? 'exito' : undefined}>{s.activa ? 'Activa' : 'Inactiva'}</EtiquetaAdmin>
                          {s.pendiente && <EtiquetaAdmin tono="aviso">Pendiente de conciliación</EtiquetaAdmin>}
                        </div>
                      </td>
                      <td>
                        {s.ultimo} / {s.siguiente ?? 'Agotada'}
                      </td>
                      <td>
                        <div className="adm-acciones">
                          <button
                            className="gt-button"
                            disabled={
                              !!integracion ||
                              cred.guardando ||
                              num.guardando ||
                              s.pendiente ||
                              (!s.activa && s.siguiente === null)
                            }
                            onClick={() => {
                              num.setError('');
                              setCambio({
                                serie: s,
                                otra: d.series.find(
                                  (o) => o.tipo === s.tipo && o.activa && o.id !== s.id
                                ),
                              });
                            }}
                          >
                            {s.activa ? 'Desactivar' : 'Activar'}
                          </button>
                          <button
                            className="gt-button"
                            disabled={
                              !!integracion ||
                              cred.guardando ||
                              num.guardando ||
                              s.pendiente ||
                              s.siguiente === null
                            }
                            onClick={() => {
                              num.setError('');
                              setForm({
                                serie: s,
                                tipo: s.tipo,
                                codigo: s.codigo,
                                ultimo: String(s.ultimo + 1),
                                referencia: '',
                              });
                            }}
                          >
                            Aumentar piso
                          </button>
                        </div>
                      </td>
                    </tr>
                  ))}
                  {!d.series.length && (
                    <tr>
                      <td colSpan={5}>No hay series registradas en este ambiente.</td>
                    </tr>
                  )}
                </tbody>
              </table>
            </div>
          </section>
          {form && (
            <DialogoAdmin titulo={form.serie ? `Aumentar piso · ${form.serie.codigo}` : 'Nueva serie'} cerrar={solicitarCierreSerie} ocupado={num.guardando}>
              <form
                className="gt-dialog-form"
                onSubmit={(e) => {
                  e.preventDefault();
                  void num.ejecutar(
                    async () => {
                      if (form.serie)
                        await admin('aumentar_serie', {
                          id: form.serie.id,
                          ultimo: Number(form.ultimo),
                          referencia: form.referencia,
                          maximo_esperado: form.serie.ultimo,
                        });
                      else
                        await admin('registrar_serie', {
                          tipo: form.tipo,
                          codigo: form.codigo,
                          ultimo: Number(form.ultimo),
                          referencia: Number(form.ultimo) > 0 ? form.referencia : null,
                        });
                      await consulta.recargar();
                      limpiarSerie();
                    },
                    form.serie ? 'Piso de numeración actualizado.' : 'Serie registrada como inactiva.'
                  );
                }}
              >
                <EstadoAdmin error={num.error} />
                <fieldset className="adm-campos adm-fieldset" disabled={num.guardando}>
                  {!form.serie && (
                    <>
                      <Campo etiqueta="Tipo de comprobante">
                        <select
                          className="gt-input"
                          value={form.tipo}
                          onChange={(e) => actualizar({ tipo: e.target.value as TipoSerie })}
                        >
                          {Object.entries(tipos).map(([k, v]) => (
                            <option key={k} value={k}>
                              {v}
                            </option>
                          ))}
                        </select>
                      </Campo>
                      <Campo etiqueta="Código de serie">
                        <input
                          className="gt-input"
                          required
                          minLength={4}
                          maxLength={4}
                          value={form.codigo}
                          onChange={(e) => actualizar({ codigo: e.target.value.toUpperCase() })}
                        />
                      </Campo>
                    </>
                  )}
                  <Campo etiqueta="Último número real usado">
                    <input
                      className="gt-input"
                      type="number"
                      min={form.serie ? form.serie.ultimo + 1 : 0}
                      max={LIMITE}
                      step={1}
                      required
                      value={form.ultimo}
                      onChange={(e) => actualizar({ ultimo: e.target.value })}
                    />
                  </Campo>
                  {(!!form.serie || Number(form.ultimo) > 0) && (
                    <Campo etiqueta="Referencia de verificación">
                      <input
                        className="gt-input"
                        required
                        minLength={5}
                        maxLength={120}
                        value={form.referencia}
                        onChange={(e) => actualizar({ referencia: e.target.value })}
                      />
                    </Campo>
                  )}
                </fieldset>
                {!form.serie && (
                  <p>
                    Usa 0 únicamente si la serie nunca se utilizó. Al guardar se aplican los máximos
                    conservados.
                  </p>
                )}
                <Botones cancelar={solicitarCierreSerie} guardando={num.guardando} />
              </form>
            </DialogoAdmin>
          )}
          {cambio && (
            <DialogoAdmin titulo={`${cambio.serie.activa ? 'Desactivar' : 'Activar'} serie ${cambio.serie.codigo}`} cerrar={solicitarCierreSerie} ocupado={num.guardando}>
              <form
                className="gt-dialog-form"
                onSubmit={(e) => {
                  e.preventDefault();
                  void num.ejecutar(async () => {
                    await admin('estado_serie', {
                      id: cambio.serie.id,
                      activa: !cambio.serie.activa,
                      activa_esperada: cambio.otra?.id ?? null,
                    });
                    await consulta.recargar();
                    limpiarSerie();
                  }, 'Estado de la serie actualizado.');
                }}
              >
                <EstadoAdmin error={num.error} />
                {cambio.otra ? (
                  <p>
                    La serie {cambio.otra.codigo} quedará inactiva. Su historial y los números
                    reservados se conservan.
                  </p>
                ) : (
                  <p>Los comprobantes previos conservarán su numeración.</p>
                )}
                <Botones cancelar={solicitarCierreSerie} guardando={num.guardando} texto="Confirmar" />
              </form>
            </DialogoAdmin>
          )}
        </>
      )}
    </>
  );
}
