<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import type { Snippet } from 'svelte';
  import { X } from '@lucide/svelte';

  let {
    isOpen = $bindable(false),
    title = '',
    description = '',
    children,
    footer,
    onClose
  }: {
    isOpen: boolean;
    title?: string;
    description?: string;
    children?: Snippet;
    footer?: Snippet;
    onClose?: () => void;
  } = $props();

  let dialogElement: HTMLDivElement | null = $state(null);
  let previousActiveElement: HTMLElement | null = null;

  function close() {
    isOpen = false;
    if (onClose) onClose();
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape' && isOpen) {
      close();
    }
  }

  $effect(() => {
    if (isOpen) {
      previousActiveElement = document.activeElement as HTMLElement;
      document.body.style.overflow = 'hidden';
      window.addEventListener('keydown', handleKeydown);
      
      // Simple focus trapping
      setTimeout(() => {
        if (dialogElement) {
          const focusables = dialogElement.querySelectorAll('button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])');
          if (focusables.length > 0) {
            (focusables[0] as HTMLElement).focus();
          }
        }
      }, 50);
    } else {
      document.body.style.overflow = '';
      window.removeEventListener('keydown', handleKeydown);
      if (previousActiveElement) {
        previousActiveElement.focus();
      }
    }
  });

  onDestroy(() => {
    document.body.style.overflow = '';
    window.removeEventListener('keydown', handleKeydown);
  });
</script>

{#if isOpen}
  <!-- Backdrop -->
  <div class="dialog-backdrop flex items-center justify-center" onclick={close} role="presentation">
    <!-- Modal container -->
    <div 
      class="dialog-content flex flex-col gap-4" 
      onclick={(e) => e.stopPropagation()} 
      role="dialog" 
      aria-modal="true"
      bind:this={dialogElement}
    >
      <div class="dialog-header flex items-start justify-between gap-4">
        <div class="flex flex-col gap-1">
          {#if title}
            <h2 class="dialog-title text-base font-semibold text-primary">{title}</h2>
          {/if}
          {#if description}
            <p class="dialog-description text-xs text-secondary">{description}</p>
          {/if}
        </div>
        <button class="close-btn focusable flex items-center justify-center" onclick={close} aria-label="Close dialog">
          <X size={14} />
        </button>
      </div>

      {#if children}
        <div class="dialog-body">
          {@render children()}
        </div>
      {/if}

      {#if footer}
        <div class="dialog-footer flex justify-end gap-2">
          {@render footer()}
        </div>
      {/if}
    </div>
  </div>
{/if}

<style>
  .dialog-backdrop {
    position: fixed;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    background-color: rgba(0, 0, 0, 0.5);
    backdrop-filter: blur(4px);
    z-index: 1000;
  }

  .dialog-content {
    background-color: var(--bg-elevated);
    border: 1px solid var(--border-default);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-lg);
    padding: var(--space-4);
    width: 100%;
    max-width: 480px;
    animation: scaleUp var(--duration-fast) var(--ease-out);
  }

  .close-btn {
    width: 20px;
    height: 20px;
    border-radius: var(--radius-sm);
    color: var(--text-muted);
    cursor: pointer;
    transition: all var(--duration-fast) var(--ease-out);
  }

  .close-btn:hover {
    background-color: var(--bg-hover);
    color: var(--text-primary);
  }

  @keyframes scaleUp {
    from {
      transform: scale(0.95);
      opacity: 0;
    }
    to {
      transform: scale(1);
      opacity: 1;
    }
  }
</style>
