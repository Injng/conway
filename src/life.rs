/// Apply the rules of the game of life with a vector slice to the center cell
fn apply_rules(cells: &Vec<Vec<bool>>) -> bool {
    // count number of adjacent cells that are alive
    let mut count = 0;
    if cells[0][0] { count += 1; }
    if cells[0][1] { count += 1; }
    if cells[0][2] { count += 1; }
    if cells[1][0] { count += 1; }
    if cells[1][2] { count += 1; }
    if cells[2][0] { count += 1; }
    if cells[2][1] { count += 1; }
    if cells[2][2] { count += 1; }
    
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
pub fn simulate(cells: Vec<Vec<bool>>, is_wrap: bool) -> Vec<Vec<bool>> {
    // the new vector of cells to return, representing the next generation
    let rows = cells.len();
    let cols = cells[0].len();
    let mut ret_cells: Vec<Vec<bool>> = vec![vec![false; cols]; rows];

    // iterate through cells and apply the rules
    for i in 0..rows {
        // if the previous row, this row, and the next row are all false, can skip
        if i > 0 && i < rows - 1 {
            let mut all_false = true;
            for row in i-1..=i+1 {
                for j in 0..cols {
                    if cells[row][j] {
                        all_false = false;
                        break;
                    }
                }
                if !all_false { break; }
            }
            if all_false { continue; }
        }
        
        for j in 0..cols {
            // slice to pass in for rule application
            let mut slice: Vec<Vec<bool>> = vec![vec![false; 3]; 3];
            let mut is_zero = true;

            if is_wrap {
                // utilize wrap-around coordinates and fill the slice 
                for di in 0..3 {
                    for dj in 0..3 {
                        let row = (i + rows - 1 + di) % rows;
                        let col = (j + cols - 1 + dj) % cols;
                        slice[di][dj] = cells[row][col];
                        if cells[row][col] { is_zero = false; }
                    }
                }
            } else {
                // make cells disappear after they go past boundary
                if i == 0 || i == rows - 1 || j == 0 || j == rows - 1 {
                    ret_cells[i][j] = false;
                    continue;
                } else {
                    for di in 0..3 {
                        for dj in 0..3 {
                            let row = i + di - 1;
                            let col = j + dj - 1;
                            slice[di][dj] = cells[row][col];
                            if cells[row][col] { is_zero = false; }
                        }
                    }
                }
            }

            // if it is all zeroes, then no need to apply rules
            if is_zero {
                ret_cells[i][j] = false;
                continue;
            }

            // apply the rules
            ret_cells[i][j] = apply_rules(&slice);
        }
    }
    
    ret_cells
}
