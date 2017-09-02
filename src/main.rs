extern crate piston_window;
extern crate rand;
extern crate time;

use piston_window::*;
use rand::Rng;
use time::PreciseTime;

const HEIGHT: u32 = 320;
const WIDTH: u32 = 240;

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

    let mut board = Board::new_random(HEIGHT as usize, WIDTH as usize);

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
                for hpos in 1..HEIGHT as usize {
                    for wpos in 1..WIDTH as usize {
                        if board.grid[hpos][wpos] {
                            rectangle(
                                [1.0, 0.0, 0.0, 1.0], // red
                                [hpos as f64, wpos as f64, 1.0, 1.0],
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

    height: usize,
    width: usize,
}

impl Board {
    fn new(height: usize, width: usize) -> Board {
        Board {
            grid: vec![vec![false; width]; height],
            height: height,
            width: width,
        }
    }

    fn new_random(height: usize, width: usize) -> Board {
        let mut grid = Board::new(height, width);

        {
            let mut rng = rand::thread_rng();
            for hpos in 1..height as usize {
                for wpos in 1..width as usize {

                    grid.grid[hpos][wpos] = rng.gen();
                }
            }
        }

        grid
    }

    fn step(&mut self) {
        let mut rng = rand::thread_rng();
        for hpos in 1..self.height as usize {
            for wpos in 1..self.width as usize {
                self.grid[hpos][wpos] = rng.gen();
            }
        }
    }
}
