# Frontend Design: Ask Dexter Tauri v2

## 1. Introduction and Scope

This document specifies the complete frontend architecture for the Ask Dexter Tauri v2 desktop application. It maps every user-facing feature to its visual components, defines screen layouts, interaction flows, state management patterns, and design tokens.

**Design Philosophy:**
- **Density over Decoration**: Information-dense layouts suited for professional developer workflows. No decorative whitespace. Every pixel earns its place.
- **Keyboard-First**: Every action accessible via keyboard shortcut. Mouse is optional for power users.
- **Professional Tool Aesthetic**: Inspired by Linear, Raycast, and Warp — clean, fast, dark-first.
- **Native Desktop Patterns**: Drag-and-drop, context menus, resizable panels, native window chrome — not web app patterns dressed as desktop.

**Reference**: All UI components use 21st.dev component patterns as design reference, manually ported to Svelte 5 equivalents.

---

## 2. Design Token System

All visual values are expressed as CSS custom properties. No inline colors, hardcoded spacing, or magic numbers in component code.

### 2.1 Color Palette

#### 2.1.1 Dark Theme (Default Navy Blue Palette)

**Background Layers:**
| Token | Value | Usage |
|-------|-------|-------|
| `--bg-base` | `#0b1a33` | Application background (navy) |
| `--bg-surface` | `#0f2247` | Panels, cards, sidebar containers |
| `--bg-elevated` | `#142d5c` | Dropdowns, modals, popovers |
| `--bg-hover` | `rgba(255, 255, 255, 0.06)` | Hover states |
| `--bg-active` | `rgba(255, 255, 255, 0.1)` | Active/selected states |
| `--bg-input` | `#0b1a33` | Input field backgrounds |

**Text:**
| Token | Value | Usage |
|-------|-------|-------|
| `--text-primary` | `#fcfcfc` | Headings, primary body text |
| `--text-secondary` | `#8fa4c4` | Descriptions, metadata, secondary text |
| `--text-muted` | `#5a7299` | Placeholders, disabled states |
| `--text-inverse` | `#0b1a33` | Text on light/accent backgrounds |

**Accent & Semantic:**
| Token | Value | Usage |
|-------|-------|-------|
| `--accent` | `#0169cc` | Primary brand blue, links, focus rings |
| `--accent-hover` | `#0180f0` | Accent hover state |
| `--success` | `#00a240` | Success states, online indicators |
| `--warning` | `#f59e0b` | Warnings, quota alerts |
| `--error` | `#e02e2a` | Error states, destructive actions |
| `--info` | `#0169cc` | Informational badges |

**Borders:**
| Token | Value | Usage |
|-------|-------|-------|
| `--border-subtle` | `#0f2247` | Panel separators |
| `--border-default` | `#1a3666` | Card borders, input borders |
| `--border-focus` | `#0169cc` | Focus ring color |

**Chat Bubble Specific:**
| Token | Value | Usage |
|-------|-------|-------|
| `--chat-user-bg` | `#0169cc` | User message bubble background |
| `--chat-user-text` | `#fcfcfc` | User message bubble text |
| `--chat-assistant-bg` | `#0f2247` | Assistant message bubble background |
| `--chat-assistant-text` | `#fcfcfc` | Assistant message bubble text |

**Sidebar Specific:**
| Token | Value | Usage |
|-------|-------|-------|
| `--sidebar-bg-gradient` | `linear-gradient(180deg, #0b3772 0%, #0169cc 100%)` | Sidebar branded background gradient |
| `--sidebar-text` | `#fcfcfc` | Sidebar primary text and active icons |
| `--sidebar-text-muted` | `rgba(252, 252, 252, 0.6)` | Sidebar secondary labels, muted icons |
| `--sidebar-hover` | `rgba(255, 255, 255, 0.08)` | Sidebar items hover background |
| `--sidebar-active` | `rgba(255, 255, 255, 0.14)` | Sidebar items active background |
| `--sidebar-border` | `rgba(255, 255, 255, 0.1)` | Sidebar internal borders/dividers |
| `--sidebar-surface` | `#b06dff` | Purple indicator pill/badges |

#### 2.1.2 Light Theme (Alternative Light Palette)

**Background Layers:**
| Token | Value | Usage |
|-------|-------|-------|
| `--bg-base` | `#fcfcfc` | Application background |
| `--bg-surface` | `#ffffff` | Panels, cards, sidebar containers |
| `--bg-elevated` | `#f4f6f9` | Dropdowns, modals, popovers |
| `--bg-hover` | `rgba(0, 0, 0, 0.04)` | Hover states |
| `--bg-active` | `rgba(0, 0, 0, 0.07)` | Active/selected states |
| `--bg-input` | `#f4f6f9` | Input field backgrounds |

**Text:**
| Token | Value | Usage |
|-------|-------|-------|
| `--text-primary` | `#0b1a33` | Headings, primary body text |
| `--text-secondary` | `#4a5e7a` | Descriptions, metadata, secondary text |
| `--text-muted` | `#8a9bb5` | Placeholders, disabled states |
| `--text-inverse` | `#fcfcfc` | Text on dark/accent backgrounds |

**Accent & Semantic:**
| Token | Value | Usage |
|-------|-------|-------|
| `--accent` | `#0169cc` | Primary brand blue, links, focus rings |
| `--accent-hover` | `#0155a3` | Accent hover state |
| `--success` | `#00a240` | Success states |
| `--warning` | `#d97706` | Warnings |
| `--error` | `#e02e2a` | Error states |
| `--info` | `#0169cc` | Informational badges |

**Borders:**
| Token | Value | Usage |
|-------|-------|-------|
| `--border-subtle` | `#e8ecf1` | Panel separators |
| `--border-default` | `#d4dbe5` | Card borders, input borders |
| `--border-focus` | `#0169cc` | Focus ring color |

**Chat Bubble Specific:**
| Token | Value | Usage |
|-------|-------|-------|
| `--chat-user-bg` | `#0169cc` | User message bubble background |
| `--chat-user-text` | `#fcfcfc` | User message bubble text |
| `--chat-assistant-bg` | `#ffffff` | Assistant message bubble background |
| `--chat-assistant-text` | `#0b1a33` | Assistant message bubble text |

**Sidebar Specific:**
| Token | Value | Usage |
|-------|-------|-------|
| `--sidebar-bg-gradient` | `linear-gradient(180deg, #1e3a5f 0%, #3b82f6 100%)` | Light mode sidebar gradient |
| `--sidebar-text` | `#fcfcfc` | Sidebar primary text and active icons |
| `--sidebar-text-muted` | `rgba(252, 252, 252, 0.6)` | Sidebar secondary labels, muted icons |
| `--sidebar-hover` | `rgba(255, 255, 255, 0.08)` | Sidebar items hover background |
| `--sidebar-active` | `rgba(255, 255, 255, 0.14)` | Sidebar items active background |
| `--sidebar-border` | `rgba(255, 255, 255, 0.1)` | Sidebar internal borders/dividers |
| `--sidebar-surface` | `#7c3aed` | Purple indicator pill/badges |

### 2.2 Spacing (4px Grid)

All spacing values are derived from a 4px base grid, using hyphenated tokens:

| Token | Value | Usage |
|-------|-------|-------|
| `--space-0-5` | `2px` | Micro gaps (icon-to-text) |
| `--space-1` | `4px` | Tight padding |
| `--space-1-5` | `6px` | Inline elements gap |
| `--space-2` | `8px` | Input padding, list item gap |
| `--space-2-5` | `10px` | Intermediate component spacing |
| `--space-3` | `12px` | Card padding (compact) |
| `--space-3-5` | `14px` | Section elements padding |
| `--space-4` | `16px` | Standard card padding |
| `--space-5` | `20px` | Section spacing |
| `--space-6` | `24px` | Panel padding |
| `--space-8` | `32px` | Section margins |
| `--space-10` | `40px` | Page margins |
| `--space-12` | `48px` | Major section breaks |

### 2.3 Typography

**Font Families:**
| Token | Value | Usage |
|-------|-------|-------|
| `--font-sans` | `-apple-system, BlinkMacSystemFont, 'SF Pro Text', 'Segoe UI', system-ui, sans-serif` | All UI text |
| `--font-mono` | `'JetBrains Mono', 'SF Mono', 'Fira Code', 'Cascadia Code', monospace` | Code blocks, editor, technical content |

