<script lang="ts">
  import { Search } from '@lucide/svelte';
</script>

<div class="titlebar drag-region" data-tauri-drag-region>
  <div class="traffic-lights-spacer no-drag"></div>
  <div class="title-container no-drag">
    <span class="window-title-badge">
      <svg viewBox="0 0 24 24"><path d="M12 2L2 7l10 5 10-5-10-5z"/><path d="M2 17l10 5 10-5"/><path d="M2 12l10 5 10-5"/></svg>
      Ask Dexter
    </span>
  </div>
  <div class="actions no-drag">
    <button class="search-btn focusable" aria-label="Search" onclick={() => {}}>
      <Search size={13} class="text-secondary" />
      <span class="search-shortcut">⌘K</span>
    </button>
  </div>
</div>

<style>
  .titlebar {
    height: 38px;
    background-color: transparent;
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding-right: var(--space-3);
    user-select: none;
    -webkit-user-select: none;
    position: relative;
    z-index: 100;
  }

  .traffic-lights-spacer {
    width: 80px; /* Space for macOS native traffic lights overlay */
    height: 100%;
  }

  .title-container {
    position: absolute;
    left: 50%;
    transform: translateX(-50%);
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .window-title-badge {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 3px 14px;
    background: var(--sidebar-bg);
    border-radius: var(--radius-full);
    color: white;
    font-size: 12px;
    font-weight: 600;
    letter-spacing: 0.2px;
  }

  .window-title-badge svg {
    width: 14px;
    height: 14px;
    stroke: currentColor;
    fill: none;
    stroke-width: 1.8;
  }

  .actions {
    display: flex;
    align-items: center;
    gap: var(--space-2);
  }

  .search-btn {
    display: flex;
    align-items: center;
    gap: var(--space-1.5);
    background-color: var(--bg-surface);
    border: 1px solid var(--border-default);
    border-radius: var(--radius-md);
    padding: 4px var(--space-2.5);
    cursor: pointer;
    font-size: 11px;
    color: var(--text-secondary);
    transition: all var(--duration-fast) var(--ease-out);
  }

  .search-btn:hover {
    background-color: var(--bg-hover);
    border-color: var(--border-strong);
  }

  .search-shortcut {
    font-size: 9px;
    color: var(--text-muted);
    background-color: var(--bg-elevated);
    padding: 0 4px;
    border-radius: 2px;
    border: 1px solid var(--border-default);
  }
</style>
