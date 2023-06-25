use num::Float;

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
    fn construct(r: f64, phi: f64, v_r: f64, v_phi: f64) -> OrbitState {
        OrbitState { r, phi, v_r, v_phi }
    }

    fn get_entries(&self) -> (f64, f64, f64, f64) {
        (self.r, self.phi, self.v_r, self.v_phi)
    }
}

impl OrbitUpdate {
    fn construct(dr: f64, dphi: f64, dv_r: f64, dv_phi: f64) -> OrbitUpdate {
        OrbitUpdate {
            dr,
            dphi,
            dv_r,
            dv_phi,
        }
    }

    fn get_entries(&self) -> (f64, f64, f64, f64) {
        (self.dr, self.dphi, self.dv_r, self.dv_phi)
    }
}

fn square<T>(x: T) -> T
where
    T: Float,
{
    x * x
}

fn cube<T>(x: T) -> T
where
    T: Float,
{
    x * x * x
}

fn ode_Newtonian(state: &OrbitState, M: f64) -> OrbitUpdate {
    let (r, _phi, v_r, v_phi) = state.get_entries();
    OrbitUpdate::construct(
        v_r,
        v_phi,
        r * square(v_phi) - M / square(r),
        -2. * v_r * v_phi / r,
    )
}

fn ode_Schwarzschild(state: &OrbitState, M: f64) -> OrbitUpdate {
    let (r, _phi, v_r, v_phi) = state.get_entries();

    let twoM = 2. * M; // 2M
    let rmtM = r - 2. * M; // r - 2M

    let dv_r = -M * rmtM / cube(r) + 3. * M / (r * rmtM) * square(v_r) + rmtM * square(v_phi);
    let dv_phi = twoM / (r * rmtM) * v_r * v_phi - 2. * v_r * v_phi / r;
    OrbitUpdate::construct(v_r, v_phi, dv_r, dv_phi)
}

fn step_euler<T>(state: &OrbitState, M: f64, dt: f64, f: T) -> OrbitState
where
    T: Fn(&OrbitState, f64) -> OrbitUpdate,
{
    let update = f(state, M);

    // TODO: x = x0 + x' * dt

    OrbitState::construct(0., 0., 0., 0.) // TODO: implement
}

fn step_rk4<T>(state: &OrbitState, M: f64, dt: f64, f: T) -> OrbitState
where
    T: Fn(&OrbitState, f64) -> OrbitUpdate,
{
    let update_1 = f(state, M);

    // TODO: RK4 steps

    OrbitState::construct(0., 0., 0., 0.) // TODO: implement
}
