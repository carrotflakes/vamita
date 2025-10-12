use gense::envelope::Exponential as exp;
use gense::oscillator as osc;

fn main() {
    let sample_rate = 44100;

    let fun = {
        let mut ph1 = osc::phase(sample_rate as f32);
        let mut ph2 = osc::phase(sample_rate as f32);
        let env = exp::new(0.0001, 0.1);
        move |t| (osc::sin(ph1(880.0 + 800.0 * env.get(t) * osc::sin(ph2(1100.0)))) * env.get(t) * 0.25)
    };

    let buffer = gense::render(0.3, sample_rate, fun);

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create("exp.wav", spec).unwrap();
    for sample in buffer {
        let amplitude = (sample * i16::MAX as f32) as i16;
        writer.write_sample(amplitude).unwrap();
    }
}
