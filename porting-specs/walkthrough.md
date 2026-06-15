# Walkthrough: Porting Ask Dexter to macOS Native (Tauri)

We have successfully created and updated the **Technical Specification Document** for porting the **Ask Dexter** web application into a local-first, offline-capable macOS-native desktop application using Tauri.

All 10 recommendations from the Technical Due Diligence Audit have been resolved and incorporated.

## Key Revisions and Improvements Made
1. **Zustand Removal (ADR-001)**: Removed Zustand. State management is now entirely Svelte 5 reactive Runes and state modules, eliminating the incompatible React-specific dependency.
2. **Pi SDK Integration Architecture (ADR-007)**: Explicitly defined that the TypeScript Pi SDK runs inside a Node.js sidecar process spawned by Tauri, communicating with the Rust backend via JSON-RPC 2.0 over standard I/O (stdio).
3. **Database Access Consolidation (ADR-008)**: Removed `tauri-plugin-sql` and `tauri-plugin-store`. The application utilizes a single `rusqlite` database engine on the Rust side, exposing operations to the Svelte frontend via Tauri IPC Commands. Key-value preferences are handled via a dedicated `AppSettings` table.
4. **FastAPI Inference Replacement**: Removed the middleman Python FastAPI sidecar. The system connects to the model-serving sidecar (`llama-server`/Cortex.cpp) directly via localhost HTTP from the Rust backend.
5. **Knowledge Graph Technology (ADR-003)**: Demoted LadybugDB to Phase 2. Embedded property graph relationships are stored inside SQLite using standard `graph_node` and `graph_edge` tables, traversed via recursive Common Table Expressions (CTEs).
6. **Loro CRDT Sync Deferral (ADR-005)**: Deferred Loro CRDT multi-device sync to Phase 3. Phase 1 provides simple manual import and export for local database backups and restores.
7. **Tauri IPC Input Validation**: Added Section 6 specifying path canonicalization, workspace containment verification, and command input schemas to prevent path escape vulnerabilities.
8. **v1 to v2 Migration**: Added Section 7 declaring a clean-slate privacy policy, offering a manual JSON export/import tool for conversations and preferences.
9. **WKWebView Stream Optimization**: Replaced Server-Sent Events (SSE) with native Tauri Emitters (`app.emit`) to stream LLM responses from Rust to the Svelte frontend without network overhead.
10. **Unified Clock Rollback**: Specified Keychain check intervals (every launch and 30 minutes) and clarified that subscription-gated features lock upon time tampering while local inference remains active.
11. **Multi-Surface UI Architecture**: Updated navigation based on the UI-design-handoff. Migrated from a single generic sidebar to a 5-tab system (Chat, Agent, Notes, Work, Playground) with 11 contextual views, including Projects (Knowledge Graph view) and Memories (fact database).
12. **Canvas & Bottom Terminal Refactoring**: Extracted the PTY Terminal into a collapsible bottom workspace panel, removing it from the Canvas panel. The Canvas panel was simplified to a tabbed system: Artifacts List (displaying file list; clicking opens dynamic multiple Preview tabs simultaneously), Collaborative Document Editor, and WebView Browser.

## Approved Specification File
- [technical_specification.md](file:///Users/marcadrian/Workspace/Dev/dexterslab/porting-specs/technical_specification.md)

This updated spec is ready to be handed over to the implementation agent.
