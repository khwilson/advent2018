#[macro_use] extern crate lazy_static;

extern crate regex;

use std::env;
use std::process;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

use regex::Regex;

mod cmdline;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = cmdline::Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let input = read_file(&config.filename);

    if config.is_first_puzzle {
        let max_point = input.iter().map(|p| p.maxx()).max().unwrap();
        let areas_first = get_areas(2 * max_point, &input);

        let shifted_input: Vec<Point> = input.iter().map(|p| Point::new(p.x + 20, p.y + 40, p.num)).collect();
        let areas_second = get_areas(3 * (max_point + 40), &shifted_input);

        let mut max_point: i64 = -1;
        let mut max_area: i64 = -1;
        for i in 0..input.len() {
            let first_area = match areas_first.get(&i) {
                Some(area) => *area,
                None => -1
            };
            let second_area = match areas_second.get(&i) {
                Some(area) => *area,
                None => -1
            };
            if first_area == second_area && first_area >= 0 {
                if first_area > max_area {
                    max_point = i as i64;
                    max_area = first_area;
                }
            }
        }
        println!("Answer is {} for point {}", max_area, max_point);

    } else {
        let shifted_input: Vec<Point> = input.iter().map(|p| Point::new(p.x + 10000, p.y + 10000, p.num)).collect();
        let max_point = input.iter().map(|p| p.maxx()).max().unwrap();
        let mut total_count = 0;
        for i in 0..(10000 + max_point) {
            for j in 0..(10000 + max_point) {
                let total: i64 = shifted_input.iter().map(|p| p.dist(i, j)).sum();
                if total < 10000 {
                    total_count += 1;
                }
            }
        }
        println!("Answer is {}", total_count);
    }
}

fn get_areas(grid_size: i64, input: &Vec<Point>) -> HashMap<usize, i64> {
    let mut areas: HashMap<usize, i64> = HashMap::new();

    for x in 0..grid_size {
        for y in 0..grid_size {
            let min_val: i64 = input.iter().map(|p| p.dist(x, y)).min().unwrap();
            let min_elts: Vec<&Point> = input.iter().filter(|p| p.dist(x, y) == min_val).collect();
            if min_elts.len() == 1 {
                let min_elt = min_elts[0];
                *areas.entry(min_elt.num).or_insert(0) += 1;
            }
        }
    }
    areas
}

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
    num: usize
}

impl Point {
    fn new(x: i64, y: i64, num: usize) -> Point {
        Point { x, y, num }
    }

    fn maxx(&self) -> i64 {
        if self.x > self.y { return self.x } else { return self.y }
    }

    fn dist(&self, x: i64, y: i64) -> i64 {
        (self.x - x).abs() + (self.y - y).abs()
    }
}


fn read_file(filename: &String) -> Vec<Point> {
    let file = File::open(filename).expect("Couldn't open file");
    BufReader::new(file).lines().enumerate()
        .map(|(i, l)| (i, l.expect("Could not parse line")))
        .map(|(i, l)| read_line(l, i))
        .collect()
}

fn read_line(line: String, num: usize) -> Point {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+), (\d+)").unwrap();
    }
    match RE.captures(&line) {
        Some(capture) => Point::new(capture[1].parse().unwrap(), capture[2].parse().unwrap(), num),
        None => panic!("Nope")
    }
}
