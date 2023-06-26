use circular_queue::{self, CircularQueue};
use num;
use raylib::prelude::*;
use std::f64::consts::PI;
use std::time::Duration;

pub mod orbit;

// TODO: terminate simulation when blow-up happens

fn convert_float_coordinates_to_pixel_coordinates(
    x: f64,
    y: f64,
    screen_width: i32,
    screen_height: i32,
    graphic_scale: f64,
) -> (i32, i32) {
    let x = x * graphic_scale;
    let y = y * graphic_scale;

    let x = num::clamp(x.floor() as i32 + screen_width / 2, 0, screen_width);
    let y = num::clamp(-y.floor() as i32 + screen_height / 2, 0, screen_height);

    return (x, y);
}

#[allow(non_snake_case)]
#[allow(unused_variables)] // TODO: remove when working prototype
fn main() {
    // WINDOW SETTINGS
    let window_size_x_half = 320;
    let window_size_y_half = 240;
    let window_size_x = 2 * window_size_x_half;
    let window_size_y = 2 * window_size_y_half;

    // SIMULATION SETTINGS
    let M = 1.; // central mass
    let simulation_dt = 0.1; // simulation time step for ODE solvers
    let time_factor = 100.; // Think of this as a rate; # of updates to be shown in one unit of time.
    let graphic_dt = simulation_dt / time_factor; //
    let graphic_scale = 10.;

    // TODO: initial conditions
    let r0 = 10.;
    let phi0 = 0. * PI;
    let v_r0 = 0.;
    let v_phi0 = 0.01 * PI;

    // TODO: we aim to get the newton simulation working first and then add the Schwarzschild simulation in later.
    let mut orbit_state_Newton = orbit::OrbitState::construct(r0, phi0, v_r0, v_phi0);
    // let mut orbit_state_Schwarzschild = orbit::OrbitState::construct(r0, phi0, v_r0, v_phi0);

    // const BUFFER_SIZE: usize = 1000;
    // let mut buffer_Newton: CircularQueue<(i32, i32)> =
    //     circular_queue::CircularQueue::with_capacity(BUFFER_SIZE);
    // let buffer_Schwarzschild: CircularQueue<(i32, i32)> =
    //     circular_queue::CircularQueue::with_capacity(1000);

    // TODO: accept user input for initial conditions?

    let (mut rl, thread) = raylib::init()
        .size(window_size_x, window_size_y)
        .title("Newtonian and General Relativistic Orbit Comparison")
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);

        // Draw center and axes
        d.draw_circle(window_size_x_half, window_size_y_half, 10.0, Color::BLACK);
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

        // Evolution of system
        orbit_state_Newton =
            orbit::step_Euler(&orbit_state_Newton, M, simulation_dt, &orbit::ode_Newtonian);
        let (x, y, _, _) = orbit_state_Newton.to_Cartesian();
        // buffer_Newton.push(convert_float_coordinates_to_pixel_coordinates(
        //     x,
        //     y,
        //     window_size_x,
        //     window_size_y,
        // ));

        // Draw the orbits
        // let (x, y, _, _) = orbit_state_Newton.to_Cartesian();
        let (x, y) = convert_float_coordinates_to_pixel_coordinates(
            x,
            y,
            window_size_x,
            window_size_y,
            graphic_scale,
        );
        d.draw_circle(x, y, 10., Color::RED);

        // TODO: draw history of orbits with circular buffer

        // TODO: is this following line needed? better way to do? Will it work to set fps and let raylib take care of it?
        std::thread::sleep(Duration::from_secs_f64(graphic_dt));

        // TODO: simulation
        // TODO: possibly multithread the computations? Unsure if much benefit since we really want to sync up the frame drawing to some rescaling of time.
    }
}
