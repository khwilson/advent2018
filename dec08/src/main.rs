use std::env;
use std::process;
use std::io::prelude::*;
use std::fs::File;

mod cmdline;

enum ValType {
    NumChildren,
    NumMetadata,
    Metadata,
    Done,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = cmdline::Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let input = read_file(&config.filename);

    if config.is_first_puzzle {
        let mut total_metadata = 0;
        let mut node_type: ValType = ValType::NumChildren;
        let mut node_stack: Vec<Node> = Vec::new();
        let mut last_val: i64 = -1;

        for sval in input.split(" ") {
            let sval = sval.trim();
            if sval.len() == 0 {
                break;
            }
            let val: i64 = sval.parse().unwrap();
            let next_node_type: ValType = match node_type {
                ValType::NumChildren => {
                    last_val = val;
                    ValType::NumMetadata
                },
                ValType::NumMetadata => {
                    let new_node = Node { 
                        num_children: last_val,
                        num_metadata: val, 
                        child_values: vec![],
                        metadata: vec![]
                    };
                    node_stack.push(new_node);
                    advance(&mut node_stack)
                },
                ValType::Metadata => {
                    total_metadata += val;
                    node_stack.last_mut().unwrap().metadata.push(val);
                    node_stack.last_mut().unwrap().num_metadata -= 1;
                    if node_stack.last().unwrap().num_metadata > 0 {
                        ValType::Metadata
                    } else {
                        advance(&mut node_stack)
                    }
                },
                _ => panic!("Shouldn't get here"),
            };
            node_type = next_node_type;
        }
        println!("Total metadata is {}", total_metadata);
    }
}

fn advance(node_stack: &mut Vec<Node>) -> ValType {
    loop {
        if node_stack.len() == 0 {
            return ValType::Done;
        }
        if node_stack.last().unwrap().num_children > 0 {
            node_stack.last_mut().unwrap().num_children -= 1;
            return ValType::NumChildren;
        }
        if node_stack.last().unwrap().num_metadata > 0 {
            return ValType::Metadata;
        }
        let node = node_stack.pop().unwrap();
        let mut node_value: i64 = 0;
        if node.child_values.len() == 0 {
            node_value = node.metadata.iter().sum();
        } else {
            for metadata in node.metadata.iter() {
                if *metadata >= 1 && *metadata <= node.child_values.len() as i64 {
                    let idx = (metadata - 1) as usize;
                    node_value += node.child_values[idx];
                }
            }
        }
        match node_stack.last_mut() {
            Some(node) => node.child_values.push(node_value),
            // Just printing the answer to avoid redoing all the types
            None => println!("Root node value is {}", node_value)
        }
    }
}

fn read_file(filename: &String) -> String {
    let mut file = File::open(filename).expect("Could not open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    contents
}

#[derive(Debug)]
struct Node {
    num_children: i64,
    num_metadata: i64,
    child_values: Vec<i64>,
    metadata: Vec<i64>,
}