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

fn b(file: &String)
{
    let mut rects: Vec<Rect> = Vec::new();
    let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    for line in file.lines()
    {
        let caps = re.captures(line).unwrap();
        let id: usize = caps[1].parse().unwrap();
        let x: usize = caps[2].parse().unwrap();
        let y: usize = caps[3].parse().unwrap();
        let width: usize = caps[4].parse().unwrap();
        let height: usize = caps[5].parse().unwrap();
        rects.push(Rect::new(id, x, y, width, height));
    }

    //Perform collision check
    for i in 0..rects.len()
    {
        let mut r1 = rects[i];
        if !r1.collision
        {
            for j in 0..rects.len()
            {
                let mut r2 = rects[j];
                if r1.intersects(&mut r2)
                {
                    rects.get_mut(i).unwrap().collision = r1.collision;
                    rects.get_mut(j).unwrap().collision = r2.collision;
                    break;
                }
            }
        }
    }

    for rect in rects
    {
        if !rect.collision
        {
            println!("Found safe claim with ID #{}", rect.id);
        }
    }
}

#[derive(Copy, Clone)]
struct Rect
{
    id: usize,
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
    collision: bool,
}

impl Rect
{
    fn new(id: usize, x: usize, y: usize, width: usize, height: usize) -> Rect
    {
        Rect {
            id: id,
            x1: x,
            y1: y,
            x2: x + width,
            y2: y + height,
            collision: false,
        }
    }
    fn intersects(&mut self, r: &mut Rect) -> bool
    {
        if self.id != r.id
        {
            if self.x2 > r.x1 && self.x1 < r.x2 && self.y2 > r.y1 && self.y1 < r.y2
            {
                self.collision = true;
                r.collision = true;
                return true;
            }
        }
        false
    }
}
