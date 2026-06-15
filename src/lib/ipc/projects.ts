import { invokeCommand } from './client';
import type { Project } from '../types';

export async function createProject(id: string, name: string, customInstructions?: string | null): Promise<void> {
  return invokeCommand<void>('create_project', { id, name, customInstructions });
}

export async function getProject(id: string): Promise<Project | null> {
  return invokeCommand<Project | null>('get_project', { id });
}

export async function updateProject(id: string, name: string, customInstructions?: string | null): Promise<void> {
  return invokeCommand<void>('update_project', { id, name, customInstructions });
}

export async function deleteProject(id: string): Promise<void> {
  return invokeCommand<void>('delete_project', { id });
}

export async function listProjects(): Promise<Project[]> {
  return invokeCommand<Project[]>('list_projects');
}

export async function assignConversationToProject(convId: string, projectId: string | null): Promise<void> {
  return invokeCommand<void>('assign_conversation_to_project', { convId, projectId });
}

export async function linkDocumentToProject(projectId: string, nodeId: string): Promise<void> {
  return invokeCommand<void>('link_document_to_project', { projectId, nodeId });
}

export async function unlinkDocumentFromProject(projectId: string, nodeId: string): Promise<void> {
  return invokeCommand<void>('unlink_document_from_project', { projectId, nodeId });
}

export async function listProjectDocuments(projectId: string): Promise<string[]> {
  return invokeCommand<string[]>('list_project_documents', { projectId });
}