**Size Scale:**
| Token | Size | Default Line Height | Default Weight | Usage |
|-------|------|---------------------|----------------|-------|
| `--text-xs` | `11px` | `16px` (`--lh-xs`) | 400 | Badges, timestamps |
| `--text-sm` | `13px` | `20px` (`--lh-sm`) | 400 | Body text, sidebar items |
| `--text-base` | `14px` | `22px` (`--lh-base`) | 400 | Default text |
| `--text-lg` | `16px` | `24px` (`--lh-lg`) | 500 | Section headers |
| `--text-xl` | `18px` | `28px` (`--lh-xl`) | 600 | Page titles |
| `--text-2xl` | `24px` | `32px` (`--lh-2xl`) | 700 | Hero headings (rare) |

**Line Heights:**
| Token | Value | Usage |
|-------|-------|-------|
| `--lh-xs` | `16px` | Compact metadata, badges |
| `--lh-sm` | `20px` | Sidebar navigation, standard UI items |
| `--lh-base` | `22px` | Default body copy |
| `--lh-lg` | `24px` | Paragraph text, subheadings |
| `--lh-xl` | `28px` | Sub-section headings |
| `--lh-2xl` | `32px` | Page headings |

### 2.4 Border Radius

| Token | Value | Usage |
|-------|-------|-------|
| `--radius-sm` | `4px` | Small buttons, badges, checkboxes |
| `--radius-md` | `6px` | Default buttons, input fields, thread items |
| `--radius-lg` | `8px` | Small panels, cards, dropdown menus |
| `--radius-xl` | `12px` | Sidebar container, main layout panels |
| `--radius-full` | `9999px` | Avatars, status pills, circular buttons |

### 2.5 Shadows & Elevation

| Token | Value | Usage |
|-------|-------|-------|
| `--shadow-sm` | `0 1px 2px rgba(0, 0, 0, 0.2)` | Subtle card elevation |
| `--shadow-md` | `0 4px 12px rgba(0, 0, 0, 0.3)` | Dropdown menus, tooltips |
| `--shadow-lg` | `0 8px 24px rgba(0, 0, 0, 0.4)` | Modals, dialog overlays |

### 2.6 Animation Tokens

| Token | Value | Usage |
|-------|-------|-------|
| `--duration-fast` | `100ms` | Micro-interactions (hover fade, checks) |
| `--duration-normal` | `200ms` | Default page transitions, toggles |
| `--duration-slow` | `300ms` | Sheet slide-in, modal scale animations |
| `--ease-out` | `cubic-bezier(0.16, 1, 0.3, 1)` | Standard layout easing |
| `--ease-spring` | `cubic-bezier(0.34, 1.56, 0.64, 1)` | Springy panel slide transitions |

---

## 3. Application Shell and Layout

### 3.1 Root Layout

Currently, the application layout adapts to the expanded or collapsed state of the sidebar. It is structured as a Tauri window layout using CSS Grid, with a vertical flex stack inside the main workspace to host the collapsible bottom Terminal:

**Expanded Sidebar Layout:**
```
┌────────────┬────────────────────────────────────────────────────┐
│ macOS Lights│ macOS Title Bar (38px drag region)                │
│            ├────────────────────────────────────────────────────┤
│  Sidebar   │                                                    │
│  (240px)   │              Main Workspace Area (Row 2)           │
│  Gradient  │    ┌────────────────────┬──────────────────┐       │
│  Rounded   │    │                    │  Canvas Panel    │       │
│  Container │    │   Chat Panel       │  (Artifact List/ │       │
│            │    │   (flex: 1)        │   Previews/Edit) │       │
│  Row 1-2   │    │                    │  (Resizable,     │       │
│            │    │                    │   collapsible)   │       │
│  Collapse  │    │                    │                  │       │
│  to 56px   │    ├────────────────────┴──────────────────┤       │
│            │    │  Collapsible Bottom Terminal Panel    │       │
│            │    │  (Height: 200px, collapsible, PTY)    │       │
│            │    └───────────────────────────────────────┘       │
├────────────┴────────────────────────────────────────────────────┤
│  Status Bar (24px) ─── sidecar status ─── model ─── tokens ─── │
└─────────────────────────────────────────────────────────────────┘
```

**Collapsed Sidebar Layout:**
```
┌─────────────────────────────────────────────────────────────────┐
│ macOS Title Bar (38px drag region)                              │
├────────────┬────────────────────────────────────────────────────┤
│ Collapsed  │                                                    │
│ Sidebar    │              Main Workspace Area (Row 2)           │
│ (56px)     │    ┌────────────────────┬──────────────────┐       │
│            │    │                    │  Canvas Panel    │       │
│ Row 2      │    │   Chat Panel       │  (Artifact List/ │       │
│            │    │   (flex: 1)        │   Previews/Edit) │       │
│            │    │                    │                  │       │
│            │    ├────────────────────┴──────────────────┤       │
│            │    │  Collapsible Bottom Terminal Panel    │       │
│            │    │  (Height: 200px, collapsible, PTY)    │       │
│            │    └───────────────────────────────────────┘       │
└────────────┴────┴───────────────────────────────────────────────┘
│  Status Bar (24px) ─── sidecar status ─── model ─── tokens ─── │
└─────────────────────────────────────────────────────────────────┘
```

### 3.2 CSS Grid and Layout Definition

```css
.app-shell {
  display: grid;
  grid-template-rows: 38px 1fr 24px;
  grid-template-columns: auto 1fr;
  height: 100vh;
  width: 100vw;
  background-color: var(--bg-base);
  overflow: hidden;
  position: relative;
  column-gap: 8px;
}

/* 1. TitleBar */
.titlebar {
  grid-row: 1;
  grid-column: 1 / span 2;
  height: 38px;
  background-color: transparent;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding-right: var(--space-3);
  user-select: none;
  z-index: 100;
  position: relative;
}

/* 2. Sidebar Container */
.sidebar-container {
  grid-row: 2;
  grid-column: 1;
  display: flex;
  position: relative;
  z-index: 50;
  border-radius: 12px;
  overflow: hidden;
}

.sidebar-container:has(.sidebar:not(.collapsed)) {
  grid-row: 1 / span 2; /* Overlaps titlebar in row 1 to contain traffic lights */
}

.sidebar {
  background: var(--sidebar-bg-gradient);
  color: var(--sidebar-text);
  display: flex;
  flex-direction: column;
  height: 100%;
  width: var(--sidebar-width, 240px);
  transition: width 0.25s var(--ease-spring), min-width 0.25s var(--ease-spring);
  overflow: hidden;
  flex-shrink: 0;
  position: relative;
}

.sidebar-container:has(.sidebar:not(.collapsed)) .sidebar {
  padding-top: 38px; /* Safe space for window traffic lights */
}

.sidebar.collapsed {
  width: 56px !important;
  min-width: 56px !important;
}

/* 3. Main Workspace */
.workspace {
  grid-row: 2;
  grid-column: 2;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.workspace-split {
  flex: 1;
  display: flex;
  overflow: hidden;
}

.terminal-bottom-panel {
  height: var(--terminal-height, 200px);
  min-height: 0;
  border-top: 1px solid var(--border-subtle);
  background-color: var(--bg-surface);
  display: flex;
  flex-direction: column;
  overflow: hidden;
  transition: height var(--duration-normal) var(--ease-out);
}

.terminal-bottom-panel.collapsed {
  height: 0px !important;
  border-top: none;
}
```

### 3.3 Layout Component Specs

#### 3.3.1 TitleBar
- **Height**: 38px
- **Window Dragging**: Serves as the window drag region (excluding interactive controls like search and canvas toggles).
- **Branding**: Hides logo/text when sidebar is expanded (since the sidebar has its own logo branding at the top), but shows title on the left when sidebar is collapsed.
- **Actions Row**: Contains the Global Search trigger (`Cmd+Shift+F` / `Cmd+K`) and layout controls (Canvas Panel toggle).

#### 3.3.2 Sidebar Container
- **Background**: `var(--sidebar-bg-gradient)`
- **Radius**: `12px` rounded container wrapping the sidebar structure.
- **Padding Top**: Dynamic padding of 38px when expanded to clear the macOS window traffic lights.
- **Resize Handle**: Horizontal drag handle on the right edge. Width range: 200px - 400px. Double-click resets to default 240px.

#### 3.3.3 Main Workspace, Split View and Bottom Terminal
- **Gap**: An 8px column gap separates the sidebar container from the main workspace area.
- **Workspace Layout**: A vertical stack separating the workspace split view (`.workspace-split`) and the collapsible bottom **Terminal Panel** (`.terminal-bottom-panel`).
- **Workspace Split**: Split horizontally between the **Chat Panel** (`flex: 1`) and the **Canvas Panel** (resizable, collapsible).
- **Canvas Visibility**:
  - **New Chat/Empty State**: Canvas panel is fully hidden. Chat Panel takes up 100% of the workspace split area.
  - **Active Session**: Canvas panel appears dynamically when artifacts, code edits, or browser previews are generated. It can also be toggled manually via `Cmd+.` or the header button.
