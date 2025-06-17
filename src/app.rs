use crate::audio::AudioEngine;
use crate::sequencer::Sequencer;
use crate::sample::SampleBank;
use std::time::Instant;

pub struct App {
    pub audio_engine: AudioEngine,
    pub sequencer: Sequencer,
    pub sample_bank: SampleBank,
    pub current_group: usize,
    pub current_pattern: usize,
    pub is_playing: bool,
    pub is_recording: bool,
    pub tempo: u32,
    pub last_tick: Instant,
    pub selected_pad: Option<usize>,
}

impl App {
    pub fn new() -> Self {
        let audio_engine = AudioEngine::new();
        let mut sample_bank = SampleBank::new();
        
        // Load default samples
        sample_bank.load_defaults();
        
        Self {
            audio_engine,
            sequencer: Sequencer::new(),
            sample_bank,
            current_group: 0,
            current_pattern: 0,
            is_playing: false,
            is_recording: false,
            tempo: 120,
            last_tick: Instant::now(),
            selected_pad: None,
        }
    }

    pub fn trigger_pad(&mut self, pad: usize) {
        if pad < 16 {
            // Play the sample
            if let Some(sample) = self.sample_bank.get_sample(self.current_group, pad) {
                self.audio_engine.play_sample(sample);
            }
            
            // Record if recording
            if self.is_recording && self.is_playing {
                self.sequencer.record_hit(
                    self.current_group,
                    self.current_pattern,
                    pad,
                );
            }
            
            self.selected_pad = Some(pad);
        }
    }

    pub fn toggle_playback(&mut self) {
        self.is_playing = !self.is_playing;
        if self.is_playing {
            self.sequencer.reset_position();
        }
    }

    pub fn toggle_recording(&mut self) {
        self.is_recording = !self.is_recording;
    }

    pub fn clear_pattern(&mut self) {
        self.sequencer.clear_pattern(self.current_group, self.current_pattern);
    }

    pub fn next_group(&mut self) {
        self.current_group = (self.current_group + 1) % 4;
    }

    pub fn prev_group(&mut self) {
        self.current_group = if self.current_group == 0 { 3 } else { self.current_group - 1 };
    }

    pub fn next_pattern(&mut self) {
        self.current_pattern = (self.current_pattern + 1) % 99;
    }

    pub fn prev_pattern(&mut self) {
        self.current_pattern = if self.current_pattern == 0 { 98 } else { self.current_pattern - 1 };
    }

    pub fn adjust_tempo(&mut self, delta: i32) {
        let new_tempo = (self.tempo as i32 + delta).clamp(60, 300) as u32;
        self.tempo = new_tempo;
    }

    pub fn tick(&mut self) {
        if self.is_playing {
            let now = Instant::now();
            let elapsed = now.duration_since(self.last_tick);
            let tick_duration = std::time::Duration::from_millis(60000 / (self.tempo * 4) as u64);
            
            if elapsed >= tick_duration {
                self.last_tick = now;
                
                // Get hits for current position
                let hits = self.sequencer.tick(self.tempo);
                
                // Play all hits
                for (group, pad) in hits {
                    if let Some(sample) = self.sample_bank.get_sample(group, pad) {
                        self.audio_engine.play_sample(sample);
                    }
                }
            }
        }
    }

    pub fn get_pattern_grid(&self) -> Vec<Vec<bool>> {
        self.sequencer.get_pattern_grid(self.current_group, self.current_pattern)
    }

    pub fn get_current_step(&self) -> usize {
        self.sequencer.get_current_step()
    }
}