use super::{Grid, Line, LineDirection, GRID_SIZE, MINIMUM_OVERLAPPING_LINES};

pub fn solve(input: &[Line]) -> u32 {
    let mut grid = Grid::new(GRID_SIZE);

    input
        .iter()
        .filter(|line| line.direction != LineDirection::Diagonal)
        .for_each(|line| grid.draw_line(line));

    grid.count_overlaps(MINIMUM_OVERLAPPING_LINES)
}

#[cfg(test)]
mod tests {
    use super::super::parse_input;
    use super::*;
    use std::io::BufRead;

    fn get_raw_input() -> Box<dyn BufRead> {
        Box::new(
            "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"
                .as_bytes(),
        )
    }

    fn get_input() -> Vec<Line> {
        parse_input(get_raw_input())
    }

    #[test]
    fn finds_right_number_of_overlapping_lines() {
        let input = get_input();

        let solution = solve(&input);

        assert_eq!(solution, 5);
    }
}
