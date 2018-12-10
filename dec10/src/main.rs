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

    let mut lines = read_lines(&config.filename);

    let mut counter = 0;
    lines.sort();
    while !print_lines(&lines) && counter < 100000 {
        advance_lines(&mut lines);
        lines.sort();
        counter += 1;
    }
    
    println!("");
    lines.sort();
    print_lines(&lines);
    advance_lines(&mut lines);
    counter += 1;

    println!("");
    lines.sort();
    print_lines(&lines);
    advance_lines(&mut lines);
    counter += 1;


    println!("");
    lines.sort();
    print_lines(&lines);
    advance_lines(&mut lines);
    counter += 1;


    println!("");
    lines.sort();
    print_lines(&lines);
    println!("{}", counter);

}


fn read_lines(filename: &String) -> Vec<Line> {
    let file = File::open(filename).expect("Could not open file");

    BufReader::new(file).lines()
        .map(|l| l.expect("Could not parse line"))
        .filter_map(|l| Line::new(&l))
        .collect()
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Line {
    y: i64,
    x: i64,
    vy: i64,
    vx: i64,
}

impl Line {
    fn new(line: &String) -> Option<Line> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"position=<([ -]*\d+),([ -]*\d+)> velocity=<([ -]*\d+),([ -]*\d+)>").unwrap();
        }
        match RE.captures(line) {
            Some(captures) => Some(Line { 
                x: parse_num(&captures[1]),
                y: parse_num(&captures[2]),
                vx: parse_num(&captures[3]), 
                vy: parse_num(&captures[4])
            }),
            None => None
        }
    }
}

fn parse_num(num: &str) -> i64 {
    // Let's not worry about ownership for a moment
    let mut num = num.trim().to_string();

    let first_char = num.chars().next().unwrap();
    let mut sign: i64 = 1;
    if first_char == '-' {
        sign = -1;
        num = num.chars().skip(1).collect();
    }
    let abs: i64 = num.parse().unwrap();
    sign * abs
}

fn print_lines(lines: &Vec<Line>) -> bool {
    let min_x = lines.iter().map(|line| line.x).min().unwrap();
    let max_x = lines.iter().map(|line| line.x).max().unwrap();
    let min_y = lines.iter().map(|line| line.y).min().unwrap();
    let max_y = lines.iter().map(|line| line.y).max().unwrap();

    if max_x - min_x > 100 || max_y - min_y > 100 {
        return false;
    }
    let mut cur_ptr = 0;
    for y in min_y..(max_y + 1) {
        for x in min_x..(max_x + 1) {
            if cur_ptr < lines.len() && lines[cur_ptr].x == x && lines[cur_ptr].y == y {
                print!("#");
                while cur_ptr < lines.len() && lines[cur_ptr].x == x && lines[cur_ptr].y == y {
                    // Avoid overlapping points
                    cur_ptr += 1;
                }
            } else {
                print!(".");
            }
        }
        println!();
    }
    return true;
}

fn advance_lines(lines: &mut Vec<Line>) {
    for line in lines.iter_mut() {
        line.x += line.vx;
        line.y += line.vy;
    }
}