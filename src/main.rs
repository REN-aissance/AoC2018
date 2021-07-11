mod day1;
mod day2;
mod day3;

use std::io;
use std::time::Instant;

const FUNCTIONS: [fn(String); 3] = [day1::run, day2::run, day3::run];

fn main()
{
    let c = Config::new();
    let now = Instant::now();
    FUNCTIONS[c.index](c.side);
    println!("\nDone! Time taken: {}ms", now.elapsed().as_millis());
}

struct Config
{
    index: usize,
    side: String,
}

impl Config
{
    fn new() -> Config
    {
        //Get day from user
        let mut index: String = "".to_string();
        println!("Select day 1-25:");
        io::stdin().read_line(&mut index).unwrap();
        let index = index.trim().parse::<usize>().unwrap() - 1;

        //Get side from user
        let mut side: String = "".to_string();
        println!("Pick side (a or b):");
        io::stdin().read_line(&mut side).unwrap();
        println!();

        Config { index, side }
    }
}
