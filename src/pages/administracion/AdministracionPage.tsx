import { useEffect, useState } from 'react';
import { Building2, FileText, Repeat2, ShieldCheck, Users, Wrench } from 'lucide-react';
import { useAuth } from '../../context/AuthContext';
import { cambiarAmbiente, type FiltrosUsuarios } from './adminApi';
import { Botones, confirmarDescarteAdmin, DialogoAdmin, EstadoAdmin, EtiquetaAdmin, useOperacion } from './AdminComun';
import { EmpresaAdmin, TallerAdmin } from './EmpresaTallerAdmin';
import { FacturacionAdmin } from './FacturacionAdmin';
import { UsuariosAdmin } from './UsuariosAdmin';
import '../../styles/gloriaTaller.css';
import './administracion.css';

const pestañas = [
  { id: 'usuarios', titulo: 'Usuarios', Icono: Users, Componente: null },
  { id: 'empresa', titulo: 'Empresa', Icono: Building2, Componente: EmpresaAdmin },
  { id: 'facturacion', titulo: 'Facturación', Icono: FileText, Componente: FacturacionAdmin },
  { id: 'taller', titulo: 'Taller', Icono: Wrench, Componente: TallerAdmin },
] as const;
export type PestanaAdmin = (typeof pestañas)[number]['id'];
export function AdministracionPage({
  onNavegar,
  pestana,
  onPestana,
  filtrosUsuarios,
  idOrigenReporte,
  onFiltrosUsuarios,
}: {
  onNavegar: (destino: string) => void;
  pestana: PestanaAdmin;
  onPestana: (destino: PestanaAdmin) => void;
  filtrosUsuarios: FiltrosUsuarios;
  idOrigenReporte?: number;
  onFiltrosUsuarios: (filtros: FiltrosUsuarios) => void;
}) {
  const { usuario, ambienteActual, hayCambiosSinGuardar, setHayCambiosSinGuardar } = useAuth();
  const [cambiar, setCambiar] = useState(false);
  const op = useOperacion();
  useEffect(() => () => setHayCambiosSinGuardar(false), [setHayCambiosSinGuardar]);
  if (usuario?.rol !== 'ADMINISTRADOR') return null;
  const ambiente = usuario.ambiente_actual ?? ambienteActual;
  const destino = ambiente === 'PRUEBA' ? 'PRODUCCION' : 'PRUEBA';
  const nombre = (a: string) => (a === 'PRUEBA' ? 'Prueba' : 'Producción');
  const autorizarSalida = () => confirmarDescarteAdmin(hayCambiosSinGuardar);
  const navegar = (destino: string) => {
    if (autorizarSalida()) {
      setHayCambiosSinGuardar(false);
      onNavegar(destino);
    }
  };
  const actual = pestañas.find((p) => p.id === pestana) ?? pestañas[0];
  const Pantalla = actual.Componente;
  return (
    <main className="gloria-theme adm-pagina">
      <div className="adm-stage gt-stage">
        <header className="adm-cabecera">
          <div>
            <p className="adm-rotulo">Configuración del negocio</p>
            <h1>Administración</h1>
            <p>Gestiona los usuarios, los datos de empresa, la facturación y el taller.</p>
          </div>
          <span className="adm-icono">
            <ShieldCheck size={23} aria-hidden="true" />
          </span>
        </header>
        <section className="adm-contexto" aria-label="Ambiente de trabajo">
          <div className="adm-acciones">
            <EtiquetaAdmin tono={ambiente === 'PRUEBA' ? 'aviso' : undefined}>Ambiente: {nombre(ambiente)}</EtiquetaAdmin>
            <span>{usuario.nombre_completo}</span>
          </div>
          <button
            type="button"
            className="gt-button"
            onClick={() => {
              op.setError('');
              setCambiar(true);
            }}
          >
            <Repeat2 size={17} aria-hidden="true" /> Cambiar ambiente
          </button>
        </section>
        {cambiar && (
          <DialogoAdmin
            titulo={`Cambiar a ${nombre(destino)}`}
            descripcion="La aplicación se reiniciará y pedirá iniciar sesión. Los datos y las credenciales se mantienen separados por ambiente."
            ocupado={op.guardando}
            cerrar={() => setCambiar(false)}
          >
            <form
              className="gt-dialog-form"
              onSubmit={(e) => {
                e.preventDefault();
                if (!autorizarSalida()) return;
                void op.ejecutar(async () => {
                  await cambiarAmbiente(destino);
                }, '');
              }}
            >
              <p>Cierra cualquier turno de Caja y espera a que terminen las tareas activas.</p>
              <EstadoAdmin error={op.error} />
              <div>
                <button type="button" className="gt-button" disabled={op.guardando} onClick={() => navegar('caja')}>
                  Ver Caja
                </button>
              </div>
              <Botones cancelar={() => setCambiar(false)} guardando={op.guardando} texto={`Cambiar a ${nombre(destino)}`} />
            </form>
          </DialogoAdmin>
        )}
        <nav className="adm-pestanas" aria-label="Secciones de Administración">
          {pestañas.map(({ id, titulo, Icono }) => (
            <button
              type="button"
              key={id}
              aria-current={actual.id === id ? 'page' : undefined}
              disabled={op.guardando}
              onClick={() => {
                if (id !== actual.id && autorizarSalida()) {
                  setHayCambiosSinGuardar(false);
                  onPestana(id);
                }
              }}
            >
              <Icono size={18} aria-hidden="true" />
              {titulo}
            </button>
          ))}
        </nav>
        {Pantalla ? (
          <Pantalla key={actual.id} marcar={setHayCambiosSinGuardar} navegar={navegar} />
        ) : (
          <UsuariosAdmin
            marcar={setHayCambiosSinGuardar}
            navegar={navegar}
            filtros={filtrosUsuarios}
            idOrigenReporte={idOrigenReporte}
            onFiltros={onFiltrosUsuarios}
          />
        )}
      </div>
    </main>
  );
}
