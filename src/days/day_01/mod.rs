mod exercise_1;
mod exercise_2;

use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve(input_file: File) {
    let depths = parse_input(input_file);

    let solution_1 = exercise_1::solve(&depths);
    println!("Exercise 1: {}", solution_1);

    let solution_2 = exercise_2::solve(&depths);
    println!("Exercise 2: {}", solution_2);
}

fn parse_input(input_file: File) -> Vec<u32> {
    BufReader::new(input_file)
        .lines()
        .map(|line| line.unwrap())
        .map(|line| line.parse::<u32>().unwrap())
        .collect()
}
