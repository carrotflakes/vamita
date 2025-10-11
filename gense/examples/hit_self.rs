use gense::envelope::Exponential as exp;
use gense::oscillator as osc;

fn main() {
    let sample_rate = 44100;

    let fun = {
        let mut ph = osc::phase(sample_rate as f32);
        let env = exp::new(0.0001, 0.5);
        let fenv = gense::envelope::Path::new(vec![(0.0, 200.0), (0.1, 100.0), (0.2, 200.0)]);
        move |t| (osc::triangle(ph(fenv.get(t))) * env.get(t) * 0.25)
    };

    let buffer = gense::render(0.3, sample_rate, fun);

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create("hit_self.wav", spec).unwrap();
    for sample in buffer {
        let amplitude = (sample * i16::MAX as f32) as i16;
        writer.write_sample(amplitude).unwrap();
    }
}
