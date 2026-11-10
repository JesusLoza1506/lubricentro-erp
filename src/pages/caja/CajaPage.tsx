import { useCallback, useEffect, useRef, useState } from 'react';
import { ArrowLeft, History, Plus, RefreshCw, Wallet } from 'lucide-react';
import { ToastNotification } from '../../components/ToastNotification';
import { useAuth } from '../../context/AuthContext';
import {
  cajaApi,
  ejecutarCaja,
  esArqueoVencido,
  leerCajaPendiente,
  mensajeCaja,
  type AccionCaja,
  type SolicitudCaja,
} from '../../services/cajaService';
import type { CajaDetalle, CajaInicio } from '../../types/caja';
import { CajaFormulario, type FormCaja } from './components/CajaFormulario';
import { CajaHistorial, CajaMovimientos, fechaCaja } from './components/CajaConsultas';
import { CajaResumenTurno } from './components/CajaResumenTurno';
import '../../styles/gloriaTaller.css';
import './cajaVisual.css';

const exito: Record<AccionCaja, string> = {
  abrir: 'Turno abierto correctamente.',
  manual: 'Movimiento registrado.',
  revertir: 'Reversión registrada.',
  cerrar: 'Turno cerrado y arqueo guardado.',
  aclarar: 'Aclaración guardada.',
};

