use rfd::FileDialog;

use std::fs;

use crate::{SIMULATED_COLS, SIMULATED_ROWS};

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

/// Parse plaintext format for a Game of Life pattern
pub fn parse_plaintext(text: String) -> Result<Vec<Vec<bool>>, String> {
    // read in lines and throw out any invalid lines
    let mut lines: Vec<Vec<char>> = Vec::new();
    for mut line in text.split("\n") {
        line = line.trim();
        if !line.starts_with("!") && !line.is_empty() {
            println!("{}", line);
            if !line.chars().all(|x| x == '.' || x == 'O') {
                return Err("Only use . or O to mark patterns".to_string());
            }
            lines.push(line.chars().collect());
        }
    }

    // error if there are no valid lines
    if lines.len() == 0 {
        return Err("Invalid format for plaintext".to_string());
    }
    
    // ensure length of each vector is consistent
    let length = lines[0].len();
    for line in &lines {
        if line.len() != length {
            return Err("Pattern lines must have consistent length".to_string());
        }
    }

    // get the start x and start y on a 60x60 grid if we center the pattern
    let cells_start_x = (SIMULATED_COLS / 2) as usize - (length / 2) as usize;
    let cells_start_y = (SIMULATED_ROWS / 2) as usize - (lines.len() / 2) as usize;

    // build the new grid
    let mut cells: Vec<Vec<bool>> = vec![vec![false; SIMULATED_COLS]; SIMULATED_ROWS];
    for i in 0..lines.len() {
        for j in 0..length {
            cells[i + cells_start_y][j + cells_start_x] = lines[i][j] == 'O';
        }
    }

    Ok(cells)
}

