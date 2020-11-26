mod circle;

use circle::Circle;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{EventSettings, Events, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent, WindowSettings};

pub struct App {
    gl: GlGraphics,
    window: GlutinWindow,
    circles: Vec<Circle>,
}

impl App {
    pub fn new(opengl: OpenGL, size: [u32; 2]) -> Self {
        App {
            gl: GlGraphics::new(opengl),
            window: WindowSettings::new("gravity", size)
                .graphics_api(opengl)
                .exit_on_esc(true)
                .build()
                .unwrap(),
            circles: vec![],
        }
    }

    pub fn run(&mut self) {
        let event = Events::new(EventSettings::new());
        while let Some(event) = events.next(&mut self.window) {
            if let Some(args) = event.render_args() {
                self.render(&args);
            }

            if let Some(args) = event.update_args() {
                self.update(&args);
            }
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {

    }

    pub fn update(&mut self, args: &UpdateArgs) {

    }
}

impl Default for App {
    fn default() -> Self {
        let opengl = OpenGL::V3_3;
        let size: [u32; 2] = [800u32, 600u32];

        App::new(opengl, size)
    }
}
