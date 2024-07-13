pub struct Sequencer {
    pub phase: f32,
    pub frequency: f32,
    pub step: usize,
    pub steps: Vec<f32>,
}

impl Sequencer {
    pub fn new(steps: Vec<f32>) -> Sequencer {
        Sequencer {
            phase: 0.0,
            frequency: 440.0,
            step: 0,
            steps,
        }
    }

    pub fn process(&mut self, sample_rate: f32) -> f32 {
        self.phase += self.frequency / sample_rate;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
            self.step = (self.step + 1) % self.steps.len();
        }
        self.phase
    }
}
