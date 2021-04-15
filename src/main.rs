use std::env;
use std::process;
use std::io;
use std::io::*;
mod life;
use life::*;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        print!("Wrong number of arguments to program.");
        print!("This program requires a filename as an argument.");
        println!("Usage: ./driver filename"); 
        println!("Example: ./driver beacon.gol");
        process::exit(1);
    }
    
    let mut rows: u8;
    let mut cols: u8;
    let filename = &args[1];
    
    let mut game_info = life::load_grid_from_file(filename.to_string());
    
    let mut grid = game_info.0;
    let rows = game_info.1;
    let cols = game_info.2;

    let mut display_string = life::to_string(&grid);
    // println!("{}\n", display_string);
    
    let mut user_input: String = "".to_string();
    
    loop {

        println!("{}\n", display_string);
        println!("Press q to quit, n to iterate, w to save to file, or any other key to move to next generation: ");

        stdin().read_line(&mut user_input);

        if user_input == "q\n" {
            break
        } else if user_input == "n\n" {
            print!("How many iterations? "); 
        } else if user_input == "w\n" {
            print!("Input file name: "); 
            stdin().read_line(&mut user_input); 
        } else {
            grid = mutate_grid(rows, cols, &grid);
            display_string = to_string(&grid);

        }
    }
}
