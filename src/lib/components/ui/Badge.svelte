<script lang="ts">
  import type { Snippet } from 'svelte';

  let {
    children,
    variant = 'default',
    class: className = ''
  }: {
    children?: Snippet;
    variant?: 'default' | 'success' | 'warning' | 'error' | 'info';
    class?: string;
  } = $props();
</script>

<span class="badge {variant} text-xs font-semibold {className}">
  {#if children}
    {@render children()}
  {/if}
</span>

<style>
  .badge {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 2px var(--space-2);
    border-radius: var(--radius-full);
    border: 1px solid transparent;
    text-transform: capitalize;
  }

  .badge.default {
    background-color: var(--bg-hover);
    color: var(--text-secondary);
    border-color: var(--border-default);
  }

  .badge.success {
    background-color: rgba(34, 197, 94, 0.15);
    color: var(--success);
  }

  .badge.warning {
    background-color: rgba(245, 158, 11, 0.15);
    color: var(--warning);
  }

  .badge.error {
    background-color: rgba(239, 68, 68, 0.15);
    color: var(--error);
  }

  .badge.info {
    background-color: rgba(59, 130, 246, 0.15);
    color: var(--info);
  }
</style>
