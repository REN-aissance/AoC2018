use std::fs;

const DAY: u32 = u32::MAX;

pub fn run(c: String)
{
    let file = fs::read_to_string(format!("{}.txt", DAY)).unwrap();
    if c.contains("a")
    {
        a(&file);
    }
    if c.contains("b")
    {
        b(&file);
    }
}

fn a(file: &String)
{
    println!("Running day {} side a", DAY);
}

fn b(file: &String)
{
    println!("Running day {} side b", DAY);
}
