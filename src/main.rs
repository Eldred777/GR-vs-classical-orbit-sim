use raylib::prelude::*;

fn main() {
    // WINDOW SETTINGS
    let window_size_x_half = 320;
    let window_size_y_half = 240;
    let window_size_x = 2 * window_size_x_half;
    let window_size_y = 2 * window_size_y_half;

    // SIMULATION SETTINGS
    // TODO: initial conditions
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

        // TODO: simulation
        // TODO: possibly multithread the computations? Unsure if much benefit since we really want to sync up the frame drawing to some rescaling of time. 

        // TODO: draw on it
    }
}
