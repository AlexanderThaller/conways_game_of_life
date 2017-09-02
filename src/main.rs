extern crate piston_window;

use piston_window::*;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", [500, 500])
        .exit_on_esc(true)
        .build()
        .unwrap();

    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics| {
            rectangle(
                [1.0, 0.0, 0.0, 1.0], // red
                [0.0, 0.0, 10.0, 10.0],
                context.transform,
                graphics,
            );
        });
    }
}
