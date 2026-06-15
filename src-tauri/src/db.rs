use std::path::Path;
use rusqlite::{params, Connection, Result};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub premium_tier: String,
    pub local_token_count: i64,
    pub last_sync: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: String,
    pub user_id: String,
    pub title: String,
    pub model_id: String,
    pub project_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub custom_instructions: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub conversation_id: String,
    pub parent_message_id: Option<String>,
    pub role: String,
    pub content: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct AppSetting {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memory {
    pub id: String,
    pub content: String,
    pub category: Option<String>,
    pub is_pinned: bool,
    pub is_auto_generated: bool,
    pub source_conversation_id: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphNode {
    pub id: String,
    pub r#type: String,
    pub properties_json: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphEdge {
    pub from_id: String,
    pub to_id: String,
    pub rel_type: String,
    pub properties_json: String,
}

pub fn init_db(db_path: &Path) -> Result<Connection> {
    let conn = Connection::open(db_path)?;
    
    // Enable Write-Ahead Logging (WAL) and foreign keys
    conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;

    // Create tables
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            email TEXT NOT NULL,
            premium_tier TEXT NOT NULL,
            local_token_count INTEGER NOT NULL DEFAULT 0,
            last_sync TEXT
        );",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS projects (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            custom_instructions TEXT,
            created_at TEXT DEFAULT (datetime('now'))
        );",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS conversations (
            id TEXT PRIMARY KEY,
            user_id TEXT NOT NULL,
            title TEXT NOT NULL,
            model_id TEXT NOT NULL,
            project_id TEXT REFERENCES projects(id) ON DELETE SET NULL,
            FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE
        );",
        [],
    )?;

    // Add project_id to conversations table if database was already initialized without it
    let _ = conn.execute("ALTER TABLE conversations ADD COLUMN project_id TEXT REFERENCES projects(id) ON DELETE SET NULL;", []);

    conn.execute(
        "CREATE TABLE IF NOT EXISTS messages (
            id TEXT PRIMARY KEY,
            conversation_id TEXT NOT NULL,
            parent_message_id TEXT,
            role TEXT NOT NULL,
            content TEXT NOT NULL,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            FOREIGN KEY(conversation_id) REFERENCES conversations(id) ON DELETE CASCADE
        );",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS app_settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS memory (
            id TEXT PRIMARY KEY,
            content TEXT NOT NULL,
            category TEXT,
            is_pinned INTEGER NOT NULL DEFAULT 0,
            is_auto_generated INTEGER NOT NULL DEFAULT 0,
            source_conversation_id TEXT REFERENCES conversations(id) ON DELETE SET NULL,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now'))
        );",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS graph_nodes (
            id TEXT PRIMARY KEY,
            type TEXT NOT NULL,
            properties_json TEXT NOT NULL
        );",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS graph_edges (
            from_id TEXT NOT NULL,
            to_id TEXT NOT NULL,
            rel_type TEXT NOT NULL,
            properties_json TEXT NOT NULL,
            PRIMARY KEY(from_id, to_id, rel_type),
            FOREIGN KEY(from_id) REFERENCES graph_nodes(id) ON DELETE CASCADE,
            FOREIGN KEY(to_id) REFERENCES graph_nodes(id) ON DELETE CASCADE
        );",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS project_documents (
            project_id TEXT NOT NULL,
            node_id TEXT NOT NULL,
            PRIMARY KEY (project_id, node_id),
            FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
            FOREIGN KEY (node_id) REFERENCES graph_nodes(id) ON DELETE CASCADE
        );",
        [],
    )?;

    Ok(conn)
}

// User operations
pub fn upsert_user(conn: &Connection, user: &User) -> Result<()> {
    conn.execute(
        "INSERT INTO users (id, email, premium_tier, local_token_count, last_sync)
         VALUES (?1, ?2, ?3, ?4, ?5)
         ON CONFLICT(id) DO UPDATE SET
            email = excluded.email,
            premium_tier = excluded.premium_tier,
            local_token_count = excluded.local_token_count,
            last_sync = excluded.last_sync;",
        params![user.id, user.email, user.premium_tier, user.local_token_count, user.last_sync],
    )?;
    Ok(())
}

