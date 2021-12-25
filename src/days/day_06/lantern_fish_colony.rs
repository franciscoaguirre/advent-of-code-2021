use std::collections::HashMap;

pub struct LanternFishColony(HashMap<usize, usize>);

impl LanternFishColony {
    pub fn new(initial_fishes: &[usize]) -> LanternFishColony {
        let mut fishes_hashmap = HashMap::new();

        for timer in 0..=8 {
            fishes_hashmap.insert(timer, 0);
        }

        for initial_fish in initial_fishes.iter() {
            fishes_hashmap.entry(*initial_fish).and_modify(|e| *e += 1);
        }

        LanternFishColony(fishes_hashmap)
    }

    pub fn tick(&mut self) {
        let mut new_fishes_hashmap = HashMap::new();

        for timer in 0..=8 {
            new_fishes_hashmap.insert(timer, 0);
        }

        for timer in &[1, 2, 3, 4, 5, 6, 7, 8, 0] {
            if *timer == 0 {
                let parents = *self.0.get(&0).unwrap();
                new_fishes_hashmap
                    .entry(8)
                    .and_modify(|count| *count = parents);
                new_fishes_hashmap
                    .entry(6)
                    .and_modify(|count| *count += parents);
            } else {
                let current_timer_count = *self.0.get(timer).unwrap();
                new_fishes_hashmap
                    .entry(timer - 1)
                    .and_modify(|count| *count = current_timer_count);
            }
        }

        self.0 = new_fishes_hashmap;
    }

    pub fn get_population_size(&self) -> usize {
        self.0.iter().map(|entry| entry.1).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tick_works() {
        let mut colony = LanternFishColony::new(&[0, 1, 1, 2, 3, 3, 4, 4, 4, 5, 6]);

        colony.tick();

        let expected_values = &[2, 1, 2, 3, 1, 1, 1, 0, 1];

        for timer in 0..=8 {
            assert_eq!(colony.0.get(&timer), Some(&expected_values[timer as usize]));
        }
    }
}
