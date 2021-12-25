use super::lantern_fish::LanternFish;
use super::lantern_fish_colony::LanternFishColony;

pub fn solve(input: &[LanternFish]) -> usize {
    let mut fish_colony =
        LanternFishColony::new(&input.iter().map(|fish| fish.0 as usize).collect::<Vec<_>>());

    for _day in 0..256 {
        fish_colony.tick();
    }

    fish_colony.get_population_size()
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
        assert_eq!(solution, 26984457539);
    }
}
