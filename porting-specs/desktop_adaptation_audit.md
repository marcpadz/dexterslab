# Desktop Adaptation Audit: Web → Local-First Tauri v2

> **Source**: `Comprehensive_Feature_Audit.md` (~330 features, 24 categories)
> **Target Architecture**: Tauri v2 / Svelte 5 / Rust backend / SQLite + LanceDB / Pi SDK sidecar / llama-server sidecar (GGUF) + MLX runtime / Bundled Chromium / Composio integration
> **Reference Documents**: `technical_specification.md`, `system_design.md`, `frontend_design.md`
> **Philosophy**: Local-first, offline-capable. Cloud is opt-in utility, never a blocker. Local models (GGUF/MLX) via AI Hub are first-class citizens.

---

## Legend

### Verdict

| Verdict | Meaning |
|---------|---------|
| **KEEP** | Feature maps directly to local equivalent. No web dependency. |
| **ADAPT** | Feature has web dependency that must be replaced with local alternative. |
| **DROP** | Feature is web/marketing-only with no desktop value. |
| **DEFER** | Feature is valuable but not needed for Phase 1–2; revisit later. |

### Migration Complexity

| Rating | Meaning | Typical Effort |
|--------|---------|----------------|
| **L** (Low) | Direct port or trivial substitution. No architectural change. | < 1 day |
| **M** (Medium) | Requires new IPC command, SQLite table, or component rewrite. | 1–3 days |
| **H** (High) | Requires new subsystem, sidecar integration, or cross-cutting change. | 3–7 days |
| **X** (Extra-High) | Novel capability with no web equivalent. Requires greenfield implementation. | 1–2 weeks |

### Effort Estimate

| Size | Meaning |
|------|---------|
| **XS** | Hours of work. Config or CSS change. |
| **S** | 1–2 days. Single component or command. |
| **M** | 3–5 days. Multiple components with integration. |
| **L** | 1–2 weeks. Subsystem with cross-cutting concerns. |
| **XL** | 2–4 weeks. Major architectural work. |

---

## 1. Authentication & User Management (12 features)

| # | Feature | Verdict | Web Dependency | Desktop Adaptation |
|---|---------|---------|----------------|--------------------|
| 1.1 | Email/Password Registration | **ADAPT** | Convex Auth server | Replace with local account creation. Single-user desktop app: store hashed password (argon2) in SQLite `User` table. No email verification needed. |
| 1.2 | Email/Password Login | **ADAPT** | Convex Auth server | Local login against SQLite stored hash. Session is the app process lifetime — no remote session management. |
| 1.3 | Password Reset Flow | **DROP** | Resend email API, 8-digit code | Not applicable for single-user desktop. If needed: macOS Keychain-based recovery or local passphrase reset. |
| 1.4 | Google OAuth | **DROP** | Google OAuth2 + callback route | Not needed for local desktop. Auth is local-only. |
| 1.5 | CSRF Protection | **DROP** | Middleware anti-CSRF tokens | Desktop apps don't have cross-site request forgery vectors. No HTTP middleware needed. |
| 1.6 | Session Management | **ADAPT** | Convex auth sessions | Session = app process lifetime. EntitlementService validates JWT lease on launch. Grace period for offline. |
| 1.7 | User Profile CRUD | **KEEP** | Convex users.ts | Store in SQLite `User` table. Profile image stored as file path in `Application Support/`. |
| 1.8 | User Role System | **ADAPT** | Convex admin email check | Replace with `EntitlementService` tier gating. **Free**: cloud-provided models only. **One-Time Purchase**: local models (GGUF/MLX via AI Hub). **Subscription**: BYOK (bring your own API key). Single user owns the app. |
| 1.9 | Auth Callback Handler | **DROP** | Post-auth URL redirect | No web auth flow to redirect. |
| 1.10 | Auth Middleware | **ADAPT** | Next.js middleware route protection | Replace with Rust-side entitlement check at app startup. IPC commands validate lease before execution. |
| 1.11 | Usage Tracking | **ADAPT** | Convex daily counters | Store `local_token_count` in SQLite. Daily reset via Rust background task (no server-side cron). |
| 1.12 | User Preferences | **KEEP** | Convex sync + local store | Store in SQLite `AppSettings` table (key-value). No server sync needed. Svelte `$state` modules read via IPC. |

**Summary**: 2 KEEP, 4 ADAPT, 5 DROP, 0 DEFER. Auth collapses from 12 features to ~6 local equivalents. Convex Auth, Google OAuth, CSRF, and password reset are eliminated entirely.

**Complexity**: All ADAPTs are **M** (new SQLite table + IPC commands). §1.8 entitlement tiers are **H** (three-tier gating matrix with model access rules).
**Effort**: **M** — Single `User` table + `EntitlementService` with three-tier model access matrix. Argon2 hashing via `argon2` crate. ~3 days total.
**Rust Crates**: `argon2 = "0.5"`, `security-framework = "3.7.0"` (Keychain), `jsonwebtoken = "10.4.0"` (JWT validation)
**Risk**: Low. Single-user auth is trivially simpler than multi-user web auth. The only subtlety is the clock rollback detection (Keychain monotonic timestamp).

---

## 2. Chat System (14 features)

| # | Feature | Verdict | Web Dependency | Desktop Adaptation |
|---|---------|---------|----------------|--------------------|
| 2.1 | Chat Creation | **ADAPT** | Convex create mutation | IPC command `chat_create` → Rust → SQLite INSERT into `Conversation` table. Model selection from local catalog. |
| 2.2 | Chat Listing | **ADAPT** | Convex list query | IPC command `chat_list_conversations` → SQLite SELECT with ORDER BY pinned, updatedAt. |
| 2.3 | Chat View | **KEEP** | None (page component) | Svelte route `/chat/[id]`. Messages loaded via IPC. |
| 2.4 | Main Chat Page | **KEEP** | None | Default Svelte route. Same component, local data source. |
| 2.5 | All Chats View | **KEEP** | None | Svelte page with search/filter over IPC-loaded conversation list. |
| 2.6 | Chat Pin/Unpin | **ADAPT** | Convex togglePin | IPC command `chat_toggle_pin` → SQLite UPDATE. |
| 2.7 | Bulk Chat Operations | **ADAPT** | Convex bulk mutations | IPC commands `chat_bulk_pin`, `chat_bulk_delete`, `chat_bulk_move_project` → SQLite transactions. |
| 2.8 | Chat Title Generation | **ADAPT** | LLM-based truncation | Keep auto-title via Pi SDK (first message summary). Truncate to 60 chars client-side. |
| 2.9 | Chat Model Update | **ADAPT** | API route | IPC command `chat_update_model` → SQLite UPDATE. |
| 2.10 | Chat Project Association | **ADAPT** | Convex update | IPC command `chat_set_project` → SQLite UPDATE with project_id FK. |
| 2.11 | Chat Public/Share | **DEFER** | Public share page | Phase 3: Export conversation as HTML/JSON. No live sharing in desktop app. |
| 2.12 | Chat Deletion | **ADAPT** | Convex remove mutation | IPC command `chat_delete_conversation` → SQLite DELETE with CASCADE (messages, attachments). |
| 2.13 | Chat Persistence | **ADAPT** | localStorage fallback | All persistence via SQLite. No localStorage needed. |
| 2.14 | Chat Session Provider | **ADAPT** | Real-time Convex connection | Replace with Tauri event listeners. `chat_stream_token` events push tokens from PiSdkBridge to Svelte. |

**Summary**: 3 KEEP, 10 ADAPT, 0 DROP, 1 DEFER. All Convex mutations become IPC → SQLite. Real-time subscriptions become Tauri native events.

**Complexity**: Most ADAPTs are **M** (standard CRUD → IPC). §2.8 auto-title is **H** (requires Pi SDK round-trip). §2.14 session provider is **H** (event-driven streaming architecture).
**Effort**: **M** — 6 IPC commands already specified in system_design.md §5.1 with full type signatures. Main effort is the streaming event pipeline: `agent-token` events → incremental Svelte rendering → SQLite persistence on complete. ~4 days.
**Rust Crates**: `rusqlite` (bundled), `uuid = "1.11"` (conversation/message IDs)
**Convex Functions Eliminated**: `chats.create`, `chats.list`, `chats.togglePin`, `chats.remove`, `chats.update` — all replaced by typed IPC commands with SQLite WHERE clauses.
**Risk**: Medium. The streaming persistence pattern (§3.5) requires careful coordination: if the app crashes mid-stream, the accumulated tokens in memory are lost. Mitigation: periodic flush to SQLite every N tokens.

---

## 3. Messaging System (12 features)

| # | Feature | Verdict | Web Dependency | Desktop Adaptation |
|---|---------|---------|----------------|--------------------|
| 3.1 | Message Sending | **ADAPT** | Convex send mutation | IPC `chat_send_message` → Rust validates → PiSdkBridge → Pi SDK sidecar → streams back via events → SQLite INSERT. |
| 3.2 | Message Listing | **ADAPT** | Convex list query | IPC `chat_get_thread` → SQLite SELECT with parent_message_id tree. |
| 3.3 | Message Deletion | **ADAPT** | Convex remove mutation | IPC `message_delete` → SQLite DELETE. |
| 3.4 | Bulk Message Removal | **ADAPT** | Convex removeAllForChat | IPC `message_delete_all` → SQLite DELETE WHERE conversation_id. |
| 3.5 | Stream Persistence | **ADAPT** | Convex stream persistence | Rust accumulates streaming tokens in memory → writes final message to SQLite on stream complete. |
| 3.6 | Message Content Parsing | **KEEP** | None (client-side) | Parse message content parts (text, tool invocations, reasoning) in Svelte frontend. Same logic. |
| 3.7 | Message Serialization | **ADAPT** | Transport serialization | Replace Convex serialization with serde_json ↔ TypeScript JSON mapping over IPC. |
| 3.8 | Message Group IDs | **ADAPT** | Convex messageGroupId | SQLite `Message.parent_message_id` column. Recursive CTE for grouped retrieval. |
| 3.9 | Message Read Tracking | **KEEP** | None | Track in Svelte `$state` or SQLite `Message.read_at` column. |
| 3.10 | AI Message Rendering | **KEEP** | None (frontend) | Streamdown → replace with `marked` or `markdown-it` in Svelte. CJK, code, math, mermaid plugins. |
| 3.11 | User Message Rendering | **KEEP** | None (frontend) | Svelte component. No change needed beyond framework port. |
| 3.12 | Markdown Rendering | **KEEP** | None (frontend) | Svelte markdown renderer with GFM + math + table support. |

**Summary**: 5 KEEP, 7 ADAPT. All Convex mutations → IPC → SQLite. Streaming changes from SSE to Tauri events. Message parsing and rendering are pure frontend — direct port to Svelte.

**Complexity**: §3.1 Message Sending is **H** (orchestrates: IPC → entitlement check → Pi SDK → streaming → persistence). §3.5 Stream Persistence is **M** (accumulator pattern). §3.7 Serialization is **L** (serde ↔ TypeScript JSON). §3.10-3.12 Rendering is **L** (pure frontend port).
**Effort**: **S** — Most work is shared with §2 (Chat System). Message-specific logic (grouping, read tracking) adds ~2 days on top.
**Rust Crates**: `rusqlite` (message storage), `uuid` (message IDs)
**Web Dependencies Eliminated**: `convex/messages.ts` (send, list, remove, removeAllForChat, reassignAfterCutoff mutations), `app/auth/api/chat/assistant-stream-persistence.ts`, `app/auth/api/chat/message-content.ts`, `app/auth/api/chat/message-serialize.ts`
**Risk**: Low-Medium. Message grouping (§3.8) with `parent_message_id` requires careful recursive CTE design. Mitigation: test with deeply nested branching scenarios (5+ levels).

---

## 4. AI Agent Engine (16 features)

| # | Feature | Verdict | Web Dependency | Desktop Adaptation |
|---|---------|---------|----------------|--------------------|
| 4.1 | Agent Chat API | **ADAPT** | AI SDK v7 streamText() | Replace with Pi SDK sidecar via JSON-RPC. Pi SDK handles multi-provider LLM orchestration internally. |
| 4.2 | SSE Streaming | **ADAPT** | HTTP SSE endpoint | Replace with Tauri native events: `chat_stream_token`, `chat_stream_tool_call`, `chat_stream_complete`. |
| 4.3 | Model Resolver | **ADAPT** | 10+ provider instances | Rust `ModelManager` resolves model → provider (API key from Keychain) or local (llama-server). Catalog in SQLite. |
| 4.4 | System Prompt | **KEEP** | None | Stored in SQLite `AppSettings` or bundled file. Injected into Pi SDK JSON-RPC params. |
| 4.5 | Agent Persistence | **ADAPT** | Convex agents.ts | SQLite `Conversation` + `Message` tables with `parent_message_id` for branching. |
| 4.6 | Tool Loader | **ADAPT** | Aggregates native + MCP tools | Rust `McpClientRouter` loads MCP tools. Native tools (file, browser, shell) implemented in Rust. Passed to Pi SDK as tool definitions. |
| 4.7 | Tool Format Adapter | **ADAPT** | AI SDK tool() definitions | Rust adapter converts MCP tool schemas → Pi SDK tool format. |
| 4.8 | MCP Tools Adapter | **ADAPT** | Remote MCP connections | `McpClientRouter` supports stdio (local) + HTTP (remote) MCP servers. Phase 1: stdio only. |
| 4.9 | **Rich Media Streaming + Suggestion Chips** | **ADAPT** | suggest_chips, widgets, etc. | **Rich media streaming** (not plain text): Markdown with syntax highlighting, LaTeX math rendering, Mermaid diagrams, inline images, charts/tables — all streamed token-by-token via Tauri events and rendered in WKWebView. **Mandatory suggestion chips** appended to every response: Pi SDK `suggestion_chips` tool emits 3–5 contextual follow-up actions as clickable chips. Svelte `SuggestionChips` component renders below each message. Chips are typed: `follow_up`, `tool_action`, `explore`, `refine`. Power-user accelerator pattern. |
| 4.10 | Think Deeper | **KEEP** | None (config param) | Pass reasoning level as Pi SDK parameter. Svelte selector store. |
| 4.11 | Skills Injection | **ADAPT** | Server-side injection | Rust loads `UserSkill` scripts from disk → injects into Pi SDK prompt as additional context. |
| 4.12 | Chat Request Normalization | **KEEP** | None | Normalize in Rust before forwarding to Pi SDK. |
| 4.13 | Prompt Budget Tracking | **ADAPT** | Server-side token counting | Pi SDK reports token usage per step. Rust tracks cumulative total and enforces step limit. |
| 4.14 | Assistant Message Collector | **ADAPT** | Server-side accumulator | Rust accumulates streaming tokens from Pi SDK events, writes final message to SQLite. |
| 4.15 | Agent Workflow Events | **ADAPT** | SSE event dispatch | Tauri events: `agent_step_start`, `agent_step_finish`, `agent_tool_call`, `agent_tool_result`. |
| 4.16 | SSE Events Client | **ADAPT** | Client-side SSE parsing | Replace with `listen()` from `@tauri-apps/api/event`. Typed event handlers in Svelte stores. |

**Summary**: 2 KEEP, 14 ADAPT. The biggest adaptation category. AI SDK v7 is entirely replaced by Pi SDK sidecar. SSE replaced by Tauri events. Tool loading moves from TypeScript to Rust MCP client.

