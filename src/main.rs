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
	fn render(&mut self, args: &RenderArgs, pendulum: &Pendulum) {
		use graphics::*;

		const RED:   [f32; 4] = [1.0 , 0.0 , 0.0 , 1.0];
		const GREY:  [f32; 4] = [0.18, 0.18, 0.18, 1.0];

		self.gl.draw(args.viewport(), |c, gl| {
			clear(GREY, gl);

			let circle = ellipse::circle(0.0,pendulum.l,20.0);
			ellipse(RED, circle, c.transform.trans(200.0,100.0).rot_deg(pendulum.angle), gl);
			line(RED, 1.0, [0.0, 0.0, 0.0, pendulum.l], c.transform.trans(200.0,100.0).rot_deg(pendulum.angle), gl); // first rot_deg()) then trans()
		});
	}

	fn update(&mut self, args: &UpdateArgs, pendulum: &mut Pendulum, last_update: &mut std::time::SystemTime) {
		// args.dt is always 0.0083333, which is only correct if this function's execution time is neglegible
		// so I'm doing my own dt
		// I sampled all dt's for 1 second and they do actually add up to 1
		let now = std::time::SystemTime::now();
		let dt = now.duration_since(*last_update).unwrap().as_secs_f64();
		*last_update = now;

		/*
			Equation of motion for the pendulum:
				a = -g/l * sin(ϕ)

			Equation for the period of the pendulum:
				T = 2π * √(l/g)

			I set g/l = 50, so T should be 2π*√(1/50) = 0.89 seconds
			The actual period is however more than 6 seconds
		*/

		let ang_acc = - 50.0 * pendulum.angle.to_radians().sin();
		pendulum.ang_vel += ang_acc * dt;
		pendulum.angle += pendulum.ang_vel * dt;
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

	let mut pendulum = Pendulum { angle: 30.0, ang_vel: 0.0, l: 223.0 };

	let mut last_update = std::time::SystemTime::now();

	let mut events = Events::new(EventSettings::new());
	while let Some(e) = events.next(&mut window) {
		if let Some(args) = e.render_args() {
			app.render(&args, &pendulum);
		}

		if let Some(args) = e.update_args() {
			app.update(&args, &mut pendulum, &mut last_update);
		}

	}
}