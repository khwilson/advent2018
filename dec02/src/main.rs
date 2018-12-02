use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;
use std::collections::HashMap;

mod cmdline;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = cmdline::Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let lines = read_lines(&config.filename);

    if config.is_first_puzzle {
        let counts = first_puzzle(lines);
        let answer = counts[0] * counts[1];
        println!("The counts {} {}", counts[0], counts[1]);
        println!("The answer is {}", answer);
    } else {
        let answers = second_puzzle(lines);
        for answer in answers {
            println!("Potential answer {}", answer);
        }
    }
}

fn read_lines(filename: &String) -> Vec<String> {
    let file = File::open(filename).expect("Could not open file");
    BufReader::new(file).lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn first_puzzle(lines: Vec<String>) -> [i32; 2] {
    let mut output: [i32; 2] = [0, 0];
    
    for line in lines.iter() {
        let line = line.trim();
        
        let mut counts: HashMap<char, i32> = HashMap::new();

        for c in line.chars() {
            let new_value = match counts.get(&c) {
                Some(count) => count + 1,
                None => 1,
            };
            counts.insert(c, new_value);
        }
        let has_two = counts.values().any(|val| { *val == 2 });
        let has_three = counts.values().any(|val| { *val == 3 });
        
        output[0] += if has_two { 1 } else { 0 };
        output[1] += if has_three { 1 } else { 0 };
        
    }
    return output;
}

fn second_puzzle(lines: Vec<String>) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    for (i, first_line) in lines.iter().enumerate() {
        let first_line = first_line.trim();
        for second_line in lines.iter().skip(i + 1) {
            let second_line = second_line.trim();
            let samesies: i64 = first_line.chars()
                .zip(second_line.chars())
                .map(|(c1, c2)| if c1 == c2 { 1 } else { 0 })
                .sum();
            if first_line.len() as i64 == samesies + 1 {
                let same_letters: String = first_line.chars()
                    .zip(second_line.chars())
                    .filter(|(c1, c2)| c1 == c2 )
                    .map(|(c1, _)| c1 )
                    .collect();
                output.push(same_letters);
            }
        }
    }
    output
}