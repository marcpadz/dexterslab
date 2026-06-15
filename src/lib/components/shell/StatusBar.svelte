<script lang="ts">
  import { Cpu, Terminal, RefreshCw } from '@lucide/svelte';

  // Component state / props
  let piSdkStatus = $state('running'); // 'stopped' | 'starting' | 'running' | 'error'
  let llamaStatus = $state('ready');  // 'stopped' | 'starting' | 'ready' | 'error'
  let activeModel = $state('llama-3.1-8b-instruct');
  let tokenCount = $state(1240);
  let connectionState = $state('online');
</script>

<div class="statusbar select-text">
  <div class="left-items">
    <!-- Pi SDK sidecar status -->
    <div class="status-item font-mono" title="Pi SDK Sidecar status">
      <Terminal size={11} class="icon" />
      <span>Pi SDK:</span>
      <span class="status-badge {piSdkStatus}">{piSdkStatus}</span>
    </div>

    <div class="separator"></div>

    <!-- Llama server sidecar status -->
    <div class="status-item font-mono" title="llama-server status">
      <Cpu size={11} class="icon" />
      <span>Model Server:</span>
      <span class="status-badge {llamaStatus === 'ready' ? 'running' : llamaStatus}">{llamaStatus}</span>
    </div>
  </div>

  <div class="right-items">
    <!-- Active model -->
    <div class="status-item font-mono text-secondary">
      <span>Model:</span>
      <span class="model-name">{activeModel}</span>
    </div>

    <div class="separator"></div>

    <!-- Session tokens -->
    <div class="status-item font-mono text-secondary">
      <span>Tokens:</span>
      <span>{tokenCount}</span>
    </div>

    <div class="separator"></div>

    <!-- Connection state -->
    <div class="status-item">
      <span class="connection-dot {connectionState}"></span>
      <span class="text-secondary">{connectionState === 'online' ? 'Local' : 'Offline'}</span>
    </div>
  </div>
</div>

<style>
  .statusbar {
    height: 24px;
    background-color: var(--bg-surface);
    border-top: 1px solid var(--border-subtle);
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 var(--space-3);
    font-size: 10px;
    user-select: none;
    -webkit-user-select: none;
  }

  .left-items, .right-items {
    display: flex;
    align-items: center;
    gap: var(--space-3);
  }

  .status-item {
    display: flex;
    align-items: center;
    gap: var(--space-1.5);
    color: var(--text-secondary);
  }

  .icon {
    color: var(--text-muted);
  }

  .separator {
    width: 1px;
    height: 12px;
    background-color: var(--border-subtle);
  }

  .status-badge {
    padding: 0 4px;
    border-radius: 2px;
    font-size: 9px;
    text-transform: uppercase;
    font-weight: 600;
  }

  .status-badge.running, .status-badge.ready {
    background-color: rgba(34, 197, 94, 0.15);
    color: var(--success);
  }

  .status-badge.starting {
    background-color: rgba(245, 158, 11, 0.15);
    color: var(--warning);
  }

  .status-badge.stopped, .status-badge.error {
    background-color: rgba(239, 68, 68, 0.15);
    color: var(--error);
  }

  .model-name {
    font-weight: 500;
    color: var(--text-primary);
  }

  .connection-dot {
    width: 6px;
    height: 6px;
    border-radius: var(--radius-full);
  }

  .connection-dot.online {
    background-color: var(--success);
    box-shadow: 0 0 4px var(--success);
  }

  .connection-dot.offline {
    background-color: var(--text-muted);
  }
</style>
