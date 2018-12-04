#[macro_use] extern crate lazy_static;

extern crate regex;
extern crate chrono;

use std::process;
use std::env;
use std::hash::Hash;
use std::iter::FromIterator;
use std::iter::repeat;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp;
use std::collections::{HashMap, HashSet};

use regex::Regex;
use chrono::prelude::*;

mod cmdline;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = cmdline::Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let mut lines = read_lines(&config.filename);
    lines.sort();
    let mut current_guard = -1;
    for line in lines.iter_mut() {
        match line.event {
            GuardEvent::START_SHIFT => {
                current_guard = line.guard;
            },
            _ => {
                line.guard = current_guard;
            }
        };
    } 

    if config.is_first_puzzle {
        let mut sleep_times: HashMap<i64, u32> = HashMap::new();
        let mut wake_times: HashMap<i64, u32> = HashMap::new();
        let mut total_sleep: HashMap<i64, u32> = HashMap::new();
        for line in lines.iter() {
            match line.event {
                GuardEvent::FALLS_ASLEEP => *sleep_times.entry(line.guard).or_insert(0) += line.date.minute(),
                GuardEvent::WAKES_UP => *wake_times.entry(line.guard).or_insert(0) += line.date.minute(),
                _ => ()
            };
        }
        for guard in sleep_times.keys() {
            total_sleep.insert(*guard, wake_times.get(guard).unwrap() - sleep_times.get(guard).unwrap());
        }
        let (max_guard, _) = total_sleep.iter().max_by_key(|x| x.1).unwrap();
        
        let mut by_minute = [0i32; 60];
        for line in lines.iter().filter(|x| x.guard == *max_guard) {
            match line.event {
                GuardEvent::FALLS_ASLEEP => for min in line.date.minute()..60 {
                    by_minute[min as usize] += 1;
                },
                GuardEvent::WAKES_UP => for min in line.date.minute()..60 {
                    by_minute[min as usize] -= 1;
                },
                _ => ()
            };
        }
        let max_min = by_minute.iter().enumerate().max_by_key(|x| Some(x.1)).unwrap().0;
        println!("The answer is {} {} {}", max_guard, max_min, *max_guard * (max_min as i64));
    } else {
        let mut by_minutes: HashMap<i64, [i64; 60]> = HashMap::new();
        for guard in lines.iter().filter(|x| x.event == GuardEvent::START_SHIFT).map(|x| x.guard).collect::<HashSet<i64>>().iter() {
            by_minutes.insert(*guard, [0; 60]);
        }
        for line in lines.iter().filter(|x| x.event != GuardEvent::START_SHIFT) {
            let val = by_minutes.get_mut(&line.guard).unwrap();
            match line.event {
                GuardEvent::FALLS_ASLEEP => for min in line.date.minute()..60 {
                    val[min as usize] += 1;
                },
                GuardEvent::WAKES_UP => for min in line.date.minute()..60 {
                    val[min as usize] -= 1;
                },
                _ => ()
            }
        }
        let mut max_guard = -1i64;
        let mut max_min = -1i64;
        let mut max_count = -1i64;
        for (guard, by_minute) in by_minutes.iter() {
            for (min, min_count) in by_minute.iter().enumerate() {
                if *min_count > max_count {
                    max_guard = *guard;
                    max_min = min as i64;
                    max_count = *min_count;
                }
            }
        }
        println!("The answer is {} {} {}", max_guard, max_min, max_guard * max_min);
    }
    
}

fn read_lines(filename: &String) -> Vec<Line> {
    let file = File::open(filename).expect("Could not open file");

    BufReader::new(file).lines()
        .map(|l| l.expect("Could not parse line"))
        .map(|l| Line::new(&l))
        .collect()
}

fn split_string(line: &String) -> (String, String) {
    // Lines are structured like "[YYYY-mm-dd HH:MM] Some text"
    (line.chars().skip(1).take(16).collect(), 
     line.chars().skip(1 + 16 + 2).collect())
}

fn parse_event(note: &String) -> GuardEvent {
    if note.starts_with("Guard") {
        return GuardEvent::START_SHIFT;
    } else if note.starts_with("falls") {
        return GuardEvent::FALLS_ASLEEP;
    } else {
        return GuardEvent::WAKES_UP;
    }
}

fn parse_guard(note: &String) -> i64 {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"Guard #(\d+) ").unwrap();
    }
    match RE.captures(note) {
        Some(capture) => capture[1].parse().unwrap(),
        None => panic!("Nope")
    }
}

#[derive(Eq, Ord, PartialEq, PartialOrd)]
enum GuardEvent {
    START_SHIFT,
    FALLS_ASLEEP,
    WAKES_UP,
}

#[derive(Eq, Ord, PartialEq, PartialOrd)]
struct Line {
    date: NaiveDateTime,
    event: GuardEvent,
    guard: i64
}

impl Line {
    fn new(line: &String) -> Line {
        let (date, note) = split_string(line);
        let date = NaiveDateTime::parse_from_str(&date, "%Y-%m-%d %H:%M").unwrap();
        let event = parse_event(&note);
        let guard = match event {
            GuardEvent::START_SHIFT => parse_guard(&note),
            _ => -1i64,
        };
        Line {date, event, guard }
    }
}

type SummerMap<T> = HashMap<T, i64>;

#[derive(Clone, PartialEq, Eq, Debug, Default)]
struct Summer<T: Hash + Eq> {
    map: SummerMap<T>,
}

impl<T> Summer<T>
where
    T: Hash + Eq,
{
    fn new() -> Summer<T> {
        Summer { map: HashMap::new() }
    }
}

impl FromIterator<(i64, i64)> for Summer<i64>
{
    fn from_iter<I: IntoIterator<Item=(i64, i64)>>(iter: I) -> Self {
        let mut summer = Summer::new();
        for (guard, time) in iter {
            let total_time : i64 = match summer.map.get(&guard) {
                Some(total) => total + time,
                None => time,
            };
            summer.map.insert(guard, total_time);
        }
        summer
    }
}