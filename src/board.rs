extern crate rand;
use board::rand::Rng;

#[derive(Clone, PartialEq)]
pub enum Cell {
    Alive,
    Dead,
    Growing,
    Dieing,
}

pub struct Board {
    pub grid: Vec<Vec<Cell>>,

    pub rows: usize,
    pub columns: usize,
}

impl Board {
    pub fn new(rows: usize, columns: usize) -> Board {
        Board {
            grid: vec![vec![Cell::Dead; columns]; rows],
            rows: rows,
            columns: columns,
        }
    }

    pub fn random(mut self) -> Board {
        let mut rng = rand::thread_rng();
        for hpos in 1..self.rows as usize {
            for wpos in 1..self.columns as usize {

                if rng.gen() {
                    self.grid[hpos][wpos] = Cell::Alive
                }
            }
        }

        self
    }

    pub fn step(&mut self) {
        let mut rng = rand::thread_rng();
        for hpos in 1..self.rows as usize {
            for wpos in 1..self.columns as usize {

                if rng.gen() {
                    self.grid[hpos][wpos] = Cell::Alive
                } else {
                    self.grid[hpos][wpos] = Cell::Dead
                }
            }
        }
    }
}
