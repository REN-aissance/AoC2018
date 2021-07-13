use std::fs;

const DAY: u32 = 2;

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
    let mut twos = 0;
    let mut threes = 0;
    for line in file.lines()
    {
        let mut foundtwo = false;
        let mut foundthree = false;
        for c in line.chars()
        {
            let count = countchars(c, &line);
            if count == 2 && foundtwo == false
            {
                twos += 1;
                foundtwo = true;
            }
            if count == 3 && foundthree == false
            {
                threes += 1;
                foundthree = true;
            }
        }
    }
    println!(
        "twos: {}, threes: {}, result: {}",
        twos,
        threes,
        threes * twos
    );
}

fn countchars(c: char, word: &str) -> u32
{
    let mut count = 0;
    for c2 in word.chars()
    {
        if c == c2
        {
            count += 1;
        }
    }
    count
}

fn b(file: &String)
{
    let linearray: Vec<&str> = file.lines().collect();
    //first word id
    'out: for i in 0..(linearray.len() - 1)
    {
        //second word id (slightly optimized)
        for j in (i + 1)..linearray.len()
        {
            //comparison
            let mut errcount = 0;
            let mut result = String::new();
            for k in 0..linearray[i].len()
            {
                let c1 = linearray[i].chars().nth(k).unwrap();
                let c2 = linearray[j].chars().nth(k).unwrap();
                if c1 != c2
                {
                    errcount += 1;
                    if errcount > 1
                    {
                        continue;
                    }
                }
                else
                {
                    result.push(c1);
                }
            }
            if errcount == 1
            {
                println!("{}", result);
                break 'out;
            }
        }
    }
}
