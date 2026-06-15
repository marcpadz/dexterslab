import { invokeCommand } from './client';

export async function getSetting(key: string): Promise<string | null> {
  return invokeCommand<string | null>('get_setting', { key });
}

export async function setSetting(key: string, value: string): Promise<void> {
  return invokeCommand<void>('set_setting', { key, value });
}

export async function setWorkspaceRoot(path: string): Promise<void> {
  return invokeCommand<void>('set_workspace_root', { path });
}

export async function readWorkspaceFile(path: string): Promise<string> {
  return invokeCommand<string>('read_workspace_file', { path });
}

export async function writeWorkspaceFile(path: string, content: string): Promise<void> {
  return invokeCommand<void>('write_workspace_file', { path, content });
}