- **Canvas Resizing**: Resizable via a vertical drag handle on the left edge of the Canvas Panel.
- **Terminal Bottom Panel**:
  - Standard sandboxed PTY console emulator that spans the entire bottom workspace area.
  - Toggled via `Ctrl+` (backtick) shortcut or the status bar console indicator.
  - Features a vertical resize handle on its top edge. Width: 100% of workspace, height bounds: 80px to 400px.
  - Persists open/collapsed state and height in `AppSettings`.

---

## 4. Screen Inventory and Component Hierarchy

The application navigation is built around a **5-Tab Sidebar System**, which serves as the top-level routing container for all views.

### 4.1 Tab Structure and SVG Definitions

The top-level tab buttons are housed in a frosted pill container (`.sidebar-tabs`) at the top of the sidebar. Each button features an SVG icon:

- **Chat (`#sidebar-tab-chat`)**:
  `viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8"`
  `path: d="M21 11.5a8.38 8.38 0 0 1-.9 3.8 8.5 8.5 0 0 1-7.6 4.7 8.38 8.38 0 0 1-3.8-.9L3 21l1.9-5.7a8.38 8.38 0 0 1-.9-3.8 8.5 8.5 0 0 1 4.7-7.6 8.38 8.38 0 0 1 3.8-.9h.5a8.48 8.48 0 0 1 8 8v.5z"`
  `lines: x1="8" y1="10" x2="8.01" y2="10"`, `x1="12" y1="10" x2="12.01" y2="10"`, `x1="16" y1="10" x2="16.01" y2="10"` (stroke-linecap="round")
- **Agent (`#sidebar-tab-agent`)**:
  `viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"`
  `circle: cx="12" cy="12" r="2" fill="currentColor"`
  `ellipses: cx="12" cy="12" rx="10" ry="3.5"`, `cx="12" cy="12" rx="10" ry="3.5" transform="rotate(60 12 12)"`, `cx="12" cy="12" rx="10" ry="3.5" transform="rotate(-60 12 12)"`
- **Notes (`#sidebar-tab-notes`)**:
  `viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8"`
  `path: d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"`
  `polyline: points="14 2 14 8 20 8"`
  `lines: x1="8" y1="12" x2="16" y2="12"`, `x1="8" y1="16" x2="14" y2="16"`, `x1="8" y1="8" x2="10" y2="8"`
- **Work Suite (`#sidebar-tab-work`)**:
  `viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8"`
  `rect: x="2" y="3" width="20" height="14" rx="2" ry="2"`
  `lines: x1="8" y1="21" x2="16" y2="21"`, `x1="12" y1="17" x2="12" y2="21"`, `x1="7" y1="8" x2="11" y2="8"`, `x1="7" y1="11" x2="14" y2="11"`
  `circle: cx="17" cy="7" r="1.5" fill="none"`
- **Playground (`#sidebar-tab-playground`)**:
  `viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8"`
  `path: d="M6 12h4m-2-2v4"`, `d="M2 8a4 4 0 0 1 4-4h12a4 4 0 0 1 4 4v5a7 7 0 0 1-7 7H9a7 7 0 0 1-7-7V8z"`
  `circles: cx="17" cy="11" r="1" fill="currentColor"`, `cx="19" cy="13" r="1" fill="currentColor"`

### 4.1.1 Collapsed Tab Cycler Behavior

When the sidebar is collapsed to its `56px` icon rail, the 5-tab pill collapses into a **single-tab cycler**. This is a deliberate interaction pattern that preserves all five navigation contexts within the constrained collapsed width.

**Visual State (Collapsed):**
- Only **one tab icon** is visible at a time — the currently active tab's icon.
- All inactive `.tab-item` buttons are hidden via `display: none` (`.sidebar.collapsed .tab-item { display: none; }`), with the active tab re-shown (`.sidebar.collapsed .tab-item.active { display: flex; }`).
- The sub-navigation list (`.sidebar-nav`) displays **only icons** for the active tab's nav items — all `.nav-label`, `.section-label`, `.badge`, `.thread-item`, `.thread-item-wrap`, `.thread-item-meta`, and `.new-chat-label` text elements are hidden.

**Interaction (Click to Cycle):**
- Clicking the single visible tab icon advances to the **next tab** in fixed sequence and wraps around:
  ```
  Chat → Agent → Notes → Work Suite → Playground → Chat → ...
  ```
- The `handleSidebarTabClick(tabName)` function detects the collapsed state and computes the next tab via modulo arithmetic: `(currentIndex + 1) % tabOrder.length`.
- `switchSidebarTab()` then rebuilds the sub-navigation list with the next tab's nav items, and CSS automatically hides their labels/badges in collapsed state.

**Rationale:** Rather than shrinking 5 icons into an unreadable rail, the cycler keeps one large, legible icon as the focal point and lets the user step through the sequence with repeated clicks. The single icon + its nav icons below form a complete, scannable navigation context for the active tab.

### 4.2 Tab Navigation Map

Selecting a top-level tab populates the sidebar's sub-navigation area (`.sidebar-nav`) with contextual routes that map directly to physical layout views:

#### 4.2.1 Chat Tab
- **New Chat**: Starts a new session (`view-chat` with empty state).
- **Library**: Opens the Workspace Explorer & Indexing status (`view-knowledge`).
- **Projects**: Opens the Knowledge Graph / Projects view (`view-graph`).
- **Files**: Opens the generated Artifacts catalog (`view-files`).
- **All Chats**: Lists all chronological user threads (`view-chat`).
- **Pinned Section**: A custom `.nav-section` containing pinned thread cards.
- **Recent Section**: A custom `.nav-section` containing non-pinned recent threads.

#### 4.2.2 Agent Tab
- **Skills**: Skill catalog and detail manager (`view-skills`).
- **Apps**: Enabled application integrations catalog (`view-apps`).
- **Marketplace**: Browse downloadable Skills and MCP servers (`view-marketplace`).
- **Connectors**: MCP server setup and tool configuration (`view-connectors`).
- **AI Hub**: Central local model downloads and status indicators (`view-aihub`).
- **Memories**: Explicit memory fact database (`view-memories`).

#### 4.2.3 Notes Tab
- **Recent Notes**: Recent workspace notes items.
- **All Notes**: Complete note directories explorer.
- **Trash**: Deleted notes container.

#### 4.2.4 Work Suite Tab
- **Email**: Integrates local email streams (badge showing unread count).
- **Calendar**: Local event listings and scheduled tasks.
- **Tasks**: Productivity task list (badge showing pending items count).

#### 4.2.5 Playground Tab
- **Tamagotchi**: Interactive companion widget and fun easter egg.

---

## 5. Component Specifications: Sidebar

### 5.1 Sidebar Layout and Sub-navigation
The `Sidebar.svelte` component manages the overall layout, top-level tabs, search bar, and bottom footer inside a resizable `12px` rounded container.

### 5.2 Thread Items
Each thread card is rendered inside a `.thread-item-wrap` container containing:
- **Button (`.thread-item`)**: Emits clicking event to trigger thread loading. Displays title (`.thread-item-title`) and truncated description/meta text (`.thread-item-meta`). Active threads receive the `.active` class.
- **Context Trigger Button (`.thread-menu-btn`)**: A 3-dot vertical menu button that opens a floating `.thread-context-menu` overlay.
- **Context Menu (`.thread-context-menu`)**: Contains the following actions:
  - **Select**: Focuses the thread (`loadThread(id)`).
  - **Pin/Unpin**: Toggles pinned status of the thread.
  - **Add to Project**: Moves the thread under a project directory.
  - **Delete**: Triggers thread deletion with confirmation.

### 5.3 Sidebar Search
- **Input (`.sidebar-search-input`)**: Inline search input with placeholder text "Search threads...". Displays a `⌘K` keyboard shortcut badge.
- **Collapsed Behavior**: When the sidebar is collapsed, the search input and shortcut badge hide, and a `.sidebar-search-icon` button is displayed in the center. Clicking this button collapses/expands the sidebar container.

### 5.4 Sidebar Bottom Bar (Footer)
The footer container (`.sidebar-bottom-bar`) hosts utility controls:
- **Theme Toggle (`.sidebar-theme-toggle`)**: An iOS-style toggle track containing moon and sun icons. Sliding the track toggle triggers `toggleTheme()` to update `[data-theme="light/dark"]`.
- **User Profile (`.sidebar-avatar`)**: Circular avatar showing user initials (e.g. `MA`). Clicking this redirects to User Account/Subscription settings.
- **Sidebar Chevron Button (`.sidebar-icon-btn`)**: A trailing chevron button that collapses or expands the sidebar. Collapsing changes the sidebar width to `56px`, hides all label text, and activates the tab cycler (see §4.1.1).


