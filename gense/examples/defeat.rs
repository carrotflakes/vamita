use gense::envelope::Exponential as exp;
use gense::oscillator as osc;

fn main() {
    let sample_rate = 44100;

    let fun = {
        let mut time = osc::time(sample_rate as f32);
        let mut ph = osc::phase(sample_rate as f32);
        let env = exp::new(0.0001, 1.0);
        let mut delay = gense::effect::delay(sample_rate as f32, 0.25, 0.25);
        move || {
            let t = time();
            delay(osc::triangle(ph(440.0 * env.get(t))) * env.get(t)) * 0.5
        }
    };

    let buffer = gense::render(2.0, sample_rate, fun);

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create("defeat.wav", spec).unwrap();
    for sample in buffer {
        let amplitude = (sample * i16::MAX as f32) as i16;
        writer.write_sample(amplitude).unwrap();
    }
}
