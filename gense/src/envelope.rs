pub struct Exponential {
    pub value: f32,
    pub end_time: f32,
}

impl Exponential {
    pub fn new(value: f32, end_time: f32) -> Self {
        Self { value, end_time }
    }

    // 1 to value over end_time
    pub fn get(&self, t: f32) -> f32 {
        if t >= self.end_time {
            self.value
        } else {
            (self.value.ln() * (t / self.end_time)).exp()
        }
    }

    pub fn duration(&self) -> f32 {
        self.end_time
    }
}

pub struct Path {
    pub points: Vec<(f32, f32)>,
}

impl Path {
    pub fn new(points: Vec<(f32, f32)>) -> Self {
        Self { points }
    }

    pub fn get(&self, t: f32) -> f32 {
        if self.points.is_empty() {
            return 0.0;
        }

        if t <= self.points[0].0 {
            return self.points[0].1;
        }

        for i in 0..self.points.len() - 1 {
            let (t0, v0) = self.points[i];
            let (t1, v1) = self.points[i + 1];
            if t0 <= t && t <= t1 {
                let ratio = (t - t0) / (t1 - t0);
                return v0 + ratio * (v1 - v0);
            }
        }

        self.points.last().unwrap().1
    }

    pub fn duration(&self) -> f32 {
        if let Some((t, _)) = self.points.last() {
            *t
        } else {
            0.0
        }
    }
}