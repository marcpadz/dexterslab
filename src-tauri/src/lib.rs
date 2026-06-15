use std::sync::Mutex;
use std::path::PathBuf;
use tauri::{State, Manager};

mod db;
mod entitlements;
mod workspace;

pub struct DbState {
    pub conn: Mutex<rusqlite::Connection>,
}

pub struct AppState {
    pub workspace_root: Mutex<Option<PathBuf>>,
}

// User commands
#[tauri::command]
fn upsert_user(user: db::User, db: State<'_, DbState>) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    db::upsert_user(&conn, &user).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_user(id: String, db: State<'_, DbState>) -> Result<Option<db::User>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    db::get_user(&conn, &id).map_err(|e| e.to_string())
}

// Conversation commands
#[tauri::command]
fn upsert_conversation(conv: db::Conversation, db: State<'_, DbState>) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    db::upsert_conversation(&conn, &conv).map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_conversation(id: String, db: State<'_, DbState>) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    db::delete_conversation(&conn, &id).map_err(|e| e.to_string())
}

#[tauri::command]
fn list_conversations(user_id: String, db: State<'_, DbState>) -> Result<Vec<db::Conversation>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    db::list_conversations(&conn, &user_id).map_err(|e| e.to_string())
}

// Message commands
#[tauri::command]
fn save_message(msg: db::Message, db: State<'_, DbState>) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    db::save_message(&conn, &msg).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_messages(conversation_id: String, db: State<'_, DbState>) -> Result<Vec<db::Message>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    db::get_messages(&conn, &conversation_id).map_err(|e| e.to_string())
}

// App settings commands
#[tauri::command]
fn set_setting(key: String, value: String, db: State<'_, DbState>) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    db::set_setting(&conn, &key, &value).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_setting(key: String, db: State<'_, DbState>) -> Result<Option<String>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    db::get_setting(&conn, &key).map_err(|e| e.to_string())
}

// GraphNode commands
#[tauri::command]
fn upsert_graph_node(node: db::GraphNode, db: State<'_, DbState>) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    db::upsert_graph_node(&conn, &node).map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_graph_node(id: String, db: State<'_, DbState>) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    db::delete_graph_node(&conn, &id).map_err(|e| e.to_string())
}

// GraphEdge commands
#[tauri::command]
fn upsert_graph_edge(edge: db::GraphEdge, db: State<'_, DbState>) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    db::upsert_graph_edge(&conn, &edge).map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_graph_edge(from_id: String, to_id: String, rel_type: String, db: State<'_, DbState>) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    db::delete_graph_edge(&conn, &from_id, &to_id, &rel_type).map_err(|e| e.to_string())
}

#[tauri::command]
fn traverse_graph(start_node_id: String, depth: i32, db: State<'_, DbState>) -> Result<Vec<db::GraphNode>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    db::traverse_graph(&conn, &start_node_id, depth).map_err(|e| e.to_string())
}

// Project commands
#[tauri::command]
fn create_project(id: String, name: String, custom_instructions: Option<String>, db: State<'_, DbState>) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    db::create_project(&conn, &id, &name, custom_instructions.as_deref()).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_project(id: String, db: State<'_, DbState>) -> Result<Option<db::Project>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    db::get_project(&conn, &id).map_err(|e| e.to_string())
}

#[tauri::command]
fn update_project(id: String, name: String, custom_instructions: Option<String>, db: State<'_, DbState>) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    db::update_project(&conn, &id, &name, custom_instructions.as_deref()).map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_project(id: String, db: State<'_, DbState>) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    db::delete_project(&conn, &id).map_err(|e| e.to_string())
}

