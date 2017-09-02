extern crate piston_window;
extern crate rand;

use piston_window::*;
use rand::distributions::{IndependentSample, Range};

const HEIGHT: u32 = 640;
const WIDTH: u32 = 480;

fn main() {
    let (height_between, widht_between) = (
        Range::new(0, HEIGHT as usize),
        Range::new(0, WIDTH as usize),
    );
    let mut rng = rand::thread_rng();

    let mut grid = vec![vec![false; WIDTH as usize]; HEIGHT as usize];

    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", [HEIGHT, WIDTH])
        .exit_on_esc(true)
        .build()
        .unwrap();

    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics| {
            clear([1.0; 4], graphics);

            {
                let (hpos, wpos) = {
                    (
                        height_between.ind_sample(&mut rng),
                        widht_between.ind_sample(&mut rng),
                    )
                };
                grid[hpos][wpos] = !grid[hpos][wpos];
            }

            for hpos in 1..HEIGHT as usize {
                for wpos in 1..WIDTH as usize {
                    if grid[hpos][wpos] {
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
    }
}
