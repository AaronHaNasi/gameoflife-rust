use std::fs; 
use std::fs::File;

pub fn to_string(grid: &Vec<Vec<u8>>) -> String {
    let mut return_string: String = "".to_string(); 
    
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 0 {
                return_string.push_str(" . ");
            } else {
                return_string.push_str(" X "); 
            }
        }
        return_string.push('\n'); 
   }
    return_string
}

pub fn load_grid_from_file(filename: String) -> (Vec<Vec<u8>>, u8, u8) {

    let contents = fs::read_to_string(filename).unwrap(); 
    let mut iterator = contents.split_whitespace(); 

    let rows: u8 = iterator.next().unwrap().parse().unwrap();
    let cols: u8 = iterator.next().unwrap().parse().unwrap();

    let mut grid = vec![vec![0; rows.into()]; cols.into()];
    
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            grid[i][j] = iterator.next().unwrap().parse().unwrap(); 
        }
    }

    (grid, rows, cols) 
}

pub fn save_grid_to_file(filename: &String, rows: u8, cols: u8, grid: &Vec<Vec<u8>>) {
    
    let mut file_string: String = "".to_string(); 
    file_string.push_str(&*rows.to_string());
    file_string.push(' ');

    file_string.push_str(&*cols.to_string());
    file_string.push(' '); 


    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            file_string.push_str(&*(grid[i][j]).to_string());
            file_string.push(' ');
        }
    }
    
    file_string.push('\n'); 

    let _output_file = File::create(filename);
    fs::write(filename, file_string).expect("unable to write to file"); 
}

fn copy_grid(rows: u8, cols: u8, grid: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut copy_of_grid = vec![vec![0; rows.into()]; cols.into()];

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            copy_of_grid[i][j] = grid[i][j];
        }
    }
    copy_of_grid
}

pub fn mutate_grid(rows: u8, cols: u8, grid: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut temp_grid = copy_grid(rows, cols, &grid);
    // let grid_copy = copy_grid(rows, cols, grid);
    for i in 0..temp_grid.len() {
        for j in 0..temp_grid[i].len() {
            let neighbors = nbr_of_neighbors(i, j, rows, cols, &grid);
            if neighbors < 2 {
                temp_grid[i][j] = 0;
            } else if ( neighbors == 2 || neighbors == 3 ) && grid[i][j] == 1 {
                temp_grid[i][j] = 1;
            } else if neighbors == 3 && grid[i][j] == 0 {
                temp_grid[i][j] = 1;
            } else if neighbors > 3 {
                temp_grid[i][j] = 0;
            }
        }
    }
    temp_grid
}

fn nbr_of_neighbors(i: usize, j: usize, rows: u8, cols: u8, grid: &Vec<Vec<u8>>) -> i32 {
    let mut neighbors = 0;
    
    if i > 0 {
        if grid[i-1][j] == 1 {
            neighbors = neighbors + 1; 
        }
    } 

    if j > 0 {
        if grid[i][j-1] == 1 {
            neighbors = neighbors + 1; 
        }
    } 

    if i+1 < rows.into() {
        if grid[i+1][j] == 1 {
            neighbors = neighbors + 1; 
        }
    } 

    if j+1 < cols.into() {
        if grid[i][j+1] == 1 {
            neighbors = neighbors + 1; 
        }
    }

    if i > 0 && j > 0 {
        if grid[i-1][j-1] == 1 {
            neighbors = neighbors + 1; 
        }
    }

    if i + 1 < rows.into() && j > 0 {
        if grid[i+1][j-1] == 1 {
            neighbors = neighbors + 1; 
        }
    } 

    if i > 0 && j + 1 < cols.into() {
        if grid[i-1][j+1] == 1 {
            neighbors = neighbors + 1; 
        }   
    }

    if i + 1 < rows.into() && j + 1 < cols.into() {
        if grid[i+1][j+1] == 1 {
            neighbors = neighbors + 1; 
        }
    }

    neighbors
}
