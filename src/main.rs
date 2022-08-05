//#![allow(dead_code)]
//#![allow(unused_variables)]

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

pub struct Pendulum {
	angle: f64,
	ang_vel: f64, // angular velocity
	l: f64,
}

pub struct App {
	gl: GlGraphics, // OpenGL drawing backend.
}

impl App {
	fn render(&mut self, args: &RenderArgs, pendulums: &Vec<Pendulum>) {
		use graphics::*;

		const RED:   [f32; 4] = [1.0 , 0.0 , 0.0 , 1.0];
		const GREY:  [f32; 4] = [0.18, 0.18, 0.18, 1.0];

		self.gl.draw(args.viewport(), |c, gl| {
			clear(GREY, gl);

			for pendulum in pendulums {
				let circle = ellipse::circle(0.0,pendulum.l*100.0,20.0);
				ellipse(RED, circle, c.transform.trans(200.0,100.0).rot_rad(pendulum.angle), gl);
				line(RED, 1.0, [0.0, 0.0, 0.0, pendulum.l*100.0], c.transform.trans(200.0,100.0).rot_rad(pendulum.angle), gl); // first rot_rad()) then trans()
			}
		});
	}

	fn update(&mut self, args: &UpdateArgs, pendulums: &mut Vec<Pendulum>) {

		for pendulum in pendulums {
			let ang_acc = - 9.81 / pendulum.l * pendulum.angle.sin();
			pendulum.ang_vel += ang_acc * args.dt;
			pendulum.ang_vel *= 1.0 - 0.10 * args.dt; // the pendulum loses 10% speed per second
			pendulum.angle += pendulum.ang_vel * args.dt;
		}
	}
}

fn main() {
	let opengl = OpenGL::V3_2;

	// Create a Glutin window.
	let mut window: Window = WindowSettings::new("Pendulum", [400, 400])
		.graphics_api(opengl)
		.exit_on_esc(true)
		.build()
		.unwrap();

	// Create a new game and run it.
	let mut app = App {
		gl: GlGraphics::new(opengl),
	};

	let mut pendulums = vec![
		Pendulum { angle: 30.0_f64.to_radians(), ang_vel: 0.0, l: 1.4 },
		Pendulum { angle: -20.0_f64.to_radians(), ang_vel: 2.0, l: 2.242}
	];

	let mut events = Events::new(EventSettings::new());
	while let Some(e) = events.next(&mut window) {
		if let Some(args) = e.render_args() {
			app.render(&args, &pendulums);
		}

		if let Some(args) = e.update_args() {
			app.update(&args, &mut pendulums);
		}

	}
}
