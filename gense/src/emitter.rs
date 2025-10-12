use rand::Rng;

pub fn periodic_emitter(
    duration: f32,
) -> impl FnMut() -> f32 + Send + Sync + 'static {
    move || duration
}

pub fn random_emitter(
    seed: u64,
    min_duration: f32,
    max_duration: f32,
) -> impl FnMut() -> f32 + Send + Sync + 'static {
    let mut rng: rand::rngs::StdRng = rand::SeedableRng::seed_from_u64(seed);
    move || rng.random_range(min_duration..max_duration)
}

pub fn emitter_stop_n(n: u32, mut trigger: impl FnMut() -> f32 + Send + Sync + 'static)
    -> impl FnMut() -> f32 + Send + Sync + 'static {
    let mut count = 0;
    move || {
        if count < n {
            count += 1;
            trigger()
        } else {
            f32::INFINITY
        }
    }
}

pub fn emitter_processer(mut emitter: impl FnMut() -> f32 + Send + Sync + 'static) -> impl FnMut(f32) -> bool + Send + Sync + 'static {
    let mut next_trigger_time = emitter();
    move |t| {
        if t >= next_trigger_time {
            next_trigger_time += emitter();
            true
        } else {
            false
        }
    }
}
