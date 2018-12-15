use std::process;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashMap, VecDeque};
use std::cell::Cell;

mod cmdline;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = cmdline::Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let mut grid = read_lines(&config.filename, config.elf_power);
    let mut round_num = 1;

    loop {

        // Remove dead combatants
        let mut new_combatants: Vec<Combatant> = Vec::new();
        for combatant in grid.combatants {
            if combatant.hit_points.get() > 0 {
                new_combatants.push(combatant);
            }
        }
        grid.combatants = new_combatants;
        grid.combatants.sort();

        for combatant in grid.combatants.iter() {
            if combatant.hit_points.get() == 0 {
                // Avoid combatants that died mid-round
                continue;
            }
            let cx = combatant.x.get();
            let cy = combatant.y.get();

            //********* Turn begins with movement
            // First, if I'm already next to someone, I do not need to move
            let neighbor_combatants = grid.get_targets(&combatant);
            if neighbor_combatants.len() == 0 {
                // So I'm not near anyone. I need to attempt to find an enemy
                let mut start_points: Vec<(usize, usize)> = Vec::new();
                
                // Up
                if cy > 1 && grid.grid[cy - 1][cx] != Space::Wall {
                    start_points.push((cx, cy - 1));
                }

                // Left
                if cx > 1 && grid.grid[cy][cx - 1] != Space::Wall {
                    start_points.push((cx - 1, cy));
                }

                // Right
                if cx + 1 < grid.width() && grid.grid[cy][cx + 1] != Space::Wall {
                    start_points.push((cx + 1, cy));
                }

                // Down
                if cy + 1 < grid.height() && grid.grid[cy + 1][cx] != Space::Wall {
                    start_points.push((cx, cy + 1));
                }

                let mut target_combatants: Vec<(usize, usize, usize, usize, usize)> = Vec::new();
                for (x, y) in start_points.iter() {
                    match grid.find_nearest(*x, *y, &combatant) {
                        Some((new_x, new_y, distance)) => {
                            target_combatants.push((distance, new_y, new_x, *y, *x));
                        },
                        None => {},
                    };
                }
                target_combatants.sort();
                if target_combatants.len() > 0 {
                    combatant.x.set(target_combatants[0].4);
                    combatant.y.set(target_combatants[0].3);
                }
            }

            //********* Turn continues with attack
            // Find the neighbors
            let mut neighbor_combatants = grid.get_targets(&combatant);
            if neighbor_combatants.len() == 0 {
                continue;
            }

            // Kill weakest followed by reading order
            neighbor_combatants.sort_by_key(|neighbor| (neighbor.hit_points.get(), neighbor.y.get(), neighbor.x.get()));
            let target = neighbor_combatants.iter().next().unwrap();
            if combatant.attack_power >= target.hit_points.get() {
                // Remove from the field
                target.hit_points.set(0);
                match target.ctype {
                    CombatantType::Elf => grid.num_elves -= 1,
                    CombatantType::Goblin => grid.num_goblins -= 1,
                }
            } else {
                target.hit_points.set(target.hit_points.get() - combatant.attack_power);
            }
        }

        if grid.num_elves == 0 || grid.num_goblins == 0 {
            break;
        }
        round_num += 1;
        // break;
    }
    let total_hitpoints: u64 = grid.combatants.iter().map(|combatant| combatant.hit_points.get()).sum();
    let who_wins = match grid.combatants[0].ctype {
        CombatantType::Elf => "Elfs",
        CombatantType::Goblin => "Goblins",
    };
    println!("After {} rounds, {} wins with total hit points {} and {} combatants remaining",
             round_num - 1, who_wins, total_hitpoints, grid.combatants.len());
}

fn read_lines(filename: &String, elf_power: u64) -> Grid {
    let file = File::open(filename).expect("Could not open file");
    let mut grid: Grid = Grid::new();

    for (y, sline) in BufReader::new(file).lines().map(|l| l.expect("Could not parse line")).enumerate() {
        let mut line: Vec<Space> = Vec::new();
        for (x, c) in sline.chars().enumerate() {
            match c {
                '#' => line.push(Space::Wall),
                '.' => line.push(Space::Empty),
                'G' => {
                    line.push(Space::Empty);
                    grid.combatants.push(Combatant::new(x, y, CombatantType::Goblin, 3));
                    grid.num_goblins += 1;
                },
                'E' => {
                    line.push(Space::Empty);
                    grid.combatants.push(Combatant::new(x, y, CombatantType::Elf, elf_power));
                    grid.num_elves += 1;
                },
                _ => panic!("Not a valid character"),
            };
        }
        grid.grid.push(line);
    }
    grid
}

