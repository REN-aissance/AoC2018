use regex::Regex;
use std::fs;

const DAY: u8 = 3;

pub fn run(c: String)
{
    let file = fs::read_to_string(format!("input/{}.txt", DAY)).unwrap();
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
    let mut fabric: Vec<Vec<u8>> = vec![vec![0]];
    let mut intersections = 0;
    let re = Regex::new(r"(\d+),(\d+): (\d+)x(\d+)").unwrap();

    for line in file.lines()
    {
        let caps = re.captures(line).unwrap();
        let x: usize = caps[1].parse().unwrap();
        let y: usize = caps[2].parse().unwrap();
        let xlen: usize = caps[3].parse().unwrap();
        let ylen: usize = caps[4].parse().unwrap();

        //Grow fabric if it cannot accomodate current instruction
        if y + ylen > fabric.len()
        {
            for _ in 0..(y + ylen - fabric.len())
            {
                fabric.push(vec![0; fabric[0].len()]);
            }
        }

        let xsize = fabric[0].len();
        if x + xlen > xsize
        {
            for i in 0..fabric.len()
            {
                for _ in 0..(x + xlen - xsize)
                {
                    fabric[i].push(0);
                }
            }
        }

        //Increment instruction zone
        for i in y..(y + ylen)
        {
            for j in x..(x + xlen)
            {
                if fabric[i][j] < 2
                {
                    fabric[i][j] += 1;
                }
            }
        }
    }

    //Count intersections
    for i in 0..fabric.len()
    {
        for j in 0..fabric[0].len()
        {
            if fabric[i][j] == 2
            {
                intersections += 1;
            }
        }
    }

    println!("{}", intersections);
}

fn b(_file: &String) {}
