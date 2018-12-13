use std::process;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod cmdline;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = cmdline::Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let mut trains = find_trains(&config.filename);
    let grid = read_lines(&config.filename);

    loop {
        trains.sort();
        //print_stuff(&grid, &trains);
        let mut i = 0;
        let mut max_i = trains.len();
        while i < max_i {
            advance(&grid, &mut trains[i]);
            let mut did_remove = false;
            for j in 0..max_i {
                if i == j {
                    continue;
                }
                if trains[i].x == trains[j].x && trains[i].y == trains[j].y {
                    println!("Crash at {},{}", trains[i].x, trains[i].y);
                    trains.remove(i);
                    if j < i {
                        i -= 1;
                        trains.remove(j);
                    } else {
                        trains.remove(j - 1);
                    }
                    max_i -= 2;
                    did_remove = true;
                    break;
                }
            }
            if ! did_remove {
                i += 1;
            }
        }
        if trains.len() == 1 {
            println!("Last train at {},{}", trains[0].x, trains[0].y);
            break;
        }
    }
}

fn print_stuff(grid: &Vec<Vec<Piece>>, trains: &Vec<Train>) {
    for (y, line) in grid.iter().enumerate() {
        for (x, piece) in line.iter().enumerate() {
            let mut toprint = match piece.ptype {
                PieceType::CurveBack => "\\",
                PieceType::CurveForward => "/",
                PieceType::Intersection => "+",
                PieceType::StraightHoriz => "-",
                PieceType:: StraightVert => "|",
                _ => " ",
            };
            for train in trains.iter() {
                if train.x == x as i64 && train.y == y as i64 {
                    toprint = match train.direction {
                        Direction::Up => "^",
                        Direction::Down => "v",
                        Direction::Left => "<",
                        Direction::Right => ">",
                    };
                }
            }
            print!("{}", toprint);
        }
        println!();
    }
    println!();
}
fn advance(grid: &Vec<Vec<Piece>>, train: &mut Train) {
    train.direction = match train.direction {
        Direction::Up => {
            train.y -= 1;
            match grid[train.y as usize][train.x as usize].ptype {
                PieceType::CurveBack => Direction::Left,
                PieceType::CurveForward => Direction::Right,
                PieceType::Intersection => {
                    train.counter = (train.counter + 1) % 3;
                    if train.counter == 1 {
                        Direction::Left
                    } else if train.counter == 2 {
                        Direction::Up
                    } else {
                        Direction::Right
                    }
                },
                PieceType::None => panic!(),
                PieceType::StraightHoriz => panic!(),
                PieceType::StraightVert => Direction::Up,
            }
        },
        Direction::Down => {
            train.y += 1;
            match grid[train.y as usize][train.x as usize].ptype {
                PieceType::CurveBack => Direction::Right,
                PieceType::CurveForward => Direction::Left,
                PieceType::Intersection => {
                    train.counter = (train.counter + 1) % 3;
                    if train.counter == 1 {
                        Direction::Right
                    } else if train.counter == 2 {
                        Direction::Down
                    } else {
                        Direction::Left
                    }
                },
                PieceType::None => panic!(),
                PieceType::StraightHoriz => panic!(),
                PieceType::StraightVert => Direction::Down,

            }
        },
        Direction::Left => {
            train.x -= 1;
            match grid[train.y as usize][train.x as usize].ptype {
                PieceType::CurveBack => Direction::Up,
                PieceType::CurveForward => Direction::Down,
                PieceType::Intersection => {
                    train.counter = (train.counter + 1) % 3;
                    if train.counter == 1 {
                        Direction::Down
                    } else if train.counter == 2 {
                        Direction::Left
                    } else {
                        Direction::Up
                    }
                    
                },
                PieceType::None => panic!(),
                PieceType::StraightHoriz => Direction::Left,
                PieceType::StraightVert => panic!(),
            }
            
        },
        Direction::Right => {
            train.x += 1;
            match grid[train.y as usize][train.x as usize].ptype {
                PieceType::CurveBack => Direction::Down,
                PieceType::CurveForward => Direction::Up,
                PieceType::Intersection => {
                    train.counter = (train.counter + 1) % 3;
                    if train.counter == 1 {
                        Direction::Up
                    } else if train.counter == 2 {
                        Direction::Right
                    } else {
                        Direction::Down
                    }
                },
                PieceType::None => panic!(),
                PieceType::StraightHoriz => Direction::Right,
                PieceType::StraightVert => panic!(),
            }
        },
    };
}

