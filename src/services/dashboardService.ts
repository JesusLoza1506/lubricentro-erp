import { invoke } from '@tauri-apps/api/core';

export type PuntoDashboard = { periodo: string; valor: number };
type BaseDashboard = { ambiente: string; fecha: string; actualizado_en: string };

export type DashboardDTO = BaseDashboard & (
  | {
      rol: 'ADMINISTRADOR';
      ventas_netas_hoy_cent: number;
      trabajos_terminados_hoy: number;
      mediana_cobro_seg: number | null;
      ventas_por_dia: PuntoDashboard[];
      trabajos_por_hora: PuntoDashboard[];
    }
  | {
      rol: 'MECANICO';
      trabajos_terminados_hoy: number;
      trabajos_abiertos: number;
      mediana_trabajo_seg: number | null;
      por_estado: { en_espera: number; en_proceso: number; finalizadas_hoy: number };
      trabajos_por_dia: PuntoDashboard[];
    }
  | {
      rol: 'CAJERO';
      turno: { estado: 'PROPIO'; apertura: string } | { estado: 'AJENO' | 'SIN_CAJA' };
      actividad_turno: { cobros: number; cobros_por_hora: PuntoDashboard[] } | null;
      pendientes: {
        total: number;
        mayor_espera_seg: number | null;
        menos_30: number;
        entre_30_y_60: number;
        mas_60: number;
      };
    }
);

export function obtenerDatosDashboard(sessionId: string): Promise<DashboardDTO> {
  return invoke<DashboardDTO>('obtener_dashboard_cmd', { sessionId });
}