**Complexity**: §4.1 Agent Chat API is **X** (core Pi SDK integration — the heart of the app). §4.6 Tool Loader is **H** (must aggregate native + MCP tools into unified schema). §4.2 SSE→Events is **H** (6 event types with precise payload schemas defined in system_design.md §6.1). §4.13 Budget Tracking is **M**.
**Effort**: **XL** — This is the critical path. Pi SDK JSON-RPC integration (§4.1), tool format adapter (§4.7), and streaming event pipeline (§4.14, §4.15) form a tightly coupled subsystem. ~3 weeks of focused work.
**Rust Crates**: `tokio = { version = "1.52.3", features = ["full"] }` (async runtime), `serde_json = "1.0"` (JSON-RPC parsing), `futures = "0.3"` (stream handling)
**Web Dependencies Eliminated**:
  - `ai-sdk` (Vercel AI SDK v7): `streamText()`, `generateText()`, provider registry — entire npm package replaced by Pi SDK sidecar
  - `lib/agents/tool-loader.ts`: TypeScript tool aggregation → Rust `McpClientRouter` + native tool registry
  - `lib/agents/tools.ts`: AI SDK `tool()` adapter → Rust JSON serialization matching Pi SDK expected schema
  - `lib/agents/mcp.ts`: TypeScript MCP client → Rust `chromiumoxide`-style stdio/HTTP MCP router
  - `app/auth/api/agents/sse.ts`: HTTP SSE endpoint → Tauri `app.emit("agent-token", payload)`
**JSON-RPC Contract** (Pi SDK ↔ Rust):
  - Request: `{ "jsonrpc": "2.0", "id": N, "method": "agent.run", "params": { conversationId, messages[], model, tools[] } }`
  - Streaming: `{ "jsonrpc": "2.0", "method": "agent.token", "params": { conversationId, chunk } }`
  - Tool Call: `{ "jsonrpc": "2.0", "id": N, "method": "tool.call", "params": { server, tool, arguments } }`
**Risk**: **High**. This is the highest-risk category. Pi SDK is a third-party npm package (`@earendil-works/pi-coding-agent`). If its JSON-RPC interface changes, the Rust bridge breaks. Mitigation: pin Pi SDK version, add integration tests that validate JSON-RPC message contracts.

---

## 5. Model Management & AI Hub (20 features)

### Entitlement Tier Gating Matrix

| Capability | Free | One-Time Purchase | Subscription |
|-----------|------|-------------------|---------------|
| Cloud models (provider-hosted) | Yes (company-provided only) | Yes (all) | Yes (all) |
| Local models (GGUF/MLX via AI Hub) | No | **Yes** | Yes |
| BYOK (bring your own API key) | No | No | **Yes** |
| AI Hub model registry | Browse only | Download + run | Download + run |

| # | Feature | Verdict | Web Dependency | Desktop Adaptation |
|---|---------|---------|----------------|---------------------|
| 5.1 | Model Catalog | **ADAPT** | Base catalog config file | SQLite `model_catalog` table seeded with default cloud models. Local models added dynamically via AI Hub downloads. |
| 5.2 | Managed Model Catalog | **ADAPT** | Convex synced catalog | Optional: fetch updated catalog from API on launch (with offline fallback to cached SQLite). |
| 5.3 | Supported Providers Registry | **KEEP** | None (config) | Rust enum of supported providers. API keys stored in macOS Keychain. |
| 5.4 | Provider Source Definitions | **KEEP** | None (config files) | Rust module per provider with model definitions. Bundled at compile time. |
| 5.5 | Model Selector UI | **KEEP** | None (frontend) | Svelte component with search, sort, favorites, **local/cloud filter tabs**. Data from IPC. |
| 5.6 | Multi-Model Selector | **DEFER** | None (frontend) | Phase 2 feature for comparison/evaluation mode. |
| 5.7 | Model Favorites | **KEEP** | Convex favoriteModels | Store in SQLite `AppSettings` as JSON array of model IDs. |
| 5.8 | Model Access Control | **ADAPT** | Server-side gating | `EntitlementService` checks tier before allowing model access. **Free**: cloud models only. **One-Time**: + local GGUF/MLX. **Subscription**: + BYOK. |
| 5.9 | Provider Health Checks | **ADAPT** | API health endpoints | Rust pings provider endpoints periodically. Status cached in memory, emitted via Tauri events. |
| 5.10 | Model Refresh API | **ADAPT** | Admin API route | IPC command `model_refresh_catalog` → fetch from remote API → update SQLite. Admin-only. |
| 5.11 | Model Store Provider | **ADAPT** | React context | Svelte `$state` module with reactive derived stores. |
| 5.12 | BYOK (Bring Your Own Key) | **ADAPT** | Convex encrypted storage | API keys stored in macOS Keychain via `security-framework` crate. **Gated to Subscription tier only.** |
| 5.13 | Seed Default Favorites | **DROP** | Dev tool page | Seed logic in Rust migration runner on first launch. |
| 5.14 | Provider Config CRUD | **ADAPT** | Convex admin mutations | IPC commands for admin to upsert provider configs in SQLite `AppSettings`. |
| 5.15 | Model Manager (Admin) | **ADAPT** | Admin panel + Convex | Settings screen in Svelte. IPC commands to enable/disable models, set access tiers in SQLite. |
| 5.16 | **AI Hub — Model Registry** | **ADAPT** | None (new) | Rust `ModelHub` service queries **Hugging Face Hub API** for GGUF/MLX models. Filters by: quantization (Q4_K_M, Q5_K_M, Q8_0, F16), size, architecture (Llama, Mistral, Phi, Qwen). Results cached in SQLite `model_hub_cache`. |
| 5.17 | **AI Hub — Model Download** | **ADAPT** | None (new) | Rust downloads GGUF files from Hugging Face via `reqwest` streaming → `~/Library/Application Support/AskDexter/models/`. Shows progress via Tauri events. Resumable downloads with byte-range requests. |
| 5.18 | **AI Hub — MLX Runtime** | **ADAPT** | None (new) | Bundle `mlx-lm` Python runtime as sidecar (similar to llama-server). Spawned via `uvicorn` or direct CLI. Apple Silicon optimized — Metal GPU acceleration. IPC `mlx_infer` for inference requests. |
| 5.19 | **AI Hub — Local Inference** | **ADAPT** | None (new) | `llama-server` sidecar for GGUF models. `mlx-lm` sidecar for MLX models. Rust `LocalModelManager` routes inference to correct sidecar based on model format. Health checks via sidecar HTTP endpoints. |
| 5.20 | **AI Hub — Model Lifecycle** | **ADAPT** | None (new) | IPC commands: `model_hub_search`, `model_hub_download`, `model_hub_delete`, `model_hub_list_local`. Download manager tracks: queued, downloading, paused, complete, error states. Disk usage tracking. |

**Summary**: 4 KEEP, 14 ADAPT, 1 DROP, 1 DEFER. **Major expansion**: AI Hub adds 5 new features (5.16–5.20) for local model discovery, download, and inference via GGUF + MLX. Three-tier entitlement gates model access.

**Complexity**: §5.16 AI Hub Registry is **H** (Hugging Face API integration, model metadata parsing, quantization filtering). §5.17 Model Download is **H** (streaming large files, resumable downloads, disk management). §5.18 MLX Runtime is **H** (Python sidecar management, Apple Silicon optimization). §5.19 Local Inference is **H** (dual sidecar routing: llama-server for GGUF, mlx-lm for MLX). §5.12 BYOK is **H** (Keychain + subscription gating). Most CRUD ADAPTs are **L**.
**Effort**: **XL** — AI Hub is a major new subsystem: Hugging Face API client, model download manager, dual sidecar orchestration (llama-server + mlx-lm), entitlement gating. ~3 weeks.
**Rust Crates**: `security-framework = "3.7.0"` (Keychain for API keys), `sysinfo = "0.39.3"` (hardware/VRAM detection), `reqwest = "0.12"` (health checks, HF model download), `sha2 = "0.10"` (model file integrity), `tokio` (async download manager)
**Sidecar Binaries**:
  - `llama-server` — GGUF inference (Metal on Apple Silicon, CUDA fallback). Spawned: `llama-server -m {model_path} --port {port} -ngl 99`
  - `mlx-lm` — MLX inference (Apple Silicon only). Spawned: `python -m mlx_lm.server --model {model_path} --port {port}`
**Hugging Face Hub Integration**:
  ```
  Search: GET https://huggingface.co/api/models?search={query}&filter=gguf&sort=downloads
  Download: GET https://huggingface.co/{repo}/resolve/main/{file}.gguf
  Metadata: GET https://huggingface.co/api/models/{repo_id}
  ```
**Convex Functions Eliminated**: `models.ts` (upsertAccessTiers), `providers.ts` (upsert), `users.ts` (favoriteModels), `userKeys.ts` (encrypted key storage)
**Risk**: **High**. Dual sidecar management (llama-server + mlx-lm) adds lifecycle complexity. Hugging Face API rate limits may throttle model discovery. Mitigation: cache model registry locally, implement exponential backoff on downloads, pre-flight VRAM check before model launch.

---

## 6. Projects (15 features)

> **Concept**: ChatGPT/Claude-style persistent project folders. Each Project groups related conversations, files, knowledge bases, and custom instructions into a named workspace. Projects appear as folders in the sidebar; selecting a project filters the chat list and injects project-level custom instructions into the agent system prompt. Users can drag-and-drop conversations between projects, bulk-assign chats, and attach reference files.

| # | Feature | Verdict | Web Dependency | Desktop Adaptation |
|---|---------|---------|----------------|--------------------|
| 6.1 | Project CRUD | **ADAPT** | Convex + API routes | IPC commands `project_create`, `project_update`, `project_delete` → SQLite `Project` table. |
| 6.2 | Project Listing | **ADAPT** | Convex list query | IPC `project_list` → SQLite SELECT. |
| 6.3 | Project Context | **KEEP** | None (page) | Projects are isolated session contexts. A Project groups system instructions and reference files, which are applied to any chat thread assigned to it. They appear in the `Chat` tab sub-navigation. |
| 6.4 | Multi-Tab Sidebar | **ADAPT** | Layout (Sidebar) | Replace generic single-list sidebar with a 5-tab system (Chat, Agent, Notes, Work, Playground) where each tab renders context-specific sub-navigation items. |
| 6.5 | Knowledge Graph / Memories | **ADAPT** | Layout | Decouple Knowledge Graph from Projects. It is now accessed via the global `Memories` surface under the `Agent` tab. |
| 6.4 | Project Instructions | **KEEP** | Convex schema field | SQLite `Project.instructions` column. Injected into agent system prompt. |
| 6.5 | Project Libraries | **ADAPT** | Convex add/removeLibrary | IPC `project_add_library`, `project_remove_library` → SQLite junction table. |
| 6.6 | Project Files | **ADAPT** | Convex attachments | IPC `project_list_files` → SQLite query joining `KnowledgeFile` → project. Files on local disk. |
| 6.7 | Project Knowledge Bases | **ADAPT** | API routes | Same as 6.5 — junction table in SQLite. |
| 6.8 | Chat-Project Association | **ADAPT** | Convex update | IPC `chat_set_project` → SQLite UPDATE `Conversation.project_id`. |
| 6.9 | Bulk Add to Project | **ADAPT** | Convex bulk mutation | IPC `project_bulk_add_chats` → SQLite batch UPDATE in transaction. |
| 6.10 | Project Drag & Drop | **KEEP** | None (frontend) | Svelte drag-and-drop handler → IPC to update ordering. |
| 6.11 | Project Chat Operations | **KEEP** | None | Svelte hooks over IPC data. |
| 6.12 | Add to Project Dialog | **KEEP** | None (frontend) | Svelte dialog component. |
| 6.13 | Project Source Cards | **KEEP** | None (frontend) | Svelte component with file-type icons. |
| 6.14 | Legacy Pod Compatibility | **DROP** | Redirect routes | No legacy pods in desktop. |
| 6.15 | Project Library Migration | **DROP** | SQL migration script | Fresh SQLite schema — no migration from pods needed. |

**Summary**: 7 KEEP, 6 ADAPT, 2 DROP. Straightforward Convex → SQLite migration. Projects are pure CRUD — no complex web dependencies.

**Complexity**: All ADAPTs are **M** (standard CRUD → IPC → SQLite). No high-risk items.
**Effort**: **S** — New `Project` table + junction table. 6 IPC commands. ~2 days.
**Risk**: Low. Projects are the simplest CRUD category.

---

## 7. Library (formerly Knowledge Base) (13 features)

| # | Feature | Verdict | Web Dependency | Desktop Adaptation |
|---|---------|---------|----------------|--------------------|
| 7.1 | Knowledge Base CRUD | **ADAPT** | Convex mutations | IPC commands → SQLite `KnowledgeBase` table. |
| 7.2 | Knowledge Base Listing | **ADAPT** | Convex query | IPC `kb_list` → SQLite SELECT. |
| 7.3 | KB File Upload | **ADAPT** | Convex generateUploadURL | Files copied/linked to `Application Support/AskDexter/knowledge/` on local disk. No upload URL needed. |
| 7.4 | KB Chunking & Embedding | **ADAPT** | Convex insertChunksBatch | Rust: read file → chunk (text_splitter) → embed (local model or API) → store chunks in SQLite `KnowledgeChunk` + vectors in LanceDB. |
| 7.5 | KB Similarity Search | **ADAPT** | Convex cosine similarity | LanceDB ANN query via `lancedb` crate. Returns `embedding_id` → join with SQLite `KnowledgeChunk`. |
| 7.6 | KB-Chat Linking | **ADAPT** | Convex linkToChat | SQLite junction table `chat_knowledge_base` (chat_id, kb_id). |
| 7.7 | KB Context Builder | **ADAPT** | Server-side context build | Rust: query LanceDB for relevant chunks → build context string → inject into Pi SDK prompt. |
| 7.8 | Library (KB Alias) | **DROP** | Re-export for compat | "Library" is now the primary name. No alias needed. |
| 7.9 | Library Listing/Create/Query | **DROP** | Deduplicated endpoints | Unified under KB IPC commands. |
| 7.10 | Library Context Builder | **DROP** | Duplicate of 7.7 | Merged into KB Context Builder. |
| 7.11 | Library Search | **ADAPT** | API search endpoint | IPC `kb_search` → LanceDB vector search + SQLite full-text search (FTS5). |
| 7.12 | KB File Deletion | **ADAPT** | Convex cascade delete | IPC `kb_delete_file` → SQLite DELETE cascade (chunks) + LanceDB vector deletion + local file removal. |
| 7.13 | Library Page | **KEEP** | None (frontend) | Svelte page for KB management. |

**Summary**: 1 KEEP, 8 ADAPT, 4 DROP. Convex vector search → LanceDB. File upload URL → local filesystem copy. "Library" is the canonical name (not "Knowledge Base").

