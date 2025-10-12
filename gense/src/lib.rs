pub mod effect;
pub mod emitter;
pub mod envelope;
pub mod granular;
pub mod oscillator;

pub fn render<T: Copy + Send + Sync>(
    duration: f32,
    sample_rate: u32,
    mut f: impl FnMut() -> T,
) -> Vec<T> {
    let total_samples = (duration * sample_rate as f32) as usize;
    (0..total_samples).map(|_| f()).collect()
}
