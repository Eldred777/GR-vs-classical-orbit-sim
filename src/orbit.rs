// ? I wonder if there's a way to generalise this so I extend this code to general ODE systems.
struct OrbitState {
    r: f64,
    phi: f64,
    v_r: f64,
    v_phi: f64,
}

// Keeping the updates separate for logic reasons.
struct OrbitUpdate {
    dr: f64,
    dphi: f64,
    dv_r: f64,
    dv_phi: f64,
}

impl OrbitState {
    fn construct(r: f64, phi: f64, v_r: f64, v_phi: f64) {
        struct OrbitState {
            r: r,
            phi: phi,
            v_r: v_r,
            v_phi: v_phi,
        }
    }
}

impl OrbitUpdate {
    fn construct(dr: f64, dphi: f64, dv_r: f64, dv_phi: f64) {
        struct OrbitUpdate {
            dr: dr,
            dphi: dphi,
            dv_r: dv_r,
            dv_phi: dv_phi,
        }
    }
}

fn Newtonian_system(&state: OrbitState, M: f64) -> OrbitUpdate {
    let (r, phi, v_r, v_phi) = state;
    OrbitUpdate::construct(
        v_r,
        v_phi,
        r * v_phi * *2 - M / r * *2,
        -2 * v_r * v_phi / r,
    )
}

fn Schwarzschild_system(&state: OrbitState, M: f64) -> OrbitUpdate {
    let (r, phi, v_r, v_phi) = state;
    let dv_r = -M * (r - 2 * M) / (r * *3)
        + 3 * M / (r * (r - 2 * M)) * v_r * *2
        + (r - 2 * M) * v_phi * *2;
    let dv_phi = 2 * M / (r * (r - 2 * M)) * v_r * v_phi - 2 * v_r * v_phi / r;
    OrbitUpdate::construct(v_r, v_phi, dv_r, dv_phi)
}

fn Euler(&state: OrbitState, &update: OrbitUpdate, M: f64, dt: f64, f: T) -> OrbitState
where
    T: Fn,
{
    OrbitState(0, 0, 0, 0) // TODO: implement
}

fn RK4(&state: OrbitState, &update: OrbitUpdate, M: f64, dt: f64, f: T) -> OrbitState
where
    T: Fn,
{
    OrbitState(0, 0, 0, 0) // TODO: implement
}
