pub fn sin(t: f32) -> f32 {
    (t * std::f32::consts::TAU).sin()
}

pub fn square(t: f32) -> f32 {
    if t % 1.0 < 0.5 { 1.0 } else { -1.0 }
}

pub fn saw(t: f32) -> f32 {
    2.0 * (t - (t + 0.5).floor())
}

pub fn triangle(t: f32) -> f32 {
    2.0 * (2.0 * (t - (t + 0.5).floor())).abs() - 1.0
}

pub fn noise(seed: u32, t: f32) -> f32 {
    let x = (t * 1000.0).floor() as u32 + seed;
    let x = (x >> 13) ^ x;
    let x = (x
        .wrapping_mul(x.wrapping_mul(x).wrapping_mul(15731).wrapping_add(789221))
        .wrapping_add(1376312589))
        & 0x7fffffff;
    1.0 - (x as f32 / 1073741824.0)
}

pub fn phase(sample_rate: f32) -> impl FnMut(f32) -> f32 {
    let mut phase = 0.0;
    move |freq| {
        let p = phase;
        phase = (phase + freq / sample_rate) % 1.0;
        p
    }
}

pub fn time(sample_rate: f32) -> impl FnMut() -> f32 {
    let delta = 1.0 / sample_rate;
    let mut acc = 0.0;
    move || {
        let a = acc;
        acc += delta;
        a
    }
}
