import re

with open("porting-specs/frontend_design.md", "r") as f:
    content = f.read()

# Replace Section 4
section_4_new = """## 4. Screen Inventory and Component Hierarchy

The application navigation is built around a 5-tab Sidebar System:

### 4.1 Tab: Chat
**Purpose**: Primary messaging and project contexts.
**Sub-navigation**:
- **Projects**: Dedicated contexts with custom instructions/files.
- **Library**: Saved documents and references.
- **All Chats**: Unfiltered list of all conversations.
- **Files**: Workspace file explorer.
- **Pinned**: Important pinned conversations.
- **Recent**: Chronological list of recent chats.

### 4.2 Tab: Agent
**Purpose**: Management of capabilities and system memory.
**Sub-navigation**:
- **Skills**: Manager for agent capabilities and scripts.
- **Models**: Local and cloud model selection.
- **Apps**: Integrated tools.
- **Marketplace**: Downloadable skills and models.
- **Connectors**: API integrations (e.g., GitHub, Linear).
- **AI Hub**: Centralized agent metrics.
- **Memories**: Global knowledge graph and explicit memory storage.

### 4.3 Tab: Notes
**Purpose**: Markdown and rich-text note taking.
**Sub-navigation**: Recent, All, Trash, Settings, Subscription.

### 4.4 Tab: Work
**Purpose**: Integration with productivity tools.
**Sub-navigation**: Email, Calendar, Tasks.

### 4.5 Tab: Playground
**Purpose**: Experimental features and fun components.
**Sub-navigation**: Tamagotchi.
"""

content = re.sub(r'## 4\. Screen Inventory and Component Hierarchy.*?(?=---|\n## 5\.)', section_4_new + '\n\n---\n\n', content, flags=re.DOTALL)

# Replace Section 5
section_5_new = """## 5. Component Specifications: Sidebar

### 5.1 Sidebar Layout
The `Sidebar.svelte` component manages the top-level 5-tab navigation (`Chat`, `Agent`, `Notes`, `Work`, `Playground`). Clicking a tab updates the active state and renders the corresponding context-aware sub-navigation array.

### 5.2 Contextual Sub-navigation
Each tab dynamically injects its own list of routes (e.g., `Projects`, `Library`, `Memories`). 
Navigation links map strictly to physical route paths like `/library`, `/mcp`, or `/memories`.

### 5.3 Active Context
When a `Project` is selected, the sidebar visually highlights it, and the `conversationStore` applies the project's specific context (system instructions, linked files) to the active session.
"""

content = re.sub(r'## 5\. Component Specifications: Sidebar.*?(?=---|\n## 6\.)', section_5_new + '\n\n---\n\n', content, flags=re.DOTALL)

# Replace Section 15 Appendix A
appendix_a_new = """### Appendix A: Proposed Svelte Component File Tree

```
src/
├── app.html
├── lib/
│   ├── components/
│   │   ├── shell/
│   │   │   ├── AppShell.svelte
│   │   │   ├── Sidebar.svelte
│   │   │   ├── StatusBar.svelte
│   │   │   └── TitleBar.svelte
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
│   ├── models/+page.svelte
│   ├── apps/+page.svelte
│   ├── marketplace/+page.svelte
│   ├── connectors/+page.svelte
│   ├── aihub/+page.svelte
│   ├── memories/+page.svelte (Knowledge Graph)
│   ├── notes/
│   │   ├── recent/+page.svelte
│   │   ├── all/+page.svelte
│   │   ├── trash/+page.svelte
│   │   ├── settings/+page.svelte
│   │   └── subscription/+page.svelte
│   ├── work/
│   │   ├── email/+page.svelte
│   │   ├── calendar/+page.svelte
│   │   └── tasks/+page.svelte
│   └── playground/
│       └── tamagotchi/+page.svelte
```
"""

content = re.sub(r'### Appendix A: Proposed Svelte Component File Tree.*?(?=### Appendix B:)', appendix_a_new + '\n\n', content, flags=re.DOTALL)

with open("porting-specs/frontend_design.md", "w") as f:
    f.write(content)
print("Updated frontend_design.md")