---

---

## 6. Component Specifications: Chat Workspace

The Chat Workspace is represented by `view-chat` (`ChatView.svelte`). It consists of a header (in the title bar), a messages scroll area, a robot mascot widget, suggestion chips, and a highly featured chat composer.

### 6.1 Chat Pane and Message List

**Svelte Signature:** `<MessageList messages={$conversationStore.activeMessages} streaming={$conversationStore.isStreaming} />`

#### 6.1.1 Empty State (Welcome Screen)
When no active thread is selected, the message list renders a `.welcome-container` card containing:
- **Logo**: Branded stacked layers SVG logo.
- **Heading**: "Ask Dexter" (24px, weight 600).
- **Description**: "Your local-first, offline-capable AI desktop assistant. Start by typing a message below or importing a workspace folder."
- **CTA Actions**: "Start New Chat" (`btn primary`) and "Import Workspace" (`btn`).
- **Empty State Heading**: Above the composer, the heading "What can I do for you?" is displayed.

#### 6.1.2 Message Bubbles
- **User Messages**: Left-padded, right-aligned, using `--chat-user-bg` (brand blue) for background and `--chat-user-text` (white) for text. Max width: 70%.
- **Assistant Messages**: Left-aligned, full width, using `--chat-assistant-bg` for background and `--chat-assistant-text` for text. Includes a left-aligned circular avatar placeholder displaying the branded stacked layers SVG icon.
- **Message Time**: Rendered below the message bubble in `11px` `--text-secondary` color.
- **Message Hover Actions**: Appears on hover over a message bubble.
  - User bubble actions: Copy, Edit.
  - Assistant bubble actions: Copy, Branch, Delete.

#### 6.1.3 Specialized Message Cards
- **Code Block Cards**: Renders code snippets inside a container with a filename header bar (featuring a file icon, language badge, and "Copy" button) and syntax-highlighted code block.
- **Tool Use Cards**: Collapsible accordion panels (`.tool-use-card`) displaying:
  - Header: Tool icon + Tool name + "Done" green badge + collapse chevron.
  - Body: Code diffs, CLI outputs, or API request details formatted in `--font-mono`.

---

### 6.2 Chat Composer and Mascot

The composer area (`.composer`) is located at the bottom-center of the chat area, max-width 600px.

#### 6.2.1 Mascot Widget
An interactive inline SVG robot mascot (`.composer-mascot`) sits above the composer. It performs micro-animations (subtle hover tracking, floating shadow animation, sweep glare on eyes, and periodic eye-blink animation). It hides dynamically when messages begin streaming.

#### 6.2.2 Suggestion Chips
Housed in a `.suggestion-chips` container above the composer. Displays a 2x2 grid of quick-action pills. Clicking a chip populates the input textarea:
- "Create a Svelte 5 component"
- "Audit workspace accessibility"
- "Explain how the model sidecar works"
- "Show my knowledge graph"

#### 6.2.3 Composer Container and Input
- **Area**: The input container `.composer-container` has a subtle outline glow (`.composer-glow`).
- **Textarea (`.composer-input`)**: Auto-growing text area with placeholder "Ask Dexter a question...". pressing `Enter` without `Shift` triggers message submission.

#### 6.2.4 Composer Toolbar
Located at the bottom of the composer container, hosting action buttons:
- **Toolbar Left**:
  - **Attach Button (`.composer-btn`)**: Opens a floating dropdown menu (`#attachDropdown`) with options:
    - *Attach Image*: Upload screenshot or mockups.
    - *Attach Document*: Upload PDFs or code files.
    - *Import Folder*: Add workspace directory.
  - **Tools Button**: Triggers the sandboxed tools panel.
  - **Connectors Toggle Button (`.composer-btn-connectors`)**: Toggles a floating `.connectors-dropdown` showing app toggles:
    - Gmail (with unread count badge).
    - Google Drive.
    - Google Calendar.
    - Canva (with "Connect" CTA).
    - Notion (with "Connect" CTA).
    - Status Indicator: Active connector icons are rendered inline on the toggle button itself when enabled.
- **Toolbar Right**:
  - **Model Selector Pill**: Displays the active model name (e.g. `GPT-4o` or `Llama 3.1 8B`). Opens a tabbed dropdown (`#modelDropdown`):
    - *Cloud Tab*: Cloud API models (GPT-4o, Claude 3.5 Sonnet, Gemini 1.5 Pro).
    - *Local Tab*: Sandboxed models running via sidecar (Llama 3.1 8B, Mistral 7B).
  - **Send/Voice Action Button (`.composer-send-btn`)**: A dual-state button:
    - Default state: Shows a microphone SVG icon (`.btn-icon-mic`) for triggering voice inputs.
    - Text present state: Switches to an up-arrow SVG icon (`.btn-icon-send`) for message submission.

#### 6.2.5 Composer Footer (Metadata Row)
Located directly below the composer toolbar, displaying:
- Left: Keyboard shortcut hints ("Dexter uses local and cloud models. Cmd+K to search").
- Right: Live session token counter ("Tokens: 1,240 used").

---

## 7. Component Specifications: Canvas Panel

The Canvas Panel (`CanvasPanel.svelte`) is the resizable side panel that renders interactive preview screens, collaborative document editors, and browser viewports side-by-side with the chat area.

### 7.1 Layout and Router

**Svelte Signature:** `<CanvasPanel activeTab={$conversationStore.canvasActiveTab} open={$conversationStore.canvasOpen} />`

- **Resize Handle**: A vertical resize drag handle (`.canvas-resize-handle`) is positioned on the left edge. Left-drag adjusts panel width from a minimum of 300px to a maximum of `containerWidth - 300px`. Double-clicking the handle resets the width to a default 400px.
- **Toggle State**: Toggled via the canvas button in the title bar or the `Cmd+.` keyboard shortcut. Toggling animate-slides the panel from the right edge with a 200ms ease-out transition.

---

### 7.2 The Canvas Tabs and Views

The panel header contains a frosted `.canvas-tabs` row with the following views:

#### 7.2.1 Artifacts List Tab (`canvas-tab-artifacts`)
- **Icon**: Stacked layers SVG icon.
- **Purpose**: Displays a categorized list of all artifacts, files, code components, or HTML assets generated by the AI in the current session.
- **Features**: 
  - Clicking an item in the list spawns a new **Preview Tab** for that specific artifact.
  - Displays file status indicators (e.g. "active", "edited", "saved").

#### 7.2.2 Active Preview Tabs (Dynamic)
- **Icon**: Dynamic icon based on the file type of the open artifact (e.g. Code, HTML, Image, Markdown).
- **Purpose**: Displays the live preview of a specific artifact.
- **Features**:
  - Multiple preview tabs can be opened at the same time, allowing side-by-side comparison or quick tab switching in the header.
  - Close button (`x`) on each preview tab to dismiss it.
  - Renders HTML previews in isolated iframe structures, code in read-only syntax blocks, and SVGs/images directly.

#### 7.2.3 Editor Tab (`canvas-tab-editor`)
- **Icon**: Square pen-and-paper SVG icon.
- **Purpose**: A collaborative document workspace. This is where rich text documents, specifications, or code components generated by the AI are populated.
- **Features**:
  - Dual edit mode: users can write text/code directly or let the AI edit it in place.
  - Collaboration: Users can upload documents directly to the editor workspace (via drag-and-drop or file upload picker) to collaborate with the AI.
  - Inline AI Prompter: Selecting text inside the editor displays a mini overlay prompt input to ask the AI to refactor, summarize, or explain the selected range.
  - File saving: `Cmd+S` writes the active document contents back to the workspace.

#### 7.2.4 Browser Tab (`canvas-tab-browser`)
- **Icon**: Globe/external link SVG icon.
- **Purpose**: WebView viewport for testing live running web previews or displaying agent-orchestrated web searches.

---

## 7.a. Component Specifications: Surface Views

This section documents the layout, properties, and specific component items for the 11 secondary page views that are mapped from the sidebar sub-navigation.

### 7.a.1 Knowledge Base View (`view-knowledge`)
- **Header**: Title "Knowledge Base" and description "Configure workspace directories and local indexing engines."
- **Workspace Explorer card**:
  - Displays a tree file directory layout (`.kb-tree`) showing files in the current workspace.
  - Files and directories display Lucide-style icons (folders and files).
  - Toggles selection of files/directories for indexing.
