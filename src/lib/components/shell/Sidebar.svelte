<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { onMount } from 'svelte';
  import { Plus, Trash2, Folder, Globe } from '@lucide/svelte';
  import { conversationStore } from '$lib/stores/conversations.svelte';

  // Svelte 5 Runes for properties
  let { isCollapsed = $bindable(false), sidebarWidth = $bindable(240) } = $props();

  // Active tab state derived from current SvelteKit route path
  let activeTab = $derived.by(() => {
    const path = $page.url.pathname;
    if (['/skills', '/mcp'].includes(path)) {
      return 'agent';
    } else if (['/apps', '/marketplace'].includes(path)) {
      return 'notes';
    } else if (path === '/work') {
      return 'work';
    } else if (path === '/playground') {
      return 'playground';
    } else {
      return 'chat';
    }
  });
  
  // Theme state
  let currentTheme = $state('dark');

  // Inline project creation state
  let isAddingProject = $state(false);
  let newProjectName = $state('');

  onMount(async () => {
    // Read initial theme
    const savedTheme = document.documentElement.getAttribute('data-theme') || 'dark';
    currentTheme = savedTheme;

    // Load real projects and conversations from DB
    await conversationStore.loadProjects();
    await conversationStore.loadConversations();

    // Seed initial project and thread if database is empty so the UI doesn't look empty at first launch
    if (conversationStore.projects.length === 0) {
      await conversationStore.createNewProject('Data Analytics', 'Always act as a Senior Data Analyst and output SQL.');
      await conversationStore.createNewProject('Web Dev', 'Focus on Svelte 5 and CSS variables.');
    }
    if (conversationStore.threads.length === 0) {
      // Create some default threads
      await conversationStore.createNewThread('React Data Table Component', null); // Standalone
      await conversationStore.createNewThread('API Migration Planning', null); // Standalone
      const projId = conversationStore.projects[0]?.id;
      if (projId) {
        await conversationStore.createNewThread('Query customer revenue metrics', projId);
      }
    }
    
    // Auto-select first thread if none is selected
    if (!conversationStore.activeThreadId && conversationStore.filteredThreads.length > 0) {
      conversationStore.activeThreadId = conversationStore.filteredThreads[0].id;
    }
  });

  function toggleTheme() {
    currentTheme = currentTheme === 'dark' ? 'light' : 'dark';
    document.documentElement.setAttribute('data-theme', currentTheme);
  }

  function toggleCollapse() {
    isCollapsed = !isCollapsed;
  }

  async function handleAddProject(e: KeyboardEvent | MouseEvent) {
    if (e.type === 'keydown' && (e as KeyboardEvent).key !== 'Enter') return;
    if (!newProjectName.trim()) return;

    try {
      const id = await conversationStore.createNewProject(newProjectName);
      conversationStore.setActiveProject(id);
      isAddingProject = false;
      newProjectName = '';
      goto('/');
    } catch (err) {
      console.error(err);
    }
  }

  function handleNewChat() {
    if (activeTab === 'notes') {
      goto('/notes/recent');
    } else if (activeTab === 'agent') {
      goto('/skills');
    } else {
      conversationStore.createNewThread('New Chat');
      goto('/');
    }
  }

  function focusOnMount(node: HTMLInputElement) {
    node.focus();
  }
</script>

