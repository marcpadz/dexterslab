<script lang="ts">
  import type { Snippet } from 'svelte';
  import Button from './Button.svelte';

  let {
    title = '',
    description = '',
    icon,
    actionLabel = '',
    onAction
  }: {
    title?: string;
    description?: string;
    icon?: Snippet;
    actionLabel?: string;
    onAction?: () => void;
  } = $props();
</script>

<div class="empty-state-container flex flex-col items-center justify-center gap-4 p-6 text-center">
  {#if icon}
    <div class="icon-wrapper flex items-center justify-center">
      {@render icon()}
    </div>
  {/if}

  <div class="flex flex-col gap-1">
    <h3 class="text-base font-semibold text-primary">{title}</h3>
    {#if description}
      <p class="text-xs text-secondary max-w-sm">{description}</p>
    {/if}
  </div>

  {#if actionLabel && onAction}
    <Button variant="secondary" size="sm" onclick={onAction}>
      {actionLabel}
    </Button>
  {/if}
</div>

<style>
  .empty-state-container {
    width: 100%;
    height: 100%;
    min-height: 240px;
    background-color: var(--bg-base);
    border-radius: var(--radius-md);
  }

  .icon-wrapper {
    width: 48px;
    height: 48px;
    border-radius: var(--radius-full);
    background-color: var(--bg-surface);
    border: 1px solid var(--border-subtle);
    color: var(--text-secondary);
  }
</style>
