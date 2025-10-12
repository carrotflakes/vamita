// cargo run --example render_to_wav

use gense::effect as fx;
use gense::envelope::Exponential as exp;
use gense::oscillator as osc;

fn main() {
    let sample_rate = 44100;

    let fun = {
        let mut time = osc::time(sample_rate as f32);
        let env = exp::new(0.0001, 0.5);
        let mut filter = fx::lpf();
        move || {
            let t = time();
            filter(
                fx::alpha(sample_rate as f32, 2000.0 * (env.get(t) + 0.1)),
                fx::soft_clip(
                    5.0,
                    (osc::saw(t * 440.0) + osc::noise(1, t * 1000.0)) * env.get(t),
                ) * 0.5,
            )
        }
    };

    let buffer = gense::render(1.0, sample_rate, fun);

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create("output.wav", spec).unwrap();
    for sample in buffer {
        let amplitude = (sample * i16::MAX as f32) as i16;
        writer.write_sample(amplitude).unwrap();
    }
}
