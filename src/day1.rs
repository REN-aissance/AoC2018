use std::fs;

const DAY: u32 = 1;

pub fn run(c: String) {
    let file = fs::read_to_string(format!("{}.txt", DAY)).unwrap();
    if c.contains("a") {
        a(&file);
    }
    if c.contains("b") {
        b(&file);
    }
}

fn a(file: &String) {
    let mut f = 0;

    for line in file.lines() {
        let delta = line.replace("+", "");
        let delta: i32 = delta.parse().unwrap();
        f = f + delta;
    }

    println!("{}", f);
}

fn b(file: &String) {
    let mut f = 0;
    let mut seen = vec![0];

    'out: loop {
        for line in file.lines() {
            let delta = line.replace("+", "");
            let delta: i32 = delta.parse().unwrap();
            f = f + delta;
            if seen.contains(&f) {
                println!("{}", f);
                break 'out;
            } else {
                seen.push(f);
            }
        }
    }
}