- **Vector Database Details card**:
  - Displays a details table showing index name, size, type (e.g. HNSW), vector dimension size (e.g. 768), and chunk count.
  - Actions: A "Recalculate Indices" button (`.btn`) that fires `kb_reindex` to force rebuild local vector indexing database.

### 7.a.2 Knowledge Graph View (`view-graph`)
- **Header**: Title "Knowledge Graph" and description "Interactive visualization of entity relationships extracted from indexed documents."
- **SVG Graph Container**:
  - Renders a interactive SVG/D3.js force-directed canvas.
  - Nodes (colored circles representing files, concepts, or entities) are connected by lines (edges representing relationships).
  - Hovering a node displays an overlay card with its metadata label.
  - Clicking a node fires `graph_query_neighbors` and centers the canvas on that node.
  - Bottom Status Hint: "Hover nodes to inspect or click to see descriptions."

### 7.a.3 Skills View (`view-skills`)
- **Layout**: Master-detail dual column split.
- **Skills List Panel (Left)**:
  - Top search input bar.
  - List of loaded skills. Each item displays a skill icon, script title, sandbox safety badge (e.g., `iframe` or `wasmtime`), and trigger type.
- **Detail Panel (Right)**:
  - Header: Skill name, enable/disable toggle switch, triggers meta tags, and permission flags.
  - Description body describing the skill's capabilities.
  - Rich content preview block or code sample showing script execution template.

### 7.a.4 Connectors View (`view-connectors`)
- **Layout**: Master-detail dual column split.
- **Connectors List Panel (Left)**:
  - List of active MCP connectors (StitchMCP, Chrome DevTools, Docker, GitHub, etc.) with online green/offline red status indicators.
- **Detail Panel (Right)**:
  - Header: Connector name, status indicator, active transport type (stdio, SSE), and toggle switch.
  - Available Tools list showing the specific MCP tools exposed by the connector (with tool description and permissions).
- **Add MCP Dialog Modal (`#mcp-add-dialog`)**:
  - Fields: Name, Transport Type (stdio / SSE / WebSocket), Command/URL, Arguments (comma-separated list), Environment Variables (key-value text block).
  - Actions: "Test Connection" button (queries `mcp_test_connection` and shows connection results) and "Save Connector" button.

### 7.a.5 AI Hub View (`view-aihub`)
- **Header**: Title "AI Hub" and description "Manage local LLM sidecars, downloads, and GPU hardware acceleration parameters."
- **Hardware Profile Grid**: Displays CPU cores, total RAM, and available VRAM.
- **Model Cards**:
  - Lists models (Llama 3.1 8B, Phi-3.5 Mini).
  - Card elements: Name, size (GB), context window size, status badge (Loaded / Download / Inactive), and action button.
  - Download Progress: When downloading a model, the action button is replaced by a simulated progress bar showing speed (MB/s) and percentage fill.

### 7.a.6 Memories View (`view-memories`)
- **Header**: Title "Memories" and description "Browse, pin, or edit facts and rules extracted by the agent."
- **Toolbar**: Search input bar + "Add Memory" primary button.
- **Memories Table (`#mem-tbody`)**:
  - Rows display: Memory content, Source (AI auto-learned showing robot icon vs User manual-added showing user avatar icon), Category (preference/fact/rule/learned badges).
  - Pin toggle button: Pins memory to always be included in LLM context.
  - Action buttons: Edit (opens modal) and Delete.
- **Add/Edit Memory Dialog Modal**:
  - Form Fields: Category select dropdown (Preference, Fact, Rule, Learned) and Memory Content textarea.
  - Actions: "Cancel" and "Save" buttons.

### 7.a.7 Apps View (`view-apps`)
- **Header**: Title "Apps" and description "Connect external productivity platforms and SaaS APIs."
- **App Cards Grid**:
  - Cards for Google Workspace, Notion, Canva.
  - Displays app brand icon, connection status badge (Enabled / Inactive), app description, and "Configure" settings button.
- **Discover Apps Card**: Redirects to the Marketplace to install new connectors.

### 7.a.8 Marketplace View (`view-marketplace`)
- **Header**: Title "Marketplace" and description "Extend Ask Dexter with community-built Skills, Apps, and MCP servers."
- **Tab Switcher**: Toggle pills to switch catalog view between "Skills" and "MCP Servers".
- **Marketplace Cards Grid**:
  - Displays creator avatar, name, description, tags, and install action button ("Install" / "Connected").

### 7.a.9 Settings View (`view-settings`)
- **Header**: Title "Settings" and description "Configure workspace directories, UI behavior, and theme modes."
- **Basic Form Card**:
  - Theme Mode select dropdown (Dark / Light / System).
  - Workspace Directory input text box with native folder picker button.
  - "Save Settings" primary button.

### 7.a.10 Subscription View (`view-subscription`)
- **Header**: Title "Subscription" and description "Manage your Ask Dexter verification token and entitlements."
- **Entitlement Card**:
  - Status badge: "Pro Edition - Offline Entitled" (green badge).
  - Description: "Your license token was verified locally using the embedded keychain public key."
  - Form Fields: JWT License Key password field and "Verify Token" action button.

### 7.a.11 Files View (`view-files`)
- **Header**: Title "Files" and description "Browse all documents and artifacts generated across sessions."
- **Filter Toolbar**: Row of filter pills (All, Documents, Images, Videos, HTML, Code).
- **Files Grid (`#filesGrid`)**:
  - Rendered card list showing file preview icon (based on file type), name, category badge (e.g. Component, Canvas, Demo), size, generated date, and actions (Open in Editor, Open in Finder, Delete).

---

## 7.b. Expanded Settings Design

To support the requested BYOK, Appearance, and System configurations that are not fully detailed in the mockup HTML, the settings panel layout is expanded into three tabbed view sub-menus.

### 7.b.1 Settings > BYOK (Bring Your Own Key)
- **API Provider Configuration**:
  - List of model providers: OpenAI, Anthropic, Google, and Custom Endpoint.
  - Input field rows per provider:
    - **API Key**: Password input field. API keys are stored locally using native macOS Keychain integrations (via Tauri client-keychain API).
    - **API Endpoint URL**: Optional endpoint text field to support proxies or local model gateways (e.g. Ollama).
    - **Model Selection**: Multi-select dropdown to specify which provider models are enabled in the composer selector.
  - Actions: "Test Key Connection" button that fires a mock API check request to verify key validation status.

### 7.b.2 Settings > Appearance
- **Theme Settings**:
  - Theme mode selector: Dark, Light, or System.
  - Color Palette selector: Navy Blue (authoritative brand default) and Neutral Dark.
  - Sidebar Position: Left-aligned (default) or Right-aligned.
- **Font Customization**:
  - Font Size: A slider control from 11px to 16px. Adjusting the slider updates the CSS variable `--text-base` in real-time.
  - Monospace Font: Dropdown selection list of monospace editor fonts (JetBrains Mono, Fira Code, SF Mono, Cascadia Code).

### 7.b.3 Settings > System
- **Workspace Sync**:
  - Multi-workspace list: Allows registering multiple folder directories as project workspaces with add/remove actions.
  - Auto-save toggle switch: Automatically saves modifications in the Canvas Code editor back to the filesystem.
  - Backup Interval: Dropdown list selecting period (Hourly, Daily, Weekly, None).
- **Cache Management**:
  - Vector Cache: Display showing size of vector embeddings cache with a "Clear Index Cache" destructive button.
  - Model Cache: Display showing disk size of GGUF model files with a "Clear Models Cache" button.
- **Update Channel**: Toggle switch between "Stable" and "Beta" release branches.

---

## 7.c. Icon Inventory

All icons used across the application are rendered as inline SVG elements. They follow a clean, consistent Lucide design (24x24 viewport, stroke width 1.8-2.0, round line join).

### 7.c.1 Sidebar Navigation Icons
- **Chat**: `MessageSquare` bubble with three dots.
- **Agent**: Orbiting atomic electron shells (`ellipse` rotate tags).
- **Notes**: Dog-eared file text document with lines.
- **Work**: Desktop workstation monitor shell.
- **Playground**: Retro gamepad/joystick handle.
- **Plus**: Simple cross lines (`line x1="12" y1="5" x2="12" y2="19"`, etc.).
- **Library**: Standalone book with spine line.
- **Folder**: Branded open project folder tab.
- **Search**: Magnifying glass (`circle` + diagonal line).
- **Theme Moon**: Crescent crescent curve path.
- **Chevron**: Left-pointing back arrow.

