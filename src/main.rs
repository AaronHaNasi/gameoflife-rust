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
    let mut gameInfo = life::loadGridFromFile(args[1]);
}
