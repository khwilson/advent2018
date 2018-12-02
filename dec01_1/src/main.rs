use std::env;
use std::fs::File;
use std::process;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let values = read_values(&config.filename);

    if config.is_first_puzzle {
        let mut total = config.base;
        for &val in values.iter() {
            total += val;
        }
        println!("The total is {}", total);
    } else {
        let mut total = config.base;
        let mut seen: HashSet<i64> = HashSet::new();
        loop {
            let mut is_done = false;
            for &val in values.iter() {
                total += val;
                if !seen.insert(total) {
                    println!("The answer is {}", total);
                    is_done = true;
                    break;
                }
            }
            if is_done {
                break;
            }
        }
    }
}

fn read_values(filename: &String) -> Vec<i64> {
    let mut values: Vec<i64> = Vec::new();

    let f = File::open(filename).expect("Couldn't open file");
    for line in BufReader::new(f).lines() {
        let line = line.unwrap();
        let line = line.trim();
        let pm = match line.chars().next().unwrap() {
            '+' => true,
            '-' => false,
            _ => true,
        };
        let rest: String = line.chars().skip(1).collect();
        let unsigned_val: i64 = rest.parse().unwrap();
        let val = if pm {
            unsigned_val
        } else {
            -unsigned_val
        };

        values.push(val);
    }
    values
}

struct Config {
    base: i64,
    filename: String,
    is_first_puzzle: bool,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("You must pass two arguments, the base and the filename");
        }

        let base = match args[1].parse() {
            Ok(num) => num,
            Err(_) => {
                return Err("Your first argument must be an integer");;
            },
        };
        let filename = args[2].clone();

        let is_first_puzzle = args.len() == 3;
        Ok(Config { base, filename, is_first_puzzle })
    }
}