### 7.c.2 Chat Composer and Message Icons
- **Paperclip**: Standard link loop element.
- **Mascot robot**: Multi-node custom robot face.
- **Gmail**: Closed letter envelope.
- **Google Drive**: Interlocking triangular lines.
- **Google Calendar**: Grid calendar layout.
- **Canva**: Double overlapping circular path.
- **Notion**: Block outline letter 'N'.
- **Microphone**: Voice input container.
- **ArrowUp**: Circle-outlined message sending button.

### 7.c.3 Canvas Tabs and View Controls
- **Artifacts**: Stacked layers/cards outline.
- **Editor**: Square pen-and-paper SVG icon.
- **Browser**: External window square box with arrow pointer.

---

## 7.d. Collapsible Bottom Terminal Panel

The Terminal Panel (`BottomTerminal.svelte`) is a collapsible and resizable console emulator situated at the bottom of the main workspace area. It executes sandboxed CLI commands and displays raw terminal output in real-time.

### 7.d.1 Component Structure and Layout
- **Container (`.terminal-bottom-panel`)**: Positioned at the bottom of the `.workspace` flex container.
- **Resizer Bar**: A horizontal resize drag handle on the top edge of the panel container. Allows dragging to adjust the height (80px to 400px).
- **Header Bar**:
  - Left: Terminal status indicator ("Terminal - offline sandboxed") and prompt command status.
  - Right: Actions row containing:
    - *Clear Console* button (sends clear command to buffer).
    - *Collapse* button (chevron down icon to hide panel).

### 7.d.2 Terminal Emulation
- **Xterm.js Canvas**: Renders a true PTY terminal stream using `xterm.js` inside the webview.
- **Theme Sync**: Inherits the default dark/light colors (`--bg-surface` for console background, `--text-primary` for text output, and semantic color badges for execution outcomes).
- **Shortcuts**: `Ctrl+`` (backtick) toggles expansion/collapse of the panel drawer. Focus defaults immediately to the active terminal input prompt upon opening.

---

## 8. User Interaction Flows

Step-by-step interaction sequences for 10 key workflows. Each step specifies: user action → component event → IPC call → state update → UI reaction.

### 8.1 Send Message and Receive Streaming Response

1. User types message in `TextareaInput`
2. User presses `Cmd+Enter` → `InputArea` emits `send` event with `{ content, attachments }`
3. `ChatView` calls `invoke('chat_send_message', { conversationId, content, modelId })`
4. `conversationStore` sets `isStreaming = true`, adds user message to list, adds placeholder assistant message
5. `MessageList` auto-scrolls to bottom, shows placeholder with blinking cursor
6. Rust processes message, Pi SDK begins streaming
7. Each `agent-token` event → `conversationStore` appends chunk to assistant message
8. `MessageList` re-renders only the last message bubble (incremental update)
9. `agent-complete` event → `conversationStore` sets `isStreaming = false`, finalizes message
10. `InputArea` re-enables send button, clears input

### 8.2 Create New Conversation

1. User presses `Cmd+N` → global shortcut handler
2. `conversationStore` calls `invoke('chat_list_conversations', { limit: 0 })` (creates new)
3. New conversation added to thread list at top
4. `conversationStore.activeId` updates → `MessageList` clears
5. Sidebar highlights new thread, `InputArea` focuses

### 8.3 Import File to Knowledge Base

1. User navigates to KB Detail screen
2. User drags file onto drop zone OR clicks Import button → native file picker
3. `KnowledgeBaseDetail` calls `invoke('kb_import_file', { kbId, filePath })`
4. Progress bar appears, listens for `embedding-progress` events
5. Each progress event updates percentage display
6. `embedding-complete` event → file row appears in FileList with chunk count
7. Success toast notification

### 8.4 Run RAG Query

1. User types query in `RAGSearchPanel.SearchInput`
2. User presses `Enter` → calls `invoke('kb_search_rag', { query, kbId, topK: 5 })`
3. Loading skeleton appears in results area
4. Results return → `SearchResultList` renders cards with chunk preview, score, source file link
5. Click on result card → expands full chunk text, links to source file in Canvas Panel

### 8.5 Visualize Knowledge Graph

1. User clicks "Projects" in the sidebar sub-navigation to open the Knowledge Graph view (`view-graph`)
2. `GraphVisualization` calls `invoke('graph_query_neighbors', { nodeId: rootId, hops: 2 })`
3. Loading skeleton → graph data returns
4. Force-directed layout animates into position
5. User clicks node → `NodeDetailPanel` slides in from right showing properties and neighbor list
6. Click on neighbor → graph recentizes on that node, fetches its neighbors

### 8.6 Import and Run a Skill

1. User navigates to Skills Manager screen
2. Clicks "Import Skill" → native file picker (.js or .wasm)
3. Calls `invoke('skill_import', { filePath, name, permissions })`
4. Skill row appears in SkillList
5. User clicks "Execute" → `SkillExecutionPanel` modal opens
6. User enters JSON input → clicks Execute
7. Calls `invoke('skill_execute', { skillId, input })`
8. Output panel shows result (JSON formatted) or error message

### 8.7 Configure MCP Server

1. User navigates to MCP Settings screen
2. Clicks "Add Server" → `ServerConfigModal` opens
3. User enters name, selects transport (stdio), enters command: `npx -y @modelcontextprotocol/server-filesystem /path`
4. Clicks "Test Connection" → calls `invoke('mcp_test_connection', { serverId })`
5. Status shows "Connected" with green checkmark
6. User saves → server row appears in list with connected status
7. Server tools now available in @-mention autocomplete and agent tool routing

### 8.8 Download and Load Local Model

1. User navigates to Model Manager screen
2. Hardware info shows VRAM availability
3. User clicks "Download" on a model card → calls `invoke('model_download', { modelUrl, modelId })`
4. Download progress bar updates via `model-download-progress` events
5. Download complete → "Load" button replaces download button
6. User clicks "Load" → calls `invoke('model_load', { modelId })`
7. VRAM pre-flight check passes → llama-server spawns
8. `sidecar-status` event with `ready` → model card shows "Active" badge
9. Status bar updates to show active model name

### 8.9 Export Backup

1. User navigates to Backup/Import screen
2. Clicks "Export Backup" → native folder picker for target directory
3. Calls `invoke('backup_export', { targetDir })`
4. Progress bar shows via `backup-status` events
5. Complete → "Export saved to /path/to/backup.tar.gz" with "Open in Finder" button

### 8.10 Import v1 Data

1. User navigates to Backup/Import screen → V1 Migration section
2. Clicks "Import from v1" → native file picker for JSON export
3. Calls `invoke('migration_import_v1', { filePath })`
4. Progress bar updates via `migration-progress` events showing phase and percentage
5. Complete → summary shows "Imported X conversations, Y knowledge chunks"
6. Conversations appear in thread list, knowledge bases appear in KB list

### 8.11 Browser Automation (Agent-Triggered Search)

1. User sends a message requiring web search (or agent decides to use browser tool)
2. Pi SDK emits tool call for `browser_search` → Rust calls `invoke('browser_search', { query })`
3. Status bar shows "Searching the web..." indicator
4. Rust connects to Chrome via CDP, navigates, extracts results
5. Tool result returned to Pi SDK → agent incorporates search results into response
6. ToolUseCard in MessageList shows "Web Search" with collapsed results preview
7. If Chrome unavailable, falls back to Obscura — status bar shows "Using built-in browser"


---

## 9. State Management Architecture

All state managed via Svelte 5 `$state` runes in module-level store files. No external state libraries.

### 9.1 Global Stores

#### `authStore` (`src/stores/auth.svelte.ts`)

```typescript
interface AuthState {
  tier: 'free' | 'paid' | 'one-time';
  expiresAt: string | null;
  graceRemaining: number | null;
  locked: boolean;
  user: { id: string; email: string } | null;
}

let state = $state<AuthState>({ tier: 'free', expiresAt: null, graceRemaining: null, locked: false, user: null });

// Initialization: called on app mount
export async function initAuth() {
  const status = await invoke('entitlement_validate');
  state = { ...status };
}

// Event listener: updates on entitlement-changed Tauri events
export function subscribeToEntitlementChanges() {
  listen('entitlement-changed', (event) => { state = { ...state, ...event.payload }; });
}
```

#### `conversationStore` (`src/stores/conversation.svelte.ts`)

```typescript
interface ConversationState {
  threads: Array<{ id: string; title: string; updatedAt: string; modelId: string }>;
  activeId: string | null;
  activeMessages: Message[];
  isStreaming: boolean;
  streamingContent: string;
  canvasOpen: boolean;
  canvasActiveTab: string; // 'artifacts' | 'editor' | 'browser' | previewTabId
  canvasOpenPreviews: string[]; // List of open preview artifact IDs
  terminalOpen: boolean;
}

