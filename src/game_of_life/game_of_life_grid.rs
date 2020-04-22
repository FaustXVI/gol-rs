use std::prelude::v1::*;
use crate::game_of_life::api_types::{Evolving, Grid, Size, Row, Column};
use std::collections::HashSet;
use itertools::Itertools;

#[derive(Clone, Debug)]
pub struct GameOfLifeGrid {
    size: Size,
    results: HashSet<(Row, Column)>,
}

impl GameOfLifeGrid {
    pub fn new<F>(size: Size, mut initializer: F) -> Self where F: FnMut(usize, usize) -> bool {
        let results = (0..size.height).cartesian_product(0..size.width)
            .filter(|(r, c)| initializer(*r, *c))
            .map(|(r, c)| (Row(r), Column(c)))
            .collect();
        GameOfLifeGrid { size, results }
    }
}

impl Evolving for GameOfLifeGrid {
    fn next_generation(self) -> Self {
        GameOfLifeGrid::new(self.size.clone(), |row, column| {
            let alive_neighbours = ((row.saturating_sub(1))..=(row.saturating_add(1))).cartesian_product((column.saturating_sub(1))..=(column.saturating_add(1)))
                .filter(|(r, c)| *r != row || *c != column)
                .map(|(r, c)| self.has_cell_at(Row(r), Column(c)))
                .filter(|alive| *alive)
                .count();

            match alive_neighbours {
                2 => self.has_cell_at(Row(row), Column(column)),
                3 => true,
                _ => false,
            }
        })
    }
}

impl Grid for GameOfLifeGrid {
    fn size(&self) -> Size {
        self.size.clone()
    }

    fn has_cell_at(&self, row: Row, column: Column) -> bool {
        self.results.contains(&(row, column))
    }
}

#[cfg(test)]
mod game_of_life_grid_should {
    use super::*;
    use crate::game_of_life::api_types::Size;

    fn as_matrix(grid: &GameOfLifeGrid) -> Vec<Vec<bool>> {
        let size = grid.size();
        (0..size.height).map(|row|
            (0..size.width).map(|column| grid.has_cell_at(Row(row), Column(column)))
                .collect()
        ).collect()
    }

    #[test]
    fn be_initialize_through_a_lambda() {
        let grid = GameOfLifeGrid::new(Size { width: 3, height: 3 }, |row, column| (row + column) % 2 == 0);
        assert_eq!(as_matrix(&grid), vec![vec![true, false, true],
                                          vec![false, true, false],
                                          vec![true, false, true]]);
    }

    #[test]
    fn stay_dead_when_all_dead() {
        let mut grid = GameOfLifeGrid::new(Size { width: 3, height: 3 }, |_, _| false);
        grid = grid.next_generation();
        assert_eq!(as_matrix(&grid), vec![vec![false, false, false],
                                          vec![false, false, false],
                                          vec![false, false, false]]);
    }

    #[test]
    fn dies_when_underpopulated() {
        let initial_grid = vec![
            vec![false, true, false],
            vec![false, false, false],
            vec![false, false, false]];

        let mut grid = GameOfLifeGrid::new(Size { width: 3, height: 3 }, |row, column| initial_grid[row][column]);
        grid = grid.next_generation();
        assert_eq!(as_matrix(&grid)[0][1], false);
    }

    #[test]
    fn give_birth_when_exactly_3_neighbours() {
        let initial_grid = vec![
            vec![false, true, false],
            vec![true, true, false],
            vec![false, false, false]];

        let mut grid = GameOfLifeGrid::new(Size { width: 3, height: 3 }, |row, column| initial_grid[row][column]);
        grid = grid.next_generation();
        assert_eq!(as_matrix(&grid)[0][0], true);
    }

    #[test]
    fn stays_alive_when_2_neighbours() {
        let initial_grid = vec![
            vec![true, false, false],
            vec![false, true, false],
            vec![false, false, true]];

        let mut grid = GameOfLifeGrid::new(Size { width: 3, height: 3 }, |row, column| initial_grid[row][column]);
        grid = grid.next_generation();
        assert_eq!(as_matrix(&grid)[1][1], true);
    }

    #[test]
    fn dies_when_overpopulated() {
        let initial_grid = vec![
            vec![true, true, false],
            vec![true, true, false],
            vec![true, false, true]];

        let mut grid = GameOfLifeGrid::new(Size { width: 3, height: 3 }, |row, column| initial_grid[row][column]);
        grid = grid.next_generation();
        assert_eq!(as_matrix(&grid)[1][0], false);
    }

    #[test]
    fn square_are_stable() {
        let initial_grid = vec![
            vec![true, true, false],
            vec![true, true, false],
            vec![false, false, false]];

        let mut grid = GameOfLifeGrid::new(Size { width: 3, height: 3 }, |row, column| initial_grid[row][column]);
        grid = grid.next_generation();
        assert_eq!(as_matrix(&grid), initial_grid);
    }

    #[test]
    fn flip_flops_fips() {
        let initial_grid = vec![
            vec![false, false, false],
            vec![true, true, true],
            vec![false, false, false]];

        let mut grid = GameOfLifeGrid::new(Size { width: 3, height: 3 }, |row, column| initial_grid[row][column]);
        grid = grid.next_generation();
        assert_eq!(as_matrix(&grid), vec![
            vec![false, true, false],
            vec![false, true, false],
            vec![false, true, false]]);
    }
}