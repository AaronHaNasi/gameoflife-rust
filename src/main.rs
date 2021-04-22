use std::env;
use std::process;
use std::io::*;
mod life;
use life::*;

fn get_user_input() -> String {
    let mut user_input = "".to_string();
    
    let _=stdout().flush(); 
    let _result = stdin().read_line(&mut user_input);
    if let Some('\n')=user_input.chars().next_back() {
        user_input.pop();
    }
    if let Some('\r')=user_input.chars().next_back() {
        user_input.pop(); 
    }
    user_input
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        print!("Wrong number of arguments to program.");
        print!("This program requires a filename as an argument.");
        println!("Usage: ./driver filename"); 
        println!("Example: ./driver beacon.gol");
        process::exit(1);
    }
    
    let filename = &args[1];
    
    let game_info = life::load_grid_from_file(filename.to_string());
    
    let mut grid = game_info.0;
    let rows = game_info.1;
    let cols = game_info.2;

    let mut display_string = life::to_string(&grid);
    
    let mut user_input: String;
    
    loop {
        println!("{}\n", display_string);
        println!("Press q to quit, n to iterate, w to save to file, or any other key to move to next generation: ");
        user_input = get_user_input(); 

        if user_input == "q" {
            break
        } else if user_input == "n" {
            print!("How many iterations? ");
            user_input = get_user_input(); 
            let mut iterations: i32 = user_input.parse().unwrap(); 
            while iterations > 0 {
                grid = mutate_grid(rows, cols, &grid); 
                iterations = iterations - 1; 
            }
            
        } else if user_input == "w" {
            print!("Input file name: "); 
            user_input = get_user_input();
            let _result = save_grid_to_file(&user_input, rows, cols, &grid); 

        } else {
            grid = mutate_grid(rows, cols, &grid);
        }
        display_string = life::to_string(&grid); 
    }
}
