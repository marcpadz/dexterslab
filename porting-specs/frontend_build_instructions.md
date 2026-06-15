# Frontend Build Instructions: Ask Dexter Tauri v2

This document provides the complete, authoritative specifications and instructions for building the frontend of the **Ask Dexter** Tauri v2 desktop application. It integrates all design tokens, layouts, component trees, and state management specifications into a single step-by-step guide.

---

## 1. Architectural & Shell Layout System

The application is built using **Svelte 5** and **Tailwind CSS**. It is structured as a Tauri window layout using CSS Grid, with a horizontal split-pane workspace and a collapsible, resizable bottom terminal panel.

### 1.1 Root Layout Grid
The root application shell spans the full viewport width and height (`100vw` by `100vh`) and is divided into three rows (Titlebar, Workspace + Sidebar, Statusbar) and two columns (Sidebar, Workspace):

```css
.app-shell {
  display: grid;
  grid-template-rows: 38px 1fr 24px;
  grid-template-columns: auto 1fr;
  height: 100vh;
  width: 100vw;
  overflow: hidden;
  background-color: var(--bg-base);
}
```

```
┌─────────────────────────────────────────────────────────────────┐
│ macOS Lights │ macOS Title Bar (38px drag region)               │
├──────────────┼──────────────────────────────────────────────────┤
│              │              Main Workspace Area (Row 2)         │
│   Sidebar    │    ┌────────────────────┬──────────────────┐     │
│   (240px)    │    │                    │  Canvas Panel    │     │
│   Gradient   │    │   Chat Panel       │  (Artifact List/ │     │
│   Rounded    │    │   (flex: 1)        │   Previews/Edit) │     │
│   Container  │    │                    │  (Resizable,     │     │
│              │    │                    │   collapsible)   │     │
│   Row 1-2    │    ├────────────────────┴──────────────────┤     │
│              │    │  Collapsible Bottom Terminal Panel    │     │
│   Collapse   │    │  (Height: 200px, collapsible, PTY)    │     │
│   to 56px    │    └───────────────────────────────────────┘     │
├──────────────┴──────────────────────────────────────────────────┤
│ Status Bar (24px) ─── sidecar status ─── model ─── tokens ───── │
└─────────────────────────────────────────────────────────────────┘
```

### 1.2 Layout Containers

* **macOS Title Bar (`.titlebar`)**: 38px height, containing custom traffic lights margin padding, drag region (`-webkit-app-region: drag`), window control spacing, active chat title, and canvas toggle.
* **Sidebar (`.sidebar`)**: 240px width (collapses to 56px). Features a rounded container layout (`border-radius: var(--radius-xl)`), custom branded background gradient, and a frosted navigation rail.
* **Workspace Pane (`.workspace`)**: Spans the remaining columns and row.
  - Spaced by `8px` gap from the sidebar.
  - Divided vertically into `.workspace-split` (top) and `.terminal-bottom-panel` (bottom).
