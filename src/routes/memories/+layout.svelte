<script lang="ts">
  import { page } from '$app/stores';
  import { Brain, Network } from 'lucide-svelte';

  let { children } = $props();
</script>

<div class="memories-container">
  <div class="memories-nav">
    <div class="nav-title">
      <h1>Memories & Context</h1>
    </div>
    
    <div class="nav-tabs">
      <a href="/memories" class="tab-btn" class:active={$page.url.pathname === '/memories'}>
        <Brain size={16} />
        <span>Audit Table</span>
      </a>
      <a href="/memories/graph" class="tab-btn" class:active={$page.url.pathname === '/memories/graph'}>
        <Network size={16} />
        <span>Knowledge Graph</span>
      </a>
    </div>
  </div>

  <div class="memories-content">
    {@render children()}
  </div>
</div>

<style>
  .memories-container {
    flex: 1;
    display: flex;
    flex-direction: column;
    height: 100%;
    background-color: var(--bg-base);
    color: var(--text-primary);
    overflow: hidden;
  }

  .memories-nav {
    display: flex;
    flex-direction: column;
    border-bottom: 1px solid var(--border-default);
    background-color: var(--bg-surface);
  }

  .nav-title {
    padding: 24px 32px 16px;
  }

  .nav-title h1 {
    font-size: 20px;
    font-weight: 600;
    margin: 0;
  }

  .nav-tabs {
    display: flex;
    gap: 24px;
    padding: 0 32px;
  }

  .tab-btn {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 12px 4px;
    color: var(--text-secondary);
    text-decoration: none;
    font-size: 13px;
    font-weight: 500;
    position: relative;
    transition: color var(--duration-fast);
  }

  .tab-btn:hover {
    color: var(--text-primary);
  }

  .tab-btn.active {
    color: var(--accent);
  }

  .tab-btn.active::after {
    content: '';
    position: absolute;
    bottom: -1px;
    left: 0;
    right: 0;
    height: 2px;
    background-color: var(--accent);
    border-top-left-radius: 2px;
    border-top-right-radius: 2px;
  }

  .memories-content {
    flex: 1;
    overflow: hidden;
    position: relative;
  }
</style>
