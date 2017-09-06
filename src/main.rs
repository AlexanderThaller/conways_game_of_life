extern crate piston_window;
extern crate time;

mod board;

use board::Cell;
use piston_window::*;
use std::thread;
use time::PreciseTime;

const HEIGHT: u32 = 1600;
const WIDTH: u32 = 900;
const SCALE: f64 = 2.0;

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

        board::Board::new(rows, cols).random()
    };


    let mut window: PistonWindow = {
        let opengl = OpenGL::V3_2;
        WindowSettings::new("Conways Game of Life", [HEIGHT, WIDTH])
            .exit_on_esc(true)
            .opengl(opengl)
            .samples(4)
            .build()
            .unwrap()
    };

    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics| {
            clear([1.0; 4], graphics);

            duration!("drawing", {
                for row in 1..board.rows {
                    for col in 1..board.columns {
                        if board.grid[row][col] == Cell::Alive {
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

            duration!("grid_calculation", {
                board.step()
            });
        });

        // thread::sleep(std::time::Duration::from_millis(2000));
    }
}
