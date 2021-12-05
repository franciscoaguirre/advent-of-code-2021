use std::{collections::HashMap, hash::Hash};

pub fn solve(report: &[Vec<char>]) -> u32 {
    let gamma_rate_binary = report
        .iter()
        .map(|digits| get_mode(digits))
        .collect::<String>();
    let epsilon_rate_binary = gamma_rate_binary
        .chars()
        .map(|char| if char == '1' { '0' } else { '1' })
        .collect::<String>();
    let gamma_rate = u32::from_str_radix(&gamma_rate_binary, 2).unwrap();
    let epsilon_rate = u32::from_str_radix(&epsilon_rate_binary, 2).unwrap();

    gamma_rate * epsilon_rate
}

fn get_mode<T: Hash + Eq + Copy>(values: &[T]) -> T {
    let frequencies = values.iter().fold(HashMap::new(), |mut freqs, value| {
        *freqs.entry(value).or_insert(0) += 1;
        freqs
    });

    frequencies
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(value, _)| *value)
        .unwrap()
}
