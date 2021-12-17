mod exercise_1;
mod exercise_2;

use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve(input_file: File) {
    let report = parse_input(input_file);

    let solution_1 = exercise_1::solve(&report);
    println!("Exercise 1: {}", solution_1);

    let solution_2 = exercise_2::solve(&report);
    println!("Exercise 2: {}", solution_2);
}

fn parse_input(input_file: File) -> Vec<Vec<char>> {
    let lines: Vec<Vec<char>> = BufReader::new(input_file)
        .lines()
        .map(|line| line.unwrap())
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    lines
}
