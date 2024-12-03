use std::fs;
use std::path::Path;

#[tauri::command]
pub fn import(folder_path: String) -> Result<(), String> {
    let path = Path::new(&folder_path);
    if !path.is_dir() {
        return Err("Provided path is not a directory".into());
    }

    let entries = fs::read_dir(path).map_err(|e| e.to_string())?;

    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.is_file() {
            if let Some(extension) = path.extension() {
                match extension.to_str() {
                    Some("eml") => parse_email(&path),
                    Some("json") => parse_metadata(&path),
                    _ => (),
                }
            }
        }
    }
    Ok(())
}

fn parse_email(path: &Path) {
    // Placeholder for email parsing logic
    println!("Parsing email file: {:?}", path);
}

fn parse_metadata(path: &Path) {
    // Placeholder for metadata parsing logic
    println!("Parsing metadata file: {:?}", path);
}
