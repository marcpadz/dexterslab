class SidecarStore {
  piSdk = $state<{
    status: 'stopped' | 'starting' | 'running' | 'error';
    lastHeartbeat: Date | null;
  }>({
    status: 'stopped',
    lastHeartbeat: null
  });

  llamaServer = $state<{
    status: 'stopped' | 'starting' | 'ready' | 'error';
    model: string | null;
  }>({
    status: 'stopped',
    model: null
  });

  updatePiSdk(status: 'stopped' | 'starting' | 'running' | 'error', heartbeat = false) {
    this.piSdk.status = status;
    if (heartbeat) {
      this.piSdk.lastHeartbeat = new Date();
    }
  }

  updateLlamaServer(status: 'stopped' | 'starting' | 'ready' | 'error', model: string | null = null) {
    this.llamaServer.status = status;
    if (model !== null) {
      this.llamaServer.model = model;
    }
  }
}

export const sidecarStore = new SidecarStore();
