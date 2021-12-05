mod exercise_1;

use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve(input_file: File) {
    let report = parse_input(input_file);

    let solution_1 = exercise_1::solve(&report);
    println!("Exercise 1: {}", solution_1);
}

fn parse_input(input_file: File) -> Vec<Vec<char>> {
    let lines: Vec<Vec<char>> = BufReader::new(input_file)
        .lines()
        .map(|line| line.unwrap())
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    transpose(lines)
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}