**Complexity**: §7.4 Chunking & Embedding is **X** (novel pipeline: file→text_splitter→embedder→LanceDB). §7.5 Similarity Search is **H** (LanceDB ANN query + SQLite JOIN). §7.7 Context Builder is **H** (must rank and truncate chunks to fit context window). Other ADAPTs are **M**.
**Effort**: **L** — The chunking pipeline is the core deliverable. Requires integration of a text splitting algorithm (`text-splitter` crate), embedding generation (via Pi SDK or local embedding model), and LanceDB IVF-PQ index management. ~2 weeks.
**Rust Crates**: `lancedb = "0.15.0"` (vector store), `text-splitter = "0.8"` (chunking), `tokio` (async pipeline)
**Web Dependencies Eliminated**:
  - `convex/knowledgebase.ts`: All 8 Convex mutations (`create`, `createFile`, `generateUploadURL`, `insertChunksBatch`, `_searchChunks`, `linkToChat`, `remove`) replaced by IPC commands in system_design.md §5.2
  - Convex vector search (cosine similarity on `knowledgeChunks` table) → LanceDB ANN search with `IVF-PQ` index
  - Server-side `generateUploadURL` → Tauri file dialog + `std::fs::copy` to local knowledge directory
**Data Flow**: File → Rust `kb_import_file` → read text → chunk (512 tokens, 50-token overlap) → batch embed via Pi SDK → INSERT SQLite `KnowledgeChunk` + INSERT LanceDB vectors → emit `embedding-progress` events
**Risk**: **High**. LanceDB integration is new territory. IVF-PQ index training requires a minimum vector count (typically 256+). For small knowledge bases, must fall back to brute-force search. Mitigation: implement hybrid search (LanceDB ANN for >256 vectors, brute-force for <256).

---

## 8. Canvas (formerly Workspace / Artifact Canvas) (20 features)

> **Naming**: "Canvas" replaces both "Workspace" (Daytona sandbox) and "Artifact Canvas" to avoid confusion with §6 Projects.
> **Behavior**: Canvas is a **resizable side panel** adjacent to chat. When toggle is **ON**, AI-generated artifacts stream into the Canvas preview panel. When **OFF**, artifacts render **inline** in chat. Canvas also serves as document viewer/editor and live preview.

| # | Feature | Verdict | Web Dependency | Desktop Adaptation |
|---|---------|---------|----------------|---------------------|
| 8.1 | Canvas Session | **ADAPT** | Daytona HTTP API | Replace Daytona with local workspace directory. Each project/chat has a workspace path on local disk. |
| 8.2 | File Browser | **ADAPT** | Daytona API | Rust reads local filesystem via `std::fs`. IPC `canvas_list_files` returns directory tree. |
| 8.3 | File Preview | **ADAPT** | Daytona preview URL | Serve local files via Tauri asset protocol (`asset://`). HTML preview in WKWebView iframe within Canvas. |
| 8.4 | Folder Upload | **ADAPT** | Daytona HTTP upload | Tauri file dialog → copy directory to workspace path. |
| 8.5 | Terminal | **ADAPT** | Daytona terminal API | Use `pty` crate or xterm.js in frontend with Tauri shell command piping. Collapsible bottom workspace panel. |
| 8.6 | Computer Control | **DROP** | Daytona desktop automation | Out of scope for Phase 1. Browser automation (CDP) covers web interaction. |
| 8.7 | Browser Automation | **ADAPT** | Daytona browser launch | Rust `BrowserClient` via **bundled Chromium** + `chromiumoxide` CDP. Canvas panel tab. (See §16.) |
| 8.8 | Task Execution | **ADAPT** | Daytona command exec | Rust `tokio::process::Command` for shell execution in workspace directory. |
| 8.9 | Google Drive Sync | **DROP** | Google Drive API | Not in local-first scope. User can manually copy files. |
| 8.10 | Canvas Store | **ADAPT** | Zustand store | Svelte `$state` module for active tab, toggle state, file list, artifact list. |
| 8.11 | Daytona API Client | **DROP** | HTTP client | No Daytona. Direct local filesystem and process access. |
| 8.12 | Canvas Tabs | **KEEP** | None (frontend) | Svelte horizontal tabs: **Artifacts** (files list; clicking opens dynamic **Previews**), **Editor**, and **Browser**. (Terminal moved to collapsible bottom panel). |
| 8.13 | Error Response Helpers | **ADAPT** | HTTP error formatting | Rust error types with Display impl → typed error responses over IPC. |
| 8.14 | **Canvas Toggle** (new) | **ADAPT** | None (new) | Global toggle (toolbar button / keyboard shortcut). **ON**: artifacts stream to Canvas panel. **OFF**: artifacts render inline in chat. Persisted in `AppSettings`. |
| 8.15 | **Artifact Streaming** (new) | **ADAPT** | None (new) | When Canvas ON, model token stream for artifacts routed to Canvas panel in real time via Tauri events. Code/Markdown → preview/editor, HTML → iframe, mermaid → diagram viewer. |
| 8.16 | **Artifact Store** (from §9) | **ADAPT** | Zustand store | Svelte `$state` module. CRUD via IPC. Tracks: type, content, version history. |
| 8.17 | **Artifact Type Detection** (from §9) | **KEEP** | None (client-side) | Content-sniffing: code, HTML, SVG, markdown, mermaid, image, JSON. Routes to correct Canvas renderer. |
| 8.18 | **Code Block Rendering** (from §9) | **KEEP** | None | `shiki` or `highlight.js` in Svelte. Syntax highlighting for 50+ languages. |
| 8.19 | **HTML Preview Sandbox** (from §9) | **ADAPT** | Sandboxed iframe | WKWebView iframe with `sandbox` attribute and CSP within Canvas panel. |
| 8.20 | **Resizable Panel** (from §9) | **ADAPT** | react-resizable-panels | Canvas width via `split.js` or CSS `resize`. Drag handle between chat and Canvas. Min 320px, max 70% viewport. Persisted. |

**Summary**: 4 KEEP, 13 ADAPT, 3 DROP. Daytona eliminated. Canvas = resizable side panel with toggle behavior. Merges old §8 (Workspace) and §9 (Artifact Canvas) into a unified surface.

**Complexity**: §8.14 Canvas Toggle is **M** (routing logic: stream to Canvas vs inline). §8.15 Artifact Streaming is **H** (real-time token routing to iframe/mermaid/preview renderers). §8.5 Terminal is **H** (PTY, bottom panel). §8.7 Browser in Canvas is **H** (CDP in embedded panel). §8.20 Resizable Panel is **M**. Others **L-M**.
**Effort**: **L** — Core Canvas (file browser, toggle, preview, resizable panel) ~1 week. Bottom Terminal + browser embed ~1 week. Artifact streaming + Editor ~1 week. Total ~3 weeks.
**Rust Crates**: `tauri-plugin-dialog = "2.2"` (file picker), `tauri-plugin-shell = "2.2"` (terminal PTY), `chromiumoxide = "0.8.0"` (CDP browser in Canvas)
**Frontend**: `monaco-editor` (editor), `mermaid` (diagram preview), `split.js` (resizable panels), `shiki` (syntax highlighting)
**Canvas Toggle Data Flow**:
  ```
  Toggle ON:  Pi SDK stream -> Tauri event "artifact-token" -> Canvas panel renderer
                                                   (Previews for code, iframe for HTML, mermaid for diagrams)
  Toggle OFF: Pi SDK stream -> Tauri event "agent-token" -> inline chat message renderer
  ```
**Risk**: Medium-High. Monaco editor in WKWebView has known performance issues with large files. Mitigation: lazy-load Monaco only when editor is active, cap file size (500KB), use `highlight.js` for read-only code blocks.

---

## 9. Artifact Canvas — *MERGED INTO §8 CANVAS*

> All 12 original Artifact Canvas features have been consolidated into **§8 Canvas** (items 8.16–8.20). The Canvas panel now serves as the unified surface for artifact rendering, code editing, HTML preview, diagram viewing, and file browsing. The separate "Artifact Canvas" category is eliminated.
>
> **Feature mapping**: §9.1→§8.16, §9.2→§8.17, §9.7→§8.18, §9.9→§8.19, §9.12→§8.20. Remaining items (Artifact Cards, Illustrations, Media Display, SVG Preview, Mermaid, Sync from Chat, Code Block Utilities) are frontend components under §8 Canvas and §22 UI Component Library.

---

## 10. Work Suite (formerly Notes & Tasks) (14 features)

> **Naming**: "Work Suite" replaces "Notes & Tasks". Accessed via a **horizontal tabbed menu** in the main navigation: **Email**, **Calendar**, **Tasks**, **Notes**.
> **Integration**: Email via IMAP/SMTP, Calendar via CalDAV — both run locally, no cloud dependency. Composio integration (see §14) can optionally connect to Gmail/Google Calendar/Outlook for users who want cloud sync.

| # | Feature | Verdict | Web Dependency | Desktop Adaptation |
|---|---------|---------|----------------|---------------------|
| 10.1 | Notes CRUD | **ADAPT** | Convex mutations | IPC commands -> SQLite `Note` table. Adapted from open source (see §10.13 Prismical reference). |
| 10.2 | Task CRUD | **ADAPT** | Convex mutations | IPC commands -> SQLite `Task` table. |
| 10.3 | Task Priority | **KEEP** | None (data field) | SQLite `Task.priority` column. |
| 10.4 | Task Due Dates | **KEEP** | None (data field) | SQLite `Task.due_date` column. |
| 10.5 | Task Assignee | **DROP** | Multi-user field | Single-user desktop. No assignee concept. |
| 10.6 | Work Suite Feature Gating | **ADAPT** | Feature flags | Gate via `EntitlementService` tier or `AppSettings` toggle. |
| 10.7 | **Calendar View** | **ADAPT** | None (frontend) | Svelte calendar view over CalDAV data. Horizontal tab in Work Suite. Local CalDAV sync via `caldav` crate or `ical` parser. |
| 10.8 | **Email Client** | **ADAPT** | Email widget (was DROP) | IMAP/SMTP client via Rust `imap` + `lettre` crates. Horizontal tab. Local inbox cache in SQLite. HTML email rendering in WKWebView. |
| 10.9 | Google Drive Sync | **DROP** | Google Drive API | Not in local-first scope. |
| 10.10 | **Calendar Sync** | **ADAPT** | None (new) | CalDAV discovery + sync for local calendars (iCal, iCloud, Google via CalDAV). Background sync via Rust task. SQLite `CalendarEvent` table. |
| 10.11 | **Email Compose** | **ADAPT** | None (new) | Compose window with rich text editor. SMTP send via `lettre` crate. Drafts saved to SQLite. Attachments from local filesystem. |
| 10.12 | **Work Suite Tabs** | **KEEP** | None (new) | Horizontal tabbed navigation: **Email** / **Calendar** / **Tasks** / **Notes**. Tabs rendered as top-level route segments. |
| 10.13 | **Open Source Adaptation** | **ADAPT** | None (new) | **Prismical** (MIT, Electron/Next.js): Adapt Notes feature with AI-powered transcription. `whisper-wrapper` package provides local STT. Adapt voice notes, structured AI summaries, and action item extraction. See §25 for full open source adaptation guidance. |
| 10.14 | **AI Transcription** | **ADAPT** | None (new) | Bundle `whisper.cpp` binary for local speech-to-text. Voice input -> transcription -> structured note with AI summaries (via Pi SDK). Reuses Prismical's `whisper-wrapper` pattern. |

**Summary**: 3 KEEP, 9 ADAPT, 2 DROP. Expanded from 9 to 14 features. Email (IMAP/SMTP) and Calendar (CalDAV) restored as local-first features. Prismical adaptation for AI-powered notes.

**Complexity**: §10.8 Email Client is **X** (IMAP sync, HTML rendering, SMTP compose, attachment handling). §10.10 Calendar Sync is **H** (CalDAV protocol, recurring events, timezone handling). §10.14 AI Transcription is **H** (whisper.cpp integration, audio capture). §10.1/10.2 Notes/Tasks are **M**. Others **L-M**.
**Effort**: **XL** — Email client and calendar sync are substantial subsystems. Prismical adaptation for notes + AI transcription adds complexity. ~3 weeks.
**Rust Crates**: `imap = "3.0"` (IMAP email), `lettre = "0.11"` (SMTP send), `ical = "0.11"` (iCalendar parsing), `caldav` (CalDAV sync, evaluate), `whisper-rs` or bundled `whisper.cpp` binary (local STT)
**Prismical Adaptation** (MIT license):
  - `packages/whisper-wrapper` -> Rust whisper.cpp integration (local STT)
  - Notes UI with AI structuring -> Svelte port of Prismical's note components
  - Voice recording + transcription pipeline -> macOS AVFoundation capture + whisper.cpp
  - **NOT adapting**: Electron shell, Next.js routing, Better-Auth, Ollama integration (we use llama-server/MLX)
**Risk**: High. Email client is notoriously complex (IMAP quirks, HTML rendering, MIME parsing). Mitigation: start with read-only inbox + compose, defer advanced features (rules, filters, threading) to Phase 2.

---

## 11. Memories (9 features)

| # | Feature | Verdict | Web Dependency | Desktop Adaptation |
|---|---------|---------|----------------|--------------------|
| 11.1 | Memory CRUD | **ADAPT** | Convex mutations | IPC commands → SQLite `Memory` table (add to schema). |
| 11.2 | Memory Listing | **ADAPT** | Convex query | IPC `memory_list` → SQLite SELECT with pinned-first ordering. |
| 11.3 | Memory Categories | **KEEP** | None (data field) | SQLite `Memory.category` column. |
| 11.4 | Auto-Generated Memories | **ADAPT** | AI flagging | Pi SDK tool call `save_memory` during agent conversation → Rust inserts into SQLite. |
| 11.5 | Memory Pinning | **KEEP** | None (data field) | SQLite `Memory.is_pinned` column. |
| 11.6 | Memory Context Builder | **ADAPT** | Server-side context build | Rust: query relevant memories from SQLite → build context string → inject into Pi SDK prompt. |
| 11.7 | Memory Settings | **KEEP** | None | Store in SQLite `AppSettings`. |
| 11.8 | Clear All Memories | **ADAPT** | Convex clearAll mutation | IPC `memory_clear_all` → SQLite DELETE FROM Memory. |
| 11.9 | Memory Source Tracking | **KEEP** | None (data field) | SQLite `Memory.source_conversation_id` column. |

**Summary**: 5 KEEP, 4 ADAPT. Memories are pure local data. Needs new SQLite table. Context builder moves to Rust.

**Complexity**: §11.6 Context Builder is **M** (rank + truncate + inject into prompt). §11.4 Auto-Generated is **M** (Pi SDK tool call → Rust INSERT). Other ADAPTs are **L**.
**Effort**: **S** — One new table, 6 IPC commands. Context builder adds ~1 day. ~3 days total.
**Risk**: Low. Memories are straightforward CRUD. The context builder must respect token budget (coordinate with §4.13 Prompt Budget Tracking).

---

## 12. File System (12 features)

