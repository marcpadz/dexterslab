<script lang="ts">
  import type { Snippet } from 'svelte';

  let {
    children,
    text = '',
    position = 'top'
  }: {
    children: Snippet;
    text: string;
    position?: 'top' | 'bottom' | 'left' | 'right';
  } = $props();

  let show = $state(false);
</script>

<div 
  class="tooltip-container" 
  onmouseenter={() => show = true}
  onmouseleave={() => show = false}
  onfocusin={() => show = true}
  onfocusout={() => show = false}
  role="presentation"
>
  {@render children()}
  {#if show && text}
    <div class="tooltip-box text-xs font-medium {position}" role="tooltip">
      {text}
    </div>
  {/if}
</div>

<style>
  .tooltip-container {
    position: relative;
    display: inline-block;
  }

  .tooltip-box {
    position: absolute;
    background-color: var(--bg-elevated);
    color: var(--text-primary);
    border: 1px solid var(--border-default);
    padding: var(--space-1) var(--space-2);
    border-radius: var(--radius-sm);
    box-shadow: var(--shadow-sm);
    white-space: nowrap;
    z-index: 1000;
    pointer-events: none;
    animation: fadeIn var(--duration-fast) var(--ease-out);
  }

  /* Positions */
  .tooltip-box.top {
    bottom: calc(100% + 6px);
    left: 50%;
    transform: translateX(-50%);
  }

  .tooltip-box.bottom {
    top: calc(100% + 6px);
    left: 50%;
    transform: translateX(-50%);
  }

  .tooltip-box.left {
    right: calc(100% + 6px);
    top: 50%;
    transform: translateY(-50%);
  }

  .tooltip-box.right {
    left: calc(100% + 6px);
    top: 50%;
    transform: translateY(-50%);
  }

  @keyframes fadeIn {
    from { opacity: 0; }
    to { opacity: 1; }
  }
</style>