pub fn get_user(conn: &Connection, id: &str) -> Result<Option<User>> {
    let mut stmt = conn.prepare("SELECT id, email, premium_tier, local_token_count, last_sync FROM users WHERE id = ?1;")?;
    let mut rows = stmt.query(params![id])?;
    if let Some(row) = rows.next()? {
        Ok(Some(User {
            id: row.get(0)?,
            email: row.get(1)?,
            premium_tier: row.get(2)?,
            local_token_count: row.get(3)?,
            last_sync: row.get(4)?,
        }))
    } else {
        Ok(None)
    }
}

// Conversation operations
pub fn upsert_conversation(conn: &Connection, conv: &Conversation) -> Result<()> {
    conn.execute(
        "INSERT INTO conversations (id, user_id, title, model_id, project_id)
         VALUES (?1, ?2, ?3, ?4, ?5)
         ON CONFLICT(id) DO UPDATE SET
            title = excluded.title,
            model_id = excluded.model_id,
            project_id = excluded.project_id;",
        params![conv.id, conv.user_id, conv.title, conv.model_id, conv.project_id],
    )?;
    Ok(())
}

pub fn delete_conversation(conn: &Connection, id: &str) -> Result<()> {
    conn.execute("DELETE FROM conversations WHERE id = ?1;", params![id])?;
    Ok(())
}

pub fn list_conversations(conn: &Connection, user_id: &str) -> Result<Vec<Conversation>> {
    let mut stmt = conn.prepare("SELECT id, user_id, title, model_id, project_id FROM conversations WHERE user_id = ?1;")?;
    let rows = stmt.query_map(params![user_id], |row| {
        Ok(Conversation {
            id: row.get(0)?,
            user_id: row.get(1)?,
            title: row.get(2)?,
            model_id: row.get(3)?,
            project_id: row.get(4)?,
        })
    })?;
    let mut list = Vec::new();
    for r in rows {
        list.push(r?);
    }
    Ok(list)
}

// Message operations
pub fn save_message(conn: &Connection, msg: &Message) -> Result<()> {
    conn.execute(
        "INSERT INTO messages (id, conversation_id, parent_message_id, role, content, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)
         ON CONFLICT(id) DO UPDATE SET
            content = excluded.content;",
        params![msg.id, msg.conversation_id, msg.parent_message_id, msg.role, msg.content, msg.created_at],
    )?;
    Ok(())
}

pub fn get_messages(conn: &Connection, conversation_id: &str) -> Result<Vec<Message>> {
    let mut stmt = conn.prepare(
        "SELECT id, conversation_id, parent_message_id, role, content, created_at
         FROM messages WHERE conversation_id = ?1 ORDER BY created_at ASC;"
    )?;
    let rows = stmt.query_map(params![conversation_id], |row| {
        Ok(Message {
            id: row.get(0)?,
            conversation_id: row.get(1)?,
            parent_message_id: row.get(2)?,
            role: row.get(3)?,
            content: row.get(4)?,
            created_at: row.get(5)?,
        })
    })?;
    let mut list = Vec::new();
    for r in rows {
        list.push(r?);
    }
    Ok(list)
}

// AppSettings operations
pub fn set_setting(conn: &Connection, key: &str, value: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO app_settings (key, value) VALUES (?1, ?2)
         ON CONFLICT(key) DO UPDATE SET value = excluded.value;",
        params![key, value],
    )?;
    Ok(())
}

pub fn get_setting(conn: &Connection, key: &str) -> Result<Option<String>> {
    let mut stmt = conn.prepare("SELECT value FROM app_settings WHERE key = ?1;")?;
    let mut rows = stmt.query(params![key])?;
    if let Some(row) = rows.next()? {
        Ok(Some(row.get(0)?))
    } else {
        Ok(None)
    }
}

