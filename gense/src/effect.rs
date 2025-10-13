pub fn alpha(sample_rate: f32, cutoff: f32) -> f32 {
    let rc = 1.0 / (2.0 * std::f32::consts::PI * cutoff);
    let dt = 1.0 / sample_rate;
    dt / (rc + dt)
}

pub fn lpf() -> impl FnMut(f32, f32) -> f32 {
    let mut prev_output = 0.0;

    move |alpha: f32, input: f32| {
        let output = prev_output + alpha * (input - prev_output);
        prev_output = output;
        output
    }
}

pub fn hpf() -> impl FnMut(f32, f32) -> f32 {
    let mut prev_input = 0.0;
    let mut prev_output = 0.0;

    move |alpha: f32, input: f32| {
        let output = alpha * (prev_output + input - prev_input);
        prev_input = input;
        prev_output = output;
        output
    }
}

pub fn soft_clip(factor: f32, value: f32) -> f32 {
    (factor * value).tanh()
}

pub fn bpf(sample_rate: f32, center_freq: f32, q: f32) -> impl FnMut(f32) -> f32 {
    let omega = 2.0 * std::f32::consts::PI * center_freq / sample_rate;
    let alpha = omega.sin() / (2.0 * q);
    let cos_omega = omega.cos();
    let a0 = 1.0 + alpha;
    let b0 = alpha / a0;
    let b1 = 0.0;
    let b2 = -alpha / a0;
    let a1 = -2.0 * cos_omega / a0;
    let a2 = (1.0 - alpha) / a0;
    let mut x1 = 0.0;
    let mut x2 = 0.0;
    let mut y1 = 0.0;
    let mut y2 = 0.0;

    move |input: f32| {
        let output = b0 * input + b1 * x1 + b2 * x2 - a1 * y1 - a2 * y2;
        x2 = x1;
        x1 = input;
        y2 = y1;
        y1 = output;
        output
    }
}

pub fn delay(sample_rate: f32, delay_time: f32, feedback: f32) -> impl FnMut(f32) -> f32 {
    let buffer_size = (sample_rate * delay_time).ceil() as usize;
    let mut buffer = vec![0.0; buffer_size];
    let mut write_index = 0;

    move |input: f32| {
        let delayed_sample = buffer[write_index];
        let output = input + delayed_sample * feedback;
        buffer[write_index] = output;
        write_index = (write_index + 1) % buffer_size;
        output
    }
}
