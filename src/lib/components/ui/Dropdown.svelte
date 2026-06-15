<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import type { Snippet } from 'svelte';

  let {
    trigger,
    content,
    isOpen = $bindable(false)
  }: {
    trigger: Snippet;
    content: Snippet;
    isOpen?: boolean;
  } = $props();

  let containerElement: HTMLDivElement | null = $state(null);

  function toggle() {
    isOpen = !isOpen;
  }

  function handleClickOutside(e: MouseEvent) {
    if (containerElement && !containerElement.contains(e.target as Node)) {
      isOpen = false;
    }
  }

  $effect(() => {
    if (isOpen) {
      window.addEventListener('click', handleClickOutside);
    } else {
      window.removeEventListener('click', handleClickOutside);
    }
  });

  onDestroy(() => {
    window.removeEventListener('click', handleClickOutside);
  });
</script>

<div class="dropdown-container" bind:this={containerElement}>
  <div class="dropdown-trigger" onclick={toggle} role="presentation">
    {@render trigger()}
  </div>

  {#if isOpen}
    <div class="dropdown-content flex flex-col p-1" onclick={() => isOpen = false} role="presentation">
      {@render content()}
    </div>
  {/if}
</div>

<style>
  .dropdown-container {
    position: relative;
    display: inline-block;
  }

  .dropdown-trigger {
    cursor: pointer;
  }

  .dropdown-content {
    position: absolute;
    top: calc(100% + var(--space-1));
    right: 0;
    min-width: 160px;
    background-color: var(--bg-elevated);
    border: 1px solid var(--border-default);
    border-radius: var(--radius-md);
    box-shadow: var(--shadow-md);
    z-index: 500;
    animation: fadeIn var(--duration-fast) var(--ease-out);
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
      transform: translateY(-4px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }
</style>
