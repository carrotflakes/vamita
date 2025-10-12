use gense::effect as fx;
use gense::envelope::Exponential as exp;
use gense::oscillator as osc;

fn main() {
    let sample_rate = 44100;

    let fun = {
        let mut time = osc::time(sample_rate as f32);
        let env = exp::new(0.0001, 0.1);
        let mut filter = fx::bpf(sample_rate as f32, 2000.0, 20.0);
        move || {
            let t = time();
            filter((osc::noise(1, t * 100.0)) * env.get(t)) * 2.0
        }
    };

    let buffer = gense::render(0.2, sample_rate, fun);

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create("shoot.wav", spec).unwrap();
    for sample in buffer {
        let amplitude = (sample * i16::MAX as f32) as i16;
        writer.write_sample(amplitude).unwrap();
    }
}
