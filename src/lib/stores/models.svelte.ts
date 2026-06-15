import type { ModelInfo, HardwareInfo } from '../types';

export interface DownloadProgress {
  progress: number;
  speedBytes: number;
  etaSeconds: number;
}

class ModelStore {
  available = $state<ModelInfo[]>([]);
  activeModelId = $state<string | null>(null);
  downloads = $state<Map<string, DownloadProgress>>(new Map());
  hardware = $state<HardwareInfo>({
    totalRam: 0,
    freeRam: 0,
    hasGpu: false
  });

  setAvailable(models: ModelInfo[]) {
    this.available = models;
  }

  setActiveModel(id: string | null) {
    this.activeModelId = id;
  }

  setHardware(info: HardwareInfo) {
    this.hardware = info;
  }

  updateDownload(modelId: string, progress: DownloadProgress) {
    this.downloads.set(modelId, progress);
    // Trigger reactivity in Svelte 5 by re-assigning or mutating state if needed.
    // In Svelte 5, mutating class states works out of the box.
  }

  removeDownload(modelId: string) {
    this.downloads.delete(modelId);
  }
}

export const modelStore = new ModelStore();
