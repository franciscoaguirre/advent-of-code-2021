pub fn solve(depths: &[u32]) -> u32 {
    let first_three_depths = depths[0..3].iter().sum();

    depths[1..]
        .windows(3)
        .fold(
            (0, first_three_depths),
            |(times_increased, previous_depths): (u32, u32), depths| {
                let depths_sum = depths.iter().sum();
                (
                    if depths_sum > previous_depths {
                        times_increased + 1
                    } else {
                        times_increased
                    },
                    depths_sum,
                )
            },
        )
        .0
}
