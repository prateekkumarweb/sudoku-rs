use crate::input::SudokuGrid;

/// A Sudoku solver.
///
/// The solver uses backtracking to find a solution to a Sudoku puzzle.
/// The solver is initialized with a Sudoku grid and can be used to find a solution
/// The solver returns the solved grid if a solution is found, or None otherwise.
pub struct Solver {
    grid: SudokuGrid,
}

impl Solver {
    pub fn new(grid: SudokuGrid) -> Self {
        Self { grid }
    }

    pub fn solve(mut self) -> Option<SudokuGrid> {
        // Keep track of choices that were made so that they could be reverted while backtracking
        let mut choices = Vec::new();
        loop {
            // Find an empty cell to make a choice
            if let Some(empty_cell) = self.choose_empty_cell() {
                let mut chosen = false;
                // Try to set a value in the empty cell
                // If a value is set, add the choice to the stack
                for value in 1..=9 {
                    let is_set = self.grid.set(empty_cell.0, empty_cell.1, value);
                    if is_set {
                        choices.push((empty_cell, value));
                        chosen = true;
                        break;
                    }
                }
                if chosen {
                    continue;
                }
                // If no value could be set, backtrack
                // Unset the last choice and try the next value
                // If all values have been tried, backtrack further
                'a: loop {
                    let last_choice = choices.pop();
                    if let Some((cell, mut value)) = last_choice {
                        self.grid.unset(cell.0, cell.1);
                        while value < 9 {
                            let is_set = self.grid.set(cell.0, cell.1, value + 1);
                            if is_set {
                                choices.push((cell, value + 1));
                                break 'a;
                            } else {
                                value += 1;
                            }
                        }
                    } else {
                        return None;
                    }
                }
            } else if self.grid.is_valid() {
                // If there are no empty cells and the grid is valid, return the solution
                break;
            } else {
                return None;
            }
        }
        Some(self.grid)
    }

    fn choose_empty_cell(&self) -> Option<(usize, usize)> {
        for r in 0..9 {
            for c in 0..9 {
                if self.grid.at(r, c) == 0 {
                    return Some((r, c));
                }
            }
        }
        None
    }
}
