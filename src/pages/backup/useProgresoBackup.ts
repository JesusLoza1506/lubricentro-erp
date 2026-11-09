import { useEffect, useState } from 'react';
import { etapasBackup, progresoBackup } from './backupService';

export function useProgresoBackup(ocupado: boolean) {
  const [etapa, setEtapa] = useState('');
  useEffect(() => {
    if (!ocupado) { setEtapa(''); return; }
    let vigente = true;
    let consultando = false;
    const consultar = async () => {
      if (consultando) return;
      consultando = true;
      try {
        const { fase, no_cerrar } = await progresoBackup();
        if (vigente) setEtapa(`${etapasBackup[fase] ?? ''}${no_cerrar && fase ? ' Espera antes de cerrar la ventana.' : ''}`);
      } catch { /* El mensaje de la operación sigue visible. */ }
      finally { consultando = false; }
    };
    // Las operaciones breves no necesitan consultar el progreso nativo.
    const timer = window.setInterval(() => void consultar(), 350);
    return () => { vigente = false; window.clearInterval(timer); };
  }, [ocupado]);
  return etapa;
}
