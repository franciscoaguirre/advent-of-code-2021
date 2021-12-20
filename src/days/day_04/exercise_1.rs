use super::Board;

pub fn solve(input: &(Vec<u32>, Vec<Board>)) -> u32 {
    let (random_numbers, boards) = input;

    let mut new_boards = boards.clone();

    for drawn_number in random_numbers {
        for board in new_boards.iter_mut() {
            let has_won = board.check_number(*drawn_number);
            if has_won {
                return board.get_final_value(*drawn_number);
            }
        }
    }

    0
}