let state = $state<ConversationState>({
  threads: [], activeId: null, activeMessages: [],
  isStreaming: false, streamingContent: '',
  canvasOpen: false, canvasActiveTab: 'artifacts', canvasOpenPreviews: [],
  terminalOpen: false
});

// Filtered threads (derived)
export const filteredThreads = $derived(
  searchQuery ? state.threads.filter(t => t.title.toLowerCase().includes(searchQuery.toLowerCase())) : state.threads
);

// Token budget remaining (derived)
export const tokenBudget = $derived(
  state.activeMessages.reduce((sum, m) => sum + (m.tokenCount || 0), 0)
);
```

#### `modelStore` (`src/stores/model.svelte.ts`)

```typescript
interface ModelState {
  available: ModelInfo[];
  active: { id: string; name: string; provider: string } | null;
  downloads: Map<string, { percent: number; speed: string }>;
  hardware: { totalRamGb: number; vramGb: number } | null;
}
```

#### `settingsStore` (`src/stores/settings.svelte.ts`)

```typescript
interface SettingsState {
  theme: 'dark' | 'light' | 'system';
  fontSize: number;
  sidebarWidth: number;
  sidebarCollapsed: boolean;
  workspaceDir: string;
  workspaces: string[];
  activeWorkspace: string;
  executionTarget: 'local' | 'cloud';
  gitBranch: string;
  autoSave: boolean;
  backupInterval: number;
}
```

#### `sidecarStore` (`src/stores/sidecar.svelte.ts`)

```typescript
interface SidecarState {
  piSdk: { status: 'starting' | 'ready' | 'stopped' | 'crashed'; pid?: number };
  llamaServer: { status: 'starting' | 'ready' | 'stopped' | 'crashed'; pid?: number };
}
```

### 9.2 Component-Local State

State that lives inside individual components and is not shared globally:

| Component | Local State |
|-----------|------------|
| `TextareaInput` | `inputText`, cursor position, auto-grow height |
| `ThreadList` | `searchQuery`, `contextMenuTarget`, `draggedItem` |
| `MentionAutocomplete` | `mentionQuery`, `mentionResults`, `selectedIndex` |
| `GraphCanvas` | `zoom`, `pan`, `selectedNode`, `layout` |
| `CodeEditor` | `content`, `language`, `isDirty`, `cursorLine` |
| `ServerConfigModal` | `formData`, `testStatus`, `validationErrors` |

### 9.3 Derived State

| Derived Value | Source Store | Expression |
|--------------|-------------|------------|
| `filteredThreads` | conversationStore | Filter by search query |
| `tokenBudgetUsed` | conversationStore | Sum of active message token counts |
| `isFeatureGated` | authStore | Function: `(featureId) => boolean` based on tier |
| `canLoadLocalModel` | modelStore + authStore | Hardware check + tier check |
| `isSidecarReady` | sidecarStore | Both sidecars in `ready` state |

---

## 10. Keyboard Shortcuts and Accessibility

### 10.1 Keyboard Shortcut Map

**Global Shortcuts:**

| Key Combo | Action | Scope |
|-----------|--------|-------|
| `Cmd+N` | New conversation | Global |
| `Cmd+1` | Switch to Chat View | Global |
| `Cmd+2` | Switch to Knowledge Bases | Global |
| `Cmd+3` | Switch to Graph View | Global |
| `Cmd+4` | Switch to Skills | Global |
| `Cmd+5` | Switch to MCP Settings | Global |
| `Cmd+6` | Switch to Model Manager | Global |
| `Cmd+7` | Switch to Settings | Global |
| `Cmd+B` | Toggle sidebar collapse | Global |
| `Cmd+.` | Toggle Canvas Panel | Global |
| `Ctrl+`` | Toggle bottom Terminal panel | Global |
| `Cmd+K` | Open command palette | Global |
| `Cmd+Shift+F` | Focus sidebar search | Global |
| `Cmd+,` | Open Settings | Global |

**Chat Shortcuts:**

| Key Combo | Action | Scope |
|-----------|--------|-------|
| `Cmd+Enter` | Send message | Chat |
| `Shift+Enter` | New line in input | Chat |
| `Escape` | Stop generation (during streaming) | Chat |
| `Cmd+Shift+N` | Branch from selected message | Chat |
| `Cmd+D` | Delete current conversation | Chat |
| `Cmd+Shift+E` | Export current conversation | Chat |

**Editor Shortcuts:**

| Key Combo | Action | Scope |
|-----------|--------|-------|
| `Cmd+S` | Save code changes | Code Editor |
| `Cmd+Z` | Undo | All text inputs |
| `Cmd+Shift+Z` | Redo | All text inputs |
| `Cmd+A` | Select all | All text inputs |
| `Tab` | Accept autocomplete suggestion | Input area |

**Navigation:**

| Key Combo | Action | Scope |
|-----------|--------|-------|
| `↑` / `↓` | Navigate list items | Sidebar, dropdowns |
| `Enter` | Select / activate | Sidebar, dropdowns |
| `Escape` | Close modal / dropdown / panel | All overlays |
| `Tab` / `Shift+Tab` | Cycle focus | All components |

### 10.2 Accessibility

**ARIA Landmarks:**
```html
<main role="main" aria-label="Workspace">
<nav role="navigation" aria-label="Conversation sidebar">
<section role="region" aria-label="Chat messages">
<section role="region" aria-label="Canvas panel">
<footer role="contentinfo" aria-label="Status bar">
```

**Focus Management:**
- Focus ring: 2px solid `--border-focus`, offset 2px. Never `outline: none` without replacement.
- Tab order follows visual layout: Sidebar → Workspace → Canvas Panel → Status Bar
- Modal traps focus (tab cycles within modal, `Escape` closes and returns focus to trigger)
- Command palette (`Cmd+K`) uses typeahead filtering with arrow key navigation

**Screen Reader:**
- Streaming content: `aria-live="polite"` on the assistant message container during streaming
- Sidecar status changes announced via `aria-live="assertive"` region
- Error toasts use `role="alert"`

**Touch/Click Targets:**
- Minimum 44×44px per Apple HIG for all interactive elements
- Icon buttons have sufficient padding to meet minimum target size

**Reduced Motion:**
- `@media (prefers-reduced-motion: reduce)` disables all transitions and animations
- Streaming cursor becomes static block instead of blinking

---

## 11. 21st.dev Component Porting Reference

21st.dev React component patterns mapped to Svelte 5 equivalents.

| 21st.dev Component | Key Features to Preserve | Svelte 5 Implementation | Fidelity |
|-------------------|------------------------|------------------------|----------|
| **Command Palette** | Cmd+K overlay, fuzzy search, categorized results, keyboard navigation | Svelte modal overlay + `$state` filtered list + keyboard handler | High |
| **Dialog / Modal** | Overlay backdrop, focus trap, enter/exit animation, close on Escape | Svelte `<dialog>` element or custom modal with `onMount` focus trap | High |
| **Dropdown Menu** | Trigger button, floating panel, arrow key navigation, typeahead | Svelte floating panel with `position: absolute` + keyboard handler | High |
| **Toast / Sonner** | Stacked notifications, auto-dismiss, action buttons, slide-in animation | Svelte fixed-position container + `$state` toast array + CSS animation | High |
| **Tooltip** | Hover trigger, delay, arrow, positioning | Svelte `title` attr for simple, custom `<Tooltip>` component for rich content | Medium |
| **Tabs** | Active indicator animation, keyboard arrow navigation, disabled state | Svelte component with `$state` activeTab + transition on indicator | High |
| **Scroll Area** | Custom scrollbar, scroll shadow indicators | CSS `overflow-y: auto` with `::-webkit-scrollbar` styling | High |
| **Sheet / Drawer** | Slide from edge, overlay, close on outside click | Svelte fixed-position panel + CSS transform transition | High |
| **Separator** | Horizontal/vertical, subtle line | `<hr>` or `<div>` with `--border-subtle` | High |
| **Badge** | Pill shape, color variants, dot indicator | Svelte component with slot + CSS variants | High |
| **Avatar** | Circular image, fallback initials, size variants | Svelte `<img>` with `on:error` fallback to initials `<span>` | High |
| **Skeleton** | Pulse animation, shape variants (text, circle, rectangle) | Svelte `<div>` with CSS `@keyframes pulse` animation | High |

---

## 12. Animation Specifications

