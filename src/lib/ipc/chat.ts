import { invokeCommand } from './client';
import type { Message } from '../types';

export async function saveMessage(message: Omit<Message, 'createdAt'>): Promise<void> {
  return invokeCommand<void>('save_message', {
    id: message.id,
    conversationId: message.conversationId,
    parentMessageId: message.parentMessageId,
    role: message.role,
    content: message.content,
    metadata: message.metadata || null
  });
}

export async function getMessages(conversationId: string): Promise<Message[]> {
  return invokeCommand<Message[]>('get_messages', { conversationId });
}
