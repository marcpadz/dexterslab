import { invokeCommand } from './client';
import type { Conversation } from '../types';

export async function upsertConversation(conv: Conversation): Promise<void> {
  return invokeCommand<void>('upsert_conversation', { conv });
}

export async function deleteConversation(id: string): Promise<void> {
  return invokeCommand<void>('delete_conversation', { id });
}

export async function listConversations(userId: string): Promise<Conversation[]> {
  return invokeCommand<Conversation[]>('list_conversations', { userId });
}
