use super::{Line, LineDirection};

#[derive(Debug)]
pub struct Grid {
    rows: Vec<Vec<u32>>,
}

impl Grid {
    pub fn new(size: usize) -> Grid {
        Grid {
            rows: vec![vec![0; size]; size],
        }
    }

    pub fn draw_line(&mut self, line: &Line) {
        let Line {
            start,
            end,
            direction,
        } = line;

        match direction {
            LineDirection::Horizontal => {
                for index in start.x..=end.x {
                    self.rows[start.y][index] += 1;
                }
            }
            LineDirection::Vertical => {
                for index in start.y..=end.y {
                    self.rows[index][start.x] += 1;
                }
            }
            LineDirection::Diagonal => {
                let y_difference = end.y - start.y;
                if start.x < end.x {
                    for index in 0..=y_difference {
                        self.rows[start.y + index][start.x + index] += 1;
                    }
                } else {
                    for index in 0..=y_difference {
                        self.rows[start.y + index][start.x - index] += 1;
                    }
                }
            }
        }
    }

    pub fn count_overlaps(&self, minimum_overlapping_lines: u32) -> u32 {
        let mut overlap_count = 0;

        for row in self.rows.iter() {
            for cell in row.iter() {
                if *cell >= minimum_overlapping_lines {
                    overlap_count += 1;
                }
            }
        }

        overlap_count
    }
}

#[cfg(test)]
mod tests {
    use super::super::Point;
    use super::*;

    #[test]
    fn can_draw_lines() {
        let mut grid = Grid::new(5);

        let line = Line::new(Point::new(1, 2), Point::new(3, 2));

        grid.draw_line(&line);

        assert_eq!(grid.rows[2], vec![0, 1, 1, 1, 0])
    }

    #[test]
    fn counts_overlapping_lines() {
        let mut grid = Grid::new(5);

        let line_1 = Line::new(Point::new(1, 2), Point::new(3, 2));
        let line_2 = Line::new(Point::new(2, 4), Point::new(2, 0));
        let line_3 = Line::new(Point::new(4, 3), Point::new(1, 0));

        grid.draw_line(&line_1);
        grid.draw_line(&line_2);
        grid.draw_line(&line_3);

        assert_eq!(grid.count_overlaps(2), 3);
    }
}
