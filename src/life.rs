use std::fs; 
use std::fs::File;
use std::io::prelude::*;

pub fn to_string(grid: &Vec<Vec<char>>) -> String {
    let mut return_string: String = "".to_string(); 
    
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '0' {
                return_string.push_str(" . ");
            } else {
                return_string.push_str(" X "); 
            }
        }
        return_string.push('\n'); 
   }
    return_string
}

pub fn load_grid_from_file(filename: String) -> (Vec<Vec<char>>, char, char) {

    let mut contents = fs::read_to_string(filename).unwrap(); 
    let mut iterator = contents.split_whitespace(); 

    // let mut read_contents = contents.lines();

    let rows: u8 = iterator.next().unwrap().parse().unwrap();
    let cols: u8 = iterator.next().unwrap().parse().unwrap();

    let mut grid:Vec<Vec<char>> = vec![vec!['0'; usize::from(rows)]; usize::from(cols)];
    
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            grid[i][j] = iterator.next().unwrap().parse().unwrap(); 
        }
    }

    (grid, char::from(rows), char::from(cols))
}

pub fn save_grid_to_file(filename: String, rows: char, cols: char, grid: Vec<Vec<char>>) {
    // TODO write all to single string
    let mut file_string: String = "".to_string();

    file_string.push(rows);
    file_string.push(' ');

    file_string.push(cols);
    file_string.push(' ');

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            file_string.push(grid[i][j]);
            file_string.push(' ');
        }
    }

    file_string.push('\n');

    let mut output_file = File::create(filename);
    fs::write(filename, &file_string);
}

fn copy_grid(rows: char, cols: char, grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut copy_of_grid = vec![vec!['0'; usize::from(rows)]; usize::from(cols)];

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            copy_of_grid[i][j] = grid[i][j];
        }
    }
    copy_of_grid
}

pub fn mutate_grid(rows: char, cols: char, grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut temp_grid = copy_grid(rows, cols, &grid);
    // let grid_copy = copy_grid(rows, cols, grid);
    for i in 0..temp_grid.len() {
        for j in 0..temp_grid[i].len() {
            let neighbors = nbr_of_neighbors(i, j, rows, cols, &grid);
            if neighbors < 2 {
                temp_grid[i][j] = '0';
            } else if ( neighbors == 2 || neighbors == 3 ) && grid[i][j] == '1' {
                temp_grid[i][j] = '1';
            } else if neighbors == 3 && grid[i][j] == '0' {
                temp_grid[i][j] = '1';
            } else if neighbors > 3 {
                temp_grid[i][j] = '0';
            }
        }
    }
    temp_grid
}

fn nbr_of_neighbors(i: usize, j: usize, rows: char, cols: char, grid: &Vec<Vec<char>>) -> i32 {
    let mut neighbors = 0;
    
    if i > 0 {
        if grid[i-1][j] == '1' {
            neighbors = neighbors + 1; 
        }
    } 

    if j > 0 {
        if grid[i][j-1] == '1' {
            neighbors = neighbors + 1; 
        }
    } 

    if i+1 < rows.into() {
        if grid[i+1][j] == '1' {
            neighbors = neighbors + 1; 
        }
    } 

    if j+1 < cols.into() {
        if grid[i][j+1] == '1' {
            neighbors = neighbors + 1; 
        }
    }

    if i > 0 && j > 0 {
        if grid[i-1][j-1] == '1' {
            neighbors = neighbors + 1; 
        }
    }

    if i + 1 < rows.into() && j > 0 {
        if grid[i+1][j-1] == '1' {
            neighbors = neighbors + 1; 
        }
    } 

    if i > 0 && j + 1 < cols.into() {
        if grid[i-1][j+1] == '1' {
            neighbors = neighbors + 1; 
        }   
    }

    if i + 1 < rows.into() && j + 1 < cols.into() {
        if grid[i+1][j+1] == '1' {
            neighbors = neighbors + 1; 
        }
    }

    neighbors
}