#[derive(Hash, Eq, PartialEq, Ord, PartialOrd, Debug)]
enum Space {
    Wall,
    Empty,
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
struct Grid {
    grid: Vec<Vec<Space>>,
    combatants: Vec<Combatant>,
    num_elves: usize,
    num_goblins: usize,
}

fn pos_diff(x: usize, y: usize) -> usize {
    std::cmp::max(x, y) - std::cmp::min(x, y)
}

impl Grid {
    fn new() -> Grid {
        Grid { grid: Vec::new(), combatants: Vec::new(), num_elves: 0, num_goblins: 0 }
    }

    fn living_combatants(&self) -> impl Iterator<Item = &Combatant> {
        self.combatants.iter().filter(|combatant| combatant.hit_points.get() > 0)
    }

    fn living_of_type(&self, enemy_type: CombatantType) -> impl Iterator<Item = &Combatant> {
        self.living_combatants().filter(move |combatant| combatant.ctype == enemy_type)
    }

    fn get_targets(&self, combatant: &Combatant) -> Vec<&Combatant> {
        let mut targets: Vec<&Combatant> = Vec::new();
        for potential_target in self.living_of_type(combatant.ctype.enemy()) {
            if combatant.x == potential_target.x && combatant.y == potential_target.y {
                // This is myself
                continue;
            }
            if pos_diff(combatant.x.get(), potential_target.x.get()) 
                + pos_diff(combatant.y.get(), potential_target.y.get()) == 1 {
                // Enemy!
                targets.push(potential_target);
            }
        }
        targets
    }

    fn width(&self) -> usize {
        if self.grid.len() == 0 {
            0
        } else {
            self.grid.iter().next().unwrap().len()
        }
    }

    fn height(&self) -> usize {
        self.grid.len()
    }

