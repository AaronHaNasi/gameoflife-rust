use std::fs; 
use std::fs::File;
use std::io::prelude::*;

fn toString(rows: u8, cols: u8, grid: Vec<Vec<u8>>) {

}

pub fn loadGridFromFile(filename: String) {

    // let mut file = File::open(filename);
    let mut contents = fs::read_to_string(filename);
    println!("{:?}", contents); 
    // file.read_to_string(&mut contents);
    // contents.split_whitespace();

    // let mut read_contents = contents.lines();

    // let rows = read_contents.next();
    // let cols = read_contents.next();

    // let mut grid = Vec::new();

    /*for i in 0..rows {
        grid.push(Vec::new()):
        for j in 0..cols {
            grid[i].push(read_contents.next());
        };
    };*/
}

pub fn saveGridToFile(filename: String, rows: u8, cols: u8, grid: Vec<Vec<u8>>) {

}

fn copyGrid(rows: u8, cols: u8, grid: Vec<Vec<u8>>) {

}

pub fn mutateGrid(rows: u8, cols: u8, grid: Vec<Vec<u8>>) {

}

fn nbrOfNeighbors(i: u8, j: u8, rows: u8, cols: u8, grid: Vec<Vec<u8>>) {

}
