use gense::emitter as em;
use gense::envelope::Exponential as exp;
use gense::oscillator as osc;
use rand::{Rng, SeedableRng};

fn main() {
    let sample_rate = 44100;

    let fun = {
        let mut rng = rand::rngs::StdRng::seed_from_u64(1);
        let mut time = osc::time(sample_rate as f32);
        let env = exp::new(0.0001, 2.0);
        let em = em::emitter_stop_n(200, em::random_emitter(1, 0.001, 0.01));
        let mut em = em::emitter_processer(em);
        let mut mg = gense::granular::MultiGen::new(sample_rate as f32);
        move || {
            let t = time();
            while em(t) {
                mg.add(
                    {
                        let mut time = osc::time(sample_rate as f32);
                        let mut ph = osc::phase(sample_rate as f32);
                        let env = exp::new(0.0001, 0.05);
                        let fr = rng.random_range(240.0..250.0);
                        let gain = rng.random_range(0.01..0.5);
                        move || osc::sin(ph(fr)) * env.get(time()) * gain
                    },
                    0.05,
                );
            }
            mg.next() * env.get(t)
        }
    };

    let buffer = gense::render(2.0, sample_rate, fun);

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create("bomb.wav", spec).unwrap();
    for sample in buffer {
        let amplitude = (sample * i16::MAX as f32) as i16;
        writer.write_sample(amplitude).unwrap();
    }
}
