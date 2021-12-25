use super::lantern_fish::LanternFish;

pub fn solve(input: &[LanternFish]) -> usize {
    let mut lantern_fishes = input.to_vec();
    let mut lantern_fish_offspring = Vec::new();

    for _day in 0..80 {
        for lantern_fish in lantern_fishes.iter_mut() {
            if let Some(offspring) = lantern_fish.tick() {
                lantern_fish_offspring.push(offspring);
            }
        }

        lantern_fishes.append(&mut lantern_fish_offspring);
    }

    lantern_fishes.len()
}

#[cfg(test)]
mod tests {
    use super::super::parse_input;
    use super::*;
    use std::io::BufRead;

    fn get_input_raw() -> Box<dyn BufRead> {
        Box::new("3,4,3,1,2".as_bytes())
    }

    fn get_input() -> Vec<LanternFish> {
        parse_input(get_input_raw())
    }

    #[test]
    fn works() {
        let input = get_input();

        let solution = solve(&input);
        assert_eq!(solution, 5934);
    }
}