    fn find_nearest(&self, from_x: usize, from_y: usize, from_combatant: &Combatant) -> Option<(usize, usize, usize)> {
        let mut enemies: HashMap<(usize, usize), &Combatant> = HashMap::new();
        let ctype = match from_combatant.ctype {
            CombatantType::Goblin => CombatantType::Goblin,
            CombatantType::Elf => CombatantType::Elf,
        };

        for combatant in self.living_of_type(from_combatant.ctype.enemy()) {
            enemies.insert((combatant.x.get(), combatant.y.get()), combatant);
        }

        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        let mut dist_grid: Vec<Vec<i64>> = Vec::new();
        for y in 0..self.height() {
            let mut line: Vec<i64> = Vec::new();
            for x in 0..self.width() {
                let val = match self.grid[y][x] {
                    Space::Wall => -2, // Can't visit
                    Space::Empty => -1, // Haven't visited
                };
                line.push(val);
            }
            dist_grid.push(line);
        }

        for combatant in self.living_of_type(ctype) {
            // We ban movement into friendly combatants that aren't dead
            dist_grid[combatant.y.get()][combatant.x.get()] = -2;
            if combatant.x.get() == from_x && combatant.y.get() == from_y {
                // This is not a valid starting place
                return None;
            }
        }

        // But we start at 0 and then this position is 1
        dist_grid[from_y][from_x] = 1;
        dist_grid[from_combatant.y.get()][from_combatant.x.get()] = 0;

        queue.push_back((from_x, from_y));
        let mut min_dist: i64 = 0;
        let mut nearest_enemies: Vec<&Combatant> = Vec::new();
        while queue.len() > 0 {
            let (x, y) = queue.pop_front().unwrap();

            match enemies.get(&(x, y)) {
                Some(combatant) => {
                    if min_dist == 0 {
                        min_dist = dist_grid[y][x];
                    }
                    if min_dist > 0 {
                        nearest_enemies.push(combatant);
                    }
                    continue;
                },
                None => {
                    if min_dist > 0 {
                        continue;
                    }
                }
            }

            // Up
            if y > 1 && dist_grid[y - 1][x] == -1 {
                dist_grid[y - 1][x] = dist_grid[y][x] + 1;
                queue.push_back((x, y - 1));
            }

            // Left
            if x > 1 && dist_grid[y][x - 1] == -1 {
                dist_grid[y][x - 1] = dist_grid[y][x] + 1;
                queue.push_back((x - 1, y));
            }

            // Right
            if x + 1 < self.width() && dist_grid[y][x + 1] == -1 {
                dist_grid[y][x + 1] = dist_grid[y][x] + 1;
                queue.push_back((x + 1, y));
            }

            // Down
            if y + 1 < self.height() && dist_grid[y + 1][x] == -1 {
                dist_grid[y + 1][x] = dist_grid[y][x] + 1;
                queue.push_back((x, y + 1));
            }
        }

        if nearest_enemies.len() > 0 {
            // We found at least one. Now figure out the nearest neighbor points
            let mut this_puzzle_is_dumb: Vec<(i64, usize, usize)> = Vec::new();
            for enemy in nearest_enemies.iter() {
                let x = enemy.x.get();
                let y = enemy.y.get();

                // Up
                if y > 1 && dist_grid[y - 1][x] >= 0 {
                    this_puzzle_is_dumb.push((dist_grid[y - 1][x], y - 1, x));
                }

                // Left
                if x > 1 && dist_grid[y][x - 1] >= 0 {
                    this_puzzle_is_dumb.push((dist_grid[y][x - 1], y, x - 1));
                }

                // Right
                if x + 1 < self.width() && dist_grid[y][x + 1] >= 0 {
                    this_puzzle_is_dumb.push((dist_grid[y][x + 1], y, x + 1));
                }

                // Down
                if y + 1 < self.height() && dist_grid[y + 1][x] >= 0 {
                    this_puzzle_is_dumb.push((dist_grid[y + 1][x], y + 1, x));
                }
            }
            this_puzzle_is_dumb.sort();
            Some((this_puzzle_is_dumb[0].2, this_puzzle_is_dumb[0].1, this_puzzle_is_dumb[0].0 as usize))
        } else {
            None
        }
    }

    fn printme(&self) {
        let mut enemies: HashMap<(usize, usize), &Combatant> = HashMap::new();
        for combatant in self.living_combatants() {
            enemies.insert((combatant.x.get(), combatant.y.get()), combatant);
        }
        for (y, line) in self.grid.iter().enumerate() {
            let mut towrite: Vec<&Combatant> = Vec::new();
            for (x, v) in line.iter().enumerate() {
                match enemies.get(&(x, y)) {
                    Some(c) => {
                        towrite.push(c);
                        match c.ctype {
                            CombatantType::Goblin => print!("G"),
                            CombatantType::Elf => print!("E"),
                        };
                    },
                    None => match v {
                        Space::Wall => print!("#"),
                        Space::Empty => print!("."),
                    },
                };
            }
            for c in towrite.iter() {
                let marker = match c.ctype {
                    CombatantType::Goblin => 'G',
                    CombatantType::Elf => 'E',
                };
                print!(" {}({})", marker, c.hit_points.get());
            }
            println!();
        }
        println!();
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Clone)]
enum CombatantType {
    Goblin,
    Elf,
}

impl CombatantType {
    fn enemy(&self) -> CombatantType {
        match self {
            CombatantType::Goblin => CombatantType::Elf,
            CombatantType::Elf => CombatantType::Goblin,
        }
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
struct Combatant {
    y: Cell<usize>,
    x: Cell<usize>,
    ctype: CombatantType,
    hit_points: Cell<u64>,
    attack_power: u64,
}

impl Combatant {
    fn new(x: usize, y: usize, ctype: CombatantType, attack_power: u64) -> Combatant {
        Combatant {
            x: Cell::new(x),
            y: Cell::new(y),
            ctype: ctype,
            hit_points: Cell::new(200),
            attack_power: attack_power,
        }
    }
}