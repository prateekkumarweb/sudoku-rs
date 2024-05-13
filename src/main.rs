use std::path::PathBuf;

use clap::Parser;
use solver::Solver;

use crate::input::SudokuGrid;

mod input;
mod solver;

/// Command line utility to solve sudoku puzzles
#[derive(Parser)]
struct Cli {
    /// Input file containing the sudoku puzzle
    input: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let grid = SudokuGrid::from_file(&cli.input)?;

    println!("Input:");
    println!("{}", grid);

    let solver = Solver::new(grid);
    let solution = solver.solve();

    if let Some(solution) = solution {
        println!("Solution:");
        println!("{}", solution);
    } else {
        println!("No solution found");
    }

    Ok(())
}
