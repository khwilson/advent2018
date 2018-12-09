use std::env;
use std::process;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashMap, HashSet, VecDeque};

mod cmdline;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = cmdline::Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let input = read_file(&config.filename);
    let adj_map_in = get_adj_map_in(&input);
    let adj_map_out = get_adj_map_out(&input);

    if config.is_first_puzzle {
        // This is just a topological sort
        let mut in_degrees = get_in_degrees(adj_map_in);
        let mut answer: Vec<char> = Vec::new();
        while in_degrees.len() > 0 {
            let mut zero_degrees: Vec<char> = in_degrees.iter()
                .filter(|(_, val)| **val == 0)
                .map(|(key, _)| *key)
                .collect();

            // But we have to choose things in alphabetical order
            zero_degrees.sort();
            let to_remove = zero_degrees[0];
            answer.push(to_remove);
            in_degrees.remove(&to_remove);
            for node in adj_map_out.get(&to_remove).unwrap().iter() {
                in_degrees.entry(*node).and_modify(|val| *val -= 1);
            }
        }
        let collected: String = answer.iter().collect();
        println!("The answer is {}", collected);

    } else {
        let mut in_degrees = get_in_degrees(adj_map_in);
        let mut answer: Vec<char> = Vec::new();
        let mut tasks: Vec<Task> = Vec::new();
        let mut available_workers: HashSet<usize> = HashSet::new();
        let mut current_time: i64 = 0;
        for i in 0..config.num_workers {
            available_workers.insert(i);
        }
        loop {
            // First, if there's a free worker at the current time,
            // assign them a task
            while available_workers.len() > 0 {
                // Which worker to assign?
                let worker: usize = *available_workers.iter().next().unwrap();

                // Which tasks are available?
                let mut zero_degrees: Vec<char> = in_degrees.iter()
                    .filter(|(_, val)| **val == 0)
                    .map(|(key, _)| *key)
                    .collect();
                
                zero_degrees.sort();
                if zero_degrees.len() == 0 {
                    //  We're out of stuff to do
                    break;
                }

                // There's a worker and a task. Assign!
                let to_remove = zero_degrees[0];
                answer.push(to_remove);
                in_degrees.remove(&to_remove);
                available_workers.remove(&worker);
                tasks.push(Task::new(&config, worker, to_remove, current_time));
            }

            // We're out of either workers or tasks, so advance time
            if tasks.len() > 0 {
                // Complete the first task
                tasks.sort();
                let task = &tasks[0];
                available_workers.insert(task.worker);

                // Advance time
                current_time = task.endtime;

                // Remove dependencies
                for node in adj_map_out.get(&task.task).unwrap().iter() {
                    in_degrees.entry(*node).and_modify(|val| *val -= 1);
                }
                // Remove the task
                tasks.remove(0);
            }

            // We're done when all tasks completed and all nodes seen
            if tasks.len() == 0 && in_degrees.len() == 0 {
                break;
            }
        }
        let together: String = answer.iter().collect();
        println!("The answer is {} at time {}", together, current_time);
    }
}

#[derive(Debug, Eq, Hash, PartialOrd, Ord, PartialEq)]
struct Task {
    endtime: i64,
    starttime: i64,
    task: char,
    worker: usize,
}

impl Task {
    fn new(config: &cmdline::Config, worker: usize, task: char, starttime: i64) -> Task {
        let mut buffer: [u8; 1] = [0];
        task.encode_utf8(&mut buffer);
        let val = buffer[0];
        let endtime = starttime + (val as i64) - 0x40 + config.base_amount;
        println!("{} {} {} {}", task, val, starttime, endtime);
        Task {endtime, starttime, task, worker}
    }
}

fn make_hash_set(first: char) -> Vec<char> {
    let mut output: Vec<char> = Vec::new();
    output.push(first);
    output
}

fn get_adj_map_out(edges: &Vec<Edge>) -> HashMap<char, Vec<char>> {
    let mut adj_map: HashMap<char, Vec<char>> = HashMap::new();
    for edge in edges {
        adj_map.insert(edge.after, vec![]);
        adj_map.insert(edge.before, vec![]);
    }
    for edge in edges {
        adj_map.entry(edge.before).and_modify(|list| list.push(edge.after)).or_insert(make_hash_set(edge.after));
    }
    adj_map
}

fn get_adj_map_in(edges: &Vec<Edge>) -> HashMap<char, Vec<char>> {
    let mut adj_map: HashMap<char, Vec<char>> = HashMap::new();
    for edge in edges {
        adj_map.insert(edge.after, vec![]);
        adj_map.insert(edge.before, vec![]);
    }

    for edge in edges {
        adj_map.entry(edge.after).and_modify(|list| list.push(edge.before));
    }
    adj_map
}

fn get_in_degrees(adj_map_in: HashMap<char, Vec<char>>) -> HashMap<char, usize> {
    let mut in_degrees = HashMap::new();
    for (key, val) in adj_map_in.iter() {
        in_degrees.entry(*key).or_insert(0);
        for v in val.iter() {
            in_degrees.entry(*key).or_insert(0);
        }
    }

    for (key, val) in adj_map_in.iter() {
        in_degrees.insert(*key, val.len());
    }
    in_degrees
}

#[derive(Debug)]
struct Edge {
    before: char,
    after: char
}

impl Edge {
    fn new(before: char, after: char) -> Edge{
        Edge {before, after}
    }
}


fn read_file(filename: &String) -> Vec<Edge> {
    let file = File::open(filename).expect("Couldn't open file");
    BufReader::new(file).lines()
        .map(|l| l.expect("Could not parse line"))
        .map(|l| read_line(l))
        .collect()
}

fn read_line(line: String) -> Edge {
    let before = line.chars().skip(5).next().unwrap();
    let after = line.chars().skip(36).next().unwrap();
    Edge::new(before, after)
}
