import { invoke } from '@tauri-apps/api/core';

export interface DashboardDTO {
  ventas_del_dia: number;
  comprobantes_emitidos: number;
  ots_activas: number;
  productos_stock_critico: number;
  estado_caja: string;
}

/**
 * Invoca el comando Tauri `obtener_dashboard_cmd`
 * enviando el ID del usuario autenticado.
 */
export const obtenerDatosDashboard = async (idUsuario: number): Promise<DashboardDTO> => {
  try {
    const res = await invoke<DashboardDTO>('obtener_dashboard_cmd', {
      idUsuario: idUsuario,
    });
    return res;
  } catch (error) {
    console.error('Error al obtener datos reales del dashboard:', error);
    throw new Error(String(error));
  }
};
