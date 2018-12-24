#[macro_use] extern crate lazy_static;

extern crate regex;

use std::process;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;
use std::cmp::Reverse;
use std::cell::Cell;

use regex::Regex;

mod cmdline;

const DEBUG: bool = false;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = cmdline::Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    let mut boost: u64 = 0;
    loop {

        let platoons: Vec<Platoon> = read_lines(&config.filename, boost);
        let mut remaining_platoons: Vec<&Platoon> = platoons.iter().collect();

        loop {
            let immune_platoons: Vec<&&Platoon> = remaining_platoons.iter().filter(|x| x.affiliation == Affiliation::Immune).collect();
            let infect_platoons: Vec<&&Platoon> = remaining_platoons.iter().filter(|x| x.affiliation == Affiliation::Infect).collect();
            if immune_platoons.len() == 0 || infect_platoons.len() == 0 {
                break;
            }
            let mut all_units = remaining_platoons.clone();
            let starting_armies: u64 = remaining_platoons.iter().map(|x| x.num_units.get()).sum();

            // For all living units, sort by decreasing effective power and initiative
            all_units.sort_by_key(|x| Reverse((x.effective_power(None), x.initiative)));

            let mut to_fight: Vec<Option<&Platoon>> = std::iter::repeat(None).take(platoons.len()).collect();
            let mut chosen: Vec<bool> = std::iter::repeat(false).take(platoons.len()).collect();
            
            // For each group, choose a unit to attack
            for platoon in all_units.iter() {
                let opposing_units = match platoon.affiliation {
                    Affiliation::Infect => &immune_platoons,
                    Affiliation::Immune => &infect_platoons,
                };

                // Filter for living, unchosen units
                let mut potential_targets: Vec<&&&Platoon> = opposing_units.iter().filter(|x| x.num_units.get() > 0 && !chosen[x.id]).collect();

                // Sort by my damage ability, their damage ability, and their initiative
                potential_targets.sort_by_key(|x| Reverse((platoon.effective_power(Some(x)), x.effective_power(None), x.initiative)));

                // I fight this guy, unless there aren't any more targets for me :(
                if potential_targets.len() == 0 {
                    continue;
                }
                let enemy = potential_targets[0];
                if platoon.effective_power(Some(enemy)) == 0 {
                    // Never choose anyone with 0 damage!
                    continue;
                }
                to_fight[platoon.id] = Some(enemy);
                chosen[enemy.id] = true;
            }

            // Now we attack! Sort all the units by iniative
            all_units.sort_by_key(|x| Reverse(x.initiative));

            for platoon in all_units.iter() {
                let enemy = match to_fight[platoon.id] {
                    Some(target) => target,
                    None => continue
                };
                let damage = platoon.effective_power(Some(enemy));
                
                let num_to_die = damage / enemy.hit_points;
                if enemy.num_units.get() > num_to_die {
                    enemy.num_units.set(enemy.num_units.get() - num_to_die);
                } else {
                    enemy.num_units.set(0);
                }
            }

            // Collect only the remaining armies
            remaining_platoons = platoons.iter().filter(|x| x.num_units.get() > 0).collect();
            let ending_armies: u64 = remaining_platoons.iter().map(|x| x.num_units.get()).sum();
            if ending_armies == starting_armies {
                // It's a tie
                break;
            }
        }

        let total: u64 = remaining_platoons.iter().map(|x| x.num_units.get()).sum();
        let winner: Affiliation = remaining_platoons[0].affiliation;
        let other: Affiliation = remaining_platoons[remaining_platoons.len()-1].affiliation;
        if winner != other {
            println!("With a boost of {}, there is a stalemate", boost);
        } else {
            println!("With boost of {}, {:?} wins with {} remaining units", boost, winner, total);
        }
        boost += 1;
        if boost == 200 {
            break;
        }
    }
}

fn strip_characters(original : &str, to_strip : &str) -> String {
    let mut result = String::new();
    for c in original.chars() {
        if !to_strip.contains(c) {
           result.push(c);
       }
    }
    result
}

