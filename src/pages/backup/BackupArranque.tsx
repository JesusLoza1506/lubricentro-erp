import { useState } from 'react';
import { DatabaseBackup, TriangleAlert } from 'lucide-react';
import { errorBackup, recuperarArranqueBackup, type BackupArranque } from './backupService';
import { useProgresoBackup } from './useProgresoBackup';

export function RecuperarArranque({ datos }: { datos: BackupArranque }) {
  const [usuario, setUsuario] = useState('');
  const [clave, setClave] = useState('');
  const [frase, setFrase] = useState('');
  const [confirmado, setConfirmado] = useState(false);
  const [ocupado, setOcupado] = useState(false);
  const [error, setError] = useState('');
  const progreso = useProgresoBackup(ocupado);
  return (
    <div className="gloria-theme bkp-page">
      <main className="bkp-stage">
        <header className="bkp-header">
          <div><p className="bkp-kicker">Recuperación del sistema · {datos.ambiente}</p>
            <h1>No se pudo abrir la base activa</h1>
            <p>La operación del taller está bloqueada mientras se resuelve el arranque.</p>
          </div>
          <span className="bkp-icono bkp-icono--hero"><DatabaseBackup size={24} aria-hidden="true" /></span>
        </header>
        {datos.motivo && <div className="gt-notice" role="status"><TriangleAlert size={17} aria-hidden="true" />{datos.motivo}</div>}
        {error && <div className="gt-notice gt-notice--error" role="alert">{error}</div>}
        {ocupado && <p className="bkp-progreso" role="status">{progreso || 'Comprobando la copia previa…'} Mantén abierta la aplicación.</p>}
        {datos.previa ? <form className="bkp-panel bkp-form" onSubmit={async (e) => {
          e.preventDefault();
          if (!confirmado || ocupado) return;
          setOcupado(true);
          setError('');
          try {
            await recuperarArranqueBackup({ usuario, clave, frase, confirmado });
            window.location.reload();
          } catch (e) {
            setError(errorBackup(e));
          } finally {
            setClave(''); setFrase(''); setConfirmado(false); setOcupado(false);
          }
        }}>
          <h2>Recuperar desde la copia previa registrada</h2>
          <p>Usa la frase original de la copia y una cuenta de administrador contenida en ella. Tras recuperar, concilia las series antes de volver a vender.</p>
          <label>Usuario administrador<input className="gt-input" required autoComplete="username" value={usuario} onChange={(e) => setUsuario(e.target.value)} /></label>
          <label>Contraseña del administrador<input className="gt-input" required type="password" autoComplete="current-password" value={clave} onChange={(e) => setClave(e.target.value)} /></label>
          <label>Frase original<input className="gt-input" required type="password" autoComplete="off" value={frase} onChange={(e) => setFrase(e.target.value)} /></label>
          <label className="bkp-check"><input type="checkbox" checked={confirmado} onChange={(e) => setConfirmado(e.target.checked)} />Confirmo volver a los datos de la copia previa.</label>
          <button className="gt-button gt-button--primary" disabled={ocupado || !confirmado}>Verificar y recuperar</button>
        </form> : <section className="bkp-panel">
          <h2>No hay una copia previa registrada disponible</h2>
          <p>Conserva la carpeta de datos y los respaldos para que un administrador pueda revisar la recuperación. No se creará una base vacía.</p>
        </section>}
      </main>
    </div>
  );
}
