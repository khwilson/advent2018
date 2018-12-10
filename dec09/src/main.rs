use std::collections::HashMap;

fn main() {
    let num_players: i64 = 410;
    let max_marble: i64 = 7205900;

    let mut circle: HashMap<i64, Link> = HashMap::new();
    circle.insert(0, Link {before: 0, after: 0});

    let mut player_scores: Vec<i64> = vec![0; num_players as usize];

    let mut current_marble = 0;

    for num_marble in 1..(max_marble + 1) {
        if num_marble % 23 == 0 {
            for _ in 0..7 {
                current_marble = circle.get(&current_marble).unwrap().before;
            }

            let link = circle.get(&current_marble).unwrap();
            let link_before = link.before;
            let link_after = link.after;
            circle.remove(&current_marble);
            circle.entry(link_before).and_modify(|before_link| before_link.after = link_after);
            circle.entry(link_after).and_modify(|after_link| after_link.before = link_before);

            let current_player: usize = (num_marble % num_players) as usize;
            player_scores[current_player] += num_marble + current_marble;

            current_marble = link_after;
        } else {
            let before_marble = circle.get(&current_marble).unwrap().after;
            let after_marble = circle.get(&before_marble).unwrap().after;
            let new_link = Link { before: before_marble, after: after_marble };
            circle.entry(before_marble).and_modify(|before_link| before_link.after = num_marble);
            circle.entry(after_marble).and_modify(|after_link| after_link.before = num_marble);
            circle.insert(num_marble, new_link);
            current_marble = num_marble;
        }
    }
    println!("Answer is {:?}", player_scores.iter().max().unwrap());
}

#[derive(Debug)]
struct Link {
    before: i64,
    after: i64,
}