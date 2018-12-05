use std::env;
use std::process;
use std::io::prelude::*;
use std::fs::File;
use std::collections::LinkedList;
use std::str::Chars;

mod cmdline;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = cmdline::Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let input = read_file(&config.filename);
    let input = input.trim();

    if config.is_first_puzzle {
        println!("Answer is {}", count_reaction(input.chars()));
    } else {
        // THis makes me feel dumb. What is the parent type of Chars and Filter<Chars>?
        let answer = "abcdefghijklmnopqrstuvwxyz".chars()
            .map(|del_ch| count_reaction(input.chars().filter(|x| x.to_ascii_lowercase() != del_ch)))
            .min().unwrap();
        println!("Answer is {}", answer);
    }

}

fn count_reaction(iter: impl Iterator<Item=char>) -> usize {
    let mut stack: LinkedList<char> = LinkedList::new();
    for ch in iter {
        let should_push: bool = match stack.back() {
            Some(other_ch) => !(ch != *other_ch && ch.to_ascii_lowercase() == other_ch.to_ascii_lowercase()),
            None => true,
        };
        if should_push {
            stack.push_back(ch);
        } else {
            stack.pop_back();
        }
    }
    stack.len()
}

fn read_file(filename: &String) -> String {
    let mut file = File::open(filename).expect("Could not open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    contents
}
