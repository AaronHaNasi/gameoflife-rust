use std::fs; 
use std::fs::File;
use std::io::prelude::*;

pub fn to_string(grid: Vec<Vec<u8>>) -> String {
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

    let mut contents = fs::read_to_string(filename).unwrap(); 
    let mut iterator = contents.split_whitespace(); 

    // let mut read_contents = contents.lines();

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

pub fn save_grid_to_file(filename: String, rows: u8, cols: u8, grid: Vec<Vec<u8>>) {

}

fn copy_grid(rows: u8, cols: u8, grid: Vec<Vec<u8>>) {

}

pub fn mutate_grid(rows: u8, cols: u8, grid: Vec<Vec<u8>>) {

}

fn nbr_of_neighbors(i: u8, j: u8, rows: u8, cols: u8, grid: Vec<Vec<u8>>) {
    let neighbors = 0;
    
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

    if i+1 < rows {
        if grid[i+1][j] == 1 {
            neighbors = neighbors + 1; 
        }
    } 

    if j+1 < cols {
        if grid[i][j+1] == 1 {
            neighbors = neighbors + 1; 
        }
    }

    if i > 0 && j > 0 {
        if grid[i-1][j-1] == 1 {
            neighbors = neighbors + 1; 
        }
    }

    if i + 1 < rows && j > 0 {
        if grid[i+1][j-1] == 1 {
            neighbors = neighbors + 1; 
        }
    } 

    if i > 0 && j + 1 < cols {
        if grid[i-1][j+1] == 1 {
            neighbors = neighbors + 1; 
        }   
    }

    if i + 1 < rows && j + 1 < cols {
        if grid[i+1][j+1] == 1 {
            neighbors = neighbors + 1; 
        }
    }
}
