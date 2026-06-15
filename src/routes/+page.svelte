<script lang="ts">
  import { onMount } from 'svelte';
  import { 
    Send, 
    ChevronDown, 
    Code, 
    ExternalLink, 
    Terminal, 
    Layers, 
    Plus, 
    Image, 
    FileText, 
    Video, 
    Music, 
    Check, 
    Copy,
    Folder,
    Globe
  } from '@lucide/svelte';
  import { conversationStore } from '$lib/stores/conversations.svelte';
  import { getMessages, saveMessage } from '$lib/ipc/chat';
  import { upsertConversation as ipcUpsertConversation } from '$lib/ipc/conversations';

  interface Message {
    id: string;
    role: string;
    avatar: string;
    isAssistant?: boolean;
    text: string;
    time: string;
    toolUse?: {
      id: string;
      title: string;
      status: string;
      details: string;
    };
    codeSnippet?: {
      filename: string;
      code: string;
    };
    chips?: string[];
  }

  // State management
  let canvasOpen = $state(true);
  let canvasWidth = $state(400);
  let isResizingCanvas = $state(false);
  let canvasTab = $state('artifacts');
  
  let modelSelectOpen = $state(false);
  let selectedModel = $state('GPT-4o');
  let modelTab = $state('cloud');
  
  let attachDropdownOpen = $state(false);
  let composerText = $state('');
  
  let toolCardExpanded = $state(false);
  let toolCard3Expanded = $state(false);
  let isCopied = $state(false);

  // Project Settings & Assignment dropdown states
  let isAssignDropdownOpen = $state(false);
  let isProjectSettingsOpen = $state(false);
  let editingProjectName = $state('');
  let editingProjectInstructions = $state('');

  // Derived active thread from store
  let activeThread = $derived(
    conversationStore.threads.find(t => t.id === conversationStore.activeThreadId) || null
  );

  // Auto-load messages when active thread changes
  $effect(() => {
    if (conversationStore.activeThreadId) {
      loadThreadMessages(conversationStore.activeThreadId);
    }
  });

  async function loadThreadMessages(threadId: string) {
    try {
      const dbMsgs = await getMessages(threadId);
      conversationStore.activeMessages = dbMsgs;
    } catch (e) {
      console.error("Failed to load thread messages:", e);
    }
  }

  // Derive UI-compatible messages list from DB messages in store
  let uiMessages = $derived.by(() => {
    return conversationStore.activeMessages.map(m => {
      let toolUse: any = undefined;
      let codeSnippet: any = undefined;
      let chips: string[] = [];
      
      if (m.metadata) {
        try {
          const parsed = JSON.parse(m.metadata);
          toolUse = parsed.toolUse;
          codeSnippet = parsed.codeSnippet;
          chips = parsed.chips || [];
        } catch (e) {
          // ignore
        }
      }
      
      return {
        id: m.id,
        role: m.role,
        avatar: m.role === 'user' ? 'M' : 'D',
        isAssistant: m.role === 'assistant',
        text: m.content,
        time: new Date(m.createdAt || Date.now()).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }),
        toolUse,
        codeSnippet,
        chips
      };
    });
  });

  // Suggested chips
  const suggestionChips = [
    'Build a React Table',
    'Svelte 5 Runes tutorial',
    'Docker Deployment Strategy',
    'Explain Rust Mutex',
    'Design a chat UI',
    'Write a regex for emails'
  ];

  // Drag-to-resize canvas panel logic
  function handleMouseDown(e: MouseEvent) {
    isResizingCanvas = true;
    window.addEventListener('mousemove', handleMouseMove);
    window.addEventListener('mouseup', handleMouseUp);
  }

  function handleMouseMove(e: MouseEvent) {
    if (!isResizingCanvas) return;
    const containerWidth = window.innerWidth;
    // Set boundaries: min 300px, max width - 300px
    const newWidth = Math.max(300, Math.min(containerWidth - 300, containerWidth - e.clientX));
    canvasWidth = newWidth;
  }

  function handleMouseUp() {
    isResizingCanvas = false;
    window.removeEventListener('mousemove', handleMouseMove);
    window.removeEventListener('mouseup', handleMouseUp);
  }

  // Handle click on suggestion chips
  function selectChip(chip: string) {
    composerText = chip;
  }

  // Handle message send
  async function handleSend() {
    if (!composerText.trim() || !conversationStore.activeThreadId) return;
    
    const text = composerText;
    composerText = '';
    
    const userMsgId = 'msg_' + Math.random().toString(36).substring(2, 11);
    const nowStr = new Date().toISOString();

    const userDbMsg = {
      id: userMsgId,
      conversationId: conversationStore.activeThreadId,
      parentMessageId: null,
      role: 'user' as const,
      content: text,
      createdAt: nowStr,
      metadata: undefined
    };
    conversationStore.activeMessages.push(userDbMsg);
    
    // Save to SQLite
    try {
      await saveMessage(userDbMsg);
    } catch (e) {
      console.error("Failed to save user message:", e);
    }
    
    // Simulate assistant reply
    setTimeout(async () => {
      const assistantMsgId = 'msg_' + Math.random().toString(36).substring(2, 11);
      const nowStr2 = new Date().toISOString();
      
      // Determine prompt instruction: prepend project custom instructions if active
      let promptPrefix = '';
      if (activeThread?.project_id) {
        const proj = conversationStore.projects.find(p => p.id === activeThread.project_id);
        if (proj?.custom_instructions) {
          promptPrefix = `[System Instructions: ${proj.custom_instructions}]\n\n`;
        }
      }
      
      const replyContent = promptPrefix + `I've received your request. I am analyzing the workspace and will generate the code updates shortly.`;
      
      // Seed dummy toolUse in metadata for demonstration
      const metadataObj = {
        toolUse: {
          id: 'toolCard_' + assistantMsgId,
          title: 'Used Filesystem · analyze project',
          status: 'Done',
          details: 'Analyzed workspace structure. Located 5 files.'
        }
      };

      const assistantDbMsg = {
        id: assistantMsgId,
        conversationId: conversationStore.activeThreadId!,
        parentMessageId: userMsgId,
        role: 'assistant' as const,
        content: replyContent,
        createdAt: nowStr2,
        metadata: JSON.stringify(metadataObj)
      };
      
      conversationStore.activeMessages.push(assistantDbMsg);
      
      // Save to SQLite
      try {
        await saveMessage(assistantDbMsg);
      } catch (e) {
        console.error("Failed to save assistant message:", e);
      }
    }, 1000);
  }

  // Copy code block utility
  function copyCode(code: string) {
    navigator.clipboard.writeText(code);
    isCopied = true;
    setTimeout(() => isCopied = false, 2000);
  }

  // Toggle model selection dropdown
  function toggleModelSelect() {
    modelSelectOpen = !modelSelectOpen;
  }

  // Select a model
  function selectModel(modelName: string) {
    selectedModel = modelName;
    modelSelectOpen = false;
  }