| # | Feature | Verdict | Web Dependency | Desktop Adaptation |
|---|---------|---------|----------------|--------------------|
| 12.1 | File Upload | **ADAPT** | HTTP upload route, 10MB limit | Tauri file dialog (`tauri-plugin-dialog`). Files copied to workspace directory. Size limit enforced in Rust. |
| 12.2 | File Upload Limit Check | **ADAPT** | Daily limit API | Track daily upload count in SQLite `AppSettings`. Reset via Rust background task. |
| 12.3 | Attachment Management | **ADAPT** | Convex create/link | SQLite `Attachment` table with chat_id FK. Files on local disk. |
| 12.4 | File Validation | **KEEP** | None (logic) | MIME type and size validation in Rust before file copy. |
| 12.5 | Signed URLs | **DROP** | Convex signed URLs | Not needed. Local files accessed via Tauri asset protocol or direct path. |
| 12.6 | Attachment Security Scan | **DEFER** | Scanning API | Phase 2: Basic MIME validation + file extension check. No server-side scanning. |
| 12.7 | Document Extraction | **ADAPT** | Server-side extraction | Rust crates: `pdf-extract`, `docx-rs` for text extraction. Runs locally. |
| 12.8 | OCR | **ADAPT** | Convex OCR action | Rust: `tesseract` crate or bundle Tesseract binary. Local OCR. |
| 12.9 | Attachment Display | **KEEP** | None (frontend) | Svelte component with grid/inline/list variants. |
| 12.10 | Generated Files Store | **ADAPT** | Zustand store | Svelte `$state` module. Files tracked in SQLite + local disk. |
| 12.11 | File Downloads | **ADAPT** | Client-side download | Tauri `save` dialog → Rust copies file to user-chosen path. |
| 12.12 | File Icons | **KEEP** | None (frontend) | Svelte component with MIME-based icon mapping. |

**Summary**: 4 KEEP, 6 ADAPT, 1 DROP, 1 DEFER. File upload → Tauri file dialog. Signed URLs eliminated. Document extraction/OCR move to Rust crates.

**Complexity**: §12.7 Document Extraction is **H** (PDF/DOCX parsing in Rust). §12.8 OCR is **H** (Tesseract integration). §12.1 File Upload is **M** (Tauri dialog + copy). Others are **L-M**.
**Effort**: **M** — Basic file handling is quick. Document extraction and OCR are the complex items. ~5 days total.
**Rust Crates**: `tauri-plugin-dialog = "2.2"` (file picker/save), `pdf-extract = "0.7"` (PDF text), `docx-rs = "0.4"` (DOCX text), `tesseract = "0.15"` (OCR, optional)
**Risk**: Medium. Document extraction quality varies by crate. PDF extraction from scanned PDFs (image-based) requires OCR fallback. Mitigation: detect image-only PDFs and route to Tesseract.

---

## 13. MCP Integration (22 features)

| # | Feature | Verdict | Web Dependency | Desktop Adaptation |
|---|---------|---------|----------------|--------------------|
| 13.1 | MCP Server CRUD | **ADAPT** | Convex + API routes | IPC commands → SQLite `McpServer` table. Config stored as JSON. |
| 13.2 | MCP Server Listing | **ADAPT** | API route | IPC `mcp_list_servers` → SQLite SELECT. |
| 13.3 | MCP Server Test | **ADAPT** | API test route | Rust `McpClientRouter` spawns stdio process or pings HTTP endpoint. Returns success/failure. |
| 13.4 | MCP Tool Loading | **ADAPT** | Server-side loaders | Rust `McpClientRouter`: spawn stdio child process → JSON-RPC `tools/list` → parse tool definitions. |
| 13.5 | MCP Runtime | **ADAPT** | Server-side execution | Rust sends `tools/call` JSON-RPC to MCP server via stdio or HTTP. Returns result to Pi SDK. |
| 13.6 | MCP Intent Detection | **ADAPT** | Server-side intent | Simplified: Pi SDK selects MCP tools based on tool descriptions. No separate intent classifier. |
| 13.7 | MCP Cache | **ADAPT** | Server-side cache | Rust caches tool definitions in memory. Invalidated on MCP server config change. |
| 13.8 | MCP Config | **KEEP** | None | Store in SQLite `McpServer.config_json`. |
| 13.9 | MCP Connector | **ADAPT** | TypeScript connector | Rust `McpClientRouter` handles stdio and HTTP transports. |
| 13.10 | MCP Tool Filters | **KEEP** | None (frontend) | Svelte filter UI over IPC-loaded tool list. |
| 13.11 | MCP Mention Filter | **KEEP** | None (frontend) | Svelte `@mention` parser → filters available MCP tools. |
| 13.12 | MCP Budget | **ADAPT** | Server-side budget | Rust tracks tool invocation count per conversation. Enforces configurable limit. |
| 13.13 | MCP URL Safety | **ADAPT** | Server-side validation | Rust validates MCP server URLs: allow `file://`, `http://localhost`, block private IPs for HTTP transport. |
| 13.14 | MCP UI Helpers | **KEEP** | None (frontend) | Svelte components for MCP tool results rendering. |
| 13.15 | MCP Connector Store | **ADAPT** | Zustand store | Svelte `$state` module. Per-chat MCP server selection (max 4). |
| 13.16 | MCP Error Store | **ADAPT** | Zustand store | Svelte `$state` module for connection error state. |
| 13.17 | MCP Error Dialog | **KEEP** | None (frontend) | Svelte dialog component. |
| 13.18 | MCP Health Endpoint | **ADAPT** | HTTP health check | Rust pings MCP server health endpoint or tests stdio spawn. |
| 13.19 | MCP Resource Block | **KEEP** | None (frontend) | Svelte component for rendering MCP UI resources in chat. |
| 13.20 | Widget MCP Data | **ADAPT** | Hooks | Svelte reactive store subscribing to Tauri events for MCP data updates. |
| 13.21 | MCP Runtime Routing | **ADAPT** | Server-side routing | Rust `McpClientRouter` routes tool calls to correct server by name. |
| 13.22 | MCP Sync | **ADAPT** | Server-side sync | Rust re-scans MCP servers on config change. Emits updated tool list via Tauri event. |

**Summary**: 6 KEEP, 16 ADAPT. MCP moves from TypeScript/server-side to Rust `McpClientRouter`. Phase 1: stdio transport only (local MCP servers). HTTP transport for remote MCP servers in Phase 2.

**Complexity**: §13.4 Tool Loading is **H** (MCP JSON-RPC `tools/list` protocol implementation). §13.5 Runtime is **H** (bidirectional stdio communication with timeout handling). §13.13 URL Safety is **M** (IP address validation). Most others are **M**.
**Effort**: **L** — MCP client router is a core subsystem. Stdio transport, tool caching, health checking, and budget enforcement form a cohesive module. ~2 weeks.
**Rust Crates**: `tokio::process` (stdio child process), `reqwest = "0.12"` (HTTP transport, Phase 2), `serde_json` (JSON-RPC parsing), `ipnet = "2.9"` (IP address validation for URL safety)
**Web Dependencies Eliminated**:
  - `lib/agents/mcp.ts`: TypeScript MCP client (connects remote MCP servers, loads tools) → Rust `McpClientRouter` with `tokio::process::Command` for stdio transport
  - `app/auth/api/chat/mcp-tools/` (11 TypeScript files): loaders, runtime, cache, config, connector, intent, budget, url-safety, tool-filters, mention-filter, ui-helpers → all consolidated into Rust `McpClientRouter` module + Svelte `$state` stores
  - `lib/mcp-connector-store.ts`, `lib/mcp-error-store.ts`: Zustand stores → Svelte `$state` modules
**MCP Stdio Protocol** (Phase 1):
  ```
  Spawn: Command::new(server_binary).stdin(Stdio::piped()).stdout(Stdio::piped())
  List:  stdin → { "jsonrpc": "2.0", "id": 1, "method": "tools/list", "params": {} }
  Call:  stdin → { "jsonrpc": "2.0", "id": 2, "method": "tools/call", "params": { "name": "...", "arguments": {...} } }
  ```
**Risk**: Medium-High. MCP protocol is evolving (spec version changes). Stdio transport has edge cases: server crash mid-call, stdin buffer overflow, zombie processes. Mitigation: implement circuit breaker (3 failures → Open state, 30s cooldown → HalfOpen probe) as specified in system_design.md §9.2.

---

## 14. Integration System (14 features)

| # | Feature | Verdict | Web Dependency | Desktop Adaptation |
|---|---------|---------|----------------|--------------------|
| 14.1 | OAuth Integration Flow | **DEFER** | OAuth2 flows | Phase 2: Tauri deep link handler for OAuth callbacks. Store tokens in Keychain. |
| 14.2 | Token Management | **DEFER** | API routes + Convex | Phase 2: Encrypted tokens in macOS Keychain. Refresh via Rust HTTP client. |
| 14.3 | Integration Data Fetch | **DEFER** | API proxy | Phase 2: Rust HTTP client fetches from connected services. |
| 14.4 | Integration Tasks | **DROP** | API route | Out of scope. Tasks are local (see §10). |
| 14.5 | **Composio Integration** | **ADAPT** | Composio SaaS API | **Apps registry powered by Composio.** Rust `ComposioClient` connects to Composio's app registry API for discovering and connecting to external apps (Gmail, Slack, Notion, etc.). OAuth tokens stored in macOS Keychain. Composio apps surfaced in Agent tab → Apps panel. Custom MCP connectors (stdio, HTTP, SSE) visually separated from Composio-managed apps. See §14A for Agent tab details. |
| 14.6 | Integration Store | **DEFER** | Convex encrypted store | Phase 2: Keychain-backed token storage. |
| 14.7 | Integration Disconnect | **DEFER** | Convex mutation | Phase 2: IPC command to remove Keychain token. |
| 14.8 | Tool Definitions (Google) | **DROP** | Google Workspace APIs | Cloud-dependent. Replace with local equivalents (calendar: CalDAV, email: IMAP). |
| 14.9 | Native Browser Tool | **ADAPT** | Server-side browser | Rust `BrowserClient` via CDP. Agent tool: `browser_navigate`, `browser_screenshot`, `browser_extract`. |
| 14.10 | Tool Registry | **ADAPT** | TypeScript registry | Rust enum of all available tools (native + MCP). Tool definitions serialized for Pi SDK. |
| 14.11 | Tool Adapter | **ADAPT** | TypeScript adapter | Rust adapter: convert internal tool schema → Pi SDK tool format. |
| 14.12 | Tool Execution Service | **ADAPT** | Server-side orchestration | Rust dispatches tool calls from Pi SDK → native function or MCP router. |
| 14.13 | Tool Approval Config | **KEEP** | None | Svelte confirmation dialog for dangerous tools (shell exec, file delete). Config in AppSettings. |
| 14.14 | Sanitize Toolset | **ADAPT** | Server-side sanitization | Rust validates tool inputs/outputs before passing to Pi SDK. |

**Summary**: 1 KEEP, 7 ADAPT, 2 DROP, 4 DEFER. OAuth/cloud integrations deferred to Phase 2. Google Workspace dropped (cloud-only). **Composio integration adapted** for Apps registry. Browser tool and tool registry move to Rust.

**Complexity**: §14.10 Tool Registry is **H** (unified registry for native + MCP tools). §14.12 Execution Service is **H** (dispatch, timeout, error handling). §14.9 Browser Tool is **M** (delegates to §16 BrowserClient). DEFER items are Phase 2.
**Effort**: **M** — Tool registry and execution service are tightly coupled with §4 (Agent) and §13 (MCP). ~5 days for Phase 1 items.
**Risk**: Medium. Tool execution safety is critical — shell commands and file operations must be sandboxed. Mitigation: tool approval config (§14.13) requires user confirmation for dangerous operations.

---

## 14A. Agent Tab — Skills, Apps & Marketplace (10 features)

> **Concept**: Top-level "Agent" tab in the main navigation. Three sub-surfaces: **Skills** (user-installable agent capabilities), **Apps** (external service integrations powered by Composio registry + custom MCP connectors), and **Marketplace** (discovery and installation surface). The Agent tab is where users extend the app’s capabilities.

| # | Feature | Verdict | Web Dependency | Desktop Adaptation |
|---|---------|---------|----------------|--------------------|
| 14A.1 | **Skills Listing** | **ADAPT** | Server-side skill registry | Rust `SkillManager` loads `UserSkill` from SQLite + disk (`~/.dexter/skills/`). Each skill has a JSON manifest: name, description, trigger pattern, script path. Svelte skill cards with enable/disable toggles. |
| 14A.2 | **Skill Installation** | **ADAPT** | None (new) | Install from Marketplace or local file. Skills are sandboxed scripts (JS/Python) with capability declarations. Installed to `~/.dexter/skills/{skill_id}/`. IPC: `skill_install`, `skill_uninstall`, `skill_list`. |
| 14A.3 | **Skill Sandbox** | **ADAPT** | Server-side execution | Phase 1: WKWebView iframe sandbox. Phase 2: Wasmtime WASM runtime. Skills declare capabilities (file_read, network, clipboard) in manifest. Runtime enforces capability restrictions. |
| 14A.4 | **Apps — Composio Registry** | **ADAPT** | Composio SaaS API | Rust `ComposioClient` fetches app catalog from Composio registry. Svelte app browser shows available integrations (Gmail, Slack, GitHub, Notion, etc.). OAuth2 flow via Tauri deep link handler. Connected apps shown with green status indicator. |
| 14A.5 | **Apps — Custom MCP Connectors** | **ADAPT** | None (new) | Users can add custom MCP servers with three transport types: **stdio** (local binary), **HTTP** (REST endpoint), **SSE** (Server-Sent Events stream). Config: name, transport type, command/URL, environment variables. **Visually separated** from Composio apps with a "Custom Connectors" section header and distinct card style. |
| 14A.6 | **Apps — Connection Management** | **ADAPT** | None (new) | IPC: `app_connect`, `app_disconnect`, `app_list_connected`, `app_list_available`. OAuth tokens in macOS Keychain. Custom MCP connector health check (ping/pong). Auto-reconnect on app launch. |
| 14A.7 | **Marketplace — Discovery** | **ADAPT** | None (new) | Svelte marketplace panel with search, categories (Productivity, Dev Tools, Data, Media), and featured/trending sections. Fetches curated catalog from static JSON endpoint (can be offline-cached). Shows both Skills and Apps. |
| 14A.8 | **Marketplace — Install/Download** | **ADAPT** | None (new) | One-click install for Skills (download to `~/.dexter/skills/`). One-click connect for Apps (initiate OAuth flow). Progress indicator for downloads. IPC: `marketplace_search`, `marketplace_install`. |
| 14A.9 | **Marketplace — Curated Registry** | **ADAPT** | None (new) | Bundled `marketplace_catalog.json` with vetted skills and apps. Periodic sync with remote registry when online. Offline fallback to bundled catalog. Categories, descriptions, screenshots, compatibility info. |
| 14A.10 | **Agent Tab Navigation** | **KEEP** | None (frontend) | Svelte tab component: Skills \| Apps \| Marketplace. Badge counts on each tab (installed skills, connected apps, available updates). Search bar for filtering. |

**Summary**: 1 KEEP, 9 ADAPT. The Agent tab is a **net-new surface** central to the app’s extensibility story. Composio powers the Apps registry; custom MCP connectors provide escape-hatch integration; Skills enable user-authored agent capabilities.

