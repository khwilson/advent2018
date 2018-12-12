#[macro_use] extern crate lazy_static;

extern crate regex;

use std::process;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashMap, HashSet};

use regex::Regex;

mod cmdline;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = cmdline::Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let state = read_initial_state(&config.filename);
    let mut state = pad_state(&state, 2 * config.generations);
    let mut rules = read_lines(&config.filename);
    let mut final_rules: HashMap<u64, u64> = HashMap::new();
    for rule in rules {
        final_rules.insert(rule.input, rule.output);
    }
    for i in 0..32 {
        final_rules.entry(i).or_insert(0);
    }

    if config.is_first_puzzle {
        for _ in 0..config.generations {
            advance(&mut state, &final_rules);
        }
        let answer: i64 = state.iter().enumerate().filter(|(i, v)| **v > 0).map(|(i, v)| i as i64 - 2 * config.generations as i64).sum();
        println!("The answer is {}", answer);
    } else {

        println!("The answer is");
    }
}

fn advance(state: &mut Vec<u64>, rules: &HashMap<u64, u64>) {
    let mut curval: u64 = state[0] * 8 + state[1] * 4 + state[2] * 2 + state[3] * 1;
    for i in 2..(state.len() - 2) {
        curval *= 2;
        curval %= 32;
        curval += state[i + 2];
        state[i] = *rules.get(&curval).unwrap();
    }
}

fn pad_state(state: &Vec<u64>, width: u64) -> Vec<u64> {
    let mut output: Vec<u64> = Vec::new();
    for _ in 0..width {
        output.push(0);
    }
    for entry in state {
        output.push(*entry);
    }
    for _ in 0..width {
        output.push(0);
    }
    output
}

fn read_initial_state(filename: &String) -> Vec<u64> {
    let file = File::open(filename).expect("Could not open file");

    let line = BufReader::new(file).lines().take(1).next().expect("Could not parse line").unwrap();
    let line: String = line.trim().chars().skip("initial state: ".len()).collect();

    let mut output: Vec<u64> = Vec::new();
    for c in line.chars() {
        if c == '#' {
            output.push(1);
        } else if c == '.' {
            output.push(0);
        }
    }
    output
}

fn read_lines(filename: &String) -> Vec<Line> {
    let file = File::open(filename).expect("Could not open file");

    BufReader::new(file).lines().skip(2)
        .map(|l| l.expect("Could not parse line"))
        .filter_map(|l| Line::new(&l))
        .collect()
}

#[derive(Hash, Eq, PartialEq, Ord, PartialOrd, Debug)]
struct Line {
    input: u64,
    output: u64,
}

impl Line {
    fn new(line: &String) -> Option<Line> {
        let line = line.trim();
        if line.len() == 0 {
            return None;
        }
        let mut input = 0;
        for c in line.chars().take(5) {
            input *= 2;
            if c == '#' {
                input += 1;
            }
        }
        let mut output = 0;
        if line.chars().skip(9).next().unwrap() == '#' {
            output += 1;
        }
        Some(Line {input: input, output: output})
    }
}