fn read_lines(filename: &String, boost: u64) -> Vec<Platoon> {
    use self::Affiliation::*;

    let file = File::open(filename).expect("Could not open file");
    let mut affiliation = Immune;

    let mut platoons: Vec<Platoon> = Vec::new();
    let mut id: usize = 0;

    for line in BufReader::new(file).lines() {
        let line = line.expect("Could not parse line");

        // Blank line
        if line.trim().len() == 0 {
            continue;
        }
        
        // Army declaration
        if line.starts_with("Immune") {
            affiliation = Immune;
            continue;
        } else if line.starts_with("Infection") {
            affiliation = Infect;
            continue;
        }

        // Now we read a platoon in
        lazy_static! {
            static ref base_re: Regex = Regex::new(r"(\d+) units each with (\d+) hit points \(?(.*)\)? *with an attack that does (\d+) ([a-z]*) damage at initiative (\d+)").unwrap();
        }

        match base_re.captures(line.as_ref()) {
            None => unreachable!(),
            Some(captures) => {
                let num_units: u64 = captures[1].parse().unwrap();
                let hit_points: u64 = captures[2].parse().unwrap();

                let attack_damage: u64 = captures[4].parse().unwrap();
                let attack_type: String = captures[5].to_string();
                let initiative: u64 = captures[6].parse().unwrap();

                // Now for weaknesses and immunities
                let special = captures[3].trim();
                let mut weakness: HashSet<AttackType> = HashSet::new();
                let mut immunities: HashSet<AttackType> = HashSet::new();
                if special.len() > 0 {
                    for example in special.split(";") {
                        let example = strip_characters(example, ") ");
                        if example.starts_with("weakto") {
                            for weak in example["weakto".len()..].split(",") {
                                let weak = AttackType::from_str(weak);
                                weakness.insert(weak);
                            }
                        } else if example.starts_with("immuneto") {
                            for immunity in example["immuneto".len()..].split(",") {
                                let immunity = AttackType::from_str(immunity);
                                immunities.insert(immunity);
                            }
                        }
                    }
                }
                platoons.push(Platoon {
                    id, 
                    affiliation, 
                    num_units: Cell::new(num_units), 
                    hit_points, 
                    weakness, 
                    immune: immunities, 
                    attack: Attack {
                        typ: AttackType::from_str(attack_type.as_ref()),
                        power: attack_damage + if affiliation == Immune { boost } else { 0 }
                    },
                    initiative
                });
                id += 1;
            }
        }
    }

    platoons
}

#[derive(Hash, Eq, PartialEq, Ord, PartialOrd, Debug)]
enum AttackType {
    Bludgeoning,
    Fire,
    Cold,
    Slashing,
    Radiation,
}

impl AttackType {
    fn from_str(input: &str) -> AttackType {
        match input {
            "bludgeoning" => AttackType::Bludgeoning,
            "cold" => AttackType::Cold,
            "fire" => AttackType::Fire,
            "radiation" => AttackType::Radiation,
            "slashing" => AttackType::Slashing,
            _ => unreachable!(),
        }
    }
}

#[derive(Hash, Eq, PartialEq, Ord, PartialOrd, Debug, Clone, Copy)]
enum Affiliation {
    Immune,
    Infect,
}

#[derive(Hash, Eq, PartialEq, Ord, PartialOrd, Debug)]
struct Attack {
    typ: AttackType,
    power: u64,
}

#[derive(Eq, PartialEq, Debug)]
struct Platoon {
    id: usize,
    affiliation: Affiliation,
    num_units: Cell<u64>,
    hit_points: u64,
    weakness: HashSet<AttackType>,
    immune: HashSet<AttackType>,
    attack: Attack,
    initiative: u64,
}

impl Platoon {
    fn effective_power(&self, against: Option<&Platoon>) -> u64 {
        let base = self.num_units.get() * self.attack.power;
        let against = match against {
            None => return base,
            Some(platoon) => platoon,
        };

        let after_weak = match against.weakness.contains(&self.attack.typ) {
            true => base * 2,
            false => base,
        };
        let after_immune = match against.immune.contains(&self.attack.typ) {
            true => 0,
            false => after_weak,
        };
        after_immune
    }
}