export function CajaPage({ onVenta, idOrigenReporte }: { onVenta: (id: number) => void; idOrigenReporte?: number }) {
  const { usuario, ambienteActual, setHayCambiosSinGuardar } = useAuth();
  const id = usuario?.id_usuario ?? 0;
  const admin = usuario?.rol === 'ADMINISTRADOR';
  const [vista, setVista] = useState<'mi' | 'historial'>(idOrigenReporte ? 'historial' : 'mi');
  const [inicio, setInicio] = useState<CajaInicio | null>(null);
  const [detalle, setDetalle] = useState<CajaDetalle | null>(null);
  const [revision, setRevision] = useState(0);
  const [leyendo, setLeyendo] = useState(true);
  const [ocupado, setOcupado] = useState(false);
  const [error, setError] = useState('');
  const [errorForm, setErrorForm] = useState('');
  const [errorPendiente, setErrorPendiente] = useState('');
  const [form, setForm] = useState<FormCaja | null>(null);
  const [pendiente, setPendiente] = useState<SolicitudCaja | null>(null);
  const [aviso, setAviso] = useState('');
  const lock = useRef(false);
  const lectura = useRef(0);
  const cerrarAviso = useCallback(() => setAviso(''), []);
  const leerPendiente = useCallback(() => {
    try {
      setPendiente(leerCajaPendiente(id, ambienteActual));
      setErrorPendiente('');
    } catch (e) {
      setErrorPendiente(mensajeCaja(e));
    }
  }, [id, ambienteActual]);

  const actualizar = useCallback(async () => {
    const marca = ++lectura.current;
    setLeyendo(true);
    setError('');
    try {
      const r = await cajaApi<CajaInicio>('inicio');
      if (lectura.current === marca) {
        setInicio(r);
        setRevision((n) => n + 1);
      }
    } catch (e) {
      if (lectura.current === marca) setError(mensajeCaja(e));
    } finally {
      if (lectura.current === marca) setLeyendo(false);
    }
  }, []);
  useEffect(() => {
    let vigente = true;
    void actualizar().then(() => {
      if (vigente && idOrigenReporte && admin) void ver(idOrigenReporte);
    });
    leerPendiente();
    return () => {
      vigente = false;
      lectura.current++;
      setHayCambiosSinGuardar(false);
    };
  }, [actualizar, leerPendiente, setHayCambiosSinGuardar, idOrigenReporte, admin]);

  async function ver(idCaja: number, conservar = false) {
    const marca = ++lectura.current;
    if (!conservar) setDetalle(null);
    setLeyendo(true);
    setError('');
    try {
      const r = await cajaApi<CajaDetalle>('detalle', { id_caja: idCaja });
      if (lectura.current === marca) setDetalle(r);
    } catch (e) {
      if (lectura.current === marca) setError(mensajeCaja(e));
    } finally {
      if (lectura.current === marca) setLeyendo(false);
    }
  }
  function seleccionar(f: FormCaja | null) {
    setErrorForm('');
    setForm(f);
    setHayCambiosSinGuardar(false);
  }
  async function guardar(accion: AccionCaja, datos: SolicitudCaja['payload']) {
    if (lock.current) return;
    lock.current = true;
    setOcupado(true);
    setErrorForm('');
    setError('');
    setAviso('');
    try {
      const r = await ejecutarCaja<CajaDetalle | { id_caja: number }>(
        id,
        ambienteActual,
        accion,
        datos
      );
      seleccionar(null);
      setAviso(exito[accion]);
      await actualizar();
      if (accion === 'cerrar' || accion === 'aclarar') {
        const cerrado = r as CajaDetalle;
        setDetalle(cerrado);
        if (accion === 'cerrar') setVista('mi');
      }
    } catch (e) {
      const mensaje = mensajeCaja(e);
      if (form) setErrorForm(mensaje);
      else setError(mensaje);
      if (accion === 'cerrar' && esArqueoVencido(e)) {
        try {
          const nuevo = await cajaApi<CajaDetalle>('detalle', { id_caja: datos.id_caja });
          setForm((p) => (p?.tipo === 'cerrar' ? { ...p, detalle: nuevo } : p));
          if (detalle) setDetalle(nuevo);
          await actualizar();
          if (!form) setError(mensaje);
        } catch (err) {
          if (form) setErrorForm(`${mensaje} ${mensajeCaja(err)}`);
          else setError(`${mensaje} ${mensajeCaja(err)}`);
        }
      }
    } finally {
      lock.current = false;
      setOcupado(false);
      leerPendiente();
    }
  }
  async function reintentar() {
    if (!pendiente) return;
    await guardar(pendiente.accion, pendiente.payload);
  }
  const cambiarVista = (v: 'mi' | 'historial') => {
    setVista(v);
    setDetalle(null);
    setError('');
    if (leyendo) void actualizar();
  };
  const turno = detalle ?? (vista === 'mi' ? inicio?.detalle : null);
  const bloqueado = ocupado || !!pendiente || !!errorPendiente;
  const abrirVenta = (idVenta: number) => onVenta(idVenta);

  return (
    <section className="gloria-theme caja-page">
      <div className="caja-stage">
        <header className="caja-cabecera caja-linea">
          <div>
            <p className="caja-kicker">Operación de mostrador</p>
            <h1>Caja Chica</h1>
            <p>Controla el efectivo del turno, consulta sus movimientos y guarda el arqueo.</p>
          </div>
          <div className="caja-acciones">
            {inicio && <span className="caja-sello">{inicio.ambiente}</span>}
            <button
              disabled={leyendo || ocupado}
              onClick={() =>
                void (detalle
                  ? ver(detalle.turno.id_caja, true).then(() => setRevision((v) => v + 1))
                  : actualizar())
              }
            >
              <RefreshCw size={16} /> Actualizar
            </button>
          </div>
        </header>
        {error && (
          <p className="caja-alerta" role="alert">
            {error}
          </p>
        )}
        {errorPendiente && (
          <p className="caja-alerta" role="alert">
            {errorPendiente} <button onClick={leerPendiente}>Volver a leer solicitud</button>
          </p>
        )}
        {pendiente && !form && (
          <div className="caja-alerta caja-linea" role="status">
            <span>
              Hay una operación sin confirmar. Recupera su resultado para seguir registrando
              movimientos.
            </span>
            <button disabled={ocupado} onClick={() => void reintentar()}>
              {ocupado ? 'Recuperando…' : 'Reintentar operación'}
            </button>
          </div>
        )}
        {inicio?.recuperacion && (
          <p className="caja-alerta">
            Hay una recuperación pendiente. Solo el administrador puede consultar o cerrar el turno
            recuperado.
          </p>
        )}
        <nav className="caja-tabs" aria-label="Vistas de Caja Chica">
          <button
            aria-current={vista === 'mi' ? 'page' : undefined}
            onClick={() => cambiarVista('mi')}
          >
            <Wallet size={17} /> Mi caja
          </button>
          {!inicio?.recuperacion && (
            <button
              aria-current={vista === 'historial' ? 'page' : undefined}
              onClick={() => cambiarVista('historial')}
            >
              <History size={17} /> Historial
            </button>
          )}
        </nav>
        {leyendo && <p role="status">Actualizando caja…</p>}
        {!inicio && !leyendo && (
          <button onClick={() => void actualizar()}>Volver a consultar</button>
        )}
        {vista === 'historial' && inicio && !inicio.recuperacion && (
          <div hidden={!!detalle}>
            <CajaHistorial
              inicio={inicio}
              admin={admin}
              revision={revision}
              onAbrir={(n) => void ver(n)}
            />
          </div>
        )}
        {turno && (
          <>
            {detalle && (
              <button className="caja-volver" onClick={() => setDetalle(null)}>
                <ArrowLeft size={16} />{' '}
                {vista === 'historial' ? 'Volver al historial' : 'Volver a mi caja'}
              </button>
            )}
            <section className="caja-panel caja-linea">
              <div>
                <div className="caja-acciones">
                  <h2>{turno.turno.codigo_caja}</h2>
                  <span className="caja-sello" data-abierta={turno.turno.estado === 'ABIERTA'}>
                    {turno.turno.por_conciliar && turno.turno.estado === 'ABIERTA'
                      ? 'Por conciliar'
                      : turno.turno.estado === 'ABIERTA'
                        ? 'Abierta'
                        : 'Cerrada'}
                  </span>
                </div>
                <p>
                  {turno.turno.titular} · {turno.turno.ambiente} · Apertura{' '}
                  {fechaCaja(turno.turno.fecha_apertura)}
                </p>
              </div>
              <div className="caja-acciones">
                {turno.puede_mover && (
                  <button
                    disabled={bloqueado}
                    className="caja-principal"
                    onClick={() => seleccionar({ tipo: 'manual', detalle: turno })}
                  >
                    <Plus size={16} /> Registrar movimiento
                  </button>
                )}
                {turno.modo_cierre && (
                  <button
                    disabled={bloqueado}
                    onClick={() => seleccionar({ tipo: 'cerrar', detalle: turno })}
                  >
                    {turno.modo_cierre === 'AUSENCIA'
                      ? 'Cerrar por ausencia'
                      : turno.modo_cierre === 'RECUPERACION'
                        ? 'Cerrar turno recuperado'
                        : 'Cerrar turno'}
                  </button>
                )}
              </div>
            </section>
            <CajaResumenTurno
              detalle={bloqueado ? { ...turno, puede_aclarar: false } : turno}
              onAclarar={() => seleccionar({ tipo: 'aclarar', detalle: turno })}
            />
            <CajaMovimientos
              key={turno.turno.id_caja}
              detalle={bloqueado ? { ...turno, puede_mover: false } : turno}
              revision={revision}
              onVenta={abrirVenta}
              onRevertir={(movimiento) => seleccionar({ tipo: 'revertir', movimiento })}
            />
          </>
        )}
        {inicio && vista === 'mi' && !turno && (
          <section className="caja-panel caja-vacio">
            <Wallet size={29} aria-hidden="true" />
            <h2>
              {inicio.ocupada
                ? 'Caja ocupada'
                : inicio.recuperacion
                  ? 'Sin turno disponible'
                  : 'Sin turno abierto'}
            </h2>
            <p>
              {inicio.ocupada
                ? 'Espera a que el turno activo se cierre para abrir el tuyo.'
                : inicio.recuperacion
                  ? 'La apertura estará disponible al finalizar la recuperación.'
                  : 'Abre tu turno indicando el fondo recibido para comenzar a cobrar.'}
            </p>
            {inicio.puede_abrir && (
              <button
                className="caja-principal"
                disabled={bloqueado}
                onClick={() =>
                  seleccionar({
                    tipo: 'abrir',
                    fondo: inicio.fondo_sugerido_cent,
                    ambiente: inicio.ambiente,
                    titular: usuario?.nombre_completo ?? '',
                  })
                }
              >
                <Plus size={17} /> Abrir turno
              </button>
            )}
          </section>
        )}
      </div>
      {form && (
        <CajaFormulario
          form={form}
          ocupado={ocupado}
          error={errorForm}
          pendiente={!!pendiente}
          onCancelar={() => seleccionar(null)}
          onDirty={setHayCambiosSinGuardar}
          onEnviar={guardar}
          onReintentar={reintentar}
        />
      )}
      {aviso && (
        <div className="caja-aviso" role="status">
          <ToastNotification mensaje={aviso} tipo="EXITO" onCerrar={cerrarAviso} />
        </div>
      )}
    </section>
  );
}