<aside class="sidebar" class:collapsed={isCollapsed} style="--sidebar-width: {sidebarWidth}px">
  
  <!-- Tabbed Navigation Header -->
  <div class="sidebar-tabs">
    <button class="tab-item" class:active={activeTab === 'chat'} onclick={() => { activeTab = 'chat'; goto('/'); }} title="Chat">
      <span class="tab-icon">
        <svg viewBox="0 0 24 24"><path d="M21 11.5a8.38 8.38 0 0 1-.9 3.8 8.5 8.5 0 0 1-7.6 4.7 8.38 8.38 0 0 1-3.8-.9L3 21l1.9-5.7a8.38 8.38 0 0 1-.9-3.8 8.5 8.5 0 0 1 4.7-7.6 8.38 8.38 0 0 1 3.8-.9h.5a8.48 8.48 0 0 1 8 8v.5z"/><line x1="8" y1="10" x2="8.01" y2="10" stroke-linecap="round"/><line x1="12" y1="10" x2="12.01" y2="10" stroke-linecap="round"/><line x1="16" y1="10" x2="16.01" y2="10" stroke-linecap="round"/></svg>
      </span>
    </button>
    <button class="tab-item" class:active={activeTab === 'agent'} onclick={() => { activeTab = 'agent'; goto('/skills'); }} title="Agent">
      <span class="tab-icon">
        <svg viewBox="0 0 24 24"><path d="M12 2a2 2 0 0 1 2 2c0 .74-.4 1.39-1 1.73V7h1a7 7 0 0 1 7 7v1H3v-1a7 7 0 0 1 7-7h1V5.73c-.6-.34-1-.99-1-1.73a2 2 0 0 1 2-2z"/><path d="M5 15v4a2 2 0 0 0 2 2h10a2 2 0 0 0 2-2v-4"/><circle cx="9" cy="13" r="1" fill="currentColor" stroke="none"/><circle cx="15" cy="13" r="1" fill="currentColor" stroke="none"/><path d="M9 17h6"/></svg>
      </span>
    </button>
    <button class="tab-item" class:active={activeTab === 'notes'} onclick={() => { activeTab = 'notes'; goto('/notes/recent'); }} title="Notes">
      <span class="tab-icon">
        <svg viewBox="0 0 24 24"><path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"/><polyline points="14 2 14 8 20 8"/><line x1="8" y1="12" x2="16" y2="12"/><line x1="8" y1="16" x2="14" y2="16"/><line x1="8" y1="8" x2="10" y2="8"/></svg>
      </span>
    </button>
    <button class="tab-item" class:active={activeTab === 'work'} onclick={() => { activeTab = 'work'; goto('/work'); }} title="Work">
      <span class="tab-icon">
        <svg viewBox="0 0 24 24"><rect x="2" y="7" width="20" height="14" rx="2" ry="2"/><path d="M16 21V5a2 2 0 0 0-2-2h-4a2 2 0 0 0-2 2v16"/></svg>
      </span>
    </button>
    <button class="tab-item" class:active={activeTab === 'playground'} onclick={() => { activeTab = 'playground'; goto('/playground'); }} title="Playground">
      <span class="tab-icon">
        <svg viewBox="0 0 24 24"><circle cx="12" cy="12" r="10"/><path d="M8 14s1.5 2 4 2 4-2 4-2"/><line x1="9" y1="9" x2="9.01" y2="9"/><line x1="15" y1="9" x2="15.01" y2="9"/></svg>
      </span>
    </button>
  </div>
 
  <!-- Sidebar Main Nav (Always visible, changes actions dynamically) -->
  <div class="sidebar-main-nav">
    <button class="new-chat-btn" onclick={handleNewChat}>
      <span class="nc-icon">
        <Plus size={14} />
      </span>
      <span class="new-chat-label">
        {activeTab === 'notes' ? 'New Note' : activeTab === 'agent' ? 'New Agent' : 'New Chat'}
      </span>
    </button>

    {#if activeTab === 'chat'}

      <a href="/projects" class="nav-item" class:active={$page.url.pathname === '/projects'}>
        <span class="nav-icon"><svg viewBox="0 0 24 24"><path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/></svg></span>
        <span class="nav-label">Projects</span>
      </a>
      <a href="/files" class="nav-item" class:active={$page.url.pathname === '/files'}>
        <span class="nav-icon"><svg viewBox="0 0 24 24"><path d="M13 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V9z"/><polyline points="13 2 13 9 20 9"/></svg></span>
        <span class="nav-label">Files</span>
      </a>
      <a href="/memories" class="nav-item" class:active={$page.url.pathname.startsWith('/memories')}>
        <span class="nav-icon"><svg viewBox="0 0 24 24"><circle cx="12" cy="12" r="10"/><circle cx="12" cy="12" r="6"/><circle cx="12" cy="12" r="2"/></svg></span>
        <span class="nav-label">Memories</span>
      </a>
    {/if}
  </div>

  <!-- Sidebar Sub-Nav Container -->
  <nav class="sidebar-nav">
    {#if activeTab === 'chat'}
      <div class="tab-content">
        <!-- Main Chat Actions Section -->
        <div class="nav-section">
          <!-- New Chat -->
          <a href="/" class="nav-item" class:active={$page.url.pathname === '/'}>
            <span class="nav-icon"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/></svg></span>
            <span class="nav-label">New Chat</span>
          </a>
          
          <!-- Library -->
          <a href="/library" class="nav-item" class:active={$page.url.pathname === '/library'}>
            <span class="nav-icon"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20"/><path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z"/></svg></span>
            <span class="nav-label">Library</span>
          </a>
          
          <!-- Projects -->
          <a href="/projects" class="nav-item" class:active={$page.url.pathname === '/projects'}>
            <span class="nav-icon"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/></svg></span>
            <span class="nav-label">Projects</span>
          </a>
          
          <!-- Files -->
          <a href="/files" class="nav-item" class:active={$page.url.pathname === '/files'}>
            <span class="nav-icon"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"/><polyline points="14 2 14 8 20 8"/></svg></span>
            <span class="nav-label">Files</span>
          </a>
          
          <!-- All Chats -->
          <a href="/all-chats" class="nav-item" class:active={$page.url.pathname === '/all-chats'}>
            <span class="nav-icon"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"/></svg></span>
            <span class="nav-label">All Chats</span>
          </a>
        </div>

        <!-- Pinned Section -->
        <div class="nav-section">
          <div class="section-label">Pinned</div>
          {#each conversationStore.filteredThreads.filter(t => t.pinned) as thread}
            <div class="thread-item-wrapper" style="position: relative; display: flex; align-items: center; width: 100%;">
              <button 
                class="thread-item" 
                class:active={thread.id === conversationStore.activeThreadId}
                onclick={() => { conversationStore.setActiveThread(thread.id); goto('/'); }}
                style="flex: 1; text-align: left; padding-right: 28px; background: transparent; border: none; padding: 6px 12px; margin-bottom: 2px; width: 100%; border-radius: var(--radius-md); color: var(--sidebar-text); cursor: pointer;"
              >
                <div class="thread-item-title" style="overflow: hidden; text-overflow: ellipsis; white-space: nowrap; font-size: 12px; font-weight: 500;">
                  {thread.title}
                </div>
                <div class="thread-item-meta" style="font-size: 10px; color: var(--sidebar-text-muted);">
                  {thread.model_id}
                </div>
              </button>
            </div>
          {/each}
        </div>

        <!-- Recent Section -->
        <div class="nav-section">
          <div class="section-label">Recent</div>
          {#if conversationStore.filteredThreads.filter(t => !t.pinned).length === 0}
            <div style="font-size: 11px; color: var(--sidebar-text-muted); padding: 8px 12px; font-style: italic;">
              No recent chats.
            </div>
          {:else}
            {#each conversationStore.filteredThreads.filter(t => !t.pinned) as thread}
              <div class="thread-item-wrapper" style="position: relative; display: flex; align-items: center; width: 100%;">
                <button 
                  class="thread-item" 
                  class:active={thread.id === conversationStore.activeThreadId}
                  onclick={() => { conversationStore.setActiveThread(thread.id); goto('/'); }}
                  style="flex: 1; text-align: left; padding-right: 28px; background: transparent; border: none; padding: 6px 12px; margin-bottom: 2px; width: 100%; border-radius: var(--radius-md); color: var(--sidebar-text); cursor: pointer;"
                >
                  <div class="thread-item-title" style="overflow: hidden; text-overflow: ellipsis; white-space: nowrap; font-size: 12px; font-weight: 500;">
                    {thread.title}
                  </div>
                  <div class="thread-item-meta" style="font-size: 10px; color: var(--sidebar-text-muted);">
                    {thread.model_id}
                  </div>
                </button>
              </div>
            {/each}
          {/if}
        </div>
      </div>

    {:else if activeTab === 'agent'}
      <div class="tab-content">
        <div class="nav-section">
          <a href="/skills" class="nav-item" class:active={$page.url.pathname === '/skills'}>
            <span class="nav-icon"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8"><rect width="18" height="18" x="3" y="3" rx="2" ry="2"/><line x1="9" x2="15" y1="3" y2="3"/><line x1="9" x2="15" y1="21" y2="21"/><line x1="3" x2="3" y1="9" y2="15"/><line x1="21" x2="21" y1="9" y2="15"/></svg></span>
            <span class="nav-label">Skills</span>
            <span class="badge">5</span>
          </a>
          <a href="/apps" class="nav-item" class:active={$page.url.pathname === '/apps'}>
            <span class="nav-icon"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8"><rect x="3" y="3" width="7" height="7" rx="1"/><rect x="14" y="3" width="7" height="7" rx="1"/><rect x="3" y="14" width="7" height="7" rx="1"/><rect x="14" y="14" width="7" height="7" rx="1"/></svg></span>
            <span class="nav-label">Apps</span>
          </a>
          <a href="/marketplace" class="nav-item" class:active={$page.url.pathname === '/marketplace'}>
            <span class="nav-icon"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8"><path d="M6 2L3 6v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V6l-3-4z"/><line x1="3" x2="21" y1="6" y2="6"/><path d="M16 10a4 4 0 0 1-8 0"/></svg></span>
            <span class="nav-label">Marketplace</span>
          </a>
          <a href="/connectors" class="nav-item" class:active={$page.url.pathname === '/connectors'}>
            <span class="nav-icon"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8"><circle cx="12" cy="12" r="2"/><path d="M16.24 7.76a6 6 0 0 1 0 8.49"/><path d="M19.07 4.93a10 10 0 0 1 0 14.14"/><path d="M7.76 16.24a6 6 0 0 1 0-8.49"/><path d="M4.93 19.07a10 10 0 0 1 0-14.14"/></svg></span>
            <span class="nav-label">Connectors</span>
          </a>
          <a href="/aihub" class="nav-item" class:active={$page.url.pathname === '/aihub'}>
            <span class="nav-icon"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8"><rect x="4" y="4" width="16" height="16" rx="2"/><rect x="9" y="9" width="6" height="6" rx="1"/><path d="M9 1v3"/><path d="M15 1v3"/><path d="M9 20v3"/><path d="M15 20v3"/><path d="M20 9h3"/><path d="M20 15h3"/><path d="M1 9h3"/><path d="M1 15h3"/></svg></span>
            <span class="nav-label">AI Hub</span>
          </a>
          <a href="/memories" class="nav-item" class:active={$page.url.pathname.startsWith('/memories')}>
            <span class="nav-icon"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8"><ellipse cx="12" cy="5" rx="9" ry="3"/><path d="M21 12c0 1.66-4 3-9 3s-9-1.34-9-3"/><path d="M3 5v14c0 1.66 4 3 9 3s9-1.34 9-3V5"/></svg></span>
            <span class="nav-label">Memories</span>
          </a>
        </div>
      </div>

    {:else if activeTab === 'notes'}
      <div class="tab-content">
        <div class="nav-section">
          <a href="/notes/recent" class="nav-item" class:active={$page.url.pathname === '/notes/recent'}>
            <span class="nav-icon"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8"><circle cx="12" cy="12" r="10"/><polyline points="12 6 12 12 16 14"/></svg></span>
            <span class="nav-label">Recent Notes</span>
          </a>
          <a href="/notes/all" class="nav-item" class:active={$page.url.pathname === '/notes/all'}>
            <span class="nav-icon"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8"><path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"/><polyline points="14 2 14 8 20 8"/></svg></span>
            <span class="nav-label">All Notes</span>
          </a>
          <a href="/notes/trash" class="nav-item" class:active={$page.url.pathname === '/notes/trash'}>
            <span class="nav-icon"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8"><polyline points="3 6 5 6 21 6"/><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/></svg></span>
            <span class="nav-label">Trash</span>
          </a>
        </div>
      </div>

    {:else if activeTab === 'work'}
      <div class="tab-content">
        <div class="nav-section">
          <a href="/work/email" class="nav-item" class:active={$page.url.pathname === '/work/email'}>
            <span class="nav-icon"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8"><path d="M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z"/><polyline points="22,6 12,13 2,6"/></svg></span>
            <span class="nav-label">Email</span>
            <span class="badge">3</span>
          </a>
          <a href="/work/calendar" class="nav-item" class:active={$page.url.pathname === '/work/calendar'}>
            <span class="nav-icon"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8"><rect x="3" y="4" width="18" height="18" rx="2" ry="2"/><line x1="16" y1="2" x2="16" y2="6"/><line x1="8" y1="2" x2="8" y2="6"/><line x1="3" y1="10" x2="21" y2="10"/></svg></span>
            <span class="nav-label">Calendar</span>
          </a>
          <a href="/work/tasks" class="nav-item" class:active={$page.url.pathname === '/work/tasks'}>
            <span class="nav-icon"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8"><path d="M9 11l3 3L22 4"/><path d="M21 12v7a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11"/></svg></span>
            <span class="nav-label">Tasks</span>
            <span class="badge">7</span>
          </a>
        </div>
      </div>

    {:else if activeTab === 'playground'}
      <div class="tab-content">
        <div class="nav-section">
          <a href="/playground/tamagotchi" class="nav-item" class:active={$page.url.pathname === '/playground/tamagotchi'}>
            <span class="nav-icon"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8"><circle cx="12" cy="10" r="7"/><path d="M8.5 8.5c0 .83.67 1.5 1.5 1.5s1.5-.67 1.5-1.5S10.83 7 10 7s-1.5.67-1.5 1.5zM13 8.5c0-.83.67-1.5 1.5-1.5S16 7.67 16 8.5s-.67 1.5-1.5 1.5S13 9.33 13 8.5z"/><path d="M9 13c.5.5 1.5 1 3 1s2.5-.5 3-1"/><path d="M12 17v4M8 21h8"/></svg></span>
            <span class="nav-label">Tamagotchi</span>
          </a>
        </div>
      </div>
    {/if}
  </nav>

  <!-- Sidebar Footer bottom bar -->
  <div class="sidebar-bottom-bar">
    <div class="sidebar-avatar" title="Profile">MA</div>
    
    <div 
      class="sidebar-theme-toggle" 
      role="button" 
      tabindex="0"
      aria-label="Toggle dark mode"
      onclick={toggleTheme}
      onkeydown={(e) => e.key === 'Enter' && toggleTheme()}
    >
      <div class="theme-toggle-track" class:light={currentTheme === 'light'}>
        <div class="theme-toggle-icons">
          <svg class="icon-moon" viewBox="0 0 24 24"><path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"/></svg>
          <svg class="icon-sun" viewBox="0 0 24 24"><circle cx="12" cy="12" r="5"/><line x1="12" y1="1" x2="12" y2="3"/><line x1="12" y1="21" x2="12" y2="23"/><line x1="4.22" y1="4.22" x2="5.64" y2="5.64"/><line x1="18.36" y1="18.36" x2="19.78" y2="19.78"/><line x1="1" y1="12" x2="3" y2="12"/><line x1="21" y1="12" x2="23" y2="12"/><line x1="4.22" y1="19.78" x2="5.64" y2="18.36"/><line x1="18.36" y1="5.64" x2="19.78" y2="4.22"/></svg>
        </div>
        <div class="theme-toggle-knob"></div>
      </div>
    </div>

    <div class="sidebar-bottom-actions">
      <button class="sidebar-icon-btn" onclick={toggleCollapse} title="Toggle sidebar">
        <svg viewBox="0 0 24 24" class="refresh-chevron"><polyline points="15 18 9 12 15 6"/></svg>
      </button>
    </div>
  </div>

  <!-- Absolute edge collapse button -->
  <button class="sidebar-collapse-btn" onclick={toggleCollapse} aria-label="Toggle sidebar">
    <svg viewBox="0 0 24 24"><polyline points="15 18 9 12 15 6"/></svg>
  </button>
</aside>

<style>
  .sidebar {
    width: var(--sidebar-width);
    height: 100%;
    background: var(--sidebar-bg-gradient);
    display: flex;
    flex-direction: column;
    color: var(--sidebar-text);
    transition: width 0.25s var(--ease-spring), min-width 0.25s var(--ease-spring);
    position: relative;
    z-index: 50;
    overflow: hidden;
  }

  .sidebar.collapsed {
    width: 56px !important;
    min-width: 56px !important;
  }

  /* Sidebar Tabs */
  .sidebar-tabs {
    display: flex;
    gap: 2px;
    padding: 6px;
    margin: 8px 8px 4px;
    background: rgba(255, 255, 255, 0.06);
    border-radius: var(--radius-lg);
    justify-content: space-around;
    flex-shrink: 0;
  }

  .tab-item {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 7px;
    border-radius: var(--radius-md);
    cursor: pointer;
    color: var(--sidebar-text);
    transition: all 0.15s ease;
    border: none;
    background: transparent;
    flex: 1;
  }

  .tab-item:hover {
    background: rgba(255, 255, 255, 0.1);
  }

  .tab-item.active {
    background: rgba(255, 255, 255, 0.18);
    box-shadow: 0 1px 4px rgba(0, 0, 0, 0.15);
  }

  .tab-icon {
    width: 22px;
    height: 22px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .tab-icon svg {
    width: 20px;
    height: 20px;
    stroke: currentColor;
    fill: none;
    stroke-width: 1.8;
  }

  /* Main Nav Section */
  .sidebar-main-nav {
    padding: 4px 8px;
    flex-shrink: 0;
  }

  .new-chat-btn {
    display: flex;
    align-items: center;
    gap: 8px;
    width: 100%;
    padding: 8px 10px;
    background: transparent;
    color: var(--sidebar-text-muted);
    border: none;
    border-radius: var(--radius-md);
    font-size: 13px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.15s ease;
    font-family: var(--font-sans);
    margin-bottom: 4px;
    text-align: left;
  }

  .new-chat-btn:hover {
    background: rgba(255, 255, 255, 0.06);
    color: var(--sidebar-text);
  }

  .new-chat-btn .nc-icon {
    width: 20px;
    height: 20px;
    display: flex;
    align-items: center;
    justify-content: center;
    border: 1.5px solid currentColor;
    border-radius: 50%;
    flex-shrink: 0;
  }

  .new-chat-label {
    white-space: nowrap;
  }

  /* Sidebar Nav Content */
  .sidebar-nav {
    flex: 1;
    overflow-y: auto;
    padding: 6px 8px;
    min-height: 0;
  }

  .nav-section {
    margin-bottom: 12px;
  }

  .section-label {
    font-size: 10px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.8px;
    color: var(--sidebar-text-muted);
    padding: 8px 12px 4px;
    white-space: nowrap;
  }

  .nav-item {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 7px 12px;
    border-radius: var(--radius-md);
    cursor: pointer;
    font-size: 13px;
    color: var(--sidebar-text);
    transition: background 0.1s ease;
    margin-bottom: 1px;
    position: relative;
    text-decoration: none;
  }

  .nav-item:hover {
    background: var(--sidebar-hover);
  }

  .nav-icon {
    width: 18px;
    height: 18px;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
  }

  .nav-icon svg {
    width: 16px;
    height: 16px;
    stroke: currentColor;
    fill: none;
    stroke-width: 1.8;
  }

  .nav-label {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    flex: 1;
  }

  .badge {
    margin-left: auto;
    font-size: 10px;
    background: rgba(255, 255, 255, 0.2);
    padding: 1px 7px;
    border-radius: 10px;
    min-width: 18px;
    text-align: center;
    font-weight: 600;
    color: white;
  }

  /* Thread Items */
  .thread-item {
    width: 100%;
    background: transparent;
    border: none;
    color: var(--sidebar-text);
    display: flex;
    flex-direction: column;
    gap: 2px;
    padding: 8px 12px;
    border-radius: var(--radius-md);
    cursor: pointer;
    transition: background 0.1s ease;
    margin-bottom: 1px;
    position: relative;
    text-align: left;
    font-family: var(--font-sans);
  }

  .thread-item:hover {
    background: var(--sidebar-hover);
  }

  .thread-item.active {
    background: var(--sidebar-active);
  }

  .thread-item.active::before {
    content: '';
    position: absolute;
    left: 0;
    top: 8px;
    bottom: 8px;
    width: 3px;
    background: var(--sidebar-text);
    border-radius: 0 2px 2px 0;
  }

  .thread-item-title {
    font-size: 13px;
    font-weight: 500;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .thread-item-meta {
    font-size: 11px;
    color: var(--sidebar-text-muted);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  /* Sidebar Bottom Bar */
  .sidebar-bottom-bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 8px 12px;
    border-top: 1px solid var(--sidebar-border);
    flex-shrink: 0;
    height: 52px;
    position: relative;
  }

  .sidebar-avatar {
    width: 30px;
    height: 30px;
    border-radius: 50%;
    background: linear-gradient(135deg, #6366f1, #8b5cf6);
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 12px;
    font-weight: 700;
    color: white;
    cursor: pointer;
    transition: transform 0.15s;
    flex-shrink: 0;
  }

  .sidebar-avatar:hover {
    transform: scale(1.08);
  }

  /* Theme Toggle */
  .sidebar-theme-toggle {
    position: absolute;
    left: 50%;
    transform: translateX(-50%);
    outline: none;
  }

  .theme-toggle-track {
    width: 52px;
    height: 28px;
    border-radius: 14px;
    background: rgba(255, 255, 255, 0.12);
    border: 1px solid rgba(255, 255, 255, 0.1);
    cursor: pointer;
    position: relative;
    display: flex;
    align-items: center;
    padding: 0 4px;
    transition: background 0.3s ease, border-color 0.3s ease;
  }

  .theme-toggle-track:hover {
    background: rgba(255, 255, 255, 0.18);
    border-color: rgba(255, 255, 255, 0.18);
  }

  .theme-toggle-track.light {
    background: rgba(255, 255, 255, 0.25);
    border-color: rgba(255, 255, 255, 0.2);
  }

  .theme-toggle-icons {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    z-index: 1;
    pointer-events: none;
  }

  .theme-toggle-icons svg {
    width: 14px;
    height: 14px;
    stroke-width: 1.8;
    fill: none;
    stroke: rgba(255, 255, 255, 0.45);
    transition: stroke 0.3s ease;
  }

  .theme-toggle-track:not(.light) .theme-toggle-icons .icon-moon {
    stroke: rgba(255, 255, 255, 0.95);
  }

  .theme-toggle-track.light .theme-toggle-icons .icon-sun {
    stroke: rgba(255, 255, 255, 0.95);
  }

  .theme-toggle-knob {
    position: absolute;
    top: 2px;
    right: 2px;
    width: 22px;
    height: 22px;
    border-radius: 50%;
    background: white;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.25);
    transition: left 0.3s cubic-bezier(0.34, 1.56, 0.64, 1),
                right 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
  }

  .theme-toggle-track.light .theme-toggle-knob {
    left: 2px;
    right: auto;
  }

  .sidebar-bottom-actions {
    display: flex;
    align-items: center;
    gap: 2px;
  }

  .sidebar-icon-btn {
    width: 30px;
    height: 30px;
    display: flex;
    align-items: center;
    justify-content: center;
    border: none;
    background: transparent;
    border-radius: var(--radius-md);
    cursor: pointer;
    color: var(--sidebar-text-muted);
    transition: all 0.15s;
  }

  .sidebar-icon-btn:hover {
    background: var(--sidebar-hover);
    color: var(--sidebar-text);
  }

  .sidebar-icon-btn svg {
    width: 16px;
    height: 16px;
    stroke: currentColor;
    fill: none;
    stroke-width: 1.8;
  }

  .refresh-chevron {
    transition: transform 0.25s var(--ease-spring);
  }

  /* Absolute collapse button */
  .sidebar-collapse-btn {
    position: absolute;
    top: 50%;
    right: -12px;
    transform: translateY(-50%);
    width: 24px;
    height: 24px;
    border-radius: 50%;
    background: var(--bg-base);
    border: 1px solid var(--border-default);
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    z-index: 60;
    box-shadow: var(--shadow-sm);
    transition: all 0.15s;
  }

  .sidebar-collapse-btn:hover {
    background: var(--bg-surface);
    box-shadow: var(--shadow-md);
  }

  .sidebar-collapse-btn svg {
    width: 14px;
    height: 14px;
    stroke: var(--text-secondary);
    fill: none;
    stroke-width: 2;
    transition: transform 0.25s var(--ease-spring);
  }

  /* Collapsed Overrides */
  .sidebar.collapsed .nav-label,
  .sidebar.collapsed .section-label,
  .sidebar.collapsed .thread-item,
  .sidebar.collapsed .new-chat-label {
    display: none;
  }

  .sidebar.collapsed .sidebar-tabs {
    padding: 8px 4px;
    gap: 1px;
    margin: 8px 4px 4px;
  }

  .sidebar.collapsed .tab-item {
    padding: 8px;
  }

  .sidebar.collapsed .nav-item {
    justify-content: center;
    padding: 10px;
  }

  .sidebar.collapsed .nav-icon {
    margin: 0;
  }

  .sidebar.collapsed .new-chat-btn {
    justify-content: center;
    padding: 10px;
  }

  .sidebar.collapsed .new-chat-btn .nc-icon {
    margin: 0;
  }

  .sidebar.collapsed .sidebar-bottom-bar {
    flex-direction: column;
    align-items: center;
    gap: 6px;
    padding: 10px 6px;
    height: auto;
  }

  .sidebar.collapsed .sidebar-theme-toggle {
    position: static;
    transform: none;
  }

  .sidebar.collapsed .theme-toggle-track {
    width: 30px;
    height: 30px;
    border-radius: var(--radius-md);
    background: transparent;
    border: none;
    padding: 0;
    justify-content: center;
  }

  .sidebar.collapsed .theme-toggle-track:hover {
    background: var(--sidebar-hover);
  }

  .sidebar.collapsed .theme-toggle-icons {
    justify-content: center;
  }

  .sidebar.collapsed .theme-toggle-icons svg {
    width: 16px;
    height: 16px;
    stroke: var(--sidebar-text-muted);
  }

  .sidebar.collapsed .theme-toggle-track:not(.light) .icon-sun {
    display: none;
  }

  .sidebar.collapsed .theme-toggle-track.light .icon-moon {
    display: none;
  }

  .sidebar.collapsed .theme-toggle-track:not(.light) .icon-moon {
    stroke: rgba(255, 255, 255, 0.85);
  }

  .sidebar.collapsed .theme-toggle-track.light .icon-sun {
    stroke: rgba(255, 255, 255, 0.85);
  }

  .sidebar.collapsed .theme-toggle-knob {
    display: none;
  }

  .sidebar.collapsed .sidebar-avatar {
    margin-bottom: 2px;
  }

  .sidebar.collapsed .refresh-chevron {
    transform: rotate(180deg);
  }

  .sidebar.collapsed .sidebar-collapse-btn svg {
    transform: rotate(180deg);
  }
</style>
