extern crate piston_window;

use piston_window::*;
use rand::prelude::*;

// use glutin_window::GlutinWindow as Window;
// use opengl_graphics::{GlGraphics, OpenGL};
// use piston::event_loop::{EventSettings, Events};
// use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
// use piston::window::WindowSettings;
// use piston::EventLoop;

pub struct App{
    rotation: f64,  // Rotation for the square.
    timeElapsed: f64,
    rendercount: f64,
    pos: Vec<(f64,f64, [f32; 4])>,
}

impl App {
    fn render(&mut self, args: &Event, window:&mut PistonWindow) {
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 20.0);
        let rotation = self.rotation;
        // let (x, y) = (window.size().width / 2.0, window.size().height / 2.0);

        let mypos = &self.pos;
        window.draw_2d( args, |c, g, _| {
            // Clear the screen.
            clear(GREEN, g);

            for pair in mypos {
                let (x, y) = (pair.0, pair.1);
                let transform = c
                    .transform
                    .trans(x, y)
                    .rot_rad(rotation)
                    .trans(-10.0, -10.0);

                // Draw a box rotating around the middle of the screen.
                rectangle( pair.2, square, transform, g);
            }

        });
        self.rendercount += 1.;
        if self.timeElapsed > 1. {
            println!("FPS: {}", self.rendercount/self.timeElapsed);
            self.rendercount=0.;
            self.timeElapsed=0.;
        }
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;

        self.timeElapsed += args.dt;
    }
}

pub fn run() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: PistonWindow = WindowSettings::new("spinning-square", [512, 512])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    window.set_max_fps(100000);

    let mut rng = rand::thread_rng();

    let mut positions = Vec::new();
    for _ in 1 .. 1000 {
        let (x, y) = (rng.gen_range(0. .. 512.), rng.gen_range(0. .. 512.));
        positions.push((x, y, [rng.gen(), rng.gen(), rng.gen(), 1.0]));
    }
    // Create a new game and run it.
    let mut app = App {
        rotation: 0.0,
        timeElapsed: 0.0,
        rendercount: 0.0,
        pos:positions,
    };

    let mut events = Events::new(EventSettings::new());
    events.set_max_fps(100000);
    while let Some(e) = events.next(&mut window)  {
        if let Some(args) = e.render_args() {
            app.render(&e, &mut window);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
