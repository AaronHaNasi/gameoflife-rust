use std::env;
use std::process;
mod life;
use life::*;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        print!("Wrong number of arguments to program. This program requires a filename as an argument.\nUsage: ./driver filename\nExample: ./driver beacon.gol\n");
        process::exit(1);
    }
    
    let mut rows: u8;
    let mut cols: u8;
    
    let filename = &args[1];
    
    let mut game_info = life::load_grid_from_file(filename.to_string());
    
    let grid = game_info.0;
    let rows = game_info.1;
    let cols = game_info.2;

    let display_string = life::to_string(grid); 
    println!("{}\n", display_string);  
}
