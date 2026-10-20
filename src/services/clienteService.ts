import { invoke } from '@tauri-apps/api/core';
import { ClienteDTO, CrearClienteDTO, EditarClienteDTO } from '../types/cliente';

export const clienteService = {
  // Registrar un cliente de forma independiente
  registrarCliente: async (payload: CrearClienteDTO): Promise<number> => {
    return await invoke<number>('registrar_cliente_cmd', { payload });
  },

  // Actualizar datos de un cliente existente
  actualizarCliente: async (payload: EditarClienteDTO): Promise<void> => {
    return await invoke<void>('actualizar_cliente_cmd', { payload });
  },

  // Buscar cliente evaluando conjuntamente tipo y número de documento
  buscarClientePorDocumento: async (
    tipoDocumento: string,
    numeroDocumento: string
  ): Promise<ClienteDTO> => {
    return await invoke<ClienteDTO>('buscar_cliente_por_doc_cmd', {
      tipoDocumento: tipoDocumento.trim().toUpperCase(),
      numeroDocumento: numeroDocumento.trim(),
    });
  },

  // Buscar clientes por coincidencia predictiva de término (Nombre / RUC / DNI)
  buscarClientesPorTermino: async (termino: string): Promise<ClienteDTO[]> => {
    return await invoke<ClienteDTO[]>('buscar_clientes_por_termino_cmd', {
      termino: termino.trim(),
    });
  },
};
