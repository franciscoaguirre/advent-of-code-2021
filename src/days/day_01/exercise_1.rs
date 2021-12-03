pub fn solve(depths: &[u32]) -> u32 {
    let first_depth = depths[0];

    depths
        .iter()
        .skip(1)
        .fold((0, first_depth), |(times_increased, last_depth), depth| {
            (
                if *depth > last_depth {
                    times_increased + 1
                } else {
                    times_increased
                },
                *depth,
            )
        })
        .0
}
