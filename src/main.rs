extern crate piston_window;
extern crate time;
extern crate image;

#[macro_use]
extern crate log;
extern crate env_logger;

mod board;

use board::Cell;
use piston_window::*;
use std::env;
use time::PreciseTime;


macro_rules! duration {
    ($name:expr, $code:block) => (
      let start = PreciseTime::now();

      $code

      let end = PreciseTime::now();
      let duration = start.to(end);

      trace!("{} duration: {}", $name, duration);
    )
}

fn main() {
    env_logger::init().expect("can initialize logger");

    println!("");
    println!("Conways Game of Life");
    println!("");
    println!("P    : Pause/Resume simulation");
    println!("Space: Pause and step through one iteration");
    println!("D    : Display current board in terminal");
    println!("R    : Randomize board");
    println!("F    : Fill board");
    println!("G    : Fill with single glider");
    println!("B    : Fill with single block");
    println!("C    : Clear the board");
    println!("");

    let mut args = env::args();
    args.next();
    let height: u32 = args.next().unwrap_or("100".into()).parse().unwrap_or(100);
    let width: u32 = args.next().unwrap_or("100".into()).parse().unwrap_or(100);
    let scale: f64 = args.next().unwrap_or("1.0".into()).parse().unwrap_or(1.0);

    let mut board = {
        fn scale_dimension(x: u32, scale: f64) -> usize {
            (x as f64 / scale).floor() as usize
        }

        let (rows, cols) = (
            scale_dimension(height, scale),
            scale_dimension(width, scale),
        );

        info!("Board size: {}x{}", rows, cols);

        board::Board::new(rows, cols).block()
    };


    let mut window: PistonWindow = {
        let opengl = OpenGL::V3_2;
        WindowSettings::new("Conways Game of Life", [height, width])
            .exit_on_esc(true)
            .opengl(opengl)
            .samples(4)
            .build()
            .unwrap()
    };

    let mut canvas = image::ImageBuffer::new(board.rows as u32, board.columns as u32);
    let mut texture = Texture::from_image(&mut window.factory, &canvas, &TextureSettings::new())
        .unwrap();

    let mut running = false;
    let mut stepped = false;
    while let Some(e) = window.next() {
        if let Some(button) = e.press_args() {
            match button {
                Button::Keyboard(Key::P) => {
                    running = !running;
                    println!("running: {}", running)
                }
                Button::Keyboard(Key::R) => board = board.random(),
                Button::Keyboard(Key::F) => board = board.fill(),
                Button::Keyboard(Key::G) => board = board.glider(),
                Button::Keyboard(Key::C) => board = board.clear(),
                Button::Keyboard(Key::B) => board = board.block(),
                Button::Keyboard(Key::D) => println!("{}\n", board.display()),
                Button::Keyboard(Key::Space) => {
                    if running {
                        running = false
                    }

                    if !stepped {
                        board.step();
                        stepped = true
                    } else {
                        board.grow();
                        stepped = false
                    }
                }
                _ => {}
            }
        }

        if e.render_args().is_some() {
            duration!("drawing", {
                for row in 0..board.rows {
                    for col in 0..board.columns {
                        let color = match board.grid[row][col] {
                            Cell::Alive => [0, 0, 255, 255],
                            Cell::Dead => [255, 255, 255, 255],
                            Cell::Growing => [0, 255, 0, 255],
                            Cell::Dieing => [255, 0, 0, 255],
                        };

                        canvas.put_pixel(row as u32, col as u32, image::Rgba(color));
                    }
                }
            });

            texture.update(&mut window.encoder, &canvas).unwrap();

            window.draw_2d(&e, |context, graphics| {
                clear([1.0; 4], graphics);
                image(&texture, context.transform.scale(scale, scale), graphics);
            });
        }

        if e.update_args().is_some() {
            duration!("grid_calculation", {
                if running {
                    if !stepped {
                        board.step();
                        stepped = true
                    } else {
                        board.grow();
                        stepped = false
                    }
                }
            });
        }
    }
}
