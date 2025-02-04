use rodio::{OutputStream, Sink, Source, source};
use rodio::source::{SineWave, Mix};
use rand::Rng;
use std::time::Duration;

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    
    let mut rng = rand::thread_rng();
    
    /*for _ in 0..10 {
        let source = SineWave::new(rng.gen_range(0.0..1000.0))
            .take_duration(Duration::from_secs_f32(0.10))
            .amplify(0.1);
        sink.append(source);
    }*/

    let sinks: Vec<Sink> = (0..5)
        .map(|_| Sink::try_new(&stream_handle).unwrap())
        .collect();

    let frequencies = [138.591, 207.652, 246.942, 293.665, 391.995]; // C3, G3, D4, E4, G4
    for (sink, &freq) in sinks.iter().zip(frequencies.iter()) {
        let source = SineWave::new(freq)
            .take_duration(Duration::from_secs_f32(1.0))
            .amplify(0.2);
        sink.append(source);
    }

    /*let final_tone = SineWave::new(440.0)
        .take_duration(Duration::from_secs_f32(0.25))
        .amplify(0.20);
    sink.append(final_tone);
    */

    //sink.sleep_until_end();

    for sink in sinks {
        sink.sleep_until_end();
    }
}