#[tauri::command]
fn list_projects(db: State<'_, DbState>) -> Result<Vec<db::Project>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    db::list_projects(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
fn assign_conversation_to_project(conv_id: String, project_id: Option<String>, db: State<'_, DbState>) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    db::assign_conversation_to_project(&conn, &conv_id, project_id.as_deref()).map_err(|e| e.to_string())
}

#[tauri::command]
fn link_document_to_project(project_id: String, node_id: String, db: State<'_, DbState>) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    db::link_document_to_project(&conn, &project_id, &node_id).map_err(|e| e.to_string())
}

#[tauri::command]
fn unlink_document_from_project(project_id: String, node_id: String, db: State<'_, DbState>) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    db::unlink_document_from_project(&conn, &project_id, &node_id).map_err(|e| e.to_string())
}

#[tauri::command]
fn list_project_documents(project_id: String, db: State<'_, DbState>) -> Result<Vec<String>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    db::list_project_documents(&conn, &project_id).map_err(|e| e.to_string())
}
// Memory commands
#[tauri::command]
fn create_memory(id: String, content: String, category: Option<String>, is_pinned: bool, is_auto_generated: bool, source_conversation_id: Option<String>, db: State<'_, DbState>) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    db::create_memory(&conn, &id, &content, category.as_deref(), is_pinned, is_auto_generated, source_conversation_id.as_deref()).map_err(|e| e.to_string())
}

#[tauri::command]
fn update_memory(id: String, content: String, category: Option<String>, is_pinned: bool, db: State<'_, DbState>) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    db::update_memory(&conn, &id, &content, category.as_deref(), is_pinned).map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_memory(id: String, db: State<'_, DbState>) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    db::delete_memory(&conn, &id).map_err(|e| e.to_string())
}

#[tauri::command]
fn list_memories(db: State<'_, DbState>) -> Result<Vec<db::Memory>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    db::list_memories(&conn).map_err(|e| e.to_string())
}

// Workspace commands
#[tauri::command]
fn set_workspace_root(path: String, state: State<'_, AppState>) -> Result<(), String> {
    let p = PathBuf::from(path);
    if !p.exists() {
        return Err("Workspace path does not exist".into());
    }
    let mut ws = state.workspace_root.lock().map_err(|e| e.to_string())?;
    *ws = Some(std::fs::canonicalize(p).map_err(|e| e.to_string())?);
    Ok(())
}

#[tauri::command]
fn read_workspace_file(path: String, state: State<'_, AppState>) -> Result<String, String> {
    let ws_opt = state.workspace_root.lock().map_err(|e| e.to_string())?;
    let ws = ws_opt.as_ref().ok_or_else(|| "No workspace selected".to_string())?;
    workspace::read_file(ws, &path)
}

#[tauri::command]
fn write_workspace_file(path: String, contents: String, state: State<'_, AppState>) -> Result<(), String> {
    let ws_opt = state.workspace_root.lock().map_err(|e| e.to_string())?;
    let ws = ws_opt.as_ref().ok_or_else(|| "No workspace selected".to_string())?;
    workspace::write_file(ws, &path, &contents)
}

// Entitlement commands
#[tauri::command]
fn check_clock_rollback() -> Result<bool, String> {
    entitlements::check_clock_rollback()
}

#[tauri::command]
fn update_last_execution_time() -> Result<(), String> {
    entitlements::update_last_execution_time()
}

#[tauri::command]
fn validate_license(token: String) -> Result<entitlements::Claims, String> {
    entitlements::validate_license_jwt(&token)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_handle = app.handle();
            let app_dir = app_handle.path().app_local_data_dir().map_err(|e| {
                tauri::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
            })?;
            // Ensure path exists
            std::fs::create_dir_all(&app_dir).map_err(|e| tauri::Error::Io(e))?;
            let db_path = app_dir.join("askdexter.db");
            let conn = db::init_db(&db_path).map_err(|e| {
                tauri::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
            })?;

            // Initialize app settings and run entitlement clock check
            let _ = entitlements::update_last_execution_time();

            app.manage(DbState { conn: Mutex::new(conn) });
            app.manage(AppState { workspace_root: Mutex::new(None) });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            upsert_user,
            get_user,
            upsert_conversation,
            delete_conversation,
            list_conversations,
            save_message,
            get_messages,
            set_setting,
            get_setting,
            upsert_graph_node,
            delete_graph_node,
            upsert_graph_edge,
            delete_graph_edge,
            traverse_graph,
            set_workspace_root,
            read_workspace_file,
            write_workspace_file,
            check_clock_rollback,
            update_last_execution_time,
            validate_license,
            create_project,
            get_project,
            update_project,
            delete_project,
            list_projects,
            assign_conversation_to_project,
            link_document_to_project,
            unlink_document_from_project,
            list_project_documents,
            create_memory,
            update_memory,
            delete_memory,
            list_memories
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
