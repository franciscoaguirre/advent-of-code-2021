mod board;
use board::Board;

mod exercise_1;
mod exercise_2;

use std::fs::File;
use std::io::{BufRead, BufReader};

const BOARD_SIZE: usize = 5;

pub fn solve(input_file: File) {
    let input = parse_input(Box::new(BufReader::new(input_file)));

    let solution_1 = exercise_1::solve(&input);
    println!("Exercise 1: {}", solution_1);

    let solution_2 = exercise_2::solve(&input);
    println!("Exercise 2: {}", solution_2);
}

/// Will parse input from any io readable stream
pub fn parse_input(input_stream: Box<dyn BufRead>) -> (Vec<u32>, Vec<Board>) {
    let mut lines = input_stream.lines().map(|line| line.unwrap());

    let random_numbers: Vec<u32> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|number| number.parse::<u32>().unwrap())
        .collect();

    let lines: Vec<String> = lines.collect();

    let boards: Vec<Board> = lines
        .chunks_exact(BOARD_SIZE + 1)
        .map(|chunk| create_board(&chunk[1..=BOARD_SIZE]))
        .collect();

    (random_numbers, boards)
}

/// Creates a board by parsing and assembling its rows
/// Will panic if the row items can't be parsed to u32s
fn create_board(raw_rows: &[String]) -> Board {
    let mut rows = vec![];
    for raw_row in raw_rows {
        rows.push(
            raw_row
                .split_whitespace()
                .map(|cell| (cell.parse::<u32>().unwrap(), false))
                .collect::<Vec<(u32, bool)>>(),
        );
    }

    Board::new(rows)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Box<dyn super::BufRead> {
        Box::new(
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"
                .as_bytes(),
        )
    }

    #[test]
    fn parsing_works() {
        let input = get_input();

        let (random_numbers, boards) = super::parse_input(input);
        assert_eq!(random_numbers.len(), 27);
        assert_eq!(boards.len(), 3);

        let first_board = Board::new(vec![
            vec![
                (22, false),
                (13, false),
                (17, false),
                (11, false),
                (0, false),
            ],
            vec![(8, false), (2, false), (23, false), (4, false), (24, false)],
            vec![
                (21, false),
                (9, false),
                (14, false),
                (16, false),
                (7, false),
            ],
            vec![(6, false), (10, false), (3, false), (18, false), (5, false)],
            vec![
                (1, false),
                (12, false),
                (20, false),
                (15, false),
                (19, false),
            ],
        ]);
        assert_eq!(boards[0], first_board);

        let second_board = Board::new(vec![
            vec![(3, false), (15, false), (0, false), (2, false), (22, false)],
            vec![
                (9, false),
                (18, false),
                (13, false),
                (17, false),
                (5, false),
            ],
            vec![
                (19, false),
                (8, false),
                (7, false),
                (25, false),
                (23, false),
            ],
            vec![
                (20, false),
                (11, false),
                (10, false),
                (24, false),
                (4, false),
            ],
            vec![
                (14, false),
                (21, false),
                (16, false),
                (12, false),
                (6, false),
            ],
        ]);
        assert_eq!(boards[1], second_board);

        let third_board = Board::new(vec![
            vec![
                (14, false),
                (21, false),
                (17, false),
                (24, false),
                (4, false),
            ],
            vec![
                (10, false),
                (16, false),
                (15, false),
                (9, false),
                (19, false),
            ],
            vec![
                (18, false),
                (8, false),
                (23, false),
                (26, false),
                (20, false),
            ],
            vec![
                (22, false),
                (11, false),
                (13, false),
                (6, false),
                (5, false),
            ],
            vec![(2, false), (0, false), (12, false), (3, false), (7, false)],
        ]);
        assert_eq!(boards[2], third_board);
    }
}
