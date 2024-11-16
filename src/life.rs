/// Apply the rules of the game of life with a vector slice to the center cell
fn apply_rules(cells: &Vec<Vec<bool>>) -> bool {
    // count number of adjacent cells that are alive
    let mut count = 0;
    if cells[0][1] { count += 1; }
    if cells[1][0] { count += 1; }
    if cells[2][1] { count += 1; }
    if cells[1][2] { count += 1; }
    
    if cells[1][1] {
        // rule of underpopulation
        if count < 2 { return false; }
        // rule of overpopulation
        else if count > 3 { return false; }
        // otherwise lives
        else { return true; }
    } else {
        // rule of reproduction
        if count == 3 { return true; }
        // otherwise stays dead
        else { return false; }
    }
}

/// Simulates one generation of the game, returning the updated grid
pub fn simulate(cells: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    // the new vector of cells to return, representing the next generation
    let mut ret_cells: Vec<Vec<bool>> = vec![vec![false; cells[0].len()]; cells.len()];

    // iterate through cells and apply the rules
    for i in 0..cells.len() {
        for j in 0..cells[0].len() {
            // slice to pass in for rule application
            let mut slice: Vec<Vec<bool>> = vec![vec![false; 3]; 3];
            
            // handle top left corner
            if i == 0 && j == 0 {
                slice[1][1] = cells[0][0];
                slice[1][2] = cells[0][1];
                slice[2][1] = cells[1][0];
                slice[2][2] = cells[1][1];
            }

            // handle top right corner
            else if i == 0 && j == cells[0].len() - 1 {
                slice[1][0] = cells[0][cells[0].len() - 2];
                slice[1][1] = cells[0][cells[0].len() - 1];
                slice[2][0] = cells[1][cells[0].len() - 2];
                slice[2][1] = cells[1][cells[0].len() - 1];
            }

            // handle bottom right corner
            else if i == cells.len() - 1 && j == cells[0].len() - 1 {
                slice[0][0] = cells[cells.len() - 2][cells.len() - 2];
                slice[0][1] = cells[cells.len() - 2][cells.len() - 1];
                slice[1][0] = cells[cells.len() - 1][cells.len() - 2];
                slice[1][1] = cells[cells.len() - 1][cells.len() - 1];
            }

            // handle bottom left corner
            else if i == cells.len() - 1 && j == 0 {
                slice[0][1] = cells[cells.len() - 2][0];
                slice[0][2] = cells[cells.len() - 2][1];
                slice[1][1] = cells[cells.len() - 1][0];
                slice[1][2] = cells[cells.len() - 1][1];
            }

            // handle top edge
            else if i == 0 {
                for k in j-1..=j+1 {
                    slice[1][k + 1 - j] = cells[0][k];
                    slice[2][k + 1 - j] = cells[1][k];
                }
            }

            // handle right edge
            else if j == cells[0].len() - 1 {
                for k in i-1..=i+1 {
                    slice[k + 1 - i][0] = cells[k][cells[0].len() - 2];
                    slice[k + 1 - i][1] = cells[k][cells[0].len() - 1];
                }
            }

            // handle bottom edge
            else if i == cells.len() - 1 {
                for k in j-1..j+1 {
                    slice[0][k + 1 - j] = cells[cells.len() - 2][k];
                    slice[1][k + 1 - j] = cells[cells.len() - 1][k];
                }
            }

            // handle left edge
            else if j == 0 {
                for k in i-1..=i+1 {
                    slice[k + 1 - i][1] = cells[k][0];
                    slice[k + 1 - i][2] = cells[k][1];
                }
            }

            // otherwise pass the entire slice
            else {
                for k in i-1..=i+1 {
                    for l in j-1..=j+1 {
                        slice[k + 1 - i][l + 1 - j] = cells[k][l];
                    }
                }
            }

            // apply the rules
            ret_cells[i][j] = apply_rules(&slice);
        }
    }
    
    ret_cells
}
