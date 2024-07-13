use std::sync::mpsc;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, Sample, SampleFormat, Stream};

mod context;
use context::AudioContext;

use super::sequencer::Sequencer;
use std::sync::{Arc, Mutex};

pub struct AudioSystem {
    sender: mpsc::Sender<usize>,
    stream: Stream,
}

impl AudioSystem {
    pub fn new(sequencer: Arc<Mutex<Sequencer>>) -> AudioSystem {
        println!("Audio system");

        // Get audio hosts
        let host = cpal::default_host();

        let device = host
            .default_output_device()
            .expect("no output device available");

        for device in host.output_devices().unwrap() {
            println!("{:?}", device.name().unwrap());
        }

        let mut supported_configs_range = device
            .supported_output_configs()
            .expect("error while querying configs");

        let supported_config = supported_configs_range
            .next()
            .expect("no supported config?!")
            .with_max_sample_rate();
        //.with_sample_rate(SampleRate(44100));

        // Get config and sample_format
        let sample_format = supported_config.sample_format();
        let config: cpal::StreamConfig = supported_config.into();

        println!("{:?}", config);
        println!("Sample format: {:?}", sample_format);

        // Initialize sequencer context
        let (sender, receiver) = mpsc::channel();
        let mut ctx = AudioContext::new(receiver, config.sample_rate.0 as f32, sequencer);

        let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);

        // Construct stream for different types
        let stream = match sample_format {
            SampleFormat::F32 => device.build_output_stream(
                &config,
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| ctx.write_frame(data),
                err_fn,
                None,
            ),
            SampleFormat::I16 => device.build_output_stream(
                &config,
                move |data: &mut [i16], _: &cpal::OutputCallbackInfo| ctx.write_frame(data),
                err_fn,
                None,
            ),
            SampleFormat::U16 => device.build_output_stream(
                &config,
                move |data: &mut [u16], _: &cpal::OutputCallbackInfo| ctx.write_frame(data),
                err_fn,
                None,
            ),
            _ => panic!("unhandled sample format!"),
        }
        .unwrap();

        stream.play().unwrap();

        AudioSystem {
            sender,
            stream, // handle
        }
    }
}
