pub mod api_types;
mod game_of_life_grid;

use std::prelude::v1::*;
use crate::game_of_life::api_types::{Evolving, Grid, Size};
use crate::game_of_life::game_of_life_grid::GameOfLifeGrid;
use rand::{thread_rng, Rng};

/// Runs a loop that runs a game of life session.
///
/// The created grid will have a size of `size`.
/// For each iteration of the game, the observer function is called.
/// The loop stops when the `observer` returns false.
///
/// ```
/// # use kata::game_of_life::run_gol;
/// # use kata::game_of_life::api_types::{Grid, Size};
/// # use std::prelude::v1::Box;
/// fn print(grid: Box<dyn Grid>)-> bool {
///     assert_eq!(grid.size(),Size{height:2,width:3});
///     false
/// }
/// run_gol(Size{height:2,width:3}, |grid| print(grid))
/// ```
pub fn run_gol<F>(size: Size, observer: F) where F: Fn(Box<GameOfLifeGrid>) -> bool {
    let mut rng = thread_rng();
    run_gol_for(observer, GameOfLifeGrid::new(size, |_, _| rng.gen_bool(0.5)));
}

fn run_gol_for<F, G>(observer: F, grid: G)
    where
        G: Evolving + Grid + Clone,
        F: Fn(Box<G>) -> bool,
{
    let mut g = grid;
    let mut sent = observer(Box::new(g.clone()));
    while sent {
        g = g.next_generation();
        sent = observer(Box::new(g.clone()));
    }
}

#[cfg(test)]
mod game_of_life_runner_should {
    use super::*;
    use crate::game_of_life::api_types::{Size, Column, Row};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use std::cell::RefCell;

    #[derive(Debug)]
    struct SequentialAdder {
        counter: AtomicUsize
    }

    impl SequentialAdder {
        fn new() -> Self {
            SequentialAdder { counter: AtomicUsize::new(0) }
        }
        fn increment(&self) {
            self.counter.fetch_add(1, Ordering::SeqCst);
        }
        fn current_value(&self) -> usize {
            return self.counter.load(Ordering::SeqCst);
        }
    }

    #[derive(Clone, Debug)]
    struct FakeGrid {
        number_of_calls: Arc<SequentialAdder>,
    }

    impl FakeGrid {
        fn new() -> Self {
            FakeGrid {
                number_of_calls: Arc::new(SequentialAdder::new()),
            }
        }
        fn calls(&self) -> Arc<SequentialAdder> {
            Arc::clone(&self.number_of_calls)
        }
    }

    impl Grid for FakeGrid {
        fn size(&self) -> Size {
            unimplemented!()
        }

        fn has_cell_at(&self, _row: Row, _column: Column) -> bool {
            unimplemented!()
        }
    }

    impl Evolving for FakeGrid {
        fn next_generation(self) -> Self {
            self.number_of_calls.increment();
            self
        }
    }

    #[test]
    fn call_closure_for_each_generation() {
        let grid = FakeGrid::new();
        let generations = grid.calls();
        let nb_sent = SequentialAdder::new();
        let values = RefCell::new(vec![]);
        let max_generation = 4;
        run_gol_for(|b| {
            nb_sent.increment();
            values.borrow_mut().push(b.number_of_calls.current_value());
            b.number_of_calls.current_value() < max_generation
        }, grid);
        assert_eq!(generations.current_value(), max_generation);
        assert_eq!(nb_sent.current_value(), max_generation + 1);
        assert_eq!(values.into_inner(), (0..=max_generation).collect::<Vec<usize>>());
    }
}
