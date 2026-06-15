<script lang="ts">
  import TitleBar from './TitleBar.svelte';
  import Sidebar from './Sidebar.svelte';
  import StatusBar from './StatusBar.svelte';

  let { children } = $props();

  // Settings states
  let isCollapsed = $state(false);
  let sidebarWidth = $state(240);
  let isResizing = $state(false);

  function handleMouseDown(e: MouseEvent) {
    if (isCollapsed) return;
    isResizing = true;
    window.addEventListener('mousemove', handleMouseMove);
    window.addEventListener('mouseup', handleMouseUp);
  }

  function handleMouseMove(e: MouseEvent) {
    if (!isResizing) return;
    // Limit width between 160px and 400px
    const newWidth = Math.max(160, Math.min(400, e.clientX));
    sidebarWidth = newWidth;
  }

  function handleMouseUp() {
    isResizing = false;
    window.removeEventListener('mousemove', handleMouseMove);
    window.removeEventListener('mouseup', handleMouseUp);
  }
</script>

<div class="app-shell" class:resizing={isResizing}>
  <div class="titlebar-container">
    <TitleBar />
  </div>

  <div class="sidebar-container">
    <Sidebar bind:isCollapsed={isCollapsed} bind:sidebarWidth={sidebarWidth} />
    {#if !isCollapsed}
      <!-- Resizer bar -->
      <div 
        class="resizer" 
        role="separator" 
        tabindex="-1"
        aria-label="Resize sidebar"
        onmousedown={handleMouseDown}
      ></div>
    {/if}
  </div>

  <main class="workspace">
    {@render children()}
  </main>

  <div class="statusbar-container">
    <StatusBar />
  </div>
</div>

<style>
  .app-shell {
    display: grid;
    grid-template-rows: 38px 1fr 24px;
    grid-template-columns: auto 1fr;
    height: 100vh;
    width: 100vw;
    background-color: var(--bg-base);
    color: var(--text-primary);
    overflow: hidden;
  }

  .app-shell.resizing {
    cursor: col-resize;
    user-select: none;
    -webkit-user-select: none;
  }

  .titlebar-container {
    grid-row: 1;
    grid-column: 1 / span 2;
    z-index: 100;
  }

  .sidebar-container {
    grid-row: 2;
    grid-column: 1;
    display: flex;
    position: relative;
    z-index: 50;
  }

  .resizer {
    width: 4px;
    height: 100%;
    position: absolute;
    right: -2px;
    top: 0;
    cursor: col-resize;
    z-index: 60;
    transition: background-color var(--duration-fast) var(--ease-out);
  }

  .resizer:hover, .app-shell.resizing .resizer {
    background-color: var(--accent);
  }

  .workspace {
    grid-row: 2;
    grid-column: 2;
    display: flex;
    flex-direction: column;
    overflow: hidden;
    background-color: var(--canvas-bg);
    position: relative;
    min-width: 0;
    min-height: 0;
  }

  .statusbar-container {
    grid-row: 3;
    grid-column: 1 / span 2;
    z-index: 100;
  }
</style>
