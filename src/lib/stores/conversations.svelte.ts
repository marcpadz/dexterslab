import type { Conversation, Message, Project } from '../types';
import { 
  listProjects as ipcListProjects,
  createProject as ipcCreateProject,
  deleteProject as ipcDeleteProject,
  updateProject as ipcUpdateProject,
  assignConversationToProject as ipcAssignConversation
} from '../ipc/projects';
import {
  listConversations as ipcListConversations,
  upsertConversation as ipcUpsertConversation,
  deleteConversation as ipcDeleteConversation
} from '../ipc/conversations';
import { invokeCommand } from '../ipc/client';

class ConversationStore {
  threads = $state<Conversation[]>([]);
  projects = $state<Project[]>([]);
  activeThreadId = $state<string | null>(null);
  activeProjectId = $state<string | null>(null); // null means "General/Standalone" chats
  activeMessages = $state<Message[]>([]);
  isStreaming = $state(false);
  activePanel = $state<'artifact' | 'code' | 'knowledge' | 'skills' | 'mcp' | 'graph'>('artifact');
  pendingTokens = $state('');
  userId = 'default_user';

  async initUser() {
    try {
      await invokeCommand('upsert_user', {
        user: {
          id: this.userId,
          email: 'default@dexter.local',
          premium_tier: 'free',
          local_token_count: 0,
          last_sync: null
        }
      });
    } catch (e) {
      console.error("Failed to initialize user:", e);
    }
  }

  async loadConversations() {
    try {
      await this.initUser();
      this.threads = await ipcListConversations(this.userId);
    } catch (e) {
      console.error("Failed to load conversations:", e);
    }
  }

  setThreads(threadsList: Conversation[]) {
    this.threads = threadsList;
  }

  setActiveThread(id: string | null) {
    this.activeThreadId = id;
    this.pendingTokens = '';
  }

  setMessages(messagesList: Message[]) {
    this.activeMessages = messagesList;
  }

  appendPendingToken(token: string) {
    this.pendingTokens += token;
  }

  clearPendingTokens() {
    this.pendingTokens = '';
  }

  // Projects store operations
  async loadProjects() {
    try {
      this.projects = await ipcListProjects();
    } catch (e) {
      console.error("Failed to load projects:", e);
    }
  }

  async createNewProject(name: string, instructions?: string | null) {
    try {
      const id = 'proj_' + Math.random().toString(36).substring(2, 11);
      await ipcCreateProject(id, name, instructions);
      await this.loadProjects();
      return id;
    } catch (e) {
      console.error("Failed to create project:", e);
      throw e;
    }
  }

  async updateProject(id: string, name: string, instructions?: string | null) {
    try {
      await ipcUpdateProject(id, name, instructions);
      await this.loadProjects();
    } catch (e) {
      console.error("Failed to update project:", e);
      throw e;
    }
  }

  async deleteProject(id: string) {
    try {
      await ipcDeleteProject(id);
      await this.loadProjects();
      
      // Update local threads references
      this.threads = this.threads.map(t => 
        t.project_id === id ? { ...t, project_id: null } : t
      );

      if (this.activeProjectId === id) {
        this.activeProjectId = null;
      }
    } catch (e) {
      console.error("Failed to delete project:", e);
    }
  }

  async assignConversation(convId: string, projectId: string | null) {
    try {
      await ipcAssignConversation(convId, projectId);
      this.threads = this.threads.map(t => 
        t.id === convId ? { ...t, project_id: projectId } : t
      );
    } catch (e) {
      console.error("Failed to assign conversation:", e);
    }
  }

  async createNewThread(title: string, projectId: string | null = null) {
    try {
      const id = 'conv_' + Math.random().toString(36).substring(2, 11);
      const newConv: Conversation = {
        id,
        title,
        user_id: this.userId,
        model_id: 'GPT-4o',
        project_id: projectId || this.activeProjectId
      };
      await ipcUpsertConversation(newConv);
      await this.loadConversations();
      this.activeThreadId = id;
      return id;
    } catch (e) {
      console.error("Failed to create new thread:", e);
      throw e;
    }
  }

  async deleteThread(id: string) {
    try {
      await ipcDeleteConversation(id);
      await this.loadConversations();
      if (this.activeThreadId === id) {
        this.activeThreadId = null;
      }
    } catch (e) {
      console.error("Failed to delete thread:", e);
    }
  }

  setActiveProject(id: string | null) {
    this.activeProjectId = id;
    this.activeThreadId = null; // Reset thread selection when changing projects
  }

  // Svelte 5 reactive getter for filtered threads list
  get filteredThreads() {
    return this.threads.filter(t => {
      if (this.activeProjectId === null) {
        return !t.project_id;
      }
      return t.project_id === this.activeProjectId;
    });
  }
}

export const conversationStore = new ConversationStore();
