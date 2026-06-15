<script lang="ts">
  import { onMount } from 'svelte';
  import { memoryStore } from '$lib/stores/memories.svelte';
  import { Search, Plus, Edit, Trash2, Pin, PinOff } from 'lucide-svelte';

  let searchQuery = $state('');
  
  onMount(() => {
    memoryStore.init();
  });

  // Derived state to filter memories
  let filteredMemories = $derived(
    memoryStore.memories.filter(m => {
      if (!searchQuery) return true;
      return m.content.toLowerCase().includes(searchQuery.toLowerCase()) || 
             (m.category && m.category.toLowerCase().includes(searchQuery.toLowerCase()));
    })
  );

  // Edit/Add modal state
  let isModalOpen = $state(false);
  let editingId = $state<string | null>(null);
  let editContent = $state('');
  let editCategory = $state('');
  let editIsPinned = $state(false);

  function openAddModal() {
    editingId = null;
    editContent = '';
    editCategory = '';
    editIsPinned = false;
    isModalOpen = true;
  }

  function openEditModal(memory: any) {
    editingId = memory.id;
    editContent = memory.content;
    editCategory = memory.category || '';
    editIsPinned = memory.is_pinned;
    isModalOpen = true;
  }

  async function saveMemory() {
    if (!editContent.trim()) return;
    if (editingId) {
      await memoryStore.updateMemory(editingId, editContent, editCategory || null, editIsPinned);
    } else {
      await memoryStore.createMemory(editContent, editCategory || null, editIsPinned, false, null);
    }
    isModalOpen = false;
  }

  async function deleteMemory(id: string) {
    if (confirm('Are you sure you want to delete this memory?')) {
      await memoryStore.deleteMemory(id);
    }
  }

  async function togglePin(memory: any) {
    await memoryStore.updateMemory(memory.id, memory.content, memory.category, !memory.is_pinned);
  }
</script>

