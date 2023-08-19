use circular_queue::{self, CircularQueue};
use num;
use raylib::prelude::*;
use std::f64::consts::PI;
use std::time::Duration;

pub mod orbit;

// TODO: terminate simulation when blow-up or singularity happens
// TODO: history tracking slows down program significantly.

fn convert_float_coordinates_to_pixel_coordinates(
    x: f64,
    y: f64,
    screen_width: i32,
    screen_height: i32,
    graphic_scale: f64,
) -> (i32, i32) {
    let x = x * graphic_scale;
    let y = y * graphic_scale;

    // FIXME: this cast here will panic when overflow occurs
    let x = num::clamp(x.floor() as i32 + screen_width / 2, 0, screen_width);
    let y = num::clamp(-y.floor() as i32 + screen_height / 2, 0, screen_height);

    return (x, y);
}

#[allow(non_snake_case)] // for variables
fn main() {
    // WINDOW SETTINGS
    let window_size_x_half = 320;
    let window_size_y_half = 240;
    let window_size_x = 2 * window_size_x_half;
    let window_size_y = 2 * window_size_y_half;

    // SIMULATION SETTINGS
    let M: f64 = 1.; // central mass
    let simulation_dt = 0.25; // simulation time step for ODE solvers
    let time_factor = 100.; // Think of this as a rate; # of updates to be shown in one unit of time.
    let graphic_dt = simulation_dt / time_factor; // TODO: doc
    let graphic_scale = 5.; // TODO: doc

    // TODO: initial conditions
    let r0 = 10.;
    let phi0 = 0. * PI;
    let v_r0 = 0.1;
    let v_phi0 = 0.0101 * PI;

    let mut simulation_time = 0.; // keep track of total time simulated

    let mut orbit_state_Newton = orbit::OrbitState::construct(r0, phi0, v_r0, v_phi0);
    let mut orbit_state_Schwarzschild = orbit::OrbitState::construct(r0, phi0, v_r0, v_phi0);

    const BUFFER_SIZE: usize = 1000;
    let mut history_Newton: CircularQueue<(i32, i32)> =
        circular_queue::CircularQueue::with_capacity(BUFFER_SIZE);
    let mut history_Schwarzschild: CircularQueue<(i32, i32)> =
        circular_queue::CircularQueue::with_capacity(1000);

    // Initialise buffers with the initial position of each
    {
        let (x_Newton, y_Newton, _, _) = orbit_state_Newton.to_Cartesian();
        let (x_Schwarzschild, y_Schwarzschild, _, _) = orbit_state_Schwarzschild.to_Cartesian();

        // Draw the orbit current positions
        let (x_Newton_pix, y_Newton_pix) = convert_float_coordinates_to_pixel_coordinates(
            x_Newton,
            y_Newton,
            window_size_x,
            window_size_y,
            graphic_scale,
        );
        let (x_Schwarzschild_pix, y_Schwarzschild_pix) =
            convert_float_coordinates_to_pixel_coordinates(
                x_Schwarzschild,
                y_Schwarzschild,
                window_size_x,
                window_size_y,
                graphic_scale,
            );

        history_Newton.push((x_Newton_pix, y_Newton_pix));
        history_Schwarzschild.push((x_Schwarzschild_pix, y_Schwarzschild_pix));
    }

    // TODO: accept user input for initial conditions?

    let (mut rl, thread) = raylib::init()
        .size(window_size_x, window_size_y)
        .title("Newtonian and General Relativistic Orbit Comparison")
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);

        // Draw center and axes
        let Schwarzschild_radius: f32 = (2. * M * graphic_scale) as f32;
        // HACK: this may panic if `M` is large
        d.draw_circle(
            window_size_x_half,
            window_size_y_half,
            Schwarzschild_radius,
            Color::BLACK,
        );
        // x-axis
        d.draw_line(
            0,
            window_size_y_half,
            window_size_x,
            window_size_y_half,
            Color::BLACK,
        );
        // y-axis
        d.draw_line(
            window_size_x_half,
            0,
            window_size_x_half,
            window_size_y,
            Color::BLACK,
        );

        // Draw in the total simulation time and increment
        d.draw_text(
            format!("t={:.2}", simulation_time).as_str(),
            10,
            20,
            10,
            Color::BLACK,
        );
        simulation_time += simulation_dt;

        // Evolution of system
        orbit_state_Newton =
            orbit::step_Euler(&orbit_state_Newton, M, simulation_dt, &orbit::ode_Newtonian);
        orbit_state_Schwarzschild = orbit::step_Euler(
            &orbit_state_Schwarzschild,
            M,
            simulation_dt,
            &orbit::ode_Schwarzschild,
        );

        // TODO: pause the simulation if the Schwarzschild radius is hit (coordinate singularity)
        // TODO: pause the simulation if either radius gets too large

        let (x_Newton, y_Newton, _, _) = orbit_state_Newton.to_Cartesian();
        let (x_Schwarzschild, y_Schwarzschild, _, _) = orbit_state_Schwarzschild.to_Cartesian();

        // Draw the orbit current positions
        let (x_Newton_pix, y_Newton_pix) = convert_float_coordinates_to_pixel_coordinates(
            x_Newton,
            y_Newton,
            window_size_x,
            window_size_y,
            graphic_scale,
        );
        let (x_Schwarzschild_pix, y_Schwarzschild_pix) =
            convert_float_coordinates_to_pixel_coordinates(
                x_Schwarzschild,
                y_Schwarzschild,
                window_size_x,
                window_size_y,
                graphic_scale,
            );

        d.draw_circle(x_Newton_pix, y_Newton_pix, 10., Color::RED);
        d.draw_circle(x_Schwarzschild_pix, y_Schwarzschild_pix, 10., Color::BLUE);

        // Draw history of orbits as well.
        // TODO: better way of testing if these are different enough? Possibly pass a function 
        let current_Newton = (x_Newton_pix, y_Newton_pix);
        let last_Newton = *history_Newton.iter().last().unwrap();
        if current_Newton != last_Newton {
            history_Newton.push((x_Newton_pix, y_Newton_pix));
        }

        let current_Schwarzschild = (x_Schwarzschild_pix, y_Schwarzschild_pix);
        let last_Schwarzschild = *history_Schwarzschild.iter().last().unwrap();
        if current_Schwarzschild != last_Schwarzschild {
            history_Schwarzschild.push((x_Schwarzschild_pix, y_Schwarzschild_pix));
        }

        history_Schwarzschild.push((x_Schwarzschild_pix, y_Schwarzschild_pix));

        for (x, y) in history_Newton.asc_iter() {
            d.draw_circle(*x, *y, 1., Color::RED);
        }
        for (x, y) in history_Schwarzschild.asc_iter() {
            d.draw_circle(*x, *y, 1., Color::BLUE);
        }

        // TODO: is this following line needed? better way to do? Will it work to set fps and let raylib take care of it?
        std::thread::sleep(Duration::from_secs_f64(graphic_dt));

        // TODO: possibly multithread the computations? Unsure if much benefit since we really want to sync up the frame drawing to some rescaling of time.
    }
}
