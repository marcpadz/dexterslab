<script lang="ts">
  let {
    items = [],
    activeTab = $bindable(''),
    class: className = ''
  }: {
    items: { id: string; label: string }[];
    activeTab: string;
    class?: string;
  } = $props();
</script>

<div class="tabs-container flex gap-1 p-0.5 {className}" role="tablist">
  {#each items as item}
    <button
      class="tab-item text-xs font-medium focusable"
      class:active={item.id === activeTab}
      role="tab"
      aria-selected={item.id === activeTab}
      onclick={() => activeTab = item.id}
    >
      {item.label}
    </button>
  {/each}
</div>

<style>
  .tabs-container {
    background-color: var(--bg-input);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-md);
    align-self: flex-start;
  }

  .tab-item {
    padding: var(--space-1.5) var(--space-3);
    border-radius: var(--radius-sm);
    color: var(--text-secondary);
    cursor: pointer;
    transition: all var(--duration-fast) var(--ease-out);
  }

  .tab-item:hover {
    color: var(--text-primary);
  }

  .tab-item.active {
    background-color: var(--bg-surface);
    color: var(--text-primary);
    box-shadow: var(--shadow-sm);
  }
</style>
