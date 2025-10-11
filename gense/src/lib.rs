pub mod oscillator;
pub mod envelope;
pub mod effect;

pub fn render<T: Copy + Send + Sync>(duration: f32, sample_rate: u32, mut f: impl FnMut(f32) -> T) -> Vec<T> {
    let total_samples = (duration * sample_rate as f32) as usize;
    (0..total_samples)
        .map(|i| {
            let t = i as f32 / sample_rate as f32;
            f(t)
        })
        .collect()
}
