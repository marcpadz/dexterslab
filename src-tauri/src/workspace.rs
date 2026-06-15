use std::path::{Path, PathBuf};
use std::fs;

pub fn verify_path_in_workspace(workspace_root: &Path, relative_or_absolute_path: &str) -> Result<PathBuf, String> {
    let workspace_root_canonical = fs::canonicalize(workspace_root)
        .map_err(|e| format!("Workspace root canonicalization failed: {}", e))?;

    let path = Path::new(relative_or_absolute_path);
    let target_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        workspace_root_canonical.join(path)
    };

    // To prevent directory traversal attacks, we canonicalize the path.
    // If the path does not exist yet (e.g., we are about to create/write a file), 
    // fs::canonicalize will fail. In that case, we canonicalize its parent directory,
    // and then join the filename.
    let canonical_target = if target_path.exists() {
        fs::canonicalize(&target_path)
            .map_err(|e| format!("Target path canonicalization failed: {}", e))?
    } else {
        let parent = target_path.parent().ok_or("Invalid path parent")?;
        if parent.exists() {
            let canonical_parent = fs::canonicalize(parent)
                .map_err(|e| format!("Parent path canonicalization failed: {}", e))?;
            let file_name = target_path.file_name().ok_or("Invalid filename")?;
            canonical_parent.join(file_name)
        } else {
            return Err("Target directory parent does not exist".to_string());
        }
    };

    if !canonical_target.starts_with(&workspace_root_canonical) {
        return Err("Access Denied: Path escape detected".to_string());
    }

    Ok(canonical_target)
}

pub fn read_file(workspace_root: &Path, path: &str) -> Result<String, String> {
    let verified_path = verify_path_in_workspace(workspace_root, path)?;
    fs::read_to_string(verified_path).map_err(|e| format!("Failed to read file: {}", e))
}

pub fn write_file(workspace_root: &Path, path: &str, contents: &str) -> Result<(), String> {
    let verified_path = verify_path_in_workspace(workspace_root, path)?;
    
    // Create parent directories if they don't exist
    if let Some(parent) = verified_path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Failed to create parent directories: {}", e))?;
    }
    
    fs::write(verified_path, contents).map_err(|e| format!("Failed to write file: {}", e))
}
