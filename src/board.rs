extern crate rand;

use board::rand::Rng;
use std::fmt;

#[derive(Clone, PartialEq, Debug)]
pub enum Cell {
    Alive,
    Dead,
    Growing,
    Dieing,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Cell::Alive => write!(f, "{}", '@'),
            Cell::Dead => write!(f, "{}", ' '),
            Cell::Growing => write!(f, "{}", '+'),
            Cell::Dieing => write!(f, "{}", '#'),
        }
    }
}

impl Cell {
    pub fn is_alive(&self) -> bool {
        match *self {
            Cell::Alive => true,
            _ => false,
        }
    }

    pub fn is_dead(&self) -> bool {
        match *self {
            Cell::Dead => true,
            _ => false,
        }
    }

    pub fn is_alive_or_dieing(&self) -> bool {
        match *self {
            Cell::Alive | Cell::Dieing => true,
            Cell::Dead | Cell::Growing => false,
        }
    }
}

pub type Grid = Vec<Vec<Cell>>;

pub struct Board {
    pub grid: Grid,

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
        for hpos in 0..self.rows as usize {
            for wpos in 0..self.columns as usize {

                if rng.gen() {
                    self.grid[hpos][wpos] = Cell::Alive
                }
            }
        }

        self
    }

    pub fn fill(mut self) -> Board {
        for hpos in 0..self.rows as usize {
            for wpos in 0..self.columns as usize {
                self.grid[hpos][wpos] = Cell::Alive
            }
        }

        self
    }

    pub fn clear(mut self) -> Board {
        for hpos in 0..self.rows as usize {
            for wpos in 0..self.columns as usize {
                self.grid[hpos][wpos] = Cell::Dead
            }
        }

        self
    }

    pub fn block(mut self) -> Board {
        // @@
        // @@

        self.grid[0][0] = Cell::Alive;
        self.grid[0][1] = Cell::Alive;
        self.grid[1][0] = Cell::Alive;
        self.grid[1][1] = Cell::Alive;

        self
    }

    pub fn glider(mut self) -> Board {
        // #@#
        // ##@
        // @@@

        self.grid[0][1] = Cell::Alive;

        self.grid[1][2] = Cell::Alive;

        self.grid[2][0] = Cell::Alive;
        self.grid[2][1] = Cell::Alive;
        self.grid[2][2] = Cell::Alive;

        self
    }

    pub fn display(&self) -> String {
        let mut out = String::new();
        out.push(' ');
        for _ in 0..self.columns as usize {
            out.push('-');
        }
        out.push(' ');
        out.push('\n');

        for hpos in 0..self.rows as usize {
            out.push('|');
            for wpos in 0..self.columns as usize {
                out.push_str(format!("{}", self.grid[hpos][wpos]).as_str());
            }
            out.push('|');

            out.push('\n');
        }

        out.push(' ');
        for _ in 0..self.columns as usize {
            out.push('-');
        }
        out.push(' ');

        out
    }

    pub fn step_and_grow(&mut self) {
        self.step();
        self.grow();
    }

    pub fn step(&mut self) {
        for hpos in 0..self.rows as usize {
            for wpos in 0..self.columns as usize {
                self.grid[hpos][wpos] = self.new_cell_state(hpos, wpos)
            }
        }
    }

    pub fn grow(&mut self) {
        for hpos in 0..self.rows as usize {
            for wpos in 0..self.columns as usize {
                let new_state = match self.grid[hpos][wpos] {
                    Cell::Alive | Cell::Growing => Cell::Alive,
                    Cell::Dead | Cell::Dieing => Cell::Dead,
                };

                self.grid[hpos][wpos] = new_state
            }
        }
    }

    fn new_cell_state(&mut self, hpos: usize, wpos: usize) -> Cell {
        let neighbors = self.get_cell_neighbors(hpos, wpos);
        let alive_cells: Vec<_> = neighbors
            .iter()
            .filter(|x| x.is_alive_or_dieing())
            .collect();


        let alive = alive_cells.len();

        let curr = &self.grid[hpos][wpos];
        self.calculate_new_cell_state(curr, alive)
    }

    fn calculate_new_cell_state(&self, curr: &Cell, alive: usize) -> Cell {
        if curr.is_alive() {
            if alive < 2 {
                return Cell::Dieing;
            }

            if alive == 2 || alive == 3 {
                return Cell::Alive;
            }

            if alive > 3 {
                return Cell::Dieing;
            }
        }

        if curr.is_dead() && alive == 3 {
            return Cell::Growing;
        }

        curr.clone()
    }

    fn get_cell_neighbors(&self, hpos: usize, wpos: usize) -> Vec<Cell> {
        let north = {
            if hpos == 0 {
                Cell::Dead
            } else {
                self.grid[hpos - 1][wpos].clone()
            }
        };

        let north_east = {
            if hpos == 0 || wpos == self.columns - 1 {
                Cell::Dead
            } else {
                self.grid[hpos - 1][wpos + 1].clone()
            }
        };

        let east = {
            if wpos == self.columns - 1 {
                Cell::Dead
            } else {
                self.grid[hpos][wpos + 1].clone()
            }
        };

        let south_east = {
            if hpos == self.rows - 1 || wpos == self.columns - 1 {
                Cell::Dead
            } else {
                self.grid[hpos + 1][wpos + 1].clone()
            }
        };

        let south = {
            if hpos == self.rows - 1 {
                Cell::Dead
            } else {
                self.grid[hpos + 1][wpos].clone()
            }
        };

        let south_west = {
            if hpos == self.rows - 1 || wpos == 0 {
                Cell::Dead
            } else {
                self.grid[hpos + 1][wpos - 1].clone()
            }
        };

        let west = {
            if wpos == 0 {
                Cell::Dead
            } else {
                self.grid[hpos][wpos - 1].clone()
            }
        };

        let north_west = {
            if hpos == 0 || wpos == 0 {
                Cell::Dead
            } else {
                self.grid[hpos - 1][wpos - 1].clone()
            }
        };

        debug!(
            "\nhpos {}, wpos {}\n --- \n|{}{}{}|\n|{}{}{}|\n|{}{}{}|\n --- ",
            hpos,
            wpos,

            north_west,
            north,
            north_east,

            west,
                self.grid[hpos][wpos],
            east,

            south_west,
            south,
            south_east,
        );

        vec![
            north,
            north_east,
            east,
            south_east,
            south,
            south_west,
            west,
            north_west,
        ]
    }
}
