pub struct MultiGen {
    sample_rate: f32,
    gens: Vec<(Box<dyn FnMut() -> f32 + Send + Sync>, i32)>,
}

impl MultiGen {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            sample_rate,
            gens: Vec::new(),
        }
    }

    pub fn add<F>(&mut self, gen_fn: F, duration: f32)
    where
        F: FnMut() -> f32 + Send + Sync + 'static,
    {
        let total_samples = (duration * self.sample_rate) as i32;
        self.gens.push((Box::new(gen_fn), total_samples));
    }

    pub fn next(&mut self) -> f32 {
        let output = self
            .gens
            .iter_mut()
            .map(|(g, rem)| {
                *rem -= 1;
                g()
            })
            .sum();
        self.gens.retain(|(_, rem)| *rem != 0);
        output
    }
}