**Complexity**: §14A.4 Composio Registry is **H** (OAuth2 flows, token refresh, app catalog sync). §14A.5 Custom MCP Connectors is **H** (three transport types, health checks, visual separation). §14A.3 Skill Sandbox is **H** (iframe→Wasmtime migration). §14A.7 Marketplace is **M** (catalog UI + search).
**Effort**: **L** — ~2 weeks for full Agent tab implementation. Composio client, MCP connector management, skill sandbox, and marketplace UI.
**Rust Crates**: `reqwest` (Composio HTTP client), `oauth2` (OAuth2 flow), `tokio` (async MCP connector management)
**Data Flow**:
```
Agent Tab
  ├─ Skills Panel
  │   ├─ List installed skills (SQLite UserSkill + disk scan)
  │   ├─ Enable/disable per skill
  │   └─ Skill capability viewer
  ├─ Apps Panel
  │   ├─ Composio Apps (connected + available)
  │   │   └─ OAuth2 connect/disconnect via Tauri deep link
  │   └─ Custom MCP Connectors [visually separated]
  │       ├─ Add connector (stdio/HTTP/SSE)
  │       ├─ Health check + status indicator
  │       └─ Edit/delete connector config
  └─ Marketplace Panel
      ├─ Search + category filter
      ├─ Featured/trending carousel
      └─ One-click install/connect
```
**Risk**: Medium. Composio API stability is a third-party dependency. Mitigation: cache app catalog locally, graceful degradation when Composio is offline (show cached catalog + custom connectors only).

---

## 15. Cloud Integrations (9 features)

| # | Feature | Verdict | Web Dependency | Desktop Adaptation |
|---|---------|---------|----------------|--------------------|
| 15.1 | Google Cloud Auth | **DROP** | Google OAuth2 | Entirely cloud-dependent. Not in local-first scope. |
| 15.2 | Google Drive | **DROP** | Google Drive API | User can drag-drop files from Finder. iCloud/Dropbox folder sync works at OS level. |
| 15.3 | Gmail | **DROP** | Gmail API | Out of scope. |
| 15.4 | Google Notes | **DROP** | Google API | Out of scope. |
| 15.5 | Microsoft Cloud Auth | **DROP** | Microsoft OAuth2 | Entirely cloud-dependent. |
| 15.6 | OneDrive | **DROP** | OneDrive API | OS-level folder sync covers this. |
| 15.7 | Cloud Callback Handler | **DROP** | HTTP callback route | Not applicable. |
| 15.8 | Cloud Integrations Store | **DROP** | Zustand store | No cloud integrations in Phase 1. |
| 15.9 | Cloud Auth Callback Hook | **DROP** | React hook | Not applicable. |

**Summary**: 0 KEEP, 0 ADAPT, 9 DROP. **Entire category dropped.** Cloud integrations are fundamentally incompatible with local-first philosophy. Users can leverage OS-level file sync (iCloud, Dropbox) for cross-device file access.

---

## 16. Browser Automation (8 features)

| # | Feature | Verdict | Web Dependency | Desktop Adaptation |
|---|---------|---------|----------------|--------------------|
| 16.1 | Browser Session | **ADAPT** | API route | Rust `BrowserClient` manages **bundled Chromium** instance via CDP. App ships with a pinned Chromium binary (~150MB) in `Resources/chromium/`. No dependency on user's Chrome. Session = CDP connection lifetime. Auto-launches Chromium with `--headless=new --remote-debugging-port={dynamic}`. |
| 16.2 | Browser Streaming | **ADAPT** | HTTP stream | Replace with Tauri events: `browser_screenshot`, `browser_navigation`, `browser_dom_update`. |
| 16.3 | Browser Task Execution | **ADAPT** | API route | Agent tool calls `browser_navigate`, `browser_click`, `browser_extract` → Rust CDP commands. |
| 16.4 | Browser Store | **ADAPT** | Zustand store | Svelte `$state` module for tabs, sessions, screenshots. |
| 16.5 | Browser Use Integration | **ADAPT** | Python browser-use library | Replace with Rust `chromiumoxide` CDP operations against bundled Chromium. agent-browser framework for high-level automation (navigate, click, fill, extract). |
| 16.6 | Agent Browser Support | **ADAPT** | Tool loader integration | Rust registers browser tools with Pi SDK. Tool definitions: navigate, click, type, screenshot, extract. |
| 16.7 | Browser Tool Definitions | **ADAPT** | AI SDK tool definitions | Rust defines browser tools as JSON schema for Pi SDK. |
| 16.8 | Share Target | **DROP** | HTTP share endpoint | macOS share extension (Phase 2) or drag-drop into app. |

