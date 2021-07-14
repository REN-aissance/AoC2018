use regex::{Captures, Regex};
use std::collections::HashMap;
use std::fs;

const DAY: u32 = 4;

pub fn run(c: String) {
    let file = fs::read_to_string(format!("input/{}.txt", DAY)).unwrap();
    let mut instructions: Vec<Instruction> = Vec::new();
    let re = Regex::new(r"\[\d{4}-(\d{2})-(\d{2}) (\d{2}):(\d{2})\] (.+)").unwrap();
    for cap in re.captures_iter(&file) {
        instructions.push(Instruction::new(&cap));
    }
    instructions.sort();

    if c.contains("a") {
        a(&instructions);
    }
    if c.contains("b") {
        b(&instructions);
    }
}

fn a(file: &Vec<Instruction>) {
    let re_id = Regex::new(r"#(\d+)").unwrap();
    let re_wake = Regex::new(r"wakes up").unwrap();
    let re_sleep = Regex::new(r"falls asleep").unwrap();
    let mut guards: HashMap<u32, Vec<u32>> = HashMap::new();

    let mut id = 0;
    let mut sleepstart = 0;
    for instruction in file {
        let payload = instruction.payload.as_str();
        if re_id.is_match(payload) {
            id = re_id.captures(payload).unwrap()[1].parse().unwrap();
            if !guards.contains_key(&id) {
                guards.insert(id, vec![0; 60]);
            }
        }
        if re_sleep.is_match(payload) {
            sleepstart = instruction.minute;
        }
        if re_wake.is_match(payload) {
            let time = guards.get_mut(&id).unwrap();
            for i in sleepstart..instruction.minute {
                time[i as usize] += 1;
            }
        }
    }

    let mut sleepyid = 0;
    let mut amount = 0;
    for guard in &guards {
        let x = guard.1.iter().fold(0, |acc, x| acc + x);
        if x > amount {
            sleepyid = *guard.0;
            amount = x;
        }
    }

    let minute = get_max_index(guards.get(&sleepyid).unwrap());

    println!(
        "Guard #{} slept {} minutes total\nMost likely to be asleep at {} minutes past 12\n\nThe requested product is: {}",
        sleepyid,
        amount,
        minute,
        sleepyid * minute as u32
    )
}

fn b(file: &Vec<Instruction>) {
    let re_id = Regex::new(r"#(\d+)").unwrap();
    let re_wake = Regex::new(r"wakes up").unwrap();
    let re_sleep = Regex::new(r"falls asleep").unwrap();
    let mut guards: HashMap<u32, Vec<u32>> = HashMap::new();

    let mut id = 0;
    let mut sleepstart = 0;
    for instruction in file {
        let payload = instruction.payload.as_str();
        if re_id.is_match(payload) {
            id = re_id.captures(payload).unwrap()[1].parse().unwrap();
            if !guards.contains_key(&id) {
                guards.insert(id, vec![0; 60]);
            }
        }
        if re_sleep.is_match(payload) {
            sleepstart = instruction.minute;
        }
        if re_wake.is_match(payload) {
            let time = guards.get_mut(&id).unwrap();
            for i in sleepstart..instruction.minute {
                time[i as usize] += 1;
            }
        }
    }

    let mut max_guard = 0;
    let mut max_index = 0;
    let mut max_minutes = 0;
    for guard in guards {
        let new_mi = get_max_index(&guard.1);
        let new_max = guard.1[new_mi];
        if new_max > max_minutes {
            max_index = new_mi;
            max_minutes = new_max;
            max_guard = guard.0;
        }
    }

    println!(
        "Guard #{} slept at minute {} on {} separate days\n",
        max_guard, max_index, max_minutes
    );
    println!("The requested product is: {}", max_guard * max_index as u32);
}

#[derive(Eq, PartialEq, PartialOrd, Ord)]
struct Instruction {
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    payload: String,
}

impl Instruction {
    fn new(cap: &Captures) -> Instruction {
        Instruction {
            month: cap[1].parse::<u32>().unwrap(),
            day: cap[2].parse::<u32>().unwrap(),
            hour: cap[3].parse::<u32>().unwrap(),
            minute: cap[4].parse::<u32>().unwrap(),
            payload: cap[5].to_string(),
        }
    }
}

fn get_max_index<T: std::cmp::PartialOrd>(v: &Vec<T>) -> usize {
    let mut max_index = 0;
    for i in 0..v.len() {
        if v[i] > v[max_index] {
            max_index = i;
        }
    }
    max_index
}
