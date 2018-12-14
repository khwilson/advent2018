fn main() {
    let mut elf1_pos = 0;
    let mut elf2_pos = 1;
    let mut board: Vec<usize> = Vec::new();
    board.push(3);
    board.push(7);
    let puzzle_input = 505961;
    let mod_by_me = 1000000;
    let mut current_value = 37;
    let mut second_puzzle_answered = false;

    while board.len() < puzzle_input + 11 || !second_puzzle_answered {
        let total = board[elf1_pos] + board[elf2_pos];
        if total < 10 {
            board.push(total);
            current_value = ((current_value * 10) + total) % mod_by_me;
            if current_value == puzzle_input && !second_puzzle_answered {
                println!("The answer to the second puzzle is {} minus len(puzzle_input)", board.len());
                second_puzzle_answered = true;
            }
        } else {
            board.push(total / 10);
            current_value = ((current_value * 10) + (total / 10)) % mod_by_me;
            if current_value == puzzle_input && !second_puzzle_answered {
                println!("The answer to the second puzzle is {} minus len(puzzle_input)", board.len());
                second_puzzle_answered = true;
            }
            board.push(total % 10);
            current_value = ((current_value * 10) + (total % 10)) % mod_by_me;
            if current_value == puzzle_input && !second_puzzle_answered {
                println!("The answer to the second puzzle is {} minus len(puzzle_input)", board.len());
                second_puzzle_answered = true;
            }
        }
        elf1_pos = (elf1_pos + board[elf1_pos] + 1) % board.len();
        elf2_pos = (elf2_pos + board[elf2_pos] + 1) % board.len();

        // println!("Round {}: {:?} {} {}", round, board, elf1_pos, elf2_pos);
    }
    let mut answer: u64 = 0;
    for i in (puzzle_input)..(puzzle_input + 10) {
        answer *= 10;
        answer += board[i] as u64;
    }
    println!("The to the first puzzle is {}", answer);
}