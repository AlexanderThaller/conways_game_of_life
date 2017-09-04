extern crate rand;
use board::rand::Rng;

pub struct Board {
    pub grid: Vec<Vec<bool>>,

    pub rows: usize,
    pub columns: usize,
}

impl Board {
    pub fn new(rows: usize, columns: usize) -> Board {
        Board {
            grid: vec![vec![false; columns]; rows],
            rows: rows,
            columns: columns,
        }
    }

    pub fn random(mut self) -> Board {
        let mut rng = rand::thread_rng();
        for hpos in 1..self.rows as usize {
            for wpos in 1..self.columns as usize {

                self.grid[hpos][wpos] = rng.gen();
            }
        }

        self
    }

    pub fn step(&mut self) {
        let mut rng = rand::thread_rng();
        for hpos in 1..self.rows as usize {
            for wpos in 1..self.columns as usize {
                self.grid[hpos][wpos] = rng.gen();
            }
        }
    }
}
