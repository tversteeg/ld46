use cpal::traits::{EventLoopTrait, HostTrait};
use std::{
    sync::{Arc, Mutex},
    thread,
};
use usfx::{Mixer, Sample};

// Audio quality
const SAMPLE_RATE: usize = 44_100;

/// Manages the audio.
#[derive(Default)]
pub struct Audio {
    mixer: Arc<Mutex<Mixer>>,
}

impl Audio {
    /// Instantiate a new audio object without a generator.
    pub fn new() -> Self {
        Self {
            mixer: Arc::new(Mutex::new(Mixer::new(SAMPLE_RATE))),
        }
    }

    /// Play a sample.
    pub fn play(&mut self, sample: Sample) {
        self.mixer.lock().unwrap().play(sample);
    }

    /// Start a thread which will emit the audio.
    pub fn run(&mut self) {
        // Setup the audio system
        let host = cpal::default_host();
        let event_loop = host.event_loop();

        let device = host
            .default_output_device()
            .expect("no output device available");

        let format = cpal::Format {
            channels: 1,
            sample_rate: cpal::SampleRate(SAMPLE_RATE as u32),
            data_type: cpal::SampleFormat::F32,
        };

        let stream_id = event_loop
            .build_output_stream(&device, &format)
            .expect("could not build output stream");

        event_loop
            .play_stream(stream_id)
            .expect("could not play stream");

        let mixer = self.mixer.clone();

        thread::spawn(move || {
            event_loop.run(move |stream_id, stream_result| {
                let stream_data = match stream_result {
                    Ok(data) => data,
                    Err(err) => {
                        eprintln!("an error occurred on stream {:?}: {}", stream_id, err);
                        return;
                    }
                };

                match stream_data {
                    cpal::StreamData::Output {
                        buffer: cpal::UnknownTypeOutputBuffer::F32(mut buffer),
                    } => mixer.lock().unwrap().generate(&mut buffer),
                    _ => panic!("output type buffer can not be used"),
                }
            });
        });
    }
}