// GraphNode operations
pub fn upsert_graph_node(conn: &Connection, node: &GraphNode) -> Result<()> {
    conn.execute(
        "INSERT INTO graph_nodes (id, type, properties_json) VALUES (?1, ?2, ?3)
         ON CONFLICT(id) DO UPDATE SET
            type = excluded.type,
            properties_json = excluded.properties_json;",
        params![node.id, node.r#type, node.properties_json],
    )?;
    Ok(())
}

pub fn delete_graph_node(conn: &Connection, id: &str) -> Result<()> {
    conn.execute("DELETE FROM graph_nodes WHERE id = ?1;", params![id])?;
    Ok(())
}

// GraphEdge operations
pub fn upsert_graph_edge(conn: &Connection, edge: &GraphEdge) -> Result<()> {
    conn.execute(
        "INSERT INTO graph_edges (from_id, to_id, rel_type, properties_json)
         VALUES (?1, ?2, ?3, ?4)
         ON CONFLICT(from_id, to_id, rel_type) DO UPDATE SET
            properties_json = excluded.properties_json;",
        params![edge.from_id, edge.to_id, edge.rel_type, edge.properties_json],
    )?;
    Ok(())
}

pub fn delete_graph_edge(conn: &Connection, from_id: &str, to_id: &str, rel_type: &str) -> Result<()> {
    conn.execute(
        "DELETE FROM graph_edges WHERE from_id = ?1 AND to_id = ?2 AND rel_type = ?3;",
        params![from_id, to_id, rel_type],
    )?;
    Ok(())
}

// Recursive Graph Traversal (up to `depth` hops)
pub fn traverse_graph(conn: &Connection, start_node_id: &str, depth: i32) -> Result<Vec<GraphNode>> {
    let mut stmt = conn.prepare(
        "WITH RECURSIVE traversal(id, depth) AS (
            SELECT ?1, 0
            UNION
            SELECT e.to_id, t.depth + 1
            FROM traversal t
            JOIN graph_edges e ON e.from_id = t.id
            WHERE t.depth < ?2
        )
        SELECT DISTINCT n.id, n.type, n.properties_json
        FROM traversal t
        JOIN graph_nodes n ON n.id = t.id;"
    )?;

    let rows = stmt.query_map(params![start_node_id, depth], |row| {
        Ok(GraphNode {
            id: row.get(0)?,
            r#type: row.get(1)?,
            properties_json: row.get(2)?,
        })
    })?;

    let mut list = Vec::new();
    for r in rows {
        list.push(r?);
    }
    Ok(list)
}

// Project operations
pub fn create_project(conn: &Connection, id: &str, name: &str, custom_instructions: Option<&str>) -> Result<()> {
    conn.execute(
        "INSERT INTO projects (id, name, custom_instructions) VALUES (?1, ?2, ?3);",
        params![id, name, custom_instructions],
    )?;
    Ok(())
}

pub fn get_project(conn: &Connection, id: &str) -> Result<Option<Project>> {
    let mut stmt = conn.prepare("SELECT id, name, custom_instructions, created_at FROM projects WHERE id = ?1;")?;
    let mut rows = stmt.query(params![id])?;
    if let Some(row) = rows.next()? {
        Ok(Some(Project {
            id: row.get(0)?,
            name: row.get(1)?,
            custom_instructions: row.get(2)?,
            created_at: Some(row.get(3)?),
        }))
    } else {
        Ok(None)
    }
}

pub fn update_project(conn: &Connection, id: &str, name: &str, custom_instructions: Option<&str>) -> Result<()> {
    conn.execute(
        "UPDATE projects SET name = ?2, custom_instructions = ?3 WHERE id = ?1;",
        params![id, name, custom_instructions],
    )?;
    Ok(())
}