| Animation | Duration | Easing | Trigger | CSS |
|-----------|----------|--------|---------|-----|
| Panel slide (sidebar collapse) | 250ms | `--ease-spring` | `Cmd+B` toggle | `transform: translateX()` with transition |
| Panel slide (artifact panel) | 200ms | `--ease-out` | `Cmd+.` toggle | `width: 0 → 400px` with transition |
| Fade in (modals, dropdowns) | 150ms | `--ease-out` | Component mount | `opacity: 0 → 1` |
| Fade out | 100ms | `--ease-out` | Component unmount | `opacity: 1 → 0` |
| Message appear | 200ms | `--ease-out` | New message added | `opacity: 0, translateY(8px) → 1, 0` |
| Streaming cursor blink | 800ms | `step-end` | During streaming | `@keyframes blink { 50% { opacity: 0 } }` |
| Skeleton pulse | 1500ms | `ease-in-out` | Loading state | `@keyframes pulse { 50% { opacity: 0.5 } }` |
| Toast slide in | 200ms | `--ease-spring` | Toast emitted | `transform: translateX(100%) → 0` |
| Sidebar item hover | 100ms | `--ease-out` | Mouse enter | `background-color` transition |
| Graph node drag | Real-time | — | Drag interaction | Direct style update, no transition |
| Focus ring | 100ms | `--ease-out` | Focus event | `outline-offset` transition |

**Reduced Motion Override:**
```css
@media (prefers-reduced-motion: reduce) {
  *, *::before, *::after {
    animation-duration: 0.01ms !important;
    animation-iteration-count: 1 !important;
    transition-duration: 0.01ms !important;
  }
}
```

---

## 13. Error and Empty States

| State | Illustration/Icon | Message Pattern | CTA |
|-------|-------------------|-----------------|-----|
| No conversations | `MessageSquare` icon (lucide) | "No conversations yet" | "Press `Cmd+N` to start" |
| No models downloaded | `Download` icon | "No local models available" | "Browse model catalog" |
| Insufficient VRAM | `Cpu` icon with warning color | "Your device has {X}GB VRAM. Minimum 6GB required for local models." | "Use cloud models instead" |
| Sidecar crashed | `AlertTriangle` icon (warning color) | "Agent runtime encountered an error" | "Restart" button |
| Entitlement expired | `Lock` icon | "Your {tier} subscription expired on {date}" | "Renew subscription" |
| Entitlement locked (clock rollback) | `ShieldAlert` icon | "Clock inconsistency detected. Cloud features temporarily locked." | "Sync system clock to restore" |
| Network unavailable | `WifiOff` icon | "No internet connection. Local features remain available." | None (auto-retry) |
| MCP server unreachable | `ServerCrash` icon | "Cannot connect to {serverName}" | "Retry" / "Edit configuration" |
| Skill execution failed | `TerminalSquare` icon (error color) | "Skill '{name}' failed: {error}" | "View logs" / "Retry" |
| Import failed | `FileX` icon | "Failed to import: {reason}" | "Try again" |
| Graph empty | `Network` icon | "No graph data yet" | "Run entity extraction on a knowledge base" |
| Backup empty | `Archive` icon | "No backups found" | "Create your first backup" |

---

## 14. Theming Architecture

### 14.1 CSS Custom Property Structure

```css
:root {
  /* Theme tokens are set per-theme */
  /* Dark theme is the default */
}

[data-theme="dark"] {
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

  --sidebar-surface: #b06dff;
  --sidebar-bg-gradient: linear-gradient(180deg, #0b3772 0%, #0169cc 100%);
}

[data-theme="light"] {
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

  --sidebar-surface: #7c3aed;
}
```,StartLine:1144,TargetContent:
```

### 14.2 Theme Detection and Toggle

```typescript
// In settingsStore
export function setTheme(theme: 'dark' | 'light' | 'system') {
  state.theme = theme;
  applyTheme(theme);
  invoke('settings_set', { key: 'theme', value: theme });
}

function applyTheme(theme: string) {
  if (theme === 'system') {
    const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
    document.documentElement.dataset.theme = prefersDark ? 'dark' : 'light';
  } else {
    document.documentElement.dataset.theme = theme;
  }
}
```

**Persistence**: Theme choice saved to `AppSettings` table via `settings_set` IPC. Loaded on app startup before Svelte mounts.

---

## 15. Appendices

### Appendix A: Proposed Svelte Component File Tree

```
src/
├── app.html
├── lib/
│   ├── components/
│   │   ├── shell/
│   │   │   ├── AppShell.svelte
│   │   │   ├── Sidebar.svelte
│   │   │   ├── BottomTerminal.svelte
│   │   │   ├── StatusBar.svelte
│   │   │   └── TitleBar.svelte
│   │   ├── canvas/
│   │   │   ├── CanvasPanel.svelte
│   │   │   ├── EditorTab.svelte
│   │   │   └── PreviewTab.svelte
│   │   ├── ui/
│   │   │   └── EmptyState.svelte
│   ├── stores/
│   │   ├── auth.svelte.ts
│   │   └── conversation.svelte.ts
├── routes/
│   ├── +layout.svelte
│   ├── +page.svelte (Chat/Projects)
│   ├── library/+page.svelte
│   ├── all-chats/+page.svelte
│   ├── files/+page.svelte
│   ├── pinned/+page.svelte
│   ├── recent/+page.svelte
│   ├── skills/+page.svelte
│
│   ├── apps/+page.svelte
│   ├── marketplace/+page.svelte
│   ├── connectors/+page.svelte
│   ├── aihub/+page.svelte
│   ├── memories/+page.svelte (Knowledge Graph)
│   ├── notes/
│   │   ├── recent/+page.svelte
│   │   ├── all/+page.svelte
│   │   └── trash/+page.svelte
│   ├── work/
│   │   ├── email/+page.svelte
│   │   ├── calendar/+page.svelte
│   │   └── tasks/+page.svelte
│   └── playground/
│       └── tamagotchi/+page.svelte
```


### Appendix B: TypeScript Store Interfaces

```typescript
// === ipc/types.ts ===

interface Conversation {
  id: string;
  title: string;
  modelId: string;
  createdAt: string;
  updatedAt: string;
}

interface Message {
  id: string;
  conversationId: string;
  parentMessageId: string | null;
  role: 'user' | 'assistant' | 'system' | 'tool';
  content: string;
  tokenCount: number | null;
  modelId: string | null;
  createdAt: string;
}

interface KnowledgeBase {
  id: string;
  name: string;
  fileCount: number;
  chunkCount: number;
  createdAt: string;
}

interface KnowledgeChunk {
  id: string;
  fileId: string;
  chunkIndex: number;
  chunkText: string;
  embeddingId: string | null;
  tokenCount: number;
}

interface McpServer {
  id: string;
  name: string;
  transport: 'stdio' | 'http';
  url: string | null;
  command: string | null;
  config: Record<string, unknown>;
}

interface UserSkill {
  id: string;
  name: string;
  scriptPath: string;
  sandboxType: 'iframe' | 'wasmtime';
  permissions: Record<string, boolean>;
}

interface ModelInfo {
  id: string;
  name: string;
  size: string;
  quantization: string;
  vramRequired: number;
  downloaded: boolean;
  loaded: boolean;
}

interface EntitlementStatus {
  tier: 'free' | 'paid' | 'one-time';
  expiresAt: string | null;
  graceRemaining: number | null;
  locked: boolean;
}

interface SidecarStatus {
  name: string;
  status: 'starting' | 'ready' | 'stopped' | 'crashed';
  pid: number | null;
}

interface HardwareInfo {
  totalRamGb: number;
  vramGb: number;
  availableDiskGb: number;
  cpuCores: number;
}

// === Error types (mirrors Rust AppError) ===
interface AppError {
  type: 'AppError';
  variant: string;
  message: string;
}
```

### Appendix C: CSS Custom Property Master List

Complete list of all CSS custom properties used across the application:

```
Background:   --bg-base, --bg-surface, --bg-elevated, --bg-hover, --bg-active, --bg-input
Text:         --text-primary, --text-secondary, --text-muted, --text-inverse
Accent:       --accent, --accent-hover
Semantic:     --success, --warning, --error, --info
Border:       --border-subtle, --border-default, --border-focus
Spacing:      --space-0.5, --space-1, --space-2, --space-3, --space-4, --space-5, --space-6, --space-8, --space-10, --space-12
Typography:   --font-sans, --font-mono, --text-xs, --text-sm, --text-base, --text-lg, --text-xl, --text-2xl
Radius:       --radius-sm, --radius-md, --radius-lg, --radius-full
Shadow:       --shadow-sm, --shadow-md, --shadow-lg
Animation:    --duration-fast, --duration-normal, --duration-slow, --ease-out, --ease-spring
```

**Total: 47 CSS custom properties.**
