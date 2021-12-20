#[derive(PartialEq, Debug, Clone)]
pub struct Board {
    rows: Vec<Vec<(u32, bool)>>,
}

impl Board {
    pub fn new(rows: Vec<Vec<(u32, bool)>>) -> Board {
        Board { rows }
    }

    /// Checks number of the board
    /// Returns whether or not the board is a winner
    /// Doesn't do anything if board has already won
    pub fn check_number(&mut self, number: u32) -> bool {
        if self.check_if_won() {
            return false;
        }

        let mut should_check_if_won = false;

        for row in self.rows.iter_mut() {
            for mut cell in row.iter_mut() {
                if cell.0 == number {
                    cell.1 = true;
                    should_check_if_won = true;
                }
            }
        }

        if !should_check_if_won {
            return false;
        }

        self.check_if_won()
    }

    /// Gets final value of a winner board using the final number
    /// that made it win
    pub fn get_final_value(&self, final_number: u32) -> u32 {
        self.rows.iter().map(|row| self.sum_row(row)).sum::<u32>() * final_number
    }

    fn check_if_won(&self) -> bool {
        self.check_if_won_horizontal() || self.check_if_won_vertical()
    }

    fn check_if_won_horizontal(&self) -> bool {
        for row in self.rows.iter() {
            if row.iter().all(|cell| cell.1) {
                return true;
            }
        }

        false
    }

    fn check_if_won_vertical(&self) -> bool {
        for column in self.get_columns().iter() {
            if column.iter().all(|cell| cell.1) {
                return true;
            }
        }

        false
    }

    fn get_columns(&self) -> Vec<Vec<&(u32, bool)>> {
        let len = self.rows[0].len();
        let mut iters: Vec<_> = self.rows.iter().map(|row| row.iter()).collect();
        (0..len)
            .map(|_| {
                iters
                    .iter_mut()
                    .map(|row| row.next().unwrap())
                    .collect::<Vec<&(u32, bool)>>()
            })
            .collect()
    }

    fn sum_row(&self, row: &[(u32, bool)]) -> u32 {
        row.iter()
            .filter_map(|cell| if !cell.1 { Some(cell.0) } else { None })
            .sum()
    }
}
