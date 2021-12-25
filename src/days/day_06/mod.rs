use std::fs::File;
use std::io::{BufRead, BufReader};

mod lantern_fish;
use lantern_fish::LanternFish;

mod lantern_fish_colony;
use lantern_fish_colony::LanternFishColony;

mod exercise_1;
mod exercise_2;

pub fn solve(input_file: File) {
    let input = parse_input(Box::new(BufReader::new(input_file)));

    let solution_1 = exercise_1::solve(&input);
    println!("Exercise 1: {}", solution_1);

    let solution_2 = exercise_2::solve(&input);
    println!("Exercise 2: {}", solution_2);
}

pub fn parse_input(input_stream: Box<dyn BufRead>) -> Vec<LanternFish> {
    input_stream
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(LanternFish::from_string)
        .collect()
}