**Summary**: 0 KEEP, 7 ADAPT, 1 DROP. Browser automation moves from API-driven to CDP-driven against **bundled Chromium** (not user's browser). `chromiumoxide` + `agent-browser` replace all browser API routes.

**Complexity**: §16.1 Session management is **H** (bundled Chromium lifecycle, CDP WebSocket, auto-port selection). §16.5 Browser Use replacement is **H** (agent-browser integration, click/type/screenshot via CDP). §16.6-16.7 Tool definitions are **M**.
**Effort**: **L** — Bundled Chromium management, CDP integration, agent-browser framework. ~2 weeks.
**Rust Crates**: `chromiumoxide = "0.8.0"` (CDP client), `agent-browser` (high-level automation framework)
**Risk**: Medium. Bundled Chromium adds ~150MB to app size. Mitigation: lazy-download Chromium on first use (similar to AI Hub model downloads), cache in `~/Library/Application Support/Dexter/chromium/`. Use dynamic port allocation to avoid port conflicts.

---

## 17. Admin Console (13 features)

| # | Feature | Verdict | Web Dependency | Desktop Adaptation |
|---|---------|---------|----------------|--------------------|
| 17.1 | Admin Dashboard | **ADAPT** | Page redirect | Svelte settings screen (single-user "admin" = power user settings). |
| 17.2 | Feature Flags | **ADAPT** | Convex + lib | SQLite `AppSettings` toggles. Svelte switches in settings screen. |
| 17.3 | Model Manager | **ADAPT** | Admin panel + Convex | Svelte model management screen → IPC → SQLite model config updates. |
| 17.4 | Provider Manager | **ADAPT** | Admin panel | Svelte provider config screen → IPC → SQLite. API keys in Keychain. |
| 17.5 | Resource Management | **DROP** | OpenWebUI resources | OpenWebUI not part of desktop architecture. |
| 17.6 | Integrations Tab | **DEFER** | System integrations | Phase 2: MCP server management tab. |
| 17.7 | Pipelines Tab | **DROP** | OpenWebUI pipelines | Not applicable. |
| 17.8 | Functions Tab | **DROP** | OpenWebUI functions | Not applicable. |
| 17.9 | Admin Console API | **ADAPT** | API route | IPC commands for system stats: model count, disk usage, sidecar status. |
| 17.10 | App Config | **ADAPT** | Convex appConfig | SQLite `AppSettings` table. System prompt, feature flags stored locally. |
| 17.11 | OpenWebUI Health | **DROP** | HTTP health check | OpenWebUI not in desktop. Replaced by sidecar health checks. |
| 17.12 | OpenWebUI Resource CRUD | **DROP** | HTTP CRUD | Not applicable. |
| 17.13 | Connector Diagnostics | **ADAPT** | Admin cards | Svelte diagnostic cards showing MCP server status, sidecar health, DB size. |

**Summary**: 0 KEEP, 7 ADAPT, 5 DROP, 1 DEFER. OpenWebUI entirely removed. Admin console becomes power-user settings screen.

**Complexity**: All ADAPTs are **M** (settings screen with IPC commands). No high-risk items.
**Effort**: **S** — Admin panel is a settings screen variant reusing §20 settings infrastructure. ~2 days.
**Risk**: Low. The "admin" concept collapses to "power user settings" for a single-user desktop app.

---

## 18. OpenWebUI Runtime (10 features)

| # | Feature | Verdict | Web Dependency | Desktop Adaptation |
|---|---------|---------|----------------|--------------------|
| 18.1 | Plugin Sandbox | **ADAPT** | FastAPI sidecar | Replaced by `SkillSandbox`: Phase 1 WKWebView iframe, Phase 2 Wasmtime. |
| 18.2 | Tool Execution | **ADAPT** | Python execute_tool | Rust native tool execution + MCP router. |
| 18.3 | Function Execution | **DROP** | Python execute_function | Pi SDK handles function orchestration. No separate function runtime. |
| 18.4 | Pipeline Execution | **DROP** | HTTP pipeline proxy | Not applicable. Pi SDK agent loop handles multi-step workflows. |
| 18.5 | Plugin Introspection | **DROP** | Python class detection | Skills defined with explicit manifest (JSON). No runtime intros needed. |
| 18.6 | Plugin Installation | **ADAPT** | URL fetch with security | IPC `skill_install` → Rust downloads skill script to `Application Support/AskDexter/skills/`. Validates HTTPS, size limit. |
| 18.7 | Resource Storage | **ADAPT** | File-based storage | Skills stored on local disk in `Application Support/AskDexter/skills/` directory. |
| 18.8 | Skills Runtime | **ADAPT** | Server-side execution | Rust `SkillSandbox` executes skills in iframe/Wasmtime with capability restrictions. |
| 18.9 | OpenWebUI Tools | **DROP** | Server-side adaptation | MCP tools replace OpenWebUI tool format. |
| 18.10 | OpenWebUI Filters | **DROP** | Server-side filters | Not applicable. Post-processing handled by Pi SDK agent loop. |

**Summary**: 0 KEEP, 4 ADAPT, 6 DROP. OpenWebUI runtime entirely replaced by SkillSandbox + native Rust tool execution + Pi SDK orchestration.

**Complexity**: §18.1 Plugin Sandbox is **H** (WKWebView iframe with CSP lockdown, postMessage bridge). §18.8 Skills Runtime is **H** (capability-restricted execution). §18.6 Installation is **M**.
**Effort**: **M** — Phase 1 iframe sandbox. Phase 2 Wasmtime migration is separate. ~5 days for Phase 1.
**Rust Crates**: `wasmtime = "45.0.1"` (Phase 2), WKWebView iframe via Tauri webview API (Phase 1)
**Risk**: Medium-High. Skill execution safety is paramount. A misconfigured CSP could allow DOM access from user scripts. Mitigation: strict CSP (`script-src 'self'`), no `eval()`, `postMessage`-only bridge, audit all permission grants.

---

## 19. Search System (6 features)

| # | Feature | Verdict | Web Dependency | Desktop Adaptation |
|---|---------|---------|----------------|--------------------|
| 19.1 | Unified Search | **ADAPT** | API route | IPC `search_unified` → Rust queries SQLite FTS5 (conversations, files, memories) + LanceDB (knowledge chunks) in parallel. |
| 19.2 | Image Search | **DROP** | External image search API | Out of scope for local-first. |
| 19.3 | **Web Search Tool** | **ADAPT** | External provider listing | **Built-in web search as agent tool.** Rust `WebSearchService` with pluggable provider backends. Out-of-the-box: local SearXNG instance (bundled sidecar) for zero-config privacy-first search. User-selectable providers: SearXNG (default), Brave Search API, Tavily API, DuckDuckGo HTML scrape. Agent tool: `web_search(query, provider?)` → returns ranked results with snippets. Provider config in AppSettings. SearXNG sidecar: `docker run -p 8888:8080 searxng/searxng` or bundled binary. |
| 19.4 | Search Store | **ADAPT** | Zustand store | Svelte `$state` module for search panel state. |
| 19.5 | Search Indexing | **ADAPT** | 15+ search lib files | SQLite FTS5 virtual table for full-text search. Auto-indexed on INSERT/UPDATE. |
| 19.6 | Library Search | **ADAPT** | API search endpoint | IPC `kb_search` → LanceDB + SQLite FTS5 (merged with §7.11). |
| 19.7 | **Web Search Provider Registry** | **ADAPT** | None (new) | Rust `WebSearchProvider` enum: `SearXng`, `BraveSearch`, `Tavily`, `DuckDuckGo`. Each implements `trait WebSearch { async fn search(&self, query: &str) -> Result<Vec<SearchResult>>; }`. API keys stored in Keychain. Default SearXNG requires no key. |

**Summary**: 0 KEEP, 5 ADAPT, 2 DROP. Search moves from external APIs + custom indexing to SQLite FTS5 + LanceDB vector search + **built-in web search tool** with pluggable providers (SearXNG default).

**Complexity**: §19.5 Search Indexing is **H** (FTS5 virtual table design, auto-indexing triggers, multi-entity indexing). §19.1 Unified Search is **M** (parallel query across FTS5 + LanceDB + SQLite). Others are **L-M**.
**Effort**: **M** — FTS5 setup, indexing triggers, and unified search query. ~5 days.
**Rust Crates**: `rusqlite` with FTS5 feature (included in bundled build), `lancedb` (vector component of unified search)
**Risk**: Medium. FTS5 tokenizer choice affects search quality. `porter unicode61` is a good default but may need tuning for CJK content. Mitigation: implement tokenizer as configurable in AppSettings.

---

## 20. Settings & Preferences (10 features)

| # | Feature | Verdict | Web Dependency | Desktop Adaptation |
|---|---------|---------|----------------|--------------------|
| 20.1 | User Preferences API | **ADAPT** | API routes + Convex | IPC commands `settings_get`, `settings_set` → SQLite `AppSettings` table. |
| 20.2 | UI Settings Sync | **ADAPT** | Zustand ↔ server sync | Svelte `$state` modules read from SQLite on app launch. Changes write back via IPC. No server sync. |
| 20.3 | Theme System | **KEEP** | None (CSS) | 11 themes as CSS custom properties on `[data-theme]` attribute. Pure frontend. |
| 20.4 | Font Size Settings | **KEEP** | None (CSS) | 7 levels via CSS `font-size` custom property. Stored in AppSettings. |
| 20.5 | Dark/Light Mode | **KEEP** | None (CSS) | `[data-theme="dark"]` / `[data-theme="light"]` toggle. Persisted in AppSettings. |
| 20.6 | Layout Customization | **KEEP** | None (CSS) | Sidebar visibility, panel sizes in Svelte state + CSS Grid. |
| 20.7 | Sidebar Configuration | **KEEP** | None (frontend) | Svelte sidebar component with configurable icons and width. |
| 20.8 | Settings Page | **KEEP** | None (frontend) | Svelte settings screen with tabs: General, Models, Providers, Appearance, MCP, Skills. |
| 20.9 | Created Settings Store | **ADAPT** | Zustand store | Svelte `$state` module for creation preferences. |
| 20.10 | Settings Store | **ADAPT** | Zustand store | Svelte `$state` module backed by SQLite AppSettings. |

**Summary**: 6 KEEP, 4 ADAPT. Minimal web dependency. Zustand → Svelte stores. Server sync eliminated.

---

## 21. Marketing & Landing (8 features)

| # | Feature | Verdict | Web Dependency | Desktop Adaptation |
|---|---------|---------|----------------|--------------------|
| 21.1 | Landing Page | **DROP** | Web page | Not applicable for desktop app. |
| 21.2 | Features Page | **DROP** | Web page | Not applicable. |
| 21.3 | Pricing Page | **DROP** | Web page | Not applicable. |
| 21.4 | Pricing UI | **DROP** | Web component | Not applicable. |
| 21.5 | Feature Carousel | **DROP** | Web component | Not applicable. |
| 21.6 | Marketing Variants | **DROP** | Admin config | Not applicable. |
| 21.7 | OpenGraph | **DROP** | OG image | Not applicable. |
| 21.8 | Pi-Inspired Landing | **DROP** | Web page | Not applicable. |

**Summary**: 0 KEEP, 0 ADAPT, 8 DROP. **Entire category dropped.** Marketing pages have no place in a desktop application.

---

## 22. UI Component Library (14 families)

| # | Family | Verdict | Web Dependency | Desktop Adaptation |
|---|--------|---------|----------------|--------------------|
| 22.1 | Layout | **ADAPT** | React components | Port to Svelte 5: Sidebar, ScrollArea, Separator, Sheet, Drawer. Use 21st.dev as design reference only. |
| 22.2 | Navigation | **ADAPT** | React components | Port to Svelte 5: Tabs, DropdownMenu, Command palette (⌘K). |
| 22.3 | Overlays | **ADAPT** | React components | Port to Svelte 5: Dialog, AlertDialog, Drawer, Popover, Tooltip. Use `<dialog>` element. |
| 22.4 | Forms | **ADAPT** | React components | Port to Svelte 5: Input, Textarea, Select, Checkbox, Switch. Native `<input>` elements. |
| 22.5 | Data Display | **ADAPT** | React components | Port to Svelte 5: Table, Card, Badge, Avatar, Progress, Skeleton. |
| 22.6 | Feedback | **ADAPT** | React Sonner | Port to Svelte 5: Toast notifications, Alert, ErrorBoundary (Svelte `{#catch}`). |
| 22.7 | Interaction | **ADAPT** | React + Framer Motion | Port to Svelte 5: Accordion, Collapsible. Replace Framer Motion with Svelte `transition` directives. |
| 22.8 | Utilities | **ADAPT** | React utilities | Port: ClientOnly → `{#if mounted}`, cn() → same, CVA → `clsx`. |
| 22.9 | AI Components | **ADAPT** | React components | Port to Svelte 5: ChainOfThought, Reasoning, Sources, Tool, Plan, Task, Suggestion, Confirmation. |
| 22.10 | Input | **ADAPT** | React components | Port to Svelte 5: PromptInput (auto-resizing textarea), FileUpload (drag-drop). |
| 22.11 | Feedback | **ADAPT** | React components | Port to Svelte 5: FeedbackForm, Context (token usage display). |
| 22.12 | Animation | **ADAPT** | Framer Motion | Replace with Svelte built-in `transition`, `animate` directives + CSS animations. |
| 22.13 | Icons | **KEEP** | SVG files | SVG icons are framework-agnostic. Import as Svelte components or inline SVG. |
| 22.14 | Mobile Responsive | **DROP** | React responsive | Desktop app = single form factor. Responsive not needed. Optimize for 1280×800 minimum. |

**Summary**: 1 KEEP, 12 ADAPT, 1 DROP. All React components → Svelte 5 manual port. Framer Motion → Svelte transitions. Mobile responsive dropped (desktop-only).

**Complexity**: §22.9 AI Components is **H** (13 complex interactive components: ChainOfThought, Reasoning, Sources, Tool, Plan, Task, Suggestion, Confirmation, etc.). §22.10 PromptInput is **H** (auto-resizing textarea with drag-drop, suggestions, file chips). §22.7 Interaction is **M** (Accordion, Collapsible with animation). §22.1-22.6 are **M** (standard UI primitives).
**Effort**: **L** — ~45 Svelte components to port from React reference. Use 21st.dev for visual/behavioral reference only. Port in priority order: Layout → Forms → Overlays → Navigation → Data Display → AI Components → Animation. ~3 weeks.
**Svelte 5 Porting Patterns** (React → Svelte):
  | React Pattern | Svelte 5 Equivalent |
  |--------------|-------------------|
  | `useState(x)` | `let x = $state(initial)` |
  | `useMemo(() => f(x), [x])` | `let result = $derived(f(x))` |
  | `useEffect(() => {}, [x])` | `$effect(() => { ... })` |
  | `useRef()` | `let el; bind:this={el}` |
  | `useContext(Ctx)` | `import { getContext } from 'svelte'` |
  | `forwardRef` + `useImperativeHandle` | `export let prop; // bindable` |
  | Framer Motion `<motion.div>` | `<div transition:fly|fade|slide>` |
  | `react-resizable-panels` | `split.js` or CSS `resize` |
  | `react-sonner` (toasts) | `svelte-sonner` or custom `$state` toast queue |
  | `react-markdown` | `marked` + custom Svelte renderer |
  | `react-virtuoso` (chat scroll) | `svelte-tiny-virtual-list` or custom windowed list |
**Risk**: Medium. Component porting is mechanical but labor-intensive. The main risk is behavioral divergence: the Svelte port may look correct but handle edge cases (focus trapping, keyboard navigation, ARIA attributes) differently than the React original. Mitigation: write Playwright component tests for each critical interaction pattern.

---

## 23. Infrastructure & DevOps (16 features)

| # | Feature | Verdict | Web Dependency | Desktop Adaptation |
|---|---------|---------|----------------|--------------------|
| 23.1 | Health Check | **ADAPT** | HTTP endpoint | Rust function returning system status: DB size, sidecar status, disk space. Exposed via IPC. |
| 23.2 | Rate Limiting | **ADAPT** | HTTP rate limit API | Rust tracks daily message count in SQLite. EntitlementService enforces tier limits. |
| 23.3 | CSP Headers | **ADAPT** | Next.js middleware | WKWebView CSP configured via Tauri `security.csp` in `tauri.conf.json`. |
| 23.4 | API Rewrites | **DROP** | Next.js rewrites | No API routes. All communication via IPC. |
| 23.5 | 404 Page | **ADAPT** | Next.js not-found | Svelte fallback route component. |
| 23.6 | Error Boundary | **ADAPT** | React error boundary | Svelte `{#catch}` block with retry. Global error handler via `window.onerror`. |
| 23.7 | Docker Build | **DROP** | Dockerfile | Tauri build via `cargo tauri build`. Produces `.dmg` for macOS. |
| 23.8 | Staging Deploy | **DROP** | Docker VPS | Tauri builds distributed via `.dmg` download or Sparkle auto-update. |
| 23.9 | Vercel Deploy | **DROP** | Vercel CI/CD | GitHub Actions → `cargo tauri build` → release artifacts. |
| 23.10 | Env Configuration | **ADAPT** | .env files | Tauri uses `tauri.conf.json` + Rust `config` crate. Secrets in macOS Keychain, not .env files. |
| 23.11 | NVM Config | **DROP** | .nvmrc | Rust toolchain managed via `rustup`. Node.js bundled as sidecar. |
| 23.12 | Telemetry | **ADAPT** | Convex + API | Local telemetry: SQLite table for usage metrics. Optional opt-in anonymous telemetry via HTTP POST. |
| 23.13 | PWA Support | **DROP** | PWA install hook | Native desktop app — no PWA needed. |
| 23.14 | Desktop Bridge | **ADAPT** | Electron IPC | Already using Tauri IPC. `window.__TAURI__.invoke()` replaces `window.dexterDesktop`. |
| 23.15 | Convex Deployment | **DROP** | Convex config | No Convex in desktop. |
| 23.16 | Security Scanning | **ADAPT** | Server-side scanning | Rust MIME validation + file extension whitelist. No server-side scanning service. |

**Summary**: 0 KEEP, 7 ADAPT, 8 DROP, 0 DEFER. Docker/Vercel/Convex infrastructure entirely eliminated. Replaced by Tauri build pipeline, SQLite, macOS Keychain, and local config.

---

## 24. Small / Utility Features (25 features)

| # | Feature | Verdict | Web Dependency | Desktop Adaptation |
|---|---------|---------|----------------|--------------------|
| 24.1 | Keyboard Shortcuts | **KEEP** | None | Svelte action or global event listener. Desktop apps benefit from extensive shortcuts. |
| 24.2 | Click Outside Detection | **KEEP** | None | Svelte action `use:clickOutside`. |
| 24.3 | Debounce Utility | **KEEP** | None | Same TypeScript debounce function. |
| 24.4 | Breakpoint Detection | **DROP** | Responsive breakpoints | Desktop-only. Fixed minimum window size. |
| 24.5 | Mobile Detection | **DROP** | Mobile device detection | Desktop-only. |
| 24.6 | Voice Recorder | **ADAPT** | Browser MediaRecorder API | Use macOS `AVFoundation` via Tauri plugin or `tauri-plugin-global-shortcut` for push-to-talk. |
| 24.7 | Transcription API | **ADAPT** | Server-side API | Rust: use `whisper.cpp` (bundled) for local speech-to-text. Or pass to Pi SDK as audio attachment. |
| 24.8 | oEmbed | **ADAPT** | Server-side oEmbed | Rust HTTP client fetches oEmbed data. Fallback: display URL as link. |
| 24.9 | Tweet Card | **DROP** | Twitter embed | Not applicable offline. Display as quoted text. |
| 24.10 | Dynamic Suggestions | **KEEP** | None (client-side) | Svelte `$state` module with suggestion categories. |
| 24.11 | Color Palettes | **KEEP** | None (CSS) | CSS custom properties. Same tokens, Svelte-compatible. |
| 24.12 | Mermaid Pre-render | **ADAPT** | Server-side rendering | Client-side mermaid rendering in WKWebView (sufficient performance). |
| 24.13 | Markdown Table Repair | **KEEP** | None (utility) | Same TypeScript utility function. |
| 24.14 | Result Summary Block | **KEEP** | None (frontend) | Svelte component for tool result display. |
| 24.15 | Copy to Clipboard | **ADAPT** | Browser clipboard API | Tauri clipboard plugin: `@tauri-apps/plugin-clipboard-manager`. |
| 24.16 | App Icon Component | **KEEP** | None (frontend) | Svelte component with fallback icon. |
| 24.17 | Integration Widget Order | **ADAPT** | Zustand store | Svelte `$state` module. Order persisted in AppSettings. |
| 24.18 | Presentation Generator | **DEFER** | None (config) | Phase 2 feature. Generate PPTX/PDF from artifacts. |
| 24.19 | Media Generation | **ADAPT** | API-based generation | Phase 2: Use local Stable Diffusion (ComfyUI) or API keys for DALL-E. Config in AppSettings. |
| 24.20 | Visual Layout | **DEFER** | None (config) | Phase 2 feature. |
| 24.21 | Motion Primitives | **ADAPT** | Framer Motion | Svelte `transition` and `animate` directives. CSS animations for morphing effects. |
| 24.22 | Playground | **DROP** | Web teaser page | Not applicable for desktop. |
| 24.23 | Feedback Submission | **ADAPT** | Convex submit | IPC command → optional HTTP POST to feedback endpoint (opt-in). Or store locally. |
| 24.24 | Data API Proxy | **DROP** | Client-server routing | No API proxy needed. Direct IPC → Rust → data. |
| 24.25 | Auth Redirects | **DROP** | Post-login redirect | No web auth flow. |

**Summary**: 8 KEEP, 6 ADAPT, 6 DROP, 2 DEFER. Most utilities port directly. Mobile/responsive utilities dropped. Framer Motion → Svelte transitions.

---

## 25. Open Source Adaptation Guidance

> **Philosophy**: Where proven open source projects exist with compatible licenses, adapt their features rather than building from scratch. This reduces development time, leverages community-tested code, and respects upstream contributions.

### 25.1 Prismical (MIT License) — Notes & AI Transcription

| Aspect | Details |
|--------|--------|
| **Repository** | [github.com/amicalhq/prismical](https://github.com/amicalhq/prismical) |
| **License** | MIT — compatible, can adapt freely |
| **Tech Stack** | Electron / Next.js / TypeScript |
| **Adapt** | `whisper-wrapper` module: local speech-to-text pipeline using whisper.cpp binary. Adapt the recording→transcription→AI-structuring pipeline for our Work Suite Notes feature. |
| **Adapt** | Note UI patterns: voice recording button, real-time transcription overlay, AI-powered note structuring (summarize, extract action items, tag). |
| **Adapt** | Voice recording pipeline: macOS AVFoundation capture → WAV/PCM → whisper.cpp → structured text. Prismical provides the reference architecture. |
| **Do NOT Adapt** | Electron shell (we use Tauri), Next.js routing (we use Svelte), Better-Auth (single user), Ollama integration (we have AI Hub). |
| **Integration** | Adapted code becomes part of §10 Work Suite Notes subsystem. whisper.cpp binary bundled as sidecar (same pattern as llama-server). |

### 25.2 Knowledge Graph — Long-Term Memory

| Aspect | Details |
|--------|--------|
| **Concept** | Obsidian-like knowledge graph for long-term memory recall. Visualize relationships between memories, conversations, projects, and knowledge bases as an interactive graph. |
| **Candidates** | Obsidian plugins (community), `sigma.js` (graph viz library), `vis-network` (JS graph library), Neo4j (too heavy for embedded) |
| **Recommended** | `sigma.js` (MIT) for graph visualization in WKWebView. SQLite `GraphNode` + `GraphEdge` tables (already in system_design.md) for data storage. Rust builds graph from memories, conversations, and KB chunks. |
| **Integration** | Enhances §11 Memories with visual graph exploration. Agent can query graph for related memories via `graph_query(semantic_query, hops)` tool. |

### 25.3 Task Management — Kanban/List Views

| Aspect | Details |
|--------|--------|
| **Source** | Adapt patterns from Prismical task module + open source Kanban libraries |
| **Adapt** | Drag-and-drop Kanban board, priority sorting, due date management, project-scoped task lists |
| **Integration** | Part of §10 Work Suite Tasks tab. SQLite `Task` table with project association. |

### 25.4 Adaptation Rules

| Rule | Rationale |
|------|----------|
| Only adapt MIT/Apache2/BSD licensed code | License compatibility with proprietary distribution |
| Adapt logic, not UI | UI must match our Svelte 5 design system |
| Credit upstream in Settings → About | Transparency and community respect |
| Fork critical modules into `dexter-core` | Avoid dependency on upstream release cadence |
| Prefer Rust ports of adapted logic | Performance + safety guarantees over JS-sidecar bridges |

---

## Consolidated Summary

### Verdict Distribution

| Verdict | Count | Percentage |
|---------|-------|------------|
| **KEEP** | 40 | 13% |
| **ADAPT** | 171 | 55% |
| **DROP** | 82 | 26% |
| **DEFER** | 15 | 5% |
| **Categories fully dropped** | 2 (Marketing, Cloud Integrations) | — |
| **New desktop-only surfaces** | 3 (Agent Tab, AI Hub, Canvas) | — |

### Feature Reduction

| Metric | Web App | Desktop | Change |
|--------|---------|---------|--------|
| Total features | ~330 | ~210 | 36% fewer (offset by net-new surfaces) |
| Categories | 24 | 25 (+§14A Agent Tab, +§25 Open Source) | — |
| Auth features | 12 | 6 | 50% fewer |
| Infrastructure | 16 | 7 | 56% fewer |
| Cloud integrations | 9 | 0 | 100% dropped |
| Marketing | 8 | 0 | 100% dropped |
| **New desktop-only** | 0 | ~30 (AI Hub, Agent Tab, Canvas, Work Suite+) | Net additions |

### Web Dependencies Eliminated

| Web Dependency | Desktop Replacement |
|---------------|-------------------|
| Convex (real-time DB) | SQLite (rusqlite) + Tauri events |
| Next.js API routes | Tauri IPC commands |
| AI SDK v7 streamText() | Pi SDK sidecar (JSON-RPC over stdio) |
| SSE streaming | Tauri native events |
| Daytona sandbox | Local filesystem + process management |
| Google/Microsoft OAuth | Dropped (cloud-only) |
| OpenWebUI runtime | Rust native tool execution + SkillSandbox |
| Zustand state management | Svelte 5 `$state` modules |
| React component library | Svelte 5 manual port (21st.dev as design reference) |
| Framer Motion animations | Svelte transition/animate directives |
| Cloud vector search (ChromaDB) | LanceDB embedded with Metal acceleration |
| Server-side OCR/extraction | Rust crates (tesseract, pdf-extract) |
| Vercel/Docker deployment | Tauri build → .dmg installer |
| Browser MediaRecorder | macOS AVFoundation or bundled whisper.cpp |
| Signed file URLs | Tauri asset protocol + local file paths |
| Chrome CDP (user's browser) | Bundled Chromium + chromiumoxide (app-managed) |
| SearXNG / web search APIs | Bundled SearXNG sidecar (default) + pluggable providers |

### New SQLite Tables Required (beyond system_design.md)

| Table | Source Category | Purpose |
|-------|----------------|---------|
| `Note` | §10 Notes | id, title, content, is_archived, created_at, updated_at |
| `Task` | §10 Tasks | id, title, is_complete, priority, due_date, created_at |
| `Memory` | §11 Memories | id, content, category, is_pinned, is_auto_generated, source_conversation_id |
| `Project` | §6 Projects | id, name, instructions, created_at, updated_at |
| `project_knowledge_base` | §6/§7 Junction | project_id, kb_id |
| `Attachment` | §12 Files | id, conversation_id, file_path, mime_type, size_bytes, created_at |
| `Model` | §5 Models | id, provider, name, access_tier, is_enabled, metadata_json |
| `Provider` | §5 Providers | id, name, endpoint, is_enabled, display_order |
| `FtsIndex` | §19 Search | FTS5 virtual table mirroring searchable content |
| `TelemetryEvent` | §23 Infra | id, event_type, payload_json, created_at (optional, opt-in) |
| `McpConnector` | §14A Agent Tab | id, name, transport_type (stdio/http/sse), config_json, is_enabled |
| `ComposioApp` | §14A Agent Tab | id, composio_app_id, is_connected, oauth_token_ref, connected_at |
| `WebSearchProvider` | §19 Search | id, name, is_enabled, api_key_ref, config_json |

### Phase Dependency Graph

```
Phase 1: Shell & Navigation
  ├─ Svelte UI Shell (§22 all KEEP/ADAPT components)
  ├─ Sidebar, routing, keyboard shortcuts
  ├─ Settings screen (§20)
  ├─ Agent tab scaffold (§14A)
  └─ Tauri IPC scaffold

Phase 2: Core Intelligence
  ├─ Chat system (§2) + Messaging (§3)
  ├─ Pi SDK sidecar integration (§4)
  ├─ Rich media streaming + suggestion chips (§4.9)
  ├─ Model management + AI Hub (§5) + BYOK
  ├─ Knowledge base + LanceDB (§7)
  ├─ Canvas panel + artifact streaming (§8)
  ├─ Browser automation via bundled Chromium (§16)
  ├─ MCP stdio transport (§13)
  └─ File system (§12)

Phase 3: Productivity & Extensibility
  ├─ Work Suite (§10) — Email, Calendar, Notes, Tasks
  ├─ Agent tab: Skills + Apps + Marketplace (§14A)
  ├─ Composio integration (§14.5, §14A.4)
  ├─ Memories (§11) + Knowledge Graph (§25.2)
  ├─ Projects (§6) — ChatGPT/Claude-style folders
  ├─ Unified search + FTS5 + web search (§19)
  └─ Chat export/share (§2.11)

Phase 4: Advanced Features
  ├─ Skill sandbox Wasmtime migration (§18)
  ├─ MCP HTTP + SSE transport (§13)
  ├─ Voice recorder + whisper.cpp (§24)
  ├─ Prismical adaptation for Notes (§25.1)
  └─ OAuth integrations (§14 deferred items)

Phase 5: Sync & Polish
  ├─ Backup/import coordinator
  ├─ Migration service from v1 exports
  ├─ Optional: Loro CRDT sync (ADR-005 Phase 3)
  └─ Telemetry (opt-in)
```

---

## Key Findings

1. **36% feature reduction with net-new surfaces**: From ~330 web features to ~210 desktop features. Reduction comes from eliminating cloud integrations, marketing pages, multi-user auth, and server infrastructure. Offsetting this: 3 net-new desktop-only surfaces (Agent Tab, AI Hub, Canvas) add ~30 features the web app never had.

2. **Two entire categories dropped**: Cloud Integrations (9 features) and Marketing (8 features) have zero desktop equivalents.

3. **Convex is the #1 dependency to replace**: 62 features across 10 categories depend on Convex mutations/queries. All become IPC → SQLite.

4. **SSE → Tauri events is the #2 replacement**: 16 features depend on Server-Sent Events streaming. All become Tauri native event listeners.

5. **Canvas replaces Daytona sandbox with richer UX**: 20 Canvas features cover artifact preview, document editing, and resizable side panel with toggle behavior (stream to Canvas vs inline).

6. **React → Svelte port is mechanical, not architectural**: All 14 UI component families need framework porting but the interaction patterns remain identical. 21st.dev serves as design reference only.

7. **Local inference stack is net-new**: llama-server sidecar (GGUF), MLX runtime, LanceDB vector search, bundled Chromium, and whisper.cpp transcription are capabilities the web app doesn’t have. These are desktop advantages.

8. **Agent tab is the extensibility hub**: Skills, Apps (Composio + custom MCP), and Marketplace make the app a platform. Users can extend capabilities without waiting for first-party updates.

9. **No feature requires a remote server to function**: After adaptation, the application runs fully offline. Cloud model API calls and Composio are the only network-dependent operations, and BYOK + local models + custom MCP provide offline alternatives.

---

## Cross-Category Dependency Graph

Understanding how adapted features depend on each other is critical for implementation sequencing. This graph shows which ADAPT items block or enable others.

```
ENTITLEMENT (§1)
  └─► Required by: §4.1 (agent must validate tier before LLM call)
  └─► Required by: §5.8 (model access gating)
  └─► Required by: §5.12 (BYOK gating)
  └─► Required by: §23.2 (rate limiting)

DATABASE LAYER (shared infrastructure)
  └─► Required by: ALL categories with ADAPT verdict
  └─► SQLite tables: User, Conversation, Message, KnowledgeBase, KnowledgeFile,
      KnowledgeChunk, McpServer, UserSkill, GraphNode, GraphEdge, EntitlementLease,
      AppSettings, Note, Task, Memory, Project, Attachment, Model, Provider,
      McpConnector, ComposioApp, WebSearchProvider

PI SDK SIDECAR (§4)
  └─► Required by: §2.8 (auto-title generation)
  └─► Required by: §3.1 (message sending → agent loop)
  └─► Required by: §4.6 (tool loader passes tools to Pi SDK)
  └─► Required by: §4.9 (rich media streaming + suggestion chips)
  └─► Required by: §7.4 (embedding generation for KB chunking)
  └─► Required by: §11.4 (auto-generated memories)

MCP CLIENT ROUTER (§13)
  └─► Required by: §4.6 (tool loader aggregates MCP tools)
  └─► Required by: §4.8 (MCP tools adapter)
  └─► Required by: §14A.5 (custom MCP connectors)
  └─► Blocks: §14.10 (tool registry includes MCP tools)

MODEL MANAGER + AI HUB (§5)
  └─► Required by: §4.3 (model resolver)
  └─► Required by: §4.1 (local model inference via llama-server / MLX)
  └─► Blocks: §24.19 (media generation uses model manager)

BROWSER CLIENT — BUNDLED CHROMIUM (§16)
  └─► Required by: §14.9 (native browser tool)
  └─► Required by: §16.6 (agent browser support)
  └─► Independent of: Pi SDK (browser runs in Rust process)

COMPOSIO CLIENT (§14.5, §14A.4)
  └─► Required by: §14A.4 (app catalog sync)
  └─► Required by: §14A.6 (connection management)
  └─► Independent of: MCP router (Composio has its own protocol)

WEB SEARCH SERVICE (§19.3, §19.7)
  └─► Required by: §4.6 (web_search agent tool)
  └─► Independent of: FTS5 (web search is network, FTS5 is local)

LANCEDB (§7)
  └─► Required by: §7.5 (similarity search)
  └─► Required by: §7.7 (RAG context builder)
  └─► Required by: §19.1 (unified search includes vector results)
  └─► Required by: §19.6 (KB search)

SKILL SANDBOX (§18, §14A.3)
  └─► Required by: §4.11 (skills injection into agent)
  └─► Required by: §14A.2 (skill installation)
  └─► Independent of: MCP (skills are user-defined, not MCP servers)

SVELTE UI SHELL (§22)
  └─► Required by: ALL frontend-facing categories
  └─► Must be built before: §2 (chat UI), §8 (Canvas), §14A (Agent tab), §20 (settings)
  └─► Blocks: §6.10 (drag & drop), §24.1 (keyboard shortcuts)
```

### Implementation Order (Critical Path)

The dependency graph yields this strict implementation ordering:

1. **Database Layer** (SQLite schema, WAL config, migration runner) — blocks everything
2. **EntitlementService** — blocks Pi SDK and model access
3. **Svelte UI Shell** (layout, routing, design tokens) — blocks all frontend
4. **Pi SDK Sidecar** (JSON-RPC bridge, lifecycle, streaming, suggestion chips) — blocks chat, KB embedding, memories
5. **Model Manager + AI Hub** (hardware detection, llama-server, MLX, GGUF download) — blocks local inference
6. **Chat System** (IPC commands + streaming events + rich media rendering + Svelte chat UI)
7. **Canvas Panel** (resizable side panel, artifact streaming, toggle behavior) — merged §8/§9
8. **MCP Client Router** (stdio transport, tool loading, circuit breaker) — blocks tool system
9. **Tool Registry + Native Tools** (file, browser, shell tools, web_search tool)
10. **Browser Client** (bundled Chromium, CDP connection, agent-browser)
11. **Agent Tab** (Skills, Apps/Composio, Marketplace) — §14A
12. **Skill Sandbox** (iframe phase, capability restrictions)
13. **Work Suite** (Email, Calendar, Notes, Tasks) — §10
14. **Productivity** (Memories + Knowledge Graph, Projects)
15. **Search** (FTS5 indexing, unified search, web search providers)
16. **Backup/Import** (export coordinator, v1 migration service)

---

## Web Anti-Patterns Eliminated

This section catalogs web architecture patterns inherited from the Next.js/Convex stack that are actively harmful in a desktop context and must be eliminated.

### A1: Server-Side Rendering Hydration Mismatch

| Aspect | Web App | Desktop |
|--------|---------|---------|
| **Pattern** | Next.js App Router renders HTML server-side, then hydrates with React on the client | Tauri WKWebView loads a static SPA bundle. No SSR, no hydration. |
| **Harm Eliminated** | No flash of unstyled content (FOUC), no hydration mismatch errors, no `use client` vs `use server` confusion | Svelte 5 compiles to vanilla JS. First paint is instant. No server round-trip for initial render. |
| **Components Affected** | All `app/` route components lose server-side data fetching | Data loading moves to `onMount()` → IPC → SQLite |

### A2: Real-Time Database Subscription Complexity

| Aspect | Web App | Desktop |
|--------|---------|---------|
| **Pattern** | Convex real-time queries: `useQuery(api.chats.list)` auto-updates when data changes | No remote database to subscribe to. All data is local. |
| **Harm Eliminated** | No subscription management, no connection state handling, no optimistic update rollback, no Convex client initialization | Svelte `$derived` stores recompute automatically when underlying `$state` changes. Data updates via IPC → `$state` mutation → reactive re-render. |
| **Components Affected** | `convex/chats.ts` subscriptions, `lib/chat-store/session/provider.tsx` | Replaced by typed IPC calls + Svelte store modules |

### A3: API Route Layer Cake

| Aspect | Web App | Desktop |
|--------|---------|---------|
| **Pattern** | Next.js API routes (`app/api/`, `app/auth/api/`) with request/response handling, middleware, CORS | No HTTP server. No routes. No middleware. |
| **Harm Eliminated** | No route handlers to maintain, no CORS configuration, no request parsing, no response serialization, no API versioning | Tauri IPC: typed function calls with serde serialization. Zero network overhead. Sub-millisecond latency. |
| **Routes Eliminated** | ~45 Next.js API route files across `app/api/`, `app/auth/api/`, `app/api/auth/`, `app/api/cloud/`, `app/api/integrations/` | Consolidated into ~55 IPC commands in 11 domains (system_design.md §5) |

### A4: Client-Side State Synchronization

| Aspect | Web App | Desktop |
|--------|---------|---------|
| **Pattern** | Zustand stores sync with server: optimistic updates → server confirmation → rollback on failure | No server to sync with. State is authoritative. |
| **Harm Eliminated** | No sync conflicts, no offline detection, no retry queues, no reconciliation logic | Svelte `$state` is the single source of truth. Writes go to SQLite via IPC. Read-back confirms success. |
| **Stores Eliminated** | `lib/chat-store/`, `lib/workspace-store.ts`, `lib/artifact-store.ts`, `lib/browser-store.ts`, `lib/model-store/` (all Zustand) | Replaced by ~5 Svelte `$state` modules (frontend_design.md §9) |

### A5: Multi-Tenant Authorization

| Aspect | Web App | Desktop |
|--------|---------|---------|
| **Pattern** | Role-based access (anonymous, user, admin), per-user data isolation, row-level security | Single user. Single database. No isolation needed. |
| **Harm Eliminated** | No auth middleware, no user-scoped queries, no permission checks per operation, no admin email gating | `EntitlementService` validates tier (Free/Paid/One-Time) once at startup. All subsequent operations are unguarded. |
| **Convex Tables Simplified** | `users.role`, `users.email`, admin checks in `convex/admin.ts` | `User` table has 1 row. `EntitlementLease` table validates tier. |

### A6: Web-Specific Security Infrastructure

| Aspect | Web App | Desktop |
|--------|---------|---------|
| **Pattern** | CSRF tokens, CSP headers via middleware, signed URLs for file access, rate limiting per IP | Desktop has none of these attack vectors. |
| **Harm Eliminated** | No CSRF protection needed (no cross-site requests), no signed URLs (local file access), no per-IP rate limiting | WKWebView CSP is configured statically in `tauri.conf.json`. File access is direct. Rate limiting is per-entitlement-tier, not per-IP. |
| **Middleware Eliminated** | `middleware.ts` (auth redirect, CSRF injection, route protection), `lib/csrf.ts`, `lib/fetch.ts` | No HTTP middleware. Tauri IPC boundary validation (system_design.md §9.1). |

### A7: Cloud-Native Deployment Infrastructure

| Aspect | Web App | Desktop |
|--------|---------|---------|
| **Pattern** | Vercel auto-deploy, Docker staging, env files (.env, .env.local, .env.preview, .env.staging) | Tauri build → `.dmg` installer. No server infrastructure. |
| **Harm Eliminated** | No CI/CD pipeline for web deployment, no Docker management, no staging server, no env file proliferation | `cargo tauri build` produces signed `.dmg`. GitHub Actions for release artifacts. Secrets in macOS Keychain. |
| **Config Files Eliminated** | `Dockerfile`, `vercel.json`, `.nvmrc`, `.env*` (4 files), `scripts/` deploy scripts | `tauri.conf.json`, `Cargo.toml`, `package.json` (3 files total) |

---

## Data Migration Strategy: Web → Desktop

### v1 Web App Data Inventory

The web app stores data in Convex (real-time database) with the following tables relevant to desktop migration:

| Convex Table | Records per User (est.) | Desktop Destination | Migration Method |
|-------------|------------------------|--------------------|-----------------|
| `conversations` | 50–500 | SQLite `Conversation` | JSON export → IPC `migration_import_v1` |
| `messages` | 500–10,000 | SQLite `Message` | JSON export → batch INSERT |
| `knowledgeBases` | 5–20 | SQLite `KnowledgeBase` | JSON export → IPC |
| `knowledgeFiles` | 20–100 | SQLite `KnowledgeFile` + local copy | JSON export + file download → IPC + `fs::copy` |
| `knowledgeChunks` | 200–5,000 | SQLite `KnowledgeChunk` + LanceDB | JSON export → re-embed + INSERT |
| `projects` | 5–20 | SQLite `Project` | JSON export → IPC |
| `memories` | 10–50 | SQLite `Memory` | JSON export → IPC |
| `notes` | 10–100 | SQLite `Note` | JSON export → IPC |
| `tasks` | 10–50 | SQLite `Task` | JSON export → IPC |
| `userKeys` | 1–5 | macOS Keychain | Manual re-entry (security: never export API keys) |
| `mcpServers` | 2–10 | SQLite `McpServer` | JSON export → IPC |
| `userPreferences` | 1 | SQLite `AppSettings` | JSON export → IPC |

### Migration Flow

```
Web App → Export JSON → User downloads file → Opens in Desktop App
  → File picker dialog → IPC migration_import_v1(filePath)
  → Rust parses JSON → Validates schema version
  → For each entity type:
      1. INSERT into SQLite table
      2. For knowledge chunks: re-embed via Pi SDK → INSERT LanceDB
      3. Emit migration-progress event
  → Complete: emit migration-complete event
  → Report: { conversations: N, messages: N, knowledgeBases: N, ... }
```

### What Cannot Be Migrated

| Data | Reason | User Action Required |
|------|--------|---------------------|
| API keys (BYOK) | Security: never export secrets | Re-enter in Settings → Providers |
| OAuth tokens | Expired, server-side only | Re-authorize in Phase 2 |
| File attachments | Stored in Convex file storage | Download from web app before migration |
| Chat share links | Server-side public URLs | Not applicable in desktop |
| Composio integrations | Requires OAuth2 re-authorization via Tauri deep link | Re-authorize in Agent tab → Apps |
| OpenWebUI plugins | Server-side Python runtime | Reinstall as MCP servers, Skills, or Composio apps |

### Knowledge Chunk Re-Embedding

Knowledge chunks stored in Convex have embedding vectors from the web app's embedding model. These cannot be directly imported into LanceDB because:
1. The embedding model may differ (web used a cloud embedding API)
2. LanceDB IVF-PQ index requires vectors in its native format
3. Vector dimensionality may not match

**Strategy**: Import chunk text into SQLite. Run a background re-embedding job using the desktop's configured embedding model (Pi SDK or local). This ensures vector compatibility and allows the user to choose their embedding model.

---

## Effort Estimates by Phase

| Phase | Categories | Effort | Risk | Dependencies |
|-------|-----------|--------|------|-------------|
| **Phase 1: Shell** | §22 (UI), §20 (Settings), §1 (Auth), §14A (Agent tab scaffold) | **L** (~2 weeks) | Low | None — foundation layer |
| **Phase 2: Core** | §2 (Chat), §3 (Messages), §4 (Agent + rich media), §5 (Models + AI Hub), §7 (KB), §8 (Canvas), §12 (Files), §13 (MCP), §16 (Browser/bundled Chromium) | **XL** (~7 weeks) | **High** | Phase 1 complete |
| **Phase 3: Productivity & Extensibility** | §6 (Projects), §10 (Work Suite), §11 (Memories + KG), §14.5 (Composio), §14A (Skills/Apps/Marketplace), §19 (Search + web search) | **L** (~3 weeks) | Medium | Phase 2 DB layer |
| **Phase 4: Advanced** | §14 (Integrations), §18 (Skills), §24 (Utilities), §25 (Open source adaptation) | **L** (~3 weeks) | Medium | Phase 2 Pi SDK |
| **Phase 5: Polish** | §17 (Admin→Settings), §23 (Infra), Backup/Import | **M** (~2 weeks) | Low | All previous |
| **TOTAL** | 25 categories, ~210 features | **~17 weeks** | — | — |

### Effort Breakdown by Complexity Level

| Complexity | Feature Count | Avg Effort Each | Total |
|-----------|--------------|-----------------|-------|
| **L** (Low) | ~35 ADAPT items | 0.5 days | ~3.5 weeks |
| **M** (Medium) | ~90 ADAPT items | 2 days | ~36 weeks (parallelizable to ~7 weeks) |
| **H** (High) | ~35 ADAPT items | 5 days | ~35 weeks (parallelizable to ~7 weeks) |
| **X** (Extra-High) | ~6 ADAPT items | 10 days | ~12 weeks (parallelizable to ~3 weeks) |

With 2 developers working in parallel (1 Rust backend, 1 Svelte frontend), the **critical path is ~17 weeks**.

---

## Detailed Convex → IPC Command Mapping

### High-Traffic Commands (Called on Every User Interaction)

| Web: Convex Function | Desktop: IPC Command | Type Signature | Notes |
|---------------------|---------------------|----------------|-------|
| `chats.list` (query) | `chat_list_conversations` | `(limit: u32, offset: u32) → Vec<Conversation>` | Paginated. SQLite `ORDER BY pinned DESC, updated_at DESC` |
| `chats.create` (mutation) | `chat_create` | `(title: Option<String>, model_id: String, project_id: Option<String>) → String` | Returns conversation UUID |
| `messages.send` (mutation) | `chat_send_message` | `(conversation_id: String, content: String, model_id: String) → MessageId` | Triggers Pi SDK agent loop |
| `messages.list` (query) | `chat_get_thread` | `(conversation_id: String, branch_id: Option<String>) → Vec<Message>` | Recursive CTE for branching tree |
| `chats.togglePin` (mutation) | `chat_toggle_pin` | `(conversation_id: String) → ()` | Boolean flip in SQLite |
| `chats.remove` (mutation) | `chat_delete_conversation` | `(conversation_id: String) → ()` | CASCADE: messages, attachments, KB links |
| `knowledgeBases._searchChunks` (action) | `kb_search_rag` | `(query: String, kb_id: String, top_k: u32) → Vec<RagResult>` | LanceDB ANN + SQLite JOIN |
| `knowledgeBases.insertChunksBatch` (action) | `kb_import_file` | `(kb_id: String, file_path: String) → ImportResult` | Chunk + embed + store pipeline |
| `memories.list` (query) | `memory_list` | `(limit: u32, category: Option<String>) → Vec<Memory>` | Pinned-first ordering |
| `users.updatePreferences` (mutation) | `settings_set` | `(key: String, value: String) → ()` | AppSettings key-value store |

### Admin/Config Commands (Called Infrequently)

| Web: Convex Function | Desktop: IPC Command | Type Signature | Notes |
|---------------------|---------------------|----------------|-------|
| `models.upsertAccessTiers` | `model_update_access` | `(model_id: String, tier: String) → ()` | Settings screen only |
| `providers.upsert` | `provider_upsert` | `(id: String, config: ProviderConfig) → ()` | Admin settings |
| `mcpServers.create` | `mcp_add_server` | `(name: String, transport: String, config: JsonValue) → String` | MCP settings panel |
| `admin.getAppConfig` | `settings_get_all` | `() → HashMap<String, String>` | All app settings |

### Real-Time Subscriptions Eliminated

| Web: Convex Real-Time Query | Desktop: Replacement Pattern |
|----------------------------|------------------------------|
| `useQuery(api.messages.list, { chatId })` | IPC `chat_get_thread` on mount + Tauri `listen("agent-token")` for streaming |
| `useQuery(api.chats.list)` | IPC `chat_list_conversations` on mount. Refresh via IPC after mutations. |
| `useQuery(api.mcpServers.list)` | IPC `mcp_list_servers` on settings mount. Refresh on add/remove. |
| `useQuery(api.memories.list)` | IPC `memory_list` on memories panel mount. |
| Convex presence indicators | Not applicable (single user) |
| Convex typing indicators | Not applicable (single user) |

---

## Complete SQLite Schema Additions

Beyond the tables in `system_design.md` §4, these additional tables are required by the ADAPT verdicts:

```sql
-- §5 Model Management
CREATE TABLE IF NOT EXISTS model_catalog (
    id TEXT PRIMARY KEY,
    provider TEXT NOT NULL,
    name TEXT NOT NULL,
    access_tier TEXT NOT NULL DEFAULT 'free',  -- free, paid, one_time
    is_enabled INTEGER NOT NULL DEFAULT 1,
    context_window INTEGER,
    metadata_json TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS provider_config (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    endpoint TEXT,
    is_enabled INTEGER NOT NULL DEFAULT 1,
    display_order INTEGER NOT NULL DEFAULT 0,
    config_json TEXT
);

-- §6 Projects
CREATE TABLE IF NOT EXISTS project (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    instructions TEXT,
    display_order INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS project_knowledge_base (
    project_id TEXT NOT NULL REFERENCES project(id) ON DELETE CASCADE,
    kb_id TEXT NOT NULL REFERENCES knowledge_base(id) ON DELETE CASCADE,
    PRIMARY KEY (project_id, kb_id)
);

-- §10 Work Suite (Notes & Tasks)
CREATE TABLE IF NOT EXISTS note (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL DEFAULT 'Untitled',
    content TEXT NOT NULL DEFAULT '',
    is_archived INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS task (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    is_complete INTEGER NOT NULL DEFAULT 0,
    priority TEXT CHECK(priority IN ('low', 'medium', 'high', 'urgent')),
    due_date TEXT,
    project_id TEXT REFERENCES project(id) ON DELETE SET NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- §11 Memories
CREATE TABLE IF NOT EXISTS memory (
    id TEXT PRIMARY KEY,
    content TEXT NOT NULL,
    category TEXT,
    is_pinned INTEGER NOT NULL DEFAULT 0,
    is_auto_generated INTEGER NOT NULL DEFAULT 0,
    source_conversation_id TEXT REFERENCES conversation(id) ON DELETE SET NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- §12 File Attachments
CREATE TABLE IF NOT EXISTS attachment (
    id TEXT PRIMARY KEY,
    conversation_id TEXT NOT NULL REFERENCES conversation(id) ON DELETE CASCADE,
    file_path TEXT NOT NULL,
    mime_type TEXT NOT NULL,
    size_bytes INTEGER NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- §19 Full-Text Search
CREATE VIRTUAL TABLE IF NOT EXISTS fts_search USING fts5(
    entity_type,    -- 'conversation', 'message', 'note', 'memory', 'knowledge_chunk'
    entity_id,      -- UUID of the source entity
    content,        -- Searchable text content
    title,          -- Optional title for display
    tokenize='porter unicode61'
);

-- §23 Telemetry (opt-in)
CREATE TABLE IF NOT EXISTS telemetry_event (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    event_type TEXT NOT NULL,
    payload_json TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- §14A Agent Tab — Custom MCP Connectors
CREATE TABLE IF NOT EXISTS mcp_connector (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    transport_type TEXT NOT NULL CHECK(transport_type IN ('stdio', 'http', 'sse')),
    command TEXT,            -- for stdio: binary path + args
    url TEXT,                -- for http/sse: endpoint URL
    env_json TEXT,           -- environment variables as JSON
    is_enabled INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- §14A Agent Tab — Composio Connected Apps
CREATE TABLE IF NOT EXISTS composio_app (
    id TEXT PRIMARY KEY,
    composio_app_id TEXT NOT NULL UNIQUE,
    app_name TEXT NOT NULL,
    is_connected INTEGER NOT NULL DEFAULT 0,
    oauth_token_ref TEXT,    -- reference to Keychain item
    connected_at TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- §19 Web Search Providers
CREATE TABLE IF NOT EXISTS web_search_provider (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    provider_type TEXT NOT NULL CHECK(provider_type IN ('searxng', 'brave', 'tavily', 'duckduckgo')),
    is_enabled INTEGER NOT NULL DEFAULT 1,
    is_default INTEGER NOT NULL DEFAULT 0,
    api_key_ref TEXT,        -- Keychain reference (null for SearXNG/DuckDuckGo)
    endpoint TEXT,           -- custom SearXNG endpoint
    config_json TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Conversation.project_id FK (add to existing Conversation table)
-- ALTER TABLE conversation ADD COLUMN project_id TEXT REFERENCES project(id) ON DELETE SET NULL;
```

---

## Risk Register (Aggregated)

| ID | Risk | Category | Likelihood | Impact | Mitigation |
|----|------|----------|-----------|--------|-----------|
| R-ADAPT-1 | Pi SDK JSON-RPC interface changes breaking Rust bridge | §4 Agent | Medium | **Critical** | Pin version, contract tests, fork if needed |
| R-ADAPT-2 | LanceDB IVF-PQ index training failure on small datasets | §7 KB | High | Medium | Brute-force fallback for <256 vectors |
| R-ADAPT-3 | WKWebView rendering inconsistencies with complex CSS | §22 UI | Medium | Medium | Tailwind CSS, limit nested flex containers, WebKit prefix testing |
| R-ADAPT-4 | Bundled Chromium download failure or excessive disk usage | §16 Browser | Medium | Medium | Lazy-download on first use, cache in `~/Library/Application Support/Dexter/chromium/`, show progress + retry |
| R-ADAPT-5 | Node.js sidecar memory pressure (Pi SDK + agent loop) | §4 Agent | Medium | High | `--max-old-space-size=2048` cap, process monitoring |
| R-ADAPT-6 | SQLite WAL corruption on hard shutdown | §2 Chat, §3 Msg | Low | High | 30-minute auto-backup snapshots, `wal_autocheckpoint=1000` |
| R-ADAPT-7 | llama-server VRAM exhaustion on low-memory Macs | §5 Models | Medium | High | Pre-flight VRAM check (≥6GB), graceful degradation to CPU |
| R-ADAPT-8 | MCP server zombie process consuming resources | §13 MCP | Medium | Medium | Circuit breaker + process kill on timeout, PID tracking |
| R-ADAPT-9 | Tauri IPC serialization overhead for large payloads | §12 Files, §8 Canvas | Low | Medium | Cap IPC payload size (10KB), use file paths not inline content |
| R-ADAPT-10 | Svelte component behavioral divergence from React reference | §22 UI | Medium | Medium | Playwright component tests, accessibility audits per component |
| R-ADAPT-11 | Composio API breaking change or service outage | §14A Agent Tab | Medium | Medium | Cache app catalog locally, graceful degradation (show cached + custom connectors only) |
| R-ADAPT-12 | MLX runtime Python version incompatibility | §5 Models | Low | Medium | Bundle pinned Python runtime with mlx-lm, version-lock dependencies |
| R-ADAPT-13 | Rich media streaming rendering jank (LaTeX, Mermaid, charts) | §4 Agent | Medium | Medium | Lazy-render complex blocks, use requestAnimationFrame throttling, fallback to code blocks |
