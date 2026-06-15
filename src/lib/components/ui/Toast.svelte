<script lang="ts">
  import { toast } from '$lib/utils/toast.svelte';
  import { Info, CheckCircle2, AlertTriangle, AlertCircle, X } from '@lucide/svelte';

  const icons = {
    info: Info,
    success: CheckCircle2,
    warning: AlertTriangle,
    error: AlertCircle
  };
</script>

<div class="toast-viewport flex flex-col gap-2">
  {#each toast.toasts as item (item.id)}
    {@const Icon = icons[item.type]}
    <div class="toast-item {item.type} flex items-start gap-3 p-3">
      <span class="toast-icon flex items-center justify-center flex-shrink-0">
        <Icon size={16} />
      </span>
      <p class="toast-message text-xs font-medium flex-1">{item.message}</p>
      <button class="toast-close flex items-center justify-center flex-shrink-0" onclick={() => toast.remove(item.id)} aria-label="Dismiss toast">
        <X size={12} />
      </button>
    </div>
  {/each}
</div>

<style>
  .toast-viewport {
    position: fixed;
    top: 40px; /* Below title bar */
    right: var(--space-4);
    z-index: 2000;
    max-width: 360px;
    width: calc(100% - 2 * var(--space-4));
    pointer-events: none;
  }

  .toast-item {
    background-color: var(--bg-elevated);
    border: 1px solid var(--border-default);
    border-radius: var(--radius-md);
    box-shadow: var(--shadow-md);
    pointer-events: auto;
    animation: slideIn var(--duration-fast) var(--ease-spring);
  }

  @keyframes slideIn {
    from {
      opacity: 0;
      transform: translateX(100%) scale(0.9);
    }
    to {
      opacity: 1;
      transform: translateX(0) scale(1);
    }
  }

  .toast-close {
    cursor: pointer;
    color: var(--text-muted);
    border-radius: var(--radius-sm);
    transition: all var(--duration-fast) var(--ease-out);
  }

  .toast-close:hover {
    background-color: var(--bg-hover);
    color: var(--text-primary);
  }

  /* Types */
  .toast-item.info {
    border-left: 3px solid var(--info);
  }
  .toast-item.info .toast-icon {
    color: var(--info);
  }

  .toast-item.success {
    border-left: 3px solid var(--success);
  }
  .toast-item.success .toast-icon {
    color: var(--success);
  }

  .toast-item.warning {
    border-left: 3px solid var(--warning);
  }
  .toast-item.warning .toast-icon {
    color: var(--warning);
  }

  .toast-item.error {
    border-left: 3px solid var(--error);
  }
  .toast-item.error .toast-icon {
    color: var(--error);
  }
</style>
