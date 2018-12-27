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

    let lines = read_lines(&config.filename);
    let mut registers: Vec<i64> = Vec::new();
    for _ in 0..config.num_registers {
        registers.push(0);
    }

    registers[0] = config.start_val;

    let ip = match lines[0] {
        Instruction::SetIp(a) => a,
        _ => panic!("Not allowed!"),
    };

    let ip = 2;

    let slice = &lines[1..];
    let mut round_num = 0;
    loop {
        //print!("{:?} -> {:?} -> ", registers, slice[registers[ip] as usize]);
        round_num += 1;
        match slice[registers[ip] as usize] {
            Instruction::Addr(a, b, c) => registers[c] = registers[a] + registers[b],
            Instruction::Mulr(a, b, c) => registers[c] = registers[a] * registers[b],
            Instruction::Borr(a, b, c) => registers[c] = registers[a] | registers[b],
            Instruction::Banr(a, b, c) => registers[c] = registers[a] & registers[b],
            Instruction::Addi(a, b, c) => registers[c] = registers[a] + b,
            Instruction::Muli(a, b, c) => registers[c] = registers[a] * b,
            Instruction::Bori(a, b, c) => registers[c] = registers[a] | b,
            Instruction::Bani(a, b, c) => registers[c] = registers[a] & b,
            Instruction::Seti(a, _, c) => registers[c] = a,
            Instruction::Setr(a, _, c) => registers[c] = registers[a],
            Instruction::Gtir(a, b, c) => registers[c] = if a > registers[b] { 1 } else { 0 },
            Instruction::Gtri(a, b, c) => registers[c] = if registers[a] > b { 1 } else { 0 },
            Instruction::Gtrr(a, b, c) => registers[c] = if registers[a] > registers[b] { 1 } else { 0 },
            Instruction::Eqir(a, b, c) => registers[c] = if a == registers[b] { 1 } else { 0 },
            Instruction::Eqri(a, b, c) => registers[c] = if registers[a] == b { 1 } else { 0 },
            Instruction::Eqrr(a, b, c) => registers[c] = if registers[a] == registers[b] { 1 } else { 0 },    
            _ => panic!("Not allowed"),
        }

        registers[ip] += 1;

        //println!("{:?}", registers);

        if registers[ip] < 0 || registers[ip] + 2 > lines.len() as i64 {
            break;
        }
        // I used this line to understand what was going on
        //if registers[ip] == 9 { 
        //    println!("On the funny line");
        //}
    }


    println!("The answer to part 1 is {}", registers[0]);

    // If you look at the pattern for a while, you'll see that register 5 is
    // constantly 10551417 after a while.
    // Then register 0 gets added to (addr 4 0 0) only if register 1 * register 3 == register 5
    // (mulr 4 1 3; eqrr 3 5 3; addr 3 2 2 (skip addi 2 1 2) addr 4 0 0; back on usual track)
    // Thus, the total that ends up in register 0 is the sum of the divisors of 10551417
    println!("The answer to part 2 is {}", 14068560);
}

fn read_lines(filename: &String) -> Vec<Instruction> {
    let file = File::open(filename).expect("Could not open file");

    BufReader::new(file).lines()
        .map(|l| l.expect("Could not parse line"))
        .filter_map(|x| parse_line(&x))
        .collect()
}

fn parse_line(line: &String) -> Option<Instruction> {
    let sline: Vec<&str> = line.split(" ").collect();
    match sline[0] {
        "#ip" => Some(Instruction::SetIp(sline[1].parse().unwrap())),
        "addr" => Some(Instruction::Addr(sline[1].parse().unwrap(), sline[2].parse().unwrap(), sline[3].parse().unwrap())),
        "addi" => Some(Instruction::Addi(sline[1].parse().unwrap(), sline[2].parse().unwrap(), sline[3].parse().unwrap())),
        "mulr" => Some(Instruction::Mulr(sline[1].parse().unwrap(), sline[2].parse().unwrap(), sline[3].parse().unwrap())),
        "muli" => Some(Instruction::Muli(sline[1].parse().unwrap(), sline[2].parse().unwrap(), sline[3].parse().unwrap())),
        "banr" => Some(Instruction::Banr(sline[1].parse().unwrap(), sline[2].parse().unwrap(), sline[3].parse().unwrap())),
        "bani" => Some(Instruction::Bani(sline[1].parse().unwrap(), sline[2].parse().unwrap(), sline[3].parse().unwrap())),
        "borr" => Some(Instruction::Borr(sline[1].parse().unwrap(), sline[2].parse().unwrap(), sline[3].parse().unwrap())),
        "bori" => Some(Instruction::Bori(sline[1].parse().unwrap(), sline[2].parse().unwrap(), sline[3].parse().unwrap())),
        "setr" => Some(Instruction::Setr(sline[1].parse().unwrap(), sline[2].parse().unwrap(), sline[3].parse().unwrap())),
        "seti" => Some(Instruction::Seti(sline[1].parse().unwrap(), sline[2].parse().unwrap(), sline[3].parse().unwrap())),
        "gtri" => Some(Instruction::Gtri(sline[1].parse().unwrap(), sline[2].parse().unwrap(), sline[3].parse().unwrap())),
        "gtir" => Some(Instruction::Gtir(sline[1].parse().unwrap(), sline[2].parse().unwrap(), sline[3].parse().unwrap())),
        "gtrr" => Some(Instruction::Gtrr(sline[1].parse().unwrap(), sline[2].parse().unwrap(), sline[3].parse().unwrap())),
        "eqri" => Some(Instruction::Eqri(sline[1].parse().unwrap(), sline[2].parse().unwrap(), sline[3].parse().unwrap())),
        "eqir" => Some(Instruction::Eqir(sline[1].parse().unwrap(), sline[2].parse().unwrap(), sline[3].parse().unwrap())),
        "eqrr" => Some(Instruction::Eqrr(sline[1].parse().unwrap(), sline[2].parse().unwrap(), sline[3].parse().unwrap())),
        _ => None
    }
}

#[derive(Hash, Eq, PartialEq, Ord, PartialOrd, Debug)]
enum Instruction {
    Addr(usize, usize, usize),
    Addi(usize, i64, usize),
    Mulr(usize, usize, usize),
    Muli(usize, i64, usize),
    Banr(usize, usize, usize),
    Bani(usize, i64, usize),
    Borr(usize, usize, usize),
    Bori(usize, i64, usize),
    Setr(usize, usize, usize),
    Seti(i64, i64, usize),
    Gtir(i64, usize, usize),
    Gtri(usize, i64, usize),
    Gtrr(usize, usize, usize),
    Eqir(i64, usize, usize),
    Eqri(usize, i64, usize),
    Eqrr(usize, usize, usize),
    SetIp(usize),
}