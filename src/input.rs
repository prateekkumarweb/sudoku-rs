use std::path::PathBuf;

use anyhow::Context;

#[derive(Debug, Clone, Copy)]
struct BitMask(u16);

impl BitMask {
    fn new() -> Self {
        Self(0)
    }

    fn set(&mut self, bit: u8) {
        self.0 |= 1 << bit;
    }

    fn clear(&mut self, bit: u8) {
        self.0 &= !(1 << bit);
    }

    fn is_set(&self, bit: u8) -> bool {
        (self.0 & (1 << bit)) != 0
    }
}

/// A Sudoku grid
///
/// The grid is represented as a 9x9 matrix of cells.
/// Each cell contains a digit from 1 to 9, or 0 if the cell is empty.
/// The grid also keeps track of the digits in each row, column, and square
/// to quickly check if a value can be set in a cell.
#[derive(Debug)]
pub struct SudokuGrid {
    cells: [[u8; 9]; 9],
    rows: [BitMask; 9],
    cols: [BitMask; 9],
    squares: [BitMask; 9],
}

impl SudokuGrid {
    /// Get the value of a cell in the grid
    #[inline]
    pub fn at(&self, row: usize, col: usize) -> u8 {
        self.cells[row][col]
    }

    /// Set the value of a cell in the grid.
    /// Returns true if the value was set successfully, false otherwise.
    /// If the value was not set, the grid remains unchanged.
    #[must_use]
    pub fn set(&mut self, row: usize, col: usize, value: u8) -> bool {
        let square = (row / 3) * 3 + (col / 3);
        if self.rows[row].is_set(value - 1)
            || self.cols[col].is_set(value - 1)
            || self.squares[square].is_set(value - 1)
        {
            return false;
        }
        self.cells[row][col] = value;
        self.rows[row].set(value - 1);
        self.cols[col].set(value - 1);
        self.squares[square].set(value - 1);
        true
    }

    /// Unset the value of a cell in the grid.
    /// The grid remains unchanged if the cell was already empty.
    pub fn unset(&mut self, row: usize, col: usize) {
        let value = self.cells[row][col];
        let square = (row / 3) * 3 + (col / 3);
        self.rows[row].clear(value - 1);
        self.cols[col].clear(value - 1);
        self.squares[square].clear(value - 1);
        self.cells[row][col] = 0;
    }

    /// Create a new SudokuGrid from a file.
    /// The file should contain 9 lines with 9 digits each.
    /// Empty cells can be represented by 0, '.' or '_'.
    ///
    /// Example:
    /// ```text
    /// 53__7____
    /// 6__195___
    /// _98____6_
    /// 8___6___3
    /// 4__8_3__1
    /// 7___2___6
    /// _6____28_
    /// ___419__5
    /// ____8__79
    /// ```
    ///
    /// Returns an error if the file does not exist, cannot be read, or has invalid content
    /// (e.g. more than 9 lines, more than 9 digits per line, invalid characters).
    pub fn from_file(input: &PathBuf) -> anyhow::Result<Self> {
        let input = std::fs::read_to_string(input)
            .with_context(|| format!("Failed to read file {:?}", input))?;

        let mut grid = Self {
            cells: [[0; 9]; 9],
            rows: [BitMask::new(); 9],
            cols: [BitMask::new(); 9],
            squares: [BitMask::new(); 9],
        };

        for (i, line) in input.lines().enumerate() {
            if i >= 9 {
                return Err(anyhow::anyhow!("Input file has more than 9 lines"));
            }
            for (j, c) in line.trim().chars().enumerate() {
                if j >= 9 {
                    return Err(anyhow::anyhow!("Line {} has more than 9 digits", i + 1));
                }
                let _ = grid.set(
                    i,
                    j,
                    match c {
                        '.' | '0' | '_' => 0,
                        '1'..='9' => {
                            // Safe to unwrap because we know the character is a digit
                            c.to_digit(10).unwrap() as u8
                        }
                        _ => return Err(anyhow::anyhow!("Invalid character: {:?}", c)),
                    },
                );
            }
        }

        Ok(grid)
    }

    /// Check if the grid is valid.
    /// A grid is valid if all rows, columns, and squares contain unique digits.
    /// Returns true if the grid is valid, false otherwise.
    pub fn is_valid(&self) -> bool {
        for i in 0..9 {
            if !self.is_valid_row(i) || !self.is_valid_col(i) || !self.is_valid_square(i) {
                return false;
            }
        }
        true
    }

    fn is_valid_row(&self, row: usize) -> bool {
        let mut seen = [false; 9];
        for i in 0..9 {
            let value = self.at(row, i);
            if value == 0 {
                continue;
            }
            let index = value as usize - 1;
            if seen[index] {
                return false;
            }
            seen[index] = true;
        }
        true
    }

    fn is_valid_col(&self, col: usize) -> bool {
        let mut seen = [false; 9];
        for i in 0..9 {
            let value = self.at(i, col);
            if value == 0 {
                continue;
            }
            let index = value as usize - 1;
            if seen[index] {
                return false;
            }
            seen[index] = true;
        }
        true
    }

    fn is_valid_square(&self, square: usize) -> bool {
        let start_row = (square / 3) * 3;
        let start_col = (square % 3) * 3;

        let mut seen = [false; 9];
        for i in 0..3 {
            for j in 0..3 {
                let value = self.at(start_row + i, start_col + j);
                if value == 0 {
                    continue;
                }
                let index = value as usize - 1;
                if seen[index] {
                    return false;
                }
                seen[index] = true;
            }
        }
        true
    }
}

impl std::fmt::Display for SudokuGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "+-------+-------+-------+")?;
        for i in 0..9 {
            write!(f, "|")?;
            for j in 0..9 {
                if self.cells[i][j] == 0 {
                    write!(f, " _")?;
                } else {
                    write!(f, " {}", self.cells[i][j])?;
                }
                if j == 2 || j == 5 || j == 8 {
                    write!(f, " |")?;
                }
            }
            writeln!(f)?;
            if i == 2 || i == 5 || i == 8 {
                writeln!(f, "+-------+-------+-------+")?;
            }
        }
        Ok(())
    }
}
