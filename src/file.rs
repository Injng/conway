use rfd::FileDialog;

use std::fs;

/// Opens a file dialog for user to select file and get contents of file into String
pub fn get_file() -> Result<String, String> {
    let files = match FileDialog::new().pick_file() {
        Some(s) => s,
        None => return Err("No file picked".to_string()),
    };
    
    let contents: String = match fs::read_to_string(files) {
        Ok(s) => s,
        Err(_) => return Err("Invalid file picked".to_string()),
    };

    Ok(contents)
}