pub fn delete_project(conn: &Connection, id: &str) -> Result<()> {
    conn.execute("DELETE FROM projects WHERE id = ?1;", params![id])?;
    Ok(())
}

pub fn list_projects(conn: &Connection) -> Result<Vec<Project>> {
    let mut stmt = conn.prepare("SELECT id, name, custom_instructions, created_at FROM projects ORDER BY created_at DESC;")?;
    let rows = stmt.query_map([], |row| {
        Ok(Project {
            id: row.get(0)?,
            name: row.get(1)?,
            custom_instructions: row.get(2)?,
            created_at: Some(row.get(3)?),
        })
    })?;
    let mut list = Vec::new();
    for r in rows {
        list.push(r?);
    }
    Ok(list)
}

pub fn assign_conversation_to_project(conn: &Connection, conv_id: &str, project_id: Option<&str>) -> Result<()> {
    conn.execute(
        "UPDATE conversations SET project_id = ?2 WHERE id = ?1;",
        params![conv_id, project_id],
    )?;
    Ok(())
}

pub fn link_document_to_project(conn: &Connection, project_id: &str, node_id: &str) -> Result<()> {
    conn.execute(
        "INSERT OR IGNORE INTO project_documents (project_id, node_id) VALUES (?1, ?2);",
        params![project_id, node_id],
    )?;
    Ok(())
}

pub fn unlink_document_from_project(conn: &Connection, project_id: &str, node_id: &str) -> Result<()> {
    conn.execute(
        "DELETE FROM project_documents WHERE project_id = ?1 AND node_id = ?2;",
        params![project_id, node_id],
    )?;
    Ok(())
}

pub fn list_project_documents(conn: &Connection, project_id: &str) -> Result<Vec<String>> {
    let mut stmt = conn.prepare("SELECT node_id FROM project_documents WHERE project_id = ?1;")?;
    let rows = stmt.query_map(params![project_id], |row| row.get(0))?;
    let mut list = Vec::new();
    for r in rows {
        list.push(r?);
    }
    Ok(list)
}

// Memory operations
pub fn create_memory(
    conn: &Connection,
    id: &str,
    content: &str,
    category: Option<&str>,
    is_pinned: bool,
    is_auto_generated: bool,
    source_conversation_id: Option<&str>,
) -> Result<()> {
    conn.execute(
        "INSERT INTO memory (id, content, category, is_pinned, is_auto_generated, source_conversation_id)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6);",
        params![id, content, category, is_pinned, is_auto_generated, source_conversation_id],
    )?;
    Ok(())
}

pub fn update_memory(
    conn: &Connection,
    id: &str,
    content: &str,
    category: Option<&str>,
    is_pinned: bool,
) -> Result<()> {
    conn.execute(
        "UPDATE memory SET content = ?2, category = ?3, is_pinned = ?4, updated_at = datetime('now') WHERE id = ?1;",
        params![id, content, category, is_pinned],
    )?;
    Ok(())
}

pub fn delete_memory(conn: &Connection, id: &str) -> Result<()> {
    conn.execute("DELETE FROM memory WHERE id = ?1;", params![id])?;
    Ok(())
}

pub fn list_memories(conn: &Connection) -> Result<Vec<Memory>> {
    let mut stmt = conn.prepare("SELECT id, content, category, is_pinned, is_auto_generated, source_conversation_id, created_at, updated_at FROM memory ORDER BY created_at DESC;")?;
    let rows = stmt.query_map([], |row| {
        Ok(Memory {
            id: row.get(0)?,
            content: row.get(1)?,
            category: row.get(2)?,
            is_pinned: row.get(3)?,
            is_auto_generated: row.get(4)?,
            source_conversation_id: row.get(5)?,
            created_at: Some(row.get(6)?),
            updated_at: Some(row.get(7)?),
        })
    })?;
    let mut list = Vec::new();
    for r in rows {
        list.push(r?);
    }
    Ok(list)
}
