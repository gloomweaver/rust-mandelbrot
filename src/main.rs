use num::{complex::Complex64, Complex};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{EventLoop, EventSettings, Events, RenderArgs, RenderEvent, WindowSettings};
use piston_window::PistonWindow;

const WIDTH: u32 = 1200;
const HEIGHT: u32 = 800;
const MAX_ITERATIONS: i32 = 80;
const RE_START: f64 = -2.0;
const RE_END: f64 = 1.0;
const IM_START: f64 = -1.0;
const IM_END: f64 = 1.0;

fn mandlerbrot(c: Complex64) -> i32 {
    let mut z = Complex::new(0.0, 0.0);
    let mut n = 0;

    while z.norm() <= 2.0 && n < MAX_ITERATIONS {
        z = z * z + c;
        n += 1;
    }
    return n;
}

pub struct App {
    gl: GlGraphics,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        self.gl.draw(args.viewport(), |context, gl| {
            clear(BLACK, gl);

            for x in 0..WIDTH {
                for y in 0..HEIGHT {
                    let complex: Complex64 = {
                        let re = RE_START + (x as f64 / WIDTH as f64) * (RE_END - RE_START);
                        let im = IM_START + (y as f64 / HEIGHT as f64) * (IM_END - IM_START);
                        Complex { re, im }
                    };
                    let point = rectangle::square(0.0, 0.0, 1.0);
                    let iterations = mandlerbrot(complex);
                    let color =
                        (255.0 - ((iterations * 255) as f32 / MAX_ITERATIONS as f32)) / 256.0;
                    let transform = context.transform.trans(x as f64, y as f64);
                    rectangle([1.0, 1.0, 1.0, color], point, transform, gl);
                }
            }
        })
    }
}

fn main() {
    let opengl = OpenGL::V4_5;

    let mut window: PistonWindow = WindowSettings::new("Mandelbrot set", [WIDTH, HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
    };

    let mut events = Events::new(EventSettings::new().lazy(true));
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }
    }
}
