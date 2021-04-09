use std::fs::File;
use std::io::Prelude::*;

fn toString(rows: u8, cols: u8, grid: Vec<Vec<u8>>) {

}

fn loadGridFromFile(filename: String, rows: *mut u8, cols: *mut u8) {

    let mut file = File::open(filename);
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    contents.split_whitespace();

    let mut read_contents = contents.lines();

    rows = read_contents.next();
    cols = read_contents.next();

    let mut grid = Vec::new();

    for i in 0..rows {
        grid.push(Vec::new()):
        for j in 0..cols {
            grid[i].push(read_contents.next());
        }
    };
}

fn saveGridToFile(filename: str, rows: u8, cols: u8, grid: Vec<Vec<u8>>) {

}

fn copyGrid(rows: u8, cols: u8, grid: Vec<Vec<u8>>) {

}

fn mutateGrid(rows: u8, cols: u8, grid: Vec<Vec<u8>>) {

}

fn nbrOfNeighbors(i: u8, j: u8, rows: u8, cols: u8, grid: Vec<Vec<u8>>) {

}