fn find_trains(filename: &String) -> Vec<Train> {
    let mut output: Vec<Train> = Vec::new();
    let file = File::open(filename).expect("Could not open file"); 
    for (y, line) in BufReader::new(file).lines().map(|l| l.expect("Could not parse line")).enumerate() {
        let y = y as i64;
        let tline = line.trim();
        if tline.len() == 0 {
            continue;
        }
        for (x, c) in line.chars().enumerate() {
            let x = x as i64;
            if c == '^' {
                output.push(Train { x: x, y: y, direction: Direction::Up, counter: 0 } );
            }
            if c == '>' {
                output.push(Train { x: x, y: y, direction: Direction::Right, counter: 0 } );
            }
            if c == '<' {
                output.push(Train { x: x, y: y, direction: Direction::Left, counter: 0 } );
            }
            if c == 'v' {
                output.push(Train { x: x, y: y, direction: Direction::Down, counter: 0 } );
            }
        }
    }
    output
}

#[derive(Hash, Eq, PartialEq, Ord, PartialOrd, Debug)]
enum Direction {
    Up, Left, Down, Right
}

#[derive(Hash, Eq, PartialEq, Ord, PartialOrd, Debug)]
struct Train {
    y: i64,
    x: i64,
    direction: Direction,
    counter: i64,
}

fn read_lines(filename: &String) -> Vec<Vec<Piece>> {
    let file = File::open(filename).expect("Could not open file");

    let mut grid: Vec<Vec<Piece>> = Vec::new();
    for (y, line) in BufReader::new(file).lines().map(|l| l.expect("Could not parse line")).enumerate() {
        let tline = line.trim();
        if tline.len() == 0 {
            continue;
        }
        grid.push(parse_line(&line, y as i64));
    }
    grid
}

fn parse_line(line: &str, y: i64) -> Vec<Piece> {
    line.chars().enumerate().map(|(x, l)| Piece::new(x as i64, y, l)).collect()
}

#[derive(Hash, Eq, PartialEq, Ord, PartialOrd, Debug)]
enum PieceType {
    Intersection,
    CurveBack,
    CurveForward,
    StraightVert,
    StraightHoriz,
    None,
}

#[derive(Hash, Eq, PartialEq, Ord, PartialOrd, Debug)]
struct Piece {
    x: i64,
    y: i64,
    ptype: PieceType,
}

impl Piece {
    fn new(x: i64, y:i64, piece: char) -> Piece {
        let ptype: PieceType;
        if piece == '|' {
            ptype = PieceType::StraightVert;
        } else if piece == '-' {
            ptype = PieceType::StraightHoriz;
        } else if piece == '/' {
            ptype = PieceType::CurveForward;
        } else if piece == '\\' {
            ptype = PieceType::CurveBack;
        } else if piece == '+' {
            ptype = PieceType::Intersection;
        } else if piece == ' ' {
            ptype = PieceType::None;
        } else if piece == 'v' || piece == '^' {
            // Implies there must at least be a vertical track
            // I manually checked the file and trains only appear on straight track
            ptype = PieceType::StraightVert;
        } else if piece == '<' || piece == '>' {
            ptype = PieceType::StraightHoriz;
        } else {
            ptype = PieceType::None;
        }
        Piece {x: x, y: y, ptype: ptype}
    }
}