* **Workspace Split (`.workspace-split`)**: Spaced horizontally using a resizable flex layout between the **Chat Panel** (`.chat-area`) and the **Canvas Panel** (`.canvas-panel`).
* **Terminal Bottom Panel (`.terminal-bottom-panel`)**: Collapsible and resizable drawer positioned at the bottom of the workspace. Toggles via `Ctrl+`` keyboard shortcut.

---

## 2. Design Token System (CSS Variables)

All styles must reference the following CSS custom properties defined in `index.css`:

### 2.1 Navy Blue Dark Theme (Default)
```css
:root[data-theme="dark"] {
  --bg-base: #0b1a33;
  --bg-surface: #0f2247;
  --bg-elevated: #142d5c;
  --bg-hover: rgba(255, 255, 255, 0.06);
  --bg-active: rgba(255, 255, 255, 0.1);
  --bg-input: #0b1a33;

  --text-primary: #fcfcfc;
  --text-secondary: #8fa4c4;
  --text-muted: #5a7299;
  --text-inverse: #0b1a33;

  --accent: #0169cc;
  --accent-hover: #0180f0;
  --success: #00a240;
  --warning: #f59e0b;
  --error: #e02e2a;
  --info: #0169cc;

  --border-subtle: #0f2247;
  --border-default: #1a3666;
  --border-focus: #0169cc;

  --chat-user-bg: #0169cc;
  --chat-user-text: #fcfcfc;
  --chat-assistant-bg: #0f2247;
  --chat-assistant-text: #fcfcfc;

  --sidebar-bg-gradient: linear-gradient(180deg, #0b3772 0%, #0169cc 100%);
  --sidebar-text: #fcfcfc;
  --sidebar-text-muted: rgba(252, 252, 252, 0.6);
  --sidebar-hover: rgba(255, 255, 255, 0.08);
  --sidebar-active: rgba(255, 255, 255, 0.14);
  --sidebar-border: rgba(255, 255, 255, 0.1);
  --sidebar-surface: #b06dff;
}
```

### 2.2 Light Theme (Alternative)
```css
:root[data-theme="light"] {
  --bg-base: #fcfcfc;
  --bg-surface: #ffffff;
  --bg-elevated: #f4f6f9;
  --bg-hover: rgba(0, 0, 0, 0.04);
  --bg-active: rgba(0, 0, 0, 0.07);
  --bg-input: #f4f6f9;

  --text-primary: #0b1a33;
  --text-secondary: #4a5e7a;
  --text-muted: #8a9bb5;
  --text-inverse: #fcfcfc;

  --accent: #0169cc;
  --accent-hover: #0155a3;
  --success: #00a240;
  --warning: #d97706;
  --error: #e02e2a;
  --info: #0169cc;

  --border-subtle: #e8ecf1;
  --border-default: #d4dbe5;
  --border-focus: #0169cc;

  --chat-user-bg: #0169cc;
  --chat-user-text: #fcfcfc;
  --chat-assistant-bg: #ffffff;
  --chat-assistant-text: #0b1a33;

  --sidebar-bg-gradient: linear-gradient(180deg, #1e3a5f 0%, #3b82f6 100%);
  --sidebar-text: #fcfcfc;
  --sidebar-text-muted: rgba(252, 252, 252, 0.6);
  --sidebar-hover: rgba(255, 255, 255, 0.08);
  --sidebar-active: rgba(255, 255, 255, 0.14);
  --sidebar-border: rgba(255, 255, 255, 0.1);
  --sidebar-surface: #7c3aed;
}
```

### 2.3 Visual Scales
* **Spacing (4px base)**: `--space-0-5` (2px), `--space-1` (4px), `--space-1-5` (6px), `--space-2` (8px), `--space-2-5` (10px), `--space-3` (12px), `--space-3-5` (14px), `--space-4` (16px), `--space-5` (20px), `--space-6` (24px).
* **Typography**: `--text-xs` (11px / line-height 16px), `--text-sm` (13px / line-height 20px), `--text-base` (14px / line-height 22px), `--text-lg` (16px / line-height 24px), `--text-xl` (18px / line-height 28px).
* **Border Radii**: `--radius-sm` (4px), `--radius-md` (6px), `--radius-lg` (8px), `--radius-xl` (12px), `--radius-full` (9999px).
* **Transitions**: `--duration-fast` (100ms), `--duration-normal` (200ms), `--duration-slow` (300ms), `--ease-out`: `cubic-bezier(0.16, 1, 0.3, 1)`.

---

## 3. Svelte Component Specifications

### 3.1 Sidebar Component (`Sidebar.svelte`)
* **Layout**: Round container spanning grid rows 1 and 2 in expanded state; shifts to row 2 in collapsed state.
* **Branding**: Contains the stacked layers icon and "Ask Dexter" header. Text collapses cleanly into icon rail width (`56px`).
* **5-Tab Navigation**:
  1. **Chat**: MessageSquare icon. Sub-navigation: New Chat, Library, Projects, Files, All Chats.
  2. **Agent**: Orbiting electrons icon. Sub-navigation: Skills, Apps, Marketplace, Connectors, AI Hub, Memories.
  3. **Notes**: Dog-eared document icon. Sub-navigation: Recent Notes, All Notes, Trash.
  4. **Work**: Desktop workstation icon. Sub-navigation: Email, Calendar, Tasks.
  5. **Playground**: Gamepad icon. Sub-navigation: Tamagotchi.
* **Contextual menus**: Right-clicking thread list elements opens custom context overlay with Select, Pin, Move to Project, and Delete.
* **Bottom Controls**: Theme toggle slider track, Chevron collapse button, and settings/profile triggers.

### 3.2 Chat Pane Component (`ChatPane.svelte`)
* **Empty State**: Renders welcome banner, 4 suggestion chips in a 2x2 grid, and welcome quick cards (Get Started, Bring Your Code, Workspace Skills).
* **Messages Container**:
  * **User Message**: Right-aligned, colored with `--chat-user-bg` and `--chat-user-text`.
  * **Assistant Message**: Left-aligned with Mascot Avatar. Emits streaming tokens character-by-character.
  * **Tool Use Card**: Collapsible accordions with header labels ("Filesystem", "Done") and detailed JSON command payloads.
* **Composer Toolbar**:
  * Left: Model selector tabbed dropdown (Cloud Pro/Sonnet vs. Local Llama/Phi) and Connectors toggle popover (Gmail, Drive, Calendar, Canva, Notion).
  * Center: Auto-growing multiline textarea input supporting drag-and-drop.
  * Right: Dynamic send icon (Microphone SVG default → Send Arrow up SVG when text present).

### 3.3 Canvas Panel Component (`CanvasPanel.svelte`)
The Canvas Panel acts as the resizable workspace side pane adjacent to the chat area. It implements a 3-tab header and dynamically spawns **Preview Tabs** on demand.

* **Tabs Header System**:
  1. **Artifacts List Tab** (always visible): Displays list of generated assets.
  2. **Editor Tab** (always visible): A document sandbox for collaborative text and code modifications.
  3. **Browser Tab** (always visible): WebView preview viewport.
  4. **Preview Tabs** (dynamically spawned): Appends to the tab list when clicking items in the Artifacts List.
* **Artifacts List Tab (`canvas-tab-artifacts`)**:
  * Displays files generated by the AI (e.g. `DataTable.tsx`, `index.html`, `flowchart.mermaid`).
  * Clicking an artifact spawns a new tab `<button class="canvas-tab active">` with the file name and an `×` close button.
  * Spawning a preview switches the active tab automatically to the new preview.
  * Previews render code blocks in readonly syntax highlighting via `shiki`/`highlight.js`, HTML inside iframe sandboxes with strict CSP, and SVG diagrams using `mermaid`.
* **Collaborative Editor Tab (`canvas-tab-editor`)**:
  * Renders active editable files. Features an "Upload Document" dropzone area and text editor.
  * Users can upload documents directly to collaborate with the AI.
  * **Inline AI Prompter**: Selecting any text inside the editor displays a hovering inline prompt overlay to edit/explain/summarize the selection.
  * `Cmd+S` saves changes back to the filesystem.
* **WebView Browser Tab (`canvas-tab-browser`)**:
  * Displays address bar with URL placeholder and status indicator.
  * Embeds running developer web server preview (`http://localhost:5173/`).

### 3.4 Collapsible Bottom Terminal Panel (`BottomTerminal.svelte`)
The terminal is refactored from a canvas tab into a permanent bottom-docked workspace console drawer.

* **Layout & Geometry**: Sit beneath `.chat-workspace-layout` vertically. Top border features a horizontal resizer handle (`.terminal-resize-handle`) allowing height adjustment from `80px` to `400px`.
* **PTY Console Emulation**: Renders command output stream using `xterm.js`.
* **Toggles & Shortcut**:
  * Toggle chevron button on the terminal header collapses/expands the terminal.
  * `Ctrl+`` (backtick) global shortcut toggles collapse drawer and focuses active cursor prompt instantly.
  * Statusbar features a clickable "Terminal" icon button to toggle the drawer.

### 3.5 Status Bar Component (`StatusBar.svelte`)
* **Left**: Pi SDK Sidecar and Model Server (llama-server) online running badges. Clickable "Terminal" icon button.
* **Right**: Active model label (`llama-3.1-8b-instruct`), token budget usage (`1,240`), and network status pill ("Local").

---

## 4. State Management and Tauri IPC

Frontend state is managed reactively using Svelte 5 `$state` runes.

### 4.1 Conversation Store (`src/stores/conversation.svelte.ts`)
```typescript
interface ConversationState {
  threads: Array<{ id: string; title: string; updatedAt: string; modelId: string }>;
  activeId: string | null;
  activeMessages: Message[];
  isStreaming: boolean;
  streamingContent: string;
  canvasOpen: boolean;
  canvasActiveTab: string; // 'artifacts' | 'editor' | 'browser' | previewTabId
  canvasOpenPreviews: Array<{ id: string; name: string; type: string; content: string }>;
  terminalOpen: boolean;
}

export const conversationState = $state<ConversationState>({
  threads: [],
  activeId: null,
  activeMessages: [],
  isStreaming: false,
  streamingContent: '',
  canvasOpen: false,
  canvasActiveTab: 'artifacts',
  canvasOpenPreviews: [],
  terminalOpen: false
});
```

### 4.2 Dynamic Preview Methods
```typescript
export function openPreviewTab(id: string, name: string, type: string, content: string) {
  const exists = conversationState.canvasOpenPreviews.some(p => p.id === id);
  if (!exists) {
    conversationState.canvasOpenPreviews.push({ id, name, type, content });
  }
  conversationState.canvasOpen = true;
  conversationState.canvasActiveTab = `preview-${id}`;
}

export function closePreviewTab(id: string) {
  conversationState.canvasOpenPreviews = conversationState.canvasOpenPreviews.filter(p => p.id !== id);
  if (conversationState.canvasActiveTab === `preview-${id}`) {
    conversationState.canvasActiveTab = 'artifacts';
  }
}
```

---

## 5. Step-by-Step Implementation Guide

### Phase 1: Shell & Token Definition (Days 1–5)
1. Initialize the Svelte 5 project structure. Create `src/lib/components/shell/` and `src/lib/components/canvas/`.
2. Define design system variables inside `src/index.css` under `:root[data-theme="dark"]` and `:root[data-theme="light"]`.
3. Construct `AppShell.svelte` implementing the macOS-drag titlebar, sidebar navigation grids, and main workspace containers.

### Phase 2: Sidebar Navigation & Page Views (Days 6–10)
1. Implement the sidebar collapse states (56px icon rail vs. 240px expanded).
2. Set up client-side routing matching the 5 primary tabs. Make page views render within Svelte layout sub-directories.
3. Build the 11 secondary surfaces including Memories list, Knowledge Base vector tree views, and AI Hub model card tables.

### Phase 3: Chat Composer & Accordions (Days 11–15)
1. Build the composer textarea with auto-growing height, model selection pills, and app integration connector popovers.
2. Code Svelte message containers. Implement inline copy/delete hooks and floating branch selectors appearing on hover.
3. Write accordion tool-use components that parse Pi SDK command inputs.

### Phase 4: Dynamic Canvas Panel (Days 16–22)
1. Code `CanvasPanel.svelte` with the resize vertical handle and custom toggle.
2. Implement Svelte tabs header: Artifacts, Editor, and Browser.
3. Code the dynamic Preview tab creation system. Link clicking on the artifacts file list in Svelte to trigger dynamic preview additions.
4. Implement the Editor tab document dropzone and inline prompter text selection overlays.

### Phase 5: Collapsible Bottom Terminal & Verification (Days 23–30)
1. Move the terminal component from Canvas into the bottom panel container. Style the top horizontal resize bar.
2. Integrate `xterm.js` canvas rendering. Connect keyboard backtick listeners.
3. Bind Svelte stores to Tauri event listeners to stream sidecar process outputs. Verify keyboard shortcuts and layout integrity.
