import { invoke } from '@tauri-apps/api/core';

/**
 * Generic type-safe invoke helper.
 */
export async function invokeCommand<T>(cmd: string, args?: Record<string, any>): Promise<T> {
  try {
    return await invoke<T>(cmd, args);
  } catch (error) {
    console.error(`Tauri IPC Command [${cmd}] failed:`, error);
    throw error;
  }
}