<div class="memories-layout">
  <div class="header">
    <div class="header-actions">
      <div class="search-box">
        <Search size={16} />
        <input type="text" placeholder="Search memories..." bind:value={searchQuery} />
      </div>
      <button class="primary-btn" onclick={openAddModal}>
        <Plus size={16} />
        Add Memory
      </button>
    </div>
  </div>

  <div class="table-container">
    {#if memoryStore.loading}
      <div class="empty-state">Loading memories...</div>
    {:else if filteredMemories.length === 0}
      <div class="empty-state">
        <div class="empty-state-icon">
          <Search size={48} />
        </div>
        <h3>No memories found</h3>
        <p>
          {#if searchQuery}
            Try adjusting your search terms.
          {:else}
            No memories saved yet. Memories capture important context over time.
          {/if}
        </p>
        {#if !searchQuery}
          <button class="primary-btn" style="margin-top: 16px;" onclick={openAddModal}>Add First Memory</button>
        {/if}
      </div>
    {:else}
      <table class="memories-table">
        <thead>
          <tr>
            <th style="width: 48px; text-align: center;">Pin</th>
            <th>Content</th>
            <th style="width: 150px;">Category</th>
            <th style="width: 100px;">Source</th>
            <th style="width: 120px;">Date</th>
            <th style="width: 80px; text-align: right;">Actions</th>
          </tr>
        </thead>
        <tbody>
          {#each filteredMemories as memory (memory.id)}
            <tr>
              <td style="text-align: center;">
                <button class="icon-btn pin-btn" class:active={memory.is_pinned} onclick={() => togglePin(memory)}>
                  {#if memory.is_pinned}
                    <Pin size={16} fill="currentColor" />
                  {:else}
                    <PinOff size={16} />
                  {/if}
                </button>
              </td>
              <td>
                <div class="memory-content">{memory.content}</div>
              </td>
              <td>
                {#if memory.category}
                  <span class="category-pill">{memory.category}</span>
                {/if}
              </td>
              <td>
                {#if memory.is_auto_generated}
                  <span class="source-badge auto">Auto</span>
                {:else}
                  <span class="source-badge manual">Manual</span>
                {/if}
              </td>
              <td class="date-cell">
                {new Date(memory.updated_at || memory.created_at || Date.now()).toLocaleDateString()}
              </td>
              <td>
                <div class="actions">
                  <button class="icon-btn" onclick={() => openEditModal(memory)} title="Edit"><Edit size={16} /></button>
                  <button class="icon-btn delete-btn" onclick={() => deleteMemory(memory.id)} title="Delete"><Trash2 size={16} /></button>
                </div>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    {/if}
  </div>

  {#if isModalOpen}
    <!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
    <div class="modal-backdrop" onclick={() => isModalOpen = false}>
      <div class="modal-content" onclick={(e) => e.stopPropagation()}>
        <div class="modal-header">
          <h2>{editingId ? 'Edit Memory' : 'Add Memory'}</h2>
        </div>
        
        <div class="modal-body">
          <div class="form-group">
            <label for="content">Memory Content</label>
            <textarea id="content" bind:value={editContent} rows="4" placeholder="What should I remember?"></textarea>
          </div>
          
          <div class="form-group">
            <label for="category">Category (Optional)</label>
            <input id="category" type="text" bind:value={editCategory} placeholder="e.g. Preferences, Facts, Projects" />
          </div>
          
          <div class="form-group checkbox-group">
            <input id="pinned" type="checkbox" bind:checked={editIsPinned} />
            <label for="pinned">Pin this memory (always included in context)</label>
          </div>
        </div>
        
        <div class="modal-actions">
          <button class="secondary-btn" onclick={() => isModalOpen = false}>Cancel</button>
          <button class="primary-btn" onclick={saveMemory}>Save</button>
        </div>
      </div>
    </div>
  {/if}
</div>

<style>
  .memories-layout {
    flex: 1;
    display: flex;
    flex-direction: column;
    height: 100%;
    background-color: var(--bg-base);
    color: var(--text-primary);
    overflow: hidden;
  }

  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 32px 40px;
    border-bottom: 1px solid var(--border-default);
  }

  .header-actions {
    display: flex;
    align-items: center;
    gap: 16px;
  }

  .search-box {
    display: flex;
    align-items: center;
    background-color: var(--bg-surface);
    border: 1px solid var(--border-default);
    border-radius: var(--radius-md);
    padding: 6px 12px;
    width: 240px;
    color: var(--text-secondary);
    transition: border-color var(--duration-fast);
  }

  .search-box:focus-within {
    border-color: var(--accent);
    color: var(--text-primary);
  }

  .search-box input {
    background: transparent;
    border: none;
    color: var(--text-primary);
    font-size: 13px;
    width: 100%;
    margin-left: 8px;
    outline: none;
  }

  .primary-btn {
    display: flex;
    align-items: center;
    gap: 8px;
    background-color: var(--brand-primary);
    color: white;
    border: none;
    border-radius: var(--radius-md);
    padding: 8px 16px;
    font-size: 13px;
    font-weight: 500;
    cursor: pointer;
    transition: background-color var(--duration-fast);
  }

  .primary-btn:hover {
    background-color: var(--brand-secondary);
  }

  .secondary-btn {
    background-color: transparent;
    color: var(--text-secondary);
    border: 1px solid var(--border-default);
    border-radius: var(--radius-md);
    padding: 8px 16px;
    font-size: 13px;
    font-weight: 500;
    cursor: pointer;
    transition: background-color var(--duration-fast);
  }

  .secondary-btn:hover {
    background-color: var(--bg-hover);
    color: var(--text-primary);
  }

  .table-container {
    flex: 1;
    overflow-y: auto;
    padding: 0;
  }

  .memories-table {
    width: 100%;
    border-collapse: collapse;
    font-size: 13px;
  }

  .memories-table th,
  .memories-table td {
    padding: 16px 20px;
    border-bottom: 1px solid var(--border-default);
    text-align: left;
  }

  .memories-table th {
    font-weight: 600;
    color: var(--text-secondary);
    background-color: var(--bg-surface);
    position: sticky;
    top: 0;
    z-index: 10;
  }

  .memories-table tr:hover {
    background-color: rgba(255, 255, 255, 0.02);
  }

  .memory-content {
    line-height: 1.5;
    max-width: 500px;
  }

  .category-pill {
    background-color: var(--bg-elevated);
    border: 1px solid var(--border-default);
    color: var(--text-secondary);
    padding: 4px 10px;
    border-radius: var(--radius-full);
    font-size: 11px;
    font-weight: 500;
  }

  .source-badge {
    padding: 4px 8px;
    border-radius: var(--radius-sm);
    font-size: 11px;
    font-weight: 600;
    text-transform: uppercase;
  }

  .source-badge.auto {
    background-color: rgba(59, 130, 246, 0.1);
    color: #3B82F6;
  }

  .source-badge.manual {
    background-color: rgba(16, 185, 129, 0.1);
    color: #10B981;
  }

  .date-cell {
    color: var(--text-muted);
    font-size: 12px;
  }

  .actions {
    display: flex;
    justify-content: flex-end;
    gap: 8px;
  }

  .icon-btn {
    background: transparent;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    padding: 6px;
    border-radius: var(--radius-sm);
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all var(--duration-fast);
  }

  .icon-btn:hover {
    background-color: var(--bg-hover);
    color: var(--text-primary);
  }

  .pin-btn.active {
    color: var(--accent);
  }

  .delete-btn:hover {
    color: var(--error);
    background-color: rgba(239, 68, 68, 0.1);
  }

  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    padding: 60px 20px;
    color: var(--text-secondary);
    text-align: center;
  }

  .empty-state-icon {
    margin-bottom: 16px;
    opacity: 0.5;
  }

  .empty-state h3 {
    margin: 0 0 8px 0;
    color: var(--text-primary);
    font-size: 18px;
  }

  .empty-state p {
    margin: 0;
    max-width: 400px;
    line-height: 1.5;
  }

  /* Modal */
  .modal-backdrop {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background-color: rgba(0, 0, 0, 0.5);
    backdrop-filter: blur(4px);
    z-index: 100;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .modal-content {
    background-color: var(--bg-surface);
    border: 1px solid var(--border-default);
    border-radius: var(--radius-lg);
    width: 500px;
    max-width: 90vw;
    box-shadow: 0 20px 40px rgba(0, 0, 0, 0.4);
    display: flex;
    flex-direction: column;
  }

  .modal-header {
    padding: 20px 24px;
    border-bottom: 1px solid var(--border-default);
  }

  .modal-header h2 {
    margin: 0;
    font-size: 18px;
    font-weight: 600;
  }

  .modal-body {
    padding: 24px;
    display: flex;
    flex-direction: column;
    gap: 20px;
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .form-group label {
    font-size: 12px;
    font-weight: 500;
    color: var(--text-secondary);
  }

  .form-group input[type="text"],
  .form-group textarea {
    background-color: var(--bg-input);
    border: 1px solid var(--border-default);
    border-radius: var(--radius-md);
    padding: 10px 12px;
    color: var(--text-primary);
    font-size: 13px;
    font-family: inherit;
    outline: none;
    transition: border-color var(--duration-fast);
  }

  .form-group input[type="text"]:focus,
  .form-group textarea:focus {
    border-color: var(--accent);
  }

  .form-group textarea {
    resize: vertical;
    min-height: 80px;
  }

  .checkbox-group {
    flex-direction: row;
    align-items: center;
    gap: 12px;
  }

  .checkbox-group input[type="checkbox"] {
    width: 16px;
    height: 16px;
    accent-color: var(--accent);
    cursor: pointer;
  }

  .checkbox-group label {
    font-size: 13px;
    cursor: pointer;
  }

  .modal-actions {
    padding: 16px 24px;
    border-top: 1px solid var(--border-default);
    display: flex;
    justify-content: flex-end;
    gap: 12px;
    background-color: rgba(0, 0, 0, 0.2);
    border-bottom-left-radius: var(--radius-lg);
    border-bottom-right-radius: var(--radius-lg);
  }
</style>