</script>

<div class="main-layout" class:resizing-canvas={isResizingCanvas}>
  
  <!-- Left Side: Chat Area -->
  <div class="chat-area" class:empty={uiMessages.length === 0}>
    
    <!-- Chat Header -->
    <header class="chat-header">
      <div class="chat-header-left">
        <div 
          class="model-badge" 
          role="button" 
          tabindex="0"
          onclick={toggleModelSelect}
          onkeydown={(e) => e.key === 'Enter' && toggleModelSelect()}
        >
          <span class="model-dot"></span>
          <span class="model-name">{selectedModel}</span>
          <ChevronDown size={14} class="text-secondary ml-1" />
          
          {#if modelSelectOpen}
            <div class="model-dropdown" onclick={(e) => e.stopPropagation()}>
              <div class="model-dropdown-tabs">
                <button 
                  class="model-tab" 
                  class:active={modelTab === 'cloud'} 
                  onclick={() => modelTab = 'cloud'}
                >
                  Cloud
                </button>
                <button 
                  class="model-tab" 
                  class:active={modelTab === 'local'} 
                  onclick={() => modelTab = 'local'}
                >
                  Local
                </button>
              </div>

              {#if modelTab === 'cloud'}
                <div class="model-list">
                  <button class="dropdown-item" onclick={() => selectModel('GPT-4o')}>
                    <Layers size={14} />
                    <div class="dropdown-item-details">
                      <div class="dropdown-item-label">GPT-4o</div>
                      <div class="dropdown-item-desc">OpenAI &middot; 128k context</div>
                    </div>
                    <span class="tokens-left">87k left</span>
                  </button>
                  <button class="dropdown-item" onclick={() => selectModel('Claude 3.5 Sonnet')}>
                    <Layers size={14} />
                    <div class="dropdown-item-details">
                      <div class="dropdown-item-label">Claude 3.5 Sonnet</div>
                      <div class="dropdown-item-desc">Anthropic &middot; 200k context</div>
                    </div>
                  </button>
                  <button class="dropdown-item" onclick={() => selectModel('Gemini 1.5 Pro')}>
                    <Layers size={14} />
                    <div class="dropdown-item-details">
                      <div class="dropdown-item-label">Gemini 1.5 Pro</div>
                      <div class="dropdown-item-desc">Google &middot; 1M context</div>
                    </div>
                  </button>
                </div>
              {:else}
                <div class="model-list">
                  <button class="dropdown-item" onclick={() => selectModel('Llama 3.1 8B Q5')}>
                    <Terminal size={14} />
                    <div class="dropdown-item-details">
                      <div class="dropdown-item-label">Llama 3.1 8B Q5</div>
                      <div class="dropdown-item-desc">5.2 GB &middot; 4.8 GB VRAM</div>
                    </div>
                  </button>
                  <button class="dropdown-item" onclick={() => selectModel('Mistral 7B Q4')}>
                    <Terminal size={14} />
                    <div class="dropdown-item-details">
                      <div class="dropdown-item-label">Mistral 7B Q4</div>
                      <div class="dropdown-item-desc">4.1 GB &middot; 3.8 GB VRAM</div>
                    </div>
                  </button>
                </div>
              {/if}
            </div>
          {/if}
        </div>
      {#if activeThread}
        <!-- Project Selector / Move to Project dropdown -->
        <div class="project-assigner-container" style="position: relative; margin-left: 12px; display: flex; align-items: center; gap: 6px;">
          <button 
            onclick={() => isAssignDropdownOpen = !isAssignDropdownOpen}
            style="background: rgba(255, 255, 255, 0.05); border: 1px solid var(--border-default); border-radius: var(--radius-full); padding: 3px 10px; font-size: 11px; color: var(--text-secondary); cursor: pointer; display: flex; align-items: center; gap: 4px; font-weight: 500;"
          >
            {#if activeThread.project_id}
              <Folder size={11} />
              <span>{conversationStore.projects.find(p => p.id === activeThread.project_id)?.name || 'Project'}</span>
            {:else}
              <Globe size={11} />
              <span>General Chat</span>
            {/if}
            <ChevronDown size={10} />
          </button>
          
          {#if isAssignDropdownOpen}
            <div 
              class="dropdown-card" 
              style="position: absolute; top: calc(100% + 4px); left: 0; background: var(--bg-surface); border: 1px solid var(--border-default); border-radius: var(--radius-md); box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15); z-index: 100; min-width: 160px; padding: 4px;"
            >
              <div style="font-size: 10px; color: var(--text-muted); padding: 4px 8px; font-weight: 600; text-transform: uppercase; letter-spacing: 0.5px;">Move to:</div>
              <button 
                class="dropdown-item" 
                onclick={() => { 
                  conversationStore.assignConversation(activeThread.id, null); 
                  isAssignDropdownOpen = false; 
                }}
                style="display: flex; width: 100%; text-align: left; padding: 6px 8px; font-size: 11px; background: transparent; border: none; border-radius: var(--radius-sm); color: var(--text-primary); cursor: pointer;"
              >
                <Globe size={11} style="margin-right: 6px; align-self: center;" />
                <span>General Chat (Unassigned)</span>
              </button>
              {#each conversationStore.projects as project}
                <button 
                  class="dropdown-item" 
                  onclick={() => { 
                    conversationStore.assignConversation(activeThread.id, project.id); 
                    isAssignDropdownOpen = false; 
                  }}
                  style="display: flex; width: 100%; text-align: left; padding: 6px 8px; font-size: 11px; background: transparent; border: none; border-radius: var(--radius-sm); color: var(--text-primary); cursor: pointer;"
                >
                  <Folder size={11} style="margin-right: 6px; align-self: center;" />
                  <span>{project.name}</span>
                </button>
              {/each}
            </div>
          {/if}
          
          <!-- Workspace Settings gear icon button -->
          {#if activeThread.project_id}
            <button 
              onclick={() => {
                const proj = conversationStore.projects.find(p => p.id === activeThread.project_id);
                if (proj) {
                  editingProjectName = proj.name;
                  editingProjectInstructions = proj.custom_instructions || '';
                  isProjectSettingsOpen = true;
                }
              }}
              title="Workspace Settings"
              style="background: transparent; border: none; color: var(--text-muted); cursor: pointer; display: flex; align-items: center; padding: 2px;"
              onmouseover={(e) => e.currentTarget.style.color = 'var(--text-primary)'}
              onmouseout={(e) => e.currentTarget.style.color = 'var(--text-muted)'}
            >
              <svg viewBox="0 0 24 24" width="13" height="13" stroke="currentColor" stroke-width="2" fill="none"><circle cx="12" cy="12" r="3"/><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 2.83-2.83l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z"/></svg>
            </button>
          {/if}
        </div>
        
        <!-- Editable thread title -->
        <input 
          type="text" 
          bind:value={activeThread.title} 
          onblur={() => {
            if (activeThread.title.trim()) {
              ipcUpsertConversation(activeThread);
            }
          }}
          onkeydown={(e) => e.key === 'Enter' && e.currentTarget.blur()}
          style="background: transparent; border: none; color: var(--text-primary); font-size: 14px; font-weight: 600; outline: none; margin-left: 12px; flex: 1; padding: 2px 4px; border-radius: var(--radius-sm);"
        />
      {:else}
        <span class="chat-header-title">No conversation selected</span>
      {/if}
      </div>

      <div class="chat-header-right">
        <span class="token-indicator">2.4k / 128k tokens</span>
        <button class="header-btn" title="Branch">
          <svg viewBox="0 0 24 24" class="btn-icon"><line x1="6" y1="3" x2="6" y2="15"/><circle cx="18" cy="6" r="3"/><circle cx="6" cy="18" r="3"/><path d="M18 9a9 9 0 0 1-9 9"/></svg>
        </button>
        <button class="header-btn" title="Share">
          <svg viewBox="0 0 24 24" class="btn-icon"><circle cx="18" cy="5" r="3"/><circle cx="6" cy="12" r="3"/><circle cx="18" cy="19" r="3"/><line x1="8.59" y1="13.51" x2="15.42" y2="17.49"/><line x1="15.41" y1="6.51" x2="8.59" y2="10.49"/></svg>
        </button>
        <button class="header-btn" onclick={() => canvasOpen = !canvasOpen} title="Toggle Workspace" class:active={canvasOpen}>
          <svg viewBox="0 0 24 24" class="btn-icon"><rect x="3" y="3" width="18" height="18" rx="2" ry="2"/><line x1="15" y1="3" x2="15" y2="21"/></svg>
        </button>
      </div>
    </header>

    <!-- Message Pane -->
    <div class="messages-container">
      {#if uiMessages.length === 0}
        <div class="empty-welcome-message">
          <h2>How can I help you today?</h2>
          <p>
            {#if activeThread?.project_id}
              Active Workspace: <strong>{conversationStore.projects.find(p => p.id === activeThread.project_id)?.name}</strong>.
            {:else}
              This is a general standalone chat. You can assign it to a project workspace at any time.
            {/if}
          </p>
        </div>
      {:else}
        {#each uiMessages as msg}
          <div class="message" class:user={msg.role === 'user'} class:assistant={msg.role === 'assistant'}>
            <div class="message-avatar">
              {#if msg.role === 'user'}
                {msg.avatar}
              {:else}
                <!-- Mascot avatar logo -->
                <svg viewBox="0 0 24 24" class="avatar-svg"><path d="M12 2L2 7l10 5 10-5-10-5z"/><path d="M2 17l10 5 10-5"/><path d="M2 12l10 5 10-5"/></svg>
              {/if}
            </div>
            <div class="message-content">
              <div class="message-bubble">
                {#each msg.text.split('\n\n') as paragraph}
                  <p>{@html paragraph.replace(/\*\*(.*?)\*\*/g, '<strong>$1</strong>').replace(/`(.*?)`/g, '<code>$1</code>')}</p>
                {/each}
              </div>

              <!-- Optional Tool Use Card -->
              {#if msg.toolUse}
                {@const isExpanded = msg.toolUse.id === 'toolCard1' ? toolCardExpanded : toolCard3Expanded}
                <div class="tool-use-card">
                  <button 
                    class="tool-use-header" 
                    onclick={() => {
                      if (msg.toolUse?.id === 'toolCard1') {
                        toolCardExpanded = !toolCardExpanded;
                      } else {
                        toolCard3Expanded = !toolCard3Expanded;
                      }
                    }}
                  >
                    <svg viewBox="0 0 24 24" class="tool-icon"><path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"/><path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"/></svg>
                    <span>{msg.toolUse.title}</span>
                    <span class="tool-status-badge success">Done</span>
                    <svg class="tool-use-chevron" class:expanded={isExpanded} viewBox="0 0 24 24"><polyline points="6 9 12 15 18 9"/></svg>
                  </button>
                  
                  {#if isExpanded}
                    <div class="tool-use-body">
                      <pre>{msg.toolUse.details}</pre>
                    </div>
                  {/if}
                </div>
              {/if}

              <!-- Optional Code Snippet Card -->
              {#if msg.codeSnippet}
                <div class="code-block-wrapper">
                  <div class="code-block-header">
                    <span class="code-filename">{msg.codeSnippet.filename}</span>
                    <button class="copy-btn" onclick={() => copyCode(msg.codeSnippet!.code)}>
                      {#if isCopied}
                        <Check size={12} class="text-success mr-1" />
                        <span>Copied</span>
                      {:else}
                        <Copy size={12} class="mr-1" />
                        <span>Copy</span>
                      {/if}
                    </button>
                  </div>
                  <pre class="code-block-body"><code>{msg.codeSnippet.code}</code></pre>
                </div>
              {/if}

              <!-- Optional Chips -->
              {#if msg.chips}
                <div class="suggestion-chips mt-3">
                  {#each msg.chips as chip}
                    <button class="chip" onclick={() => selectChip(chip)}>{chip}</button>
                  {/each}
                </div>
              {/if}

              <div class="message-time">{msg.time}</div>
            </div>
          </div>
        {/each}
      {/if}
    </div>

    <!-- Composer Container -->
    <div class="composer">
      
      <!-- Mascot Character positioned absolute above composer input -->
      <svg class="composer-mascot" aria-hidden="true" viewBox="0 0 200 200">
        <ellipse cx="100" cy="165" rx="35" ry="6" fill="#0f172a" class="shadow-anim" opacity="0.15"/>
        <g class="mascot-body">
          <line x1="80" y1="130" x2="80" y2="155" stroke="#3B82F6" stroke-width="6" stroke-linecap="round"/>
          <line x1="120" y1="130" x2="120" y2="155" stroke="#3B82F6" stroke-width="6" stroke-linecap="round"/>
          <g class="arm-left">
            <line x1="60" y1="100" x2="35" y2="110" stroke="#3B82F6" stroke-width="6" stroke-linecap="round"/>
          </g>
          <g class="arm-right">
            <line x1="140" y1="100" x2="165" y2="110" stroke="#3B82F6" stroke-width="6" stroke-linecap="round"/>
          </g>
          <clipPath id="bodyClip">
            <rect x="60" y="60" width="80" height="80" rx="28"/>
          </clipPath>
          <g clip-path="url(#bodyClip)">
            <rect x="60" y="60" width="80" height="80" fill="#3B82F6"/>
            <rect x="60" y="60" width="80" height="10" fill="#60A5FA"/>
            <rect x="60" y="130" width="80" height="10" fill="#2563EB"/>
          </g>
          <line x1="90" y1="88" x2="110" y2="88" stroke="#0f172a" stroke-width="4"/>
          <circle cx="76" cy="88" r="20" fill="#ffffff" fill-opacity="0.2" stroke="#4F46E5" stroke-width="6"/>
          <circle cx="124" cy="88" r="20" fill="#ffffff" fill-opacity="0.2" stroke="#4F46E5" stroke-width="6"/>
          <g class="blink">
            <circle cx="80" cy="88" r="6" fill="#0f172a"/>
            <circle cx="120" cy="88" r="6" fill="#0f172a"/>
            <circle cx="82" cy="85" r="2" fill="#ffffff"/>
            <circle cx="122" cy="85" r="2" fill="#ffffff"/>
          </g>
          <clipPath id="lensClip">
            <circle cx="76" cy="88" r="17"/>
          </clipPath>
          <g clip-path="url(#lensClip)">
            <g class="glare-sweep">
              <polygon points="76,60 84,60 74,120 66,120" fill="#ffffff" opacity="0.6"/>
              <polygon points="88,60 91,60 81,120 78,120" fill="#ffffff" opacity="0.6"/>
            </g>
          </g>
        </g>
      </svg>

      <!-- Outer Box -->
      <div class="composer-container">
        <div class="composer-glow"></div>

        <textarea 
          class="composer-input" 
          rows="1" 
          placeholder="Ask Dexter a question..." 
          bind:value={composerText}
          onkeydown={(e) => {
            if (e.key === 'Enter' && !e.shiftKey) {
              e.preventDefault();
              handleSend();
            }
          }}
        ></textarea>

        <!-- Bottom Toolbar Row -->
        <div class="composer-toolbar">
          <div class="composer-toolbar-left">
            <div class="relative">
              <button 
                class="composer-btn" 
                onclick={() => attachDropdownOpen = !attachDropdownOpen} 
                title="Attach"
              >
                <Plus size={16} />
              </button>

              {#if attachDropdownOpen}
                <div class="composer-dropdown">
                  <div class="composer-dropdown-title">Upload from device</div>
                  <button class="composer-dropdown-item" onclick={() => attachDropdownOpen = false}>
                    <Image size={14} class="mr-2 text-secondary" />
                    <span>Images</span>
                  </button>
                  <button class="composer-dropdown-item" onclick={() => attachDropdownOpen = false}>
                    <FileText size={14} class="mr-2 text-secondary" />
                    <span>Documents</span>
                  </button>
                  <button class="composer-dropdown-item" onclick={() => attachDropdownOpen = false}>
                    <Video size={14} class="mr-2 text-secondary" />
                    <span>Video</span>
                  </button>
                  <button class="composer-dropdown-item" onclick={() => attachDropdownOpen = false}>
                    <Music size={14} class="mr-2 text-secondary" />
                    <span>Audio</span>
                  </button>
                </div>
              {/if}
            </div>

            <span class="attachment-pill">
              <svg viewBox="0 0 24 24" class="inline-svg mr-1"><path d="M12 2L2 7l10 5 10-5-10-5z"/><path d="M2 17l10 5 10-5"/><path d="M2 12l10 5 10-5"/></svg>
              DataTable.tsx
            </span>
          </div>

          <div class="composer-toolbar-right">
            <button class="send-btn" onclick={handleSend} disabled={!composerText.trim()} title="Send Message">
              <Send size={14} />
            </button>
          </div>
        </div>
      </div>

      <!-- Suggestion chips showing above input box -->
      {#if uiMessages.length === 0}
        <div class="suggestion-chips-grid">
          {#each suggestionChips as chip}
            <button class="chip" onclick={() => selectChip(chip)}>{chip}</button>
          {/each}
        </div>
      {/if}
    </div>
  </div>

  <!-- Right Side: Resize handle & Canvas Panel -->
  {#if canvasOpen}
    <div 
      class="canvas-resize-handle" 
      role="separator"
      tabindex="-1"
      aria-label="Resize canvas"
      onmousedown={handleMouseDown}
    ></div>
    
    <aside class="canvas-panel" style="width: {canvasWidth}px; min-width: {canvasWidth}px;">
      <div class="canvas-header">
        <div class="canvas-tabs">
          <button 
            class="canvas-tab" 
            class:active={canvasTab === 'artifacts'} 
            onclick={() => canvasTab = 'artifacts'}
          >
            <Layers size={13} />
            <span>Artifacts</span>
          </button>
          <button 
            class="canvas-tab" 
            class:active={canvasTab === 'code'} 
            onclick={() => canvasTab = 'code'}
          >
            <Code size={13} />
            <span>Code</span>
          </button>
          <button 
            class="canvas-tab" 
            class:active={canvasTab === 'browser'} 
            onclick={() => canvasTab = 'browser'}
          >
            <ExternalLink size={13} />
            <span>Browser</span>
          </button>
          <button 
            class="canvas-tab" 
            class:active={canvasTab === 'terminal'} 
            onclick={() => canvasTab = 'terminal'}
          >
            <Terminal size={13} />
            <span>Terminal</span>
          </button>
        </div>
        <button class="canvas-close" onclick={() => canvasOpen = false} title="Close workspace">
          <svg viewBox="0 0 24 24" class="btn-icon"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg>
        </button>
      </div>

      <!-- Canvas Content Container -->
      <div class="canvas-body">
        {#if canvasTab === 'artifacts'}
          <div class="canvas-card">
            <div class="canvas-card-header">
              <Layers size={14} class="text-accent" />
              <span>Live Preview</span>
            </div>
            <div class="card-preview-area">
              <div class="preview-box">
                <Layers size={36} class="text-accent mb-2" />
              </div>
              <div class="preview-title">DataTable Component</div>
              <div class="preview-subtitle">Live preview streaming ready</div>
            </div>
          </div>

          <div class="canvas-card">
            <div class="canvas-card-header">
              <Layers size={14} class="text-success" />
              <span>Available Tools</span>
            </div>
            <div class="tool-grid">
              <div class="tool-card">
                <div class="tool-card-icon bg-blue">
                  <Terminal size={14} />
                </div>
                <div class="tool-card-details">
                  <div class="tool-card-title">Filesystem</div>
                  <div class="tool-card-desc">Read, write, and manage files</div>
                </div>
              </div>
              <div class="tool-card">
                <div class="tool-card-icon bg-green">
                  <ExternalLink size={14} />
                </div>
                <div class="tool-card-details">
                  <div class="tool-card-title">Browser</div>
                  <div class="tool-card-desc">Web scraping & automation</div>
                </div>
              </div>
            </div>
          </div>

        {:else if canvasTab === 'code'}
          <div class="code-tree-view">
            <div class="code-tree-header">DataTable.tsx</div>
            <div class="code-editor-mock">
              <pre><code>{`import React, { useState, useMemo } from 'react';

interface Column<T> {
  header: string;
  accessor: keyof T;
}

interface TableProps<T> {
  columns: Column<T>[];
  data: T[];
}

export function DataTable<T extends { id: string }>({ columns, data }: TableProps<T>) {
  const [sortField, setSortField] = useState<keyof T | null>(null);
  const [sortOrder, setSortOrder] = useState<'asc' | 'desc'>('asc');

  // Handle row checkbox selection and CSV exporting utilities
  // ... (Full implementation loaded dynamically)
}`}</code></pre>
            </div>
          </div>

        {:else if canvasTab === 'browser'}
          <div class="mock-browser">
            <div class="browser-address-bar">
              <span class="dot red"></span>
              <span class="dot yellow"></span>
              <span class="dot green"></span>
              <div class="address-input">http://localhost:5173/preview</div>
            </div>
            <div class="browser-body">
              <div class="preview-app">
                <h3>Vite + Svelte dev server running...</h3>
                <p class="text-muted">App is running and rendering components on port 5173.</p>
              </div>
            </div>
          </div>

        {:else if canvasTab === 'terminal'}
          <div class="mock-terminal">
            <div class="terminal-body">
              <div class="terminal-line text-muted">dexter-agent:~/workspace$ npm run dev</div>
              <div class="terminal-line text-success">  VITE v6.0.3  ready in 234 ms</div>
              <div class="terminal-line">  ➜  Local:   http://localhost:5173/</div>
              <div class="terminal-line">  ➜  Network: use --host to expose</div>
              <div class="terminal-line text-muted">dexter-agent:~/workspace$ _</div>
            </div>
          </div>
        {/if}
      </div>
    </aside>
  {/if}
</div>

{#if isProjectSettingsOpen && activeThread && activeThread.project_id}
  {@const activeProjId = activeThread.project_id}
  <div 
    class="modal-backdrop" 
    style="position: fixed; inset: 0; background: rgba(0, 0, 0, 0.6); display: flex; align-items: center; justify-content: center; z-index: 1000;"
    onclick={() => isProjectSettingsOpen = false}
  >
    <div 
      class="modal-content" 
      style="background: var(--bg-surface); border: 1px solid var(--border-default); border-radius: var(--radius-lg); width: 420px; max-width: 90vw; padding: 20px; box-shadow: 0 10px 25px rgba(0,0,0,0.3);"
      onclick={(e) => e.stopPropagation()}
    >
      <h3 style="margin: 0 0 16px; font-size: 15px; font-weight: 600; color: var(--text-primary);">Workspace Settings</h3>
      
      <div style="display: flex; flex-direction: column; gap: 12px;">
        <div style="display: flex; flex-direction: column; gap: 4px;">
          <label style="font-size: 11px; font-weight: 500; color: var(--text-secondary);">Workspace Name</label>
          <input 
            type="text" 
            bind:value={editingProjectName}
            style="background: var(--bg-input); border: 1px solid var(--border-default); color: var(--text-primary); border-radius: var(--radius-md); padding: 8px 12px; font-size: 12px; outline: none;"
          />
        </div>
        
        <div style="display: flex; flex-direction: column; gap: 4px;">
          <label style="font-size: 11px; font-weight: 500; color: var(--text-secondary);">Custom Instructions (System prompt)</label>
          <textarea 
            bind:value={editingProjectInstructions}
            rows="4"
            placeholder="e.g. Always respond in SQL, or adopt an academic tone..."
            style="background: var(--bg-input); border: 1px solid var(--border-default); color: var(--text-primary); border-radius: var(--radius-md); padding: 8px 12px; font-size: 12px; outline: none; resize: vertical; font-family: var(--font-sans);"
          ></textarea>
        </div>
      </div>
      
      <div style="display: flex; justify-content: flex-end; gap: 8px; margin-top: 20px;">
        <button 
          onclick={() => isProjectSettingsOpen = false}
          style="background: rgba(255,255,255,0.05); color: var(--text-secondary); border: 1px solid var(--border-default); padding: 6px 12px; border-radius: var(--radius-md); font-size: 12px; cursor: pointer;"
        >
          Cancel
        </button>
        <button 
          onclick={async () => {
            await conversationStore.updateProject(activeProjId, editingProjectName, editingProjectInstructions);
            isProjectSettingsOpen = false;
          }}
          style="background: var(--brand-primary); color: white; border: none; padding: 6px 12px; border-radius: var(--radius-md); font-size: 12px; cursor: pointer; font-weight: 500;"
        >
          Save Changes
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .main-layout {
    display: flex;
    flex: 1;
    height: 100%;
    width: 100%;
    overflow: hidden;
    background-color: var(--bg-base);
  }

  .main-layout.resizing-canvas {
    cursor: col-resize;
    user-select: none;
    -webkit-user-select: none;
  }

  .chat-area {
    flex: 1;
    display: flex;
    flex-direction: column;
    height: 100%;
    background-color: var(--bg-base);
    min-width: 0;
    position: relative;
    overflow: hidden;
  }

  .chat-area.empty {
    justify-content: center;
    align-items: center;
  }

  /* Chat Header */
  .chat-header {
    height: 48px;
    background-color: var(--bg-surface);
    border-bottom: 1px solid var(--border-default);
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 var(--space-4);
    z-index: 40;
    flex-shrink: 0;
  }

  .chat-header-left {
    display: flex;
    align-items: center;
    gap: var(--space-3);
  }

  .model-badge {
    position: relative;
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 4px 10px;
    background-color: var(--bg-elevated);
    border: 1px solid var(--border-default);
    border-radius: var(--radius-full);
    font-size: 11px;
    font-weight: 600;
    color: var(--text-primary);
    cursor: pointer;
    user-select: none;
    -webkit-user-select: none;
    outline: none;
  }

  .model-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background-color: var(--success);
  }

  .chat-header-title {
    font-size: 13px;
    font-weight: 500;
    color: var(--text-secondary);
  }

  .chat-header-right {
    display: flex;
    align-items: center;
    gap: var(--space-2);
  }

  .token-indicator {
    font-size: 11px;
    color: var(--text-muted);
    margin-right: var(--space-2);
  }

  .header-btn {
    width: 28px;
    height: 28px;
    border-radius: var(--radius-md);
    border: 1px solid var(--border-default);
    background-color: var(--bg-surface);
    color: var(--text-secondary);
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: all var(--duration-fast) var(--ease-out);
  }

  .header-btn:hover {
    background-color: var(--bg-hover);
    color: var(--text-primary);
  }

  .header-btn.active {
    background-color: var(--bg-elevated);
    border-color: var(--border-strong);
    color: var(--accent);
  }

  .btn-icon {
    width: 14px;
    height: 14px;
    stroke: currentColor;
    fill: none;
    stroke-width: 1.8;
  }

  /* Model Dropdown */
  .model-dropdown {
    position: absolute;
    top: calc(100% + 6px);
    left: 0;
    width: 240px;
    background-color: var(--bg-surface);
    border: 1px solid var(--border-default);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-lg);
    padding: var(--space-2);
    z-index: 100;
  }

  .model-dropdown-tabs {
    display: flex;
    border-bottom: 1px solid var(--border-default);
    margin-bottom: var(--space-2);
    padding-bottom: 4px;
  }

  .model-tab {
    flex: 1;
    background: transparent;
    border: none;
    color: var(--text-muted);
    font-size: 11px;
    font-weight: 600;
    padding: 6px;
    cursor: pointer;
    border-radius: var(--radius-sm);
  }

  .model-tab.active {
    color: var(--text-primary);
    background-color: var(--bg-elevated);
  }

  .model-list {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .dropdown-item {
    display: flex;
    align-items: center;
    gap: var(--space-2.5);
    width: 100%;
    padding: 6px 10px;
    background: transparent;
    border: none;
    border-radius: var(--radius-md);
    cursor: pointer;
    text-align: left;
    color: var(--text-secondary);
    font-family: var(--font-sans);
    transition: background var(--duration-fast);
  }

  .dropdown-item:hover {
    background-color: var(--bg-hover);
    color: var(--text-primary);
  }

  .dropdown-item-details {
    flex: 1;
  }

  .dropdown-item-label {
    font-size: 12px;
    font-weight: 500;
  }

  .dropdown-item-desc {
    font-size: 10px;
    color: var(--text-muted);
  }

  .tokens-left {
    font-size: 9px;
    color: var(--text-muted);
    background-color: var(--bg-elevated);
    padding: 1px 4px;
    border-radius: 2px;
  }

  /* Message Area */
  .messages-container {
    flex: 1;
    overflow-y: auto;
    padding: 24px var(--space-5);
    display: flex;
    flex-direction: column;
    gap: 20px;
  }

  .chat-area.empty .messages-container {
    flex: 0 0 auto;
    width: 100%;
    max-width: 800px;
    padding-bottom: 0;
    min-height: auto;
    overflow-y: visible;
  }

  .empty-welcome-message {
    text-align: center;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    margin-bottom: 12px;
  }

  .message {
    display: flex;
    gap: 12px;
    max-width: 80%;
    animation: msgAppear 0.2s var(--ease-out);
  }

  @keyframes msgAppear {
    from { opacity: 0; transform: translateY(8px); }
    to { opacity: 1; transform: translateY(0); }
  }

  .message.user {
    align-self: flex-end;
    flex-direction: row-reverse;
  }

  .message.assistant {
    align-self: flex-start;
  }

  .message-avatar {
    width: 30px;
    height: 30px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 12px;
    flex-shrink: 0;
    font-weight: 600;
    user-select: none;
  }

  .message.user .message-avatar {
    background-color: var(--accent);
    color: var(--text-inverse);
  }

  .message.assistant .message-avatar {
    background-color: var(--bg-surface);
    border: 1px solid var(--border-default);
    color: var(--text-secondary);
  }

  .avatar-svg {
    width: 16px;
    height: 16px;
    stroke: currentColor;
    fill: none;
    stroke-width: 1.8;
  }

  .message-content {
    flex: 1;
    min-width: 0;
  }

  .message-bubble {
    padding: 12px 16px;
    border-radius: var(--radius-xl);
    font-size: 14px;
    line-height: 1.65;
  }

  .message.user .message-bubble {
    background-color: var(--user-bubble);
    color: var(--user-bubble-text);
    border-bottom-right-radius: 4px;
  }

  .message.assistant .message-bubble {
    background-color: var(--assistant-bubble);
    border: 1px solid var(--assistant-bubble-border);
    color: var(--text-primary);
    border-bottom-left-radius: 4px;
  }

  .message-bubble :global(p) {
    margin-bottom: 8px;
  }

  .message-bubble :global(p:last-child) {
    margin-bottom: 0;
  }

  .message-bubble :global(code) {
    background-color: rgba(0, 0, 0, 0.06);
    padding: 2px 6px;
    border-radius: var(--radius-sm);
    font-family: var(--font-mono);
    font-size: 12px;
  }

  .message-bubble :global(strong) {
    font-weight: 600;
  }

  .message-time {
    font-size: 11px;
    color: var(--text-muted);
    margin-top: 4px;
    padding: 0 4px;
  }

  .message.user .message-time {
    text-align: right;
  }

  /* Tool Use Card */
  .tool-use-card {
    background-color: var(--bg-surface);
    border: 1px solid var(--border-default);
    border-radius: var(--radius-lg);
    margin-top: 10px;
    overflow: hidden;
  }

  .tool-use-header {
    display: flex;
    align-items: center;
    gap: 8px;
    width: 100%;
    padding: 10px 14px;
    background: transparent;
    border: none;
    cursor: pointer;
    font-family: var(--font-sans);
    color: var(--text-secondary);
    font-size: 12px;
    font-weight: 500;
  }

  .tool-icon {
    width: 14px;
    height: 14px;
    stroke: currentColor;
    fill: none;
    stroke-width: 1.8;
  }

  .tool-status-badge {
    margin-left: auto;
    font-size: 10px;
    font-weight: 600;
    padding: 1px 6px;
    border-radius: var(--radius-sm);
  }

  .tool-status-badge.success {
    background-color: rgba(34, 197, 94, 0.1);
    color: var(--success);
  }

  .tool-use-chevron {
    width: 14px;
    height: 14px;
    stroke: currentColor;
    fill: none;
    stroke-width: 2;
    transition: transform var(--duration-fast);
  }

  .tool-use-chevron.expanded {
    transform: rotate(180deg);
  }

  .tool-use-body {
    padding: 10px 14px;
    border-top: 1px solid var(--border-default);
    background-color: var(--bg-elevated);
  }

  .tool-use-body pre {
    margin: 0;
    font-family: var(--font-mono);
    font-size: 11px;
    color: var(--text-secondary);
    white-space: pre-wrap;
    word-break: break-all;
  }

  /* Code Block Wrapper */
  .code-block-wrapper {
    margin-top: 10px;
    border: 1px solid var(--border-default);
    border-radius: var(--radius-lg);
    overflow: hidden;
  }

  .code-block-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 8px 14px;
    background-color: var(--bg-surface);
    border-bottom: 1px solid var(--border-default);
    font-size: 12px;
    font-weight: 500;
    color: var(--text-secondary);
  }

  .code-copy-btn {
    display: inline-flex;
    align-items: center;
    background: transparent;
    border: none;
    color: var(--text-muted);
    font-size: 11px;
    cursor: pointer;
    font-family: var(--font-sans);
    transition: color var(--duration-fast);
  }

  .code-copy-btn:hover {
    color: var(--text-primary);
  }

  .code-block {
    background-color: var(--bg-elevated);
    padding: 14px;
    overflow-x: auto;
  }

  .code-block pre {
    margin: 0;
  }

  .code-block code {
    font-family: var(--font-mono);
    font-size: 12.5px;
    color: var(--text-primary);
  }

  /* Suggested Chips */
  .suggestion-chips {
    display: flex;
    gap: 6px;
    flex-wrap: wrap;
  }

  .suggestion-chips-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 8px;
    margin-top: 24px;
    max-width: 800px;
    width: 100%;
    padding: 0 24px;
  }

  .chip {
    padding: 6px 12px;
    border: 1px solid var(--border-default);
    border-radius: var(--radius-full);
    padding: 12px 16px;
    background-color: var(--bg-surface);
    border: 1px solid var(--border-default);
    border-radius: var(--radius-lg);
    font-size: 13px;
    color: var(--text-secondary);
    cursor: pointer;
    transition: all var(--duration-fast) var(--ease-out);
    text-align: left;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .chip:hover {
    background-color: var(--bg-hover);
    color: var(--text-primary);
    border-color: var(--border-strong);
    transform: translateY(-1px);
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
  }

  /* Composer Input */
  .composer {
    position: relative;
    padding: 0 var(--space-5) var(--space-4);
    background-color: var(--bg-base);
    flex-shrink: 0;
    max-width: 58rem;
    margin: 0 auto;
    width: 100%;
  }

  .chat-area.empty .composer {
    flex: 0 0 auto;
    width: 100%;
    max-width: 800px;
    background: transparent;
    padding-top: 0;
    margin: 0 auto;
  }

  .composer-mascot {
    position: absolute;
    bottom: calc(100% - 10px);
    left: 20px;
    z-index: 20;
    width: 88px;
    height: 88px;
    pointer-events: none;
    filter: drop-shadow(0 12px 22px rgba(15, 23, 42, 0.16));
  }

  .mascot-body {
    transform-origin: center bottom;
  }

  .composer-mascot :global(.blink) {
    animation: mascotBlink 4s ease-in-out infinite;
  }

  .composer-mascot :global(.arm-left) {
    transform-origin: 60px 100px;
    animation: armWave 3s ease-in-out infinite;
  }

  .composer-mascot :global(.arm-right) {
    transform-origin: 140px 100px;
  }

  .composer-mascot :global(.glare-sweep) {
    animation: glareSweep 5s ease-in-out infinite;
  }

  .composer-mascot :global(.shadow-anim) {
    animation: shadowPulse 3s ease-in-out infinite;
  }

  @keyframes mascotBlink {
    0%, 95%, 100% { transform: scaleY(1); }
    97% { transform: scaleY(0.1); }
  }

  @keyframes armWave {
    0%, 100% { transform: rotate(0deg); }
    25% { transform: rotate(-8deg); }
    75% { transform: rotate(8deg); }
  }

  @keyframes glareSweep {
    0%, 100% { transform: translateX(-20px); opacity: 0; }
    40%, 60% { transform: translateX(20px); opacity: 0.6; }
  }

  @keyframes shadowPulse {
    0%, 100% { rx: 35; opacity: 0.15; }
    50% { rx: 30; opacity: 0.1; }
  }

  .composer-container {
    position: relative;
    border-radius: var(--radius-xl);
    background-color: var(--bg-surface);
    border: 1px solid var(--border-default);
    box-shadow: var(--shadow-sm);
    transition: border-color var(--duration-normal), box-shadow var(--duration-normal);
  }

  .composer-container:focus-within {
    border-color: var(--accent);
    box-shadow: 0 0 0 3px rgba(37, 99, 235, 0.08), var(--shadow-sm);
  }

  .composer-glow {
    position: absolute;
    inset: -2px;
    border-radius: 14px;
    border: 2px solid transparent;
    opacity: 0;
    transition: opacity var(--duration-normal);
    pointer-events: none;
  }

  .composer-container:focus-within .composer-glow {
    opacity: 1;
    border-color: rgba(37, 99, 235, 0.15);
  }

  .composer-input {
    width: 100%;
    padding: 14px 14px 0;
    background: transparent;
    border: none;
    outline: none;
    resize: none;
    color: var(--text-primary);
    font-family: var(--font-sans);
    font-size: 14px;
    line-height: 1.5;
    min-height: 52px;
  }

  .composer-input::placeholder {
    color: var(--text-muted);
  }

  .composer-toolbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 8px 12px;
    border-top: 1px dashed var(--border-default);
  }

  .composer-toolbar-left {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .composer-btn {
    width: 28px;
    height: 28px;
    border-radius: var(--radius-md);
    background: transparent;
    border: none;
    color: var(--text-muted);
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: all var(--duration-fast);
  }

  .composer-btn:hover {
    background-color: var(--bg-hover);
    color: var(--text-primary);
  }

  .composer-dropdown {
    position: absolute;
    bottom: calc(100% + 8px);
    left: 0;
    background-color: var(--bg-surface);
    border: 1px solid var(--border-default);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-lg);
    padding: var(--space-1.5);
    z-index: 100;
    width: 160px;
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .composer-dropdown-title {
    font-size: 10px;
    font-weight: 600;
    text-transform: uppercase;
    color: var(--text-muted);
    padding: 4px 8px;
    letter-spacing: 0.5px;
  }

  .composer-dropdown-item {
    display: flex;
    align-items: center;
    padding: 6px 8px;
    background: transparent;
    border: none;
    border-radius: var(--radius-sm);
    color: var(--text-secondary);
    font-size: 12px;
    width: 100%;
    cursor: pointer;
    font-family: var(--font-sans);
    text-align: left;
  }

  .composer-dropdown-item:hover {
    background-color: var(--bg-hover);
    color: var(--text-primary);
  }

  .attachment-pill {
    display: inline-flex;
    align-items: center;
    background-color: var(--bg-elevated);
    border: 1px solid var(--border-default);
    border-radius: var(--radius-md);
    padding: 2px 8px;
    font-size: 11px;
    font-weight: 500;
    color: var(--text-secondary);
  }

  .inline-svg {
    width: 12px;
    height: 12px;
    stroke: currentColor;
    fill: none;
    stroke-width: 2;
  }

  .composer-toolbar-right {
    display: flex;
    align-items: center;
  }

  .send-btn {
    width: 28px;
    height: 28px;
    border-radius: 50%;
    background-color: var(--accent);
    color: var(--text-inverse);
    border: none;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: all var(--duration-fast);
  }

  .send-btn:hover:not(:disabled) {
    background-color: var(--accent-hover);
    transform: scale(1.05);
  }

  .send-btn:disabled {
    background-color: var(--bg-elevated);
    color: var(--text-muted);
    cursor: not-allowed;
  }

  /* Right Canvas Panel */
  .canvas-resize-handle {
    width: 4px;
    height: 100%;
    cursor: col-resize;
    z-index: 45;
    background-color: transparent;
    transition: background-color var(--duration-fast);
  }

  .canvas-resize-handle:hover, .main-layout.resizing-canvas .canvas-resize-handle {
    background-color: var(--accent);
  }

  .canvas-panel {
    background-color: var(--canvas-bg);
    border-left: 1px solid var(--canvas-border);
    display: flex;
    flex-direction: column;
    height: 100%;
    overflow: hidden;
    position: relative;
    z-index: 40;
    flex-shrink: 0;
  }

  .canvas-header {
    height: 48px;
    padding: 8px var(--space-3);
    border-bottom: 1px solid var(--canvas-border);
    background-color: var(--canvas-bg);
    display: flex;
    align-items: center;
    justify-content: space-between;
    flex-shrink: 0;
  }

  .canvas-tabs {
    display: flex;
    gap: 2px;
  }

  .canvas-tab {
    padding: 5px 12px;
    border-radius: var(--radius-md);
    font-size: 12px;
    font-weight: 500;
    color: var(--text-secondary);
    cursor: pointer;
    transition: all 0.12s;
    border: none;
    background: transparent;
    font-family: var(--font-sans);
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .canvas-tab:hover {
    background-color: rgba(0, 0, 0, 0.04);
  }

  .canvas-tab.active {
    background-color: var(--bg-surface);
    box-shadow: var(--shadow-sm);
    color: var(--text-primary);
  }

  .canvas-close {
    width: 24px;
    height: 24px;
    border: none;
    background: transparent;
    color: var(--text-muted);
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: var(--radius-sm);
    cursor: pointer;
  }

  .canvas-close:hover {
    background-color: rgba(0, 0, 0, 0.04);
    color: var(--text-primary);
  }

  .canvas-body {
    flex: 1;
    overflow-y: auto;
    padding: var(--space-4);
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  /* Canvas Cards */
  .canvas-card {
    background-color: var(--bg-surface);
    border: 1px solid var(--canvas-border);
    border-radius: var(--radius-lg);
    overflow: hidden;
    padding: var(--space-4);
  }

  .canvas-card-header {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 13px;
    font-weight: 600;
    color: var(--text-primary);
    margin-bottom: 12px;
  }

  .card-preview-area {
    background-color: var(--bg-elevated);
    border-radius: var(--radius-md);
    padding: 20px;
    text-align: center;
    border: 1px solid var(--canvas-border);
  }

  .preview-box {
    width: 100%;
    height: 120px;
    background: linear-gradient(135deg, #dbeafe, #e0e7ff);
    border-radius: var(--radius-md);
    display: flex;
    align-items: center;
    justify-content: center;
    margin-bottom: 12px;
  }

  .preview-title {
    font-size: 13px;
    font-weight: 600;
    color: var(--text-primary);
    margin-bottom: 4px;
  }

  .preview-subtitle {
    font-size: 12px;
    color: var(--text-muted);
  }

  /* Tool Grid */
  .tool-grid {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .tool-card {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: var(--space-2.5) var(--space-3);
    border: 1px solid var(--canvas-border);
    border-radius: var(--radius-md);
    background-color: var(--bg-surface);
  }

  .tool-card-icon {
    width: 28px;
    height: 28px;
    border-radius: var(--radius-sm);
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
  }

  .tool-card-icon.bg-blue {
    background-color: #dbeafe;
    color: #2563eb;
  }

  .tool-card-icon.bg-green {
    background-color: #dcfce7;
    color: #16a34a;
  }

  .tool-card-details {
    flex: 1;
  }

  .tool-card-title {
    font-size: 12px;
    font-weight: 600;
    color: var(--text-primary);
  }

  .tool-card-desc {
    font-size: 11px;
    color: var(--text-muted);
  }

  /* Code view */
  .code-tree-view {
    display: flex;
    flex-direction: column;
    height: 100%;
  }

  .code-tree-header {
    font-size: 12px;
    font-weight: 600;
    color: var(--text-secondary);
    margin-bottom: 8px;
  }

  .code-editor-mock {
    background-color: var(--bg-elevated);
    border: 1px solid var(--canvas-border);
    border-radius: var(--radius-md);
    padding: 14px;
    overflow-x: auto;
    font-family: var(--font-mono);
    font-size: 12px;
    color: var(--text-primary);
    flex: 1;
  }

  /* Mock Browser */
  .mock-browser {
    border: 1px solid var(--canvas-border);
    border-radius: var(--radius-lg);
    overflow: hidden;
    display: flex;
    flex-direction: column;
    background-color: var(--bg-surface);
    height: 320px;
  }

  .browser-address-bar {
    height: 34px;
    background-color: var(--bg-elevated);
    border-bottom: 1px solid var(--canvas-border);
    display: flex;
    align-items: center;
    padding: 0 10px;
    gap: 6px;
  }

  .dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
  }

  .dot.red { background-color: #ef4444; }
  .dot.yellow { background-color: #f59e0b; }
  .dot.green { background-color: #22c55e; }

  .address-input {
    flex: 1;
    background-color: var(--bg-surface);
    border: 1px solid var(--canvas-border);
    border-radius: var(--radius-sm);
    font-size: 11px;
    padding: 2px 10px;
    color: var(--text-secondary);
    margin-left: 10px;
  }

  .browser-body {
    flex: 1;
    padding: 20px;
    background-color: var(--bg-base);
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .preview-app {
    text-align: center;
  }

  /* Mock Terminal */
  .mock-terminal {
    background-color: #0f172a;
    border-radius: var(--radius-lg);
    padding: 14px;
    height: 240px;
    overflow-y: auto;
  }

  .terminal-body {
    font-family: var(--font-mono);
    font-size: 12px;
    color: #cbd5e1;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .terminal-line {
    word-break: break-all;
  }

  .terminal-line.text-muted { color: #64748b; }
  .terminal-line.text-success { color: #22c55e; }
</style>
