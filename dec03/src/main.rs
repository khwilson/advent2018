#[macro_use] extern crate lazy_static;
extern crate regex;

use std::iter::repeat;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;
use std::cmp;

use regex::Regex;

mod cmdline;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = cmdline::Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let lines = read_lines(&config.filename);

    if config.is_first_puzzle {
        let mut fabric = [[0u16; 1000]; 1000];
        for line in lines.iter() {
            for i in line.start_x..line.end_x()+1 {
                let i: usize = i as usize;
                for j in line.start_y..line.end_y()+1 {
                    let j: usize = j as usize;
                    fabric[i][j] += 1;
                }
            }
        }
        let total = fabric.iter().map(|row| row.iter().filter(|elt| **elt > 1).map(|_| 1u64).sum::<u64>()).sum::<u64>();
        println!("The answer is {}", total);
    } else {
        let mut seen = vec![false; lines.len()];
        for ((i, line1), (j, line2)) in lines.iter()
                .enumerate()
                .flat_map(|(i, line1)| repeat((i, line1)).zip(lines.iter().skip(i + 1).enumerate())) {
            if line1.does_overlap(line2) {
                seen[i] = true;
                seen[j + i + 1] = true;
            }
        }
        for (line, seen_val) in lines.iter().zip(seen.iter()) {
            if !*seen_val {
                println!("Answer is {}", line.id);
            }
        }
    }
}

fn read_lines(filename: &String) -> Vec<Line> {
    let file = File::open(filename).expect("Could not open file");

    BufReader::new(file).lines()
        .map(|l| l.expect("Could not parse line"))
        .map(|l| Line::new(&l))
        .collect()
}


struct Line {
    id: u64,
    start_x: u64,
    start_y: u64,
    width: u64,
    height: u64,
}

impl Line {
    fn new(line: &String) -> Line {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
        }
        match RE.captures(line) {
            Some(captures) => Line {
                id: captures[1].parse::<u64>().unwrap(),
                start_x: captures[2].parse::<u64>().unwrap(),
                start_y: captures[3].parse::<u64>().unwrap(),
                width: captures[4].parse::<u64>().unwrap(),
                height: captures[5].parse::<u64>().unwrap(),
            },
            None => panic!("nope"),
        }
    }

    fn end_x(&self) -> u64 {
        self.start_x + self.width - 1
    }

    fn end_y(&self) -> u64 {
        self.start_y + self.height - 1
    }

    fn does_overlap(&self, other: &Line) -> bool {
        cmp::max(self.start_x, other.start_x) <= cmp::min(self.end_x(), other.end_x()) &&
            cmp::max(self.start_y, other.start_y) <= cmp::min(self.end_y(), other.end_y())
    }

}