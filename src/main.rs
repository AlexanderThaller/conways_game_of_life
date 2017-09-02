extern crate piston_window;
extern crate rand;
extern crate time;

use piston_window::*;
use rand::Rng;
use time::PreciseTime;

const HEIGHT: u32 = 1500;
const WIDTH: u32 = 800;
const SCALE: f64 = 4.0;

macro_rules! duration {
    ($name:expr, $code:block) => (
      let start = PreciseTime::now();

      $code

      let end = PreciseTime::now();
      let duration = start.to(end);

      println!("{} duration: {}", $name, duration);
    )
}

fn main() {
    let mut board = {
        fn scale_dimension(x: u32) -> usize {
            (x as f64 / SCALE).floor() as usize
        }

        let (rows, cols) = (scale_dimension(HEIGHT), scale_dimension(WIDTH));
        println!("rows {}, cols {}", rows, cols);

        Board::new_random(rows, cols)
    };

    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", [HEIGHT, WIDTH])
        .exit_on_esc(true)
        .build()
        .unwrap();

    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics| {
            clear([1.0; 4], graphics);

            duration!("grid_calculation", {
                board.step()
            });

            duration!("drawing", {
                for row in 1..board.rows {
                    for col in 1..board.columns {
                        if board.grid[row][col] {
                            rectangle(
                                [1.0, 0.0, 0.0, 1.0], // red
                                [
                                    row as f64 * SCALE,
                                    col as f64 * SCALE,
                                    1.0 * SCALE,
                                    1.0 * SCALE,
                                ],
                                context.transform,
                                graphics,
                            );
                        }
                    }
                }
            });
        });
    }
}

struct Board {
    grid: Vec<Vec<bool>>,

    rows: usize,
    columns: usize,
}

impl Board {
    fn new(rows: usize, columns: usize) -> Board {
        Board {
            grid: vec![vec![false; columns]; rows],
            rows: rows,
            columns: columns,
        }
    }

    fn new_random(rows: usize, columns: usize) -> Board {
        let mut grid = Board::new(rows, columns);

        {
            let mut rng = rand::thread_rng();
            for hpos in 1..rows as usize {
                for wpos in 1..columns as usize {

                    grid.grid[hpos][wpos] = rng.gen();
                }
            }
        }

        grid
    }

    fn step(&mut self) {
        let mut rng = rand::thread_rng();
        for hpos in 1..self.rows as usize {
            for wpos in 1..self.columns as usize {
                self.grid[hpos][wpos] = rng.gen();
            }
        }
    }
}
