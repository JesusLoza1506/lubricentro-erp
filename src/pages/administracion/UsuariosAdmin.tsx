import { useCallback, useRef, useState } from 'react';
import { KeyRound, Pencil, Search, Unlock, UserPlus } from 'lucide-react';
import {
  admin,
  mensajeError,
  type Dependencias,
  type DetalleUsuario,
  type Entrega,
  type FiltrosUsuarios,
  type PaginaUsuarios,
  type Rol,
  type Usuario,
} from './adminApi';
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

const roles: Record<Rol, string> = {
  ADMINISTRADOR: 'Administrador',
  CAJERO: 'Cajero',
  MECANICO: 'Mecánico',
};
type Formulario = {
  previo?: Usuario;
  login: string;
  nombre: string;
  rol: Rol;
  activo: boolean;
  motivo: string;
  operacion: string;
  pendientes?: Dependencias;
};
type Confirmar = { usuario: Usuario; accion: 'desbloquear' | 'restablecer'; operacion: string };

function formularioModificado(form: Formulario) {
  const rol = form.previo?.rol ?? 'CAJERO';
  const activo = form.previo?.activo ?? true;
  return form.login !== (form.previo?.login ?? '')
    || form.nombre !== (form.previo?.nombre ?? '')
    || form.rol !== rol || form.activo !== activo;
}

function CuentaDesdeReporte({ id }: { id: number }) {
  const cargar = useCallback(() => admin<DetalleUsuario>('detalle_usuario', { id }), [id]);
  const consulta = useConsulta(cargar);
  const u = consulta.datos?.usuario;
  return <section className="adm-panel" aria-label="Cuenta vinculada al reporte">
    <h2>Cuenta vinculada al reporte</h2>
    <EstadoAdmin error={consulta.error} />
    {u ? <p><strong>{u.nombre}</strong> · {u.login} · {roles[u.rol]} · {u.activo ? 'Activa' : 'Inactiva'}</p>
      : !consulta.error && <p role="status">Consultando la cuenta…</p>}
  </section>;
}

