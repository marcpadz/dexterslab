export interface User {
  id: string;
  name: string;
  email: string;
  createdAt: string;
}

export interface Conversation {
  id: string;
  title: string;
  user_id?: string;
  model_id?: string;
  project_id?: string | null;
  createdAt?: string;
  updatedAt?: string;
  pinned?: boolean;
}

export interface Project {
  id: string;
  name: string;
  custom_instructions?: string | null;
  created_at?: string | null;
}

export interface Message {
  id: string;
  conversationId: string;
  parentMessageId: string | null;
  role: 'user' | 'assistant' | 'system';
  content: string;
  createdAt: string;
  metadata?: string; // JSON string
}

export interface AppSettings {
  key: string;
  value: string;
}

export interface GraphNode {
  id: string;
  label: string;
  type: string;
  properties?: string; // JSON string
}

export interface GraphEdge {
  id: string;
  source: string;
  target: string;
  type: string;
  properties?: string; // JSON string
}

export interface KnowledgeBase {
  id: string;
  name: string;
  description?: string;
  createdAt: string;
}

export interface KnowledgeFile {
  id: string;
  kbId: string;
  name: string;
  path: string;
  sizeBytes: number;
  status: 'pending' | 'processing' | 'completed' | 'failed';
}

export interface KnowledgeChunk {
  id: string;
  fileId: string;
  content: string;
  embeddingId: string;
}

export interface McpServer {
  id: string;
  name: string;
  transport: 'stdio' | 'http';
  command?: string;
  args?: string; // JSON string
  url?: string;
  status: 'connected' | 'disconnected' | 'error';
}

export interface UserSkill {
  id: string;
  name: string;
  code: string;
  enabled: boolean;
  permissions?: string; // JSON string
}

export interface ModelInfo {
  id: string;
  name: string;
  sizeBytes: number;
  quantization: string;
  downloaded: boolean;
}

export interface HardwareInfo {
  totalRam: number;
  freeRam: number;
  hasGpu: boolean;
  gpuName?: string;
}

export interface SidecarStatus {
  status: 'stopped' | 'starting' | 'running' | 'ready' | 'error';
  lastHeartbeat?: string;
}

export interface AppError {
  code: string;
  message: string;
  details?: string;
}
