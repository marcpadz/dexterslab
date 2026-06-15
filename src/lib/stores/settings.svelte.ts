class SettingsStore {
  theme = $state<'dark' | 'light' | 'system'>('dark');
  fontSize = $state(14);
  sidebarOpen = $state(true);
  sidebarWidth = $state(260);
  artifactPanelOpen = $state(false);
  workspaceRoot = $state<string | null>(null);
  executionTarget = $state<'local' | 'cloud'>('local');
  gitBranch = $state<string | null>(null);

  updateSettings(settings: Partial<{
    theme: 'dark' | 'light' | 'system';
    fontSize: number;
    sidebarOpen: boolean;
    sidebarWidth: number;
    artifactPanelOpen: boolean;
    workspaceRoot: string | null;
    executionTarget: 'local' | 'cloud';
    gitBranch: string | null;
  }>) {
    if (settings.theme !== undefined) this.theme = settings.theme;
    if (settings.fontSize !== undefined) this.fontSize = settings.fontSize;
    if (settings.sidebarOpen !== undefined) this.sidebarOpen = settings.sidebarOpen;
    if (settings.sidebarWidth !== undefined) this.sidebarWidth = settings.sidebarWidth;
    if (settings.artifactPanelOpen !== undefined) this.artifactPanelOpen = settings.artifactPanelOpen;
    if (settings.workspaceRoot !== undefined) this.workspaceRoot = settings.workspaceRoot;
    if (settings.executionTarget !== undefined) this.executionTarget = settings.executionTarget;
    if (settings.gitBranch !== undefined) this.gitBranch = settings.gitBranch;
  }
}

export const settingsStore = new SettingsStore();