export function UsuariosAdmin({ marcar, navegar, filtros, onFiltros, idOrigenReporte }: EdicionLocal & {
  filtros: FiltrosUsuarios;
  idOrigenReporte?: number;
  onFiltros: (filtros: FiltrosUsuarios) => void;
}) {
  const { busqueda: filtro, rol, estado, pagina } = filtros;
  const [busqueda, setBusqueda] = useState(filtro);
  const [seleccion, setSeleccion] = useState<Usuario | null>(null);
  const [errorDetalle, setErrorDetalle] = useState('');
  const solicitudDetalle = useRef(0);
  const [form, setForm] = useState<Formulario | null>(null);
  const [confirmar, setConfirmar] = useState<Confirmar | null>(null);
  const [entrega, setEntrega] = useState<Entrega | null>(null);
  const cargar = useCallback(
    () => admin<PaginaUsuarios>('usuarios', { busqueda: filtro, rol, estado, pagina }),
    [filtro, rol, estado, pagina]
  );
  const consulta = useConsulta(cargar);
  const op = useOperacion();
  const cerrar = () => {
    solicitudDetalle.current += 1;
    setSeleccion(null);
    setErrorDetalle('');
    setForm(null);
    setConfirmar(null);
    setEntrega(null);
    marcar(false);
    op.setError('');
  };
  const solicitarCierre = () => {
    if (!op.guardando && confirmarDescarteAdmin(!!form && formularioModificado(form))) cerrar();
  };
  const editar = async (usuario: Usuario) => {
    const solicitud = ++solicitudDetalle.current;
    setSeleccion(usuario);
    setErrorDetalle('');
    op.setError('');
    try {
      const d = await admin<DetalleUsuario>('detalle_usuario', { id: usuario.id });
      if (solicitud !== solicitudDetalle.current) return;
      const u = d.usuario;
      setForm({
        previo: u,
        login: u.login,
        nombre: u.nombre,
        rol: u.rol,
        activo: u.activo,
        motivo: '',
        operacion: crypto.randomUUID(),
        pendientes: d.pendientes,
      });
    } catch (e) {
      if (solicitud === solicitudDetalle.current) setErrorDetalle(mensajeError(e));
    }
  };
  const cambioAcceso =
    !!form?.previo && (form.rol !== form.previo.rol || form.activo !== form.previo.activo);
  const dependencias = form?.pendientes;
  const resuelve =
    !!form?.previo && (form.rol !== form.previo.rol || (form.previo.activo && !form.activo));
  const bloqueado = resuelve && !!(dependencias?.caja || dependencias?.ordenes.length);
  const actualizar = (d: Partial<Formulario>) => {
    if (!form) return;
    const siguiente = { ...form, ...d };
    setForm(siguiente);
    marcar(formularioModificado(siguiente));
  };
  return (
    <>
      {idOrigenReporte && <CuentaDesdeReporte id={idOrigenReporte} />}
      <section className="adm-panel" aria-labelledby="usuarios-titulo">
        <div className="adm-fila">
          <div>
            <h2 id="usuarios-titulo">Cuentas de usuario</h2>
            <p>Gestiona el acceso del equipo y sus roles.</p>
          </div>
          <button
            className="gt-button gt-button--primary"
            disabled={op.guardando}
            onClick={() => {
              op.setError('');
              setForm({
                login: '',
                nombre: '',
                rol: 'CAJERO',
                activo: true,
                motivo: '',
                operacion: crypto.randomUUID(),
              });
            }}
          >
            <UserPlus size={17} aria-hidden="true" /> Nueva cuenta
          </button>
        </div>
        {!form && !seleccion && !confirmar && !entrega && <EstadoAdmin aviso={op.aviso} />}
        <form
          className="adm-filtros"
          onSubmit={(e) => {
            e.preventDefault();
            onFiltros({ ...filtros, busqueda: busqueda.trim(), pagina: 1 });
          }}
        >
          <Campo etiqueta="Buscar por login o nombre">
            <input
              className="gt-input"
              value={busqueda}
              maxLength={100}
              placeholder="Ej. cajero01"
              onChange={(e) => setBusqueda(e.target.value)}
            />
          </Campo>
          <Campo etiqueta="Rol">
            <select
              className="gt-input"
              value={rol}
              onChange={(e) => {
                onFiltros({ ...filtros, rol: e.target.value, pagina: 1 });
              }}
            >
              <option value="">Todos</option>
              {Object.entries(roles).map(([v, t]) => (
                <option key={v} value={v}>
                  {t}
                </option>
              ))}
            </select>
          </Campo>
          <Campo etiqueta="Estado">
            <select
              className="gt-input"
              value={estado}
              onChange={(e) => {
                onFiltros({ ...filtros, estado: e.target.value, pagina: 1 });
              }}
            >
              <option value="">Todos</option>
              <option value="ACTIVO">Activos</option>
              <option value="INACTIVO">Inactivos</option>
            </select>
          </Campo>
          <button className="gt-button" type="submit">
            <Search size={16} aria-hidden="true" /> Buscar
          </button>
        </form>
        <EstadoAdmin
          cargando={consulta.cargando}
          error={consulta.error}
          reintentar={consulta.recargar}
        />
        {consulta.datos && !consulta.error && (
          <>
            <div className="adm-tabla">
              <table>
                <thead>
                  <tr>
                    <th scope="col">Nombre y login</th>
                    <th scope="col">Rol</th>
                    <th scope="col">Acceso</th>
                    <th scope="col">Acciones</th>
                  </tr>
                </thead>
                <tbody>
                  {consulta.datos.usuarios.map((u) => (
                    <tr key={u.id}>
                      <td>
                        <strong>{u.nombre}</strong>
                        <small>{u.login}</small>
                      </td>
                      <td>{roles[u.rol]}</td>
                      <td>
                        <div className="adm-etiquetas">
                          <EtiquetaAdmin tono={u.activo ? 'exito' : undefined}>{u.activo ? 'Activa' : 'Inactiva'}</EtiquetaAdmin>
                          {u.bloqueado && <EtiquetaAdmin tono="error">Bloqueada</EtiquetaAdmin>}
                          {u.debe_cambiar_clave && <EtiquetaAdmin tono="aviso">Cambio de clave pendiente</EtiquetaAdmin>}
                        </div>
                      </td>
                      <td>
                        {u.id === consulta.datos?.actor ? (
                          <span className="adm-muted">Tu cuenta</span>
                        ) : (
                          <div className="adm-acciones">
                            <button
                              className="gt-button"
                              disabled={op.guardando}
                              onClick={() => void editar(u)}
                            >
                              <Pencil size={15} aria-hidden="true" /> Editar
                            </button>
                            {u.activo && u.bloqueado && (
                              <button
                                className="gt-button"
                                disabled={op.guardando}
                                onClick={() => {
                                  op.setError('');
                                  setConfirmar({
                                    usuario: u,
                                    accion: 'desbloquear',
                                    operacion: crypto.randomUUID(),
                                  });
                                }}
                              >
                                <Unlock size={15} aria-hidden="true" /> Desbloquear
                              </button>
                            )}
                            {u.activo && (
                              <button
                                className="gt-button"
                                disabled={op.guardando}
                                onClick={() => {
                                  op.setError('');
                                  setConfirmar({
                                    usuario: u,
                                    accion: 'restablecer',
                                    operacion: crypto.randomUUID(),
                                  });
                                }}
                              >
                                <KeyRound size={15} aria-hidden="true" /> Restablecer clave
                              </button>
                            )}
                          </div>
                        )}
                      </td>
                    </tr>
                  ))}
                  {!consulta.datos.usuarios.length && (
                    <tr>
                      <td colSpan={4} className="adm-vacio">No se encontraron cuentas.</td>
                    </tr>
                  )}
                </tbody>
              </table>
            </div>
            <div className="adm-fila adm-paginador">
              <span>
                {consulta.datos.total} cuentas · Página {consulta.datos.pagina} de{' '}
                {consulta.datos.paginas}
              </span>
              <div className="adm-acciones">
                <button
                  className="gt-button"
                  disabled={consulta.cargando || consulta.datos.pagina <= 1}
                  onClick={() => onFiltros({ ...filtros, pagina: consulta.datos!.pagina - 1 })}
                >
                  Anterior
                </button>
                <button
                  className="gt-button"
                  disabled={consulta.cargando || consulta.datos.pagina >= consulta.datos.paginas}
                  onClick={() => onFiltros({ ...filtros, pagina: consulta.datos!.pagina + 1 })}
                >
                  Siguiente
                </button>
              </div>
            </div>
          </>
        )}
      </section>
      {(seleccion || form || confirmar || entrega) && (
        <DialogoAdmin
          titulo={
            entrega ? `Entrega de clave · ${entrega.login}`
              : confirmar ? (confirmar.accion === 'desbloquear' ? 'Desbloquear cuenta' : 'Restablecer clave')
              : seleccion ? 'Editar cuenta' : 'Nueva cuenta'
          }
          descripcion={
            seleccion ? `${seleccion.nombre} · ${seleccion.login}`
              : form ? 'Crea el acceso y asigna un rol a la cuenta.' : undefined
          }
          cerrar={solicitarCierre}
          ocupado={op.guardando}
        >
          {!form && seleccion && (
            <>
              <EstadoAdmin cargando={!errorDetalle} error={errorDetalle} reintentar={() => void editar(seleccion)} />
              <div className="gt-actions">
                <button type="button" className="gt-button" onClick={solicitarCierre}>Cancelar</button>
              </div>
            </>
          )}
          {form && (
            <form
              className="gt-dialog-form"
              onSubmit={(e) => {
                e.preventDefault();
                if (bloqueado) return;
                void op.ejecutar(
                  async () => {
                    if (form.previo) {
                      await admin('editar_usuario', {
                        datos: {
                          id: form.previo.id,
                          version: form.previo.version,
                          nombre: form.nombre,
                          rol: form.rol,
                          activo: form.activo,
                          motivo: cambioAcceso ? form.motivo : null,
                        },
                      });
                      cerrar();
                    } else {
                      const r = await admin<Entrega>('crear_usuario', {
                        login: form.login,
                        nombre: form.nombre,
                        rol: form.rol,
                        operacion: form.operacion,
                      });
                      cerrar();
                      setEntrega(r);
                    }
                    void consulta.recargar();
                  },
                  form.previo ? 'Cuenta actualizada.' : 'Cuenta registrada.'
                );
              }}
            >
              <EstadoAdmin error={op.error} />
              <fieldset className="adm-campos adm-fieldset" disabled={op.guardando}>
                <Campo etiqueta="Login">
                  <input
                    className="gt-input"
                    readOnly={!!form.previo}
                    required
                    minLength={3}
                    maxLength={30}
                    pattern={String.raw`[a-zA-Z0-9._\-]{3,30}`}
                    value={form.login}
                    onChange={(e) => actualizar({ login: e.target.value })}
                  />
                </Campo>
                <Campo etiqueta="Nombre completo">
                  <input
                    className="gt-input"
                    required
                    minLength={3}
                    maxLength={100}
                    value={form.nombre}
                    onChange={(e) => actualizar({ nombre: e.target.value })}
                  />
                </Campo>
                <Campo etiqueta="Rol de la cuenta">
                  <select
                    className="gt-input"
                    value={form.rol}
                    onChange={(e) => actualizar({ rol: e.target.value as Rol })}
                  >
                    {Object.entries(roles).map(([v, t]) => (
                      <option key={v} value={v}>
                        {t}
                      </option>
                    ))}
                  </select>
                </Campo>
                {form.previo && (
                  <label className="adm-check">
                    <input
                      type="checkbox"
                      checked={form.activo}
                      onChange={(e) => actualizar({ activo: e.target.checked })}
                    />{' '}
                    Cuenta activa
                  </label>
                )}
                {cambioAcceso && (
                  <Campo etiqueta="Motivo del cambio">
                    <input
                      className="gt-input"
                      minLength={5}
                      maxLength={250}
                      required
                      value={form.motivo}
                      onChange={(e) => actualizar({ motivo: e.target.value })}
                    />
                  </Campo>
                )}
              </fieldset>
              {(dependencias?.caja || !!dependencias?.ordenes.length) && (
                <aside className="gt-notice adm-dependencias" aria-label="Restricciones de acceso">
                  <p>El nombre se puede editar. Para cambiar el rol o desactivar la cuenta, resuelve primero:</p>
                  {dependencias.caja && (
                    <div className="adm-fila">
                      <span>Turno abierto: <strong>{dependencias.caja}</strong></span>
                      <button type="button" className="gt-button" disabled={op.guardando} onClick={() => navegar('caja')}>
                        Ver en Caja
                      </button>
                    </div>
                  )}
                  {!!dependencias.ordenes.length && (
                    <div className="adm-fila">
                      <span>Órdenes pendientes: <strong>{dependencias.ordenes.join(', ')}</strong></span>
                      <button type="button" className="gt-button" disabled={op.guardando} onClick={() => navegar('ordenes')}>
                        Ver en Taller
                      </button>
                    </div>
                  )}
                </aside>
              )}
              <Botones cancelar={solicitarCierre} guardando={op.guardando} deshabilitado={bloqueado} />
            </form>
          )}
          {confirmar && (
            <form
              className="gt-dialog-form"
              onSubmit={(e) => {
                e.preventDefault();
                const { usuario, accion, operacion } = confirmar;
                void op.ejecutar(
                  async () => {
                    const r = await admin<Entrega>(accion, {
                      id: usuario.id,
                      version: usuario.version,
                      ...(accion === 'restablecer' ? { operacion } : {}),
                    });
                    cerrar();
                    if (accion === 'restablecer') setEntrega(r);
                    void consulta.recargar();
                  },
                  accion === 'desbloquear' ? 'Cuenta desbloqueada.' : 'Restablecimiento confirmado.'
                );
              }}
            >
              <EstadoAdmin error={op.error} />
              <p>
                {confirmar.usuario.nombre} ({confirmar.usuario.login}).{' '}
                {confirmar.accion === 'desbloquear'
                  ? 'La contraseña actual seguirá vigente.'
                  : 'La contraseña actual dejará de funcionar.'}
              </p>
              <Botones cancelar={solicitarCierre} guardando={op.guardando} texto="Confirmar" />
            </form>
          )}
          {entrega && (
            <>
              <EstadoAdmin aviso={op.aviso} />
              {entrega.temporal ? (
                <>
                  <p>
                    Entrega esta clave temporal al titular por un canal privado. Deberá cambiarla al
                    ingresar.
                  </p>
                  <output className="adm-temporal">{entrega.temporal}</output>
                </>
              ) : (
                <p>
                  La operación ya fue confirmada. La clave anterior no puede mostrarse otra vez;
                  restablécela si no se entregó.
                </p>
              )}
              <div className="gt-actions">
                <button className="gt-button gt-button--primary" type="button" onClick={solicitarCierre}>Cerrar entrega</button>
              </div>
            </>
          )}
        </DialogoAdmin>
      )}
    </>
  );
}
