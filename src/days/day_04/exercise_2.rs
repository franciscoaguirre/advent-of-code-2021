use super::Board;

pub fn solve(input: &(Vec<u32>, Vec<Board>)) -> u32 {
    let (random_numbers, boards) = input;

    let mut new_boards = boards.clone();
    let number_of_boards = boards.len();
    let mut boards_won = 0;

    for drawn_number in random_numbers {
        for board in new_boards.iter_mut() {
            let has_won = board.check_number(*drawn_number);
            if has_won {
                boards_won += 1;
                if boards_won == number_of_boards {
                    return board.get_final_value(*drawn_number);
                }
            }
        }
    }

    0
}
