use cpal::{FromSample, Sample};
use std::sync::mpsc;

use crate::sequencer::Sequencer;
use std::sync::{Arc, Mutex};

pub struct AudioContext {
    channels: usize,
    receiver: mpsc::Receiver<usize>,
    //graph: AudioGraph,
    sample_rate: f32,
    sequencer: Arc<Mutex<Sequencer>>,
}

impl AudioContext {
    pub fn new(
        receiver: mpsc::Receiver<usize>,
        sample_rate: f32,
        sequencer: Arc<Mutex<Sequencer>>,
    ) -> AudioContext {
        AudioContext {
            channels: 2,
            receiver,
            //graph: AudioGraph::demo_graph(),
            sample_rate,
            sequencer,
        }
    }

    pub fn write_frame<T>(&mut self, output: &mut [T])
    where
        T: Sample + FromSample<f32>,
    {
        // Process messages
        while let Ok(message) = self.receiver.try_recv() {
            // if message == 0 {
            //     self.simple_synth.release();
            // } else {
            //     self.simple_synth.trigger_note(message as u8);
            // }
        }

        {
            let mut sequencer = self.sequencer.lock().unwrap();
            // Write frame
            for frame in output.chunks_mut(self.channels) {
                let value = sequencer.process(self.sample_rate);
                //let value = self.graph.process(self.sample_rate);
                frame[0] = T::from_sample(value);
                frame[1] = T::from_sample(value);
            }
        }
    }
}
