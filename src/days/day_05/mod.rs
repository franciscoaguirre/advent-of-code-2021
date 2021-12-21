use std::{
    fs::File,
    io::{BufRead, BufReader},
};

mod point;
use point::Point;

mod line;
use line::{Line, LineDirection};

mod grid;
use grid::Grid;

mod exercise_1;
mod exercise_2;

pub const GRID_SIZE: usize = 1000;
pub const MINIMUM_OVERLAPPING_LINES: u32 = 2;

pub fn solve(input_file: File) {
    let input = parse_input(Box::new(BufReader::new(input_file)));

    let solution_1 = exercise_1::solve(&input);
    println!("Exercise 1: {}", solution_1);

    let solution_2 = exercise_2::solve(&input);
    println!("Exercise 2: {}", solution_2);
}

pub fn parse_input(input_stream: Box<dyn BufRead>) -> Vec<Line> {
    input_stream
        .lines()
        .map(|line| line.unwrap())
        .map(|line| Line::from_string(&line))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Box<dyn BufRead> {
        Box::new(
            "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"
                .as_bytes(),
        )
    }

    #[test]
    fn parsing_works() {
        let input = get_input();

        let lines = super::parse_input(input);

        assert_eq!(
            &lines[0..4],
            vec![
                Line {
                    start: Point::new(0, 9),
                    end: Point::new(5, 9),
                    direction: LineDirection::Horizontal
                },
                Line {
                    start: Point::new(8, 0),
                    end: Point::new(0, 8),
                    direction: LineDirection::Diagonal
                },
                Line {
                    start: Point::new(3, 4),
                    end: Point::new(9, 4),
                    direction: LineDirection::Horizontal
                },
                Line {
                    start: Point::new(2, 1),
                    end: Point::new(2, 2),
                    direction: LineDirection::Vertical
                }
            ]
        )
    }
}
