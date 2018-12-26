use std::process;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

mod cmdline;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = cmdline::Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let points = read_lines(&config.filename);
    let mut edges: Vec<usize> = Vec::new();
    
    for (i, point) in points.iter().enumerate() {
        // If distances are small and the number of points large, it's probably
        // faster to do a HashMap and explore all the points in the max_dist
        // ball around the centered point. If distances are large and points
        // are small, this will blow up, and so it's better to actually explore
        // all points instead.
        let near_points: Vec<usize> = points.iter()
            .enumerate()
            .take(i)
            .filter(|(_j, x)| x.dist(point) <= config.max_dist)
            .map(|(j, _x)| j)
            .collect();
        for j in near_points {
            let j_cluster = edges[j];
            for k in 0..i {
                if edges[k] == j_cluster {
                    edges[k] = i;
                }
            }
        }
        edges.push(i);
    }

    let unique_collections: HashSet<&usize> = edges.iter().collect();
    println!("Number of unique constellations: {}", unique_collections.len());
}

fn read_lines(filename: &String) -> Vec<Point> {
    let file = File::open(filename).expect("Could not open file");

    BufReader::new(file).lines()
        .map(|l| l.expect("Could not parse line"))
        .filter_map(|l| Point::new(&l))
        .collect()
}

#[derive(Hash, Eq, PartialEq, Ord, PartialOrd, Debug)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
    t: i64,
}

impl Point {
    fn new(line: &str) -> Option<Point> {
        let splits: Vec<&str> = line.trim().split(",").collect();
        if splits.len() < 4 {
            None
        } else {
            Some(Point { 
                x: splits[0].parse().unwrap(),
                y: splits[1].parse().unwrap(),
                z: splits[2].parse().unwrap(),
                t: splits[3].parse().unwrap(),
            })
        }
    }

    fn dist(&self, other: &Point) -> u64 {
        ((self.x - other.x).abs() +
         (self.y - other.y).abs() +
         (self.z - other.z).abs() +
         (self.t - other.t).abs()) as u64
    }
}