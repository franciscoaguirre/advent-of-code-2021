use std::cmp::Ordering;

pub fn solve(report: &[Vec<char>]) -> u32 {
    let oxygen_generator_rating = get_rating(RatingType::OxygenGenerator, report);
    let co2_scrubber_rating = get_rating(RatingType::CO2Scrubber, report);

    oxygen_generator_rating * co2_scrubber_rating
}

enum RatingType {
    OxygenGenerator,
    CO2Scrubber,
}

fn get_rating(rating_type: RatingType, report: &[Vec<char>]) -> u32 {
    let max_digits = report[0].len();
    let mut filtered_report = report.to_vec();

    for digit in 0..max_digits {
        if filtered_report.len() == 1 {
            break;
        }

        let mut number_of_ones = 0;
        let mut number_of_zeros = 0;

        for binary_string in &filtered_report {
            let current_digit = binary_string.get(digit).unwrap();

            if *current_digit == '1' {
                number_of_ones += 1;
            } else if *current_digit == '0' {
                number_of_zeros += 1;
            }
        }

        let char_to_retain = match rating_type {
            RatingType::OxygenGenerator => match number_of_ones.cmp(&number_of_zeros) {
                Ordering::Greater => '1',
                Ordering::Equal => '1',
                Ordering::Less => '0',
            },
            RatingType::CO2Scrubber => match number_of_ones.cmp(&number_of_zeros) {
                Ordering::Greater => '0',
                Ordering::Equal => '0',
                Ordering::Less => '1',
            },
        };

        filtered_report
            .retain(|binary_string| *binary_string.get(digit).unwrap() == char_to_retain);
    }

    let binary_string = filtered_report[0].iter().collect::<String>();
    u32::from_str_radix(&binary_string, 2).unwrap()
}

#[cfg(test)]
mod tests {
    fn get_input() -> Vec<Vec<char>> {
        vec![
            vec!['0', '0', '1', '0', '0'],
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '0', '1', '1', '0'],
            vec!['1', '0', '1', '1', '1'],
            vec!['1', '0', '1', '0', '1'],
            vec!['0', '1', '1', '1', '1'],
            vec!['0', '0', '1', '1', '1'],
            vec!['1', '1', '1', '0', '0'],
            vec!['1', '0', '0', '0', '0'],
            vec!['1', '1', '0', '0', '1'],
            vec!['0', '0', '0', '1', '0'],
            vec!['0', '1', '0', '1', '0'],
        ]
    }

    #[test]
    fn solution_works() {
        let report = get_input();

        let solution = super::solve(&report);
        assert_eq!(solution, 230);
    }
}
