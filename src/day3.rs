use std::fs;

const DAY: u8 = 3;

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
    let mut fabric: Vec<Vec<u32>> = vec![vec![0]];
    let mut intersections = 0;

    for line in file.lines()
    {
        let linearray: Vec<&str> = line.split(" ").collect();

        let coords = linearray.get(2).unwrap();
        let coords: String = coords.clone().replace(":", "");
        let coords: Vec<&str> = coords.split(",").collect();
        let x = coords.get(0).unwrap().parse::<usize>().unwrap();
        let y = coords.get(1).unwrap().parse::<usize>().unwrap();

        let dimensions = linearray.get(3).unwrap();
        let dimensions: Vec<&str> = dimensions.split("x").collect();
        let xlen = dimensions.get(0).unwrap().parse::<usize>().unwrap();
        let ylen = dimensions.get(0).unwrap().parse::<usize>().unwrap();

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
                    fabric[i][j] = fabric[i][j] + 1;
                }
            }
        }
    }
    //Count intersections
    for i in 0..fabric.len()
    {
        for j in 0..fabric[0].len()
        {
            //print!("{} ", fabric[i][j]);
            if fabric[i][j] == 2
            {
                intersections = intersections + 1;
            }
        }
        //println!();
    }

    println!("{}", intersections);
}

fn b(_file: &String)
{
    println!("Running day {} side b!", DAY);
}
