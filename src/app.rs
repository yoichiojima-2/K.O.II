use crate::audio::AudioEngine;
use crate::sequencer::Sequencer;
use crate::sample::SampleBank;
use std::time::Instant;

pub struct App {
    pub audio_engine: AudioEngine,
    pub sequencer: Sequencer,
    pub sample_bank: SampleBank,
    pub current_group: usize,
    pub group_patterns: [usize; 4], // Each group has its own current pattern
    pub is_playing: bool,
    pub is_recording: bool,
    pub tempo: u32,
    pub last_tick: Instant,
    pub selected_pad: Option<usize>,
    pub flashing_pads: Vec<(usize, usize)>, // (group, pad) pairs that are currently flashing
    pub flash_timer: Instant,
}

impl App {
    pub fn new() -> Self {
        let audio_engine = AudioEngine::new();
        let mut sample_bank = SampleBank::new();
        
        // Load default samples
        sample_bank.load_defaults();
        
        let mut sequencer = Sequencer::new();
        
        // Initialize all groups to use pattern 0
        for group in 0..4 {
            sequencer.set_active_pattern(group, 0);
        }
        
        Self {
            audio_engine,
            sequencer,
            sample_bank,
            current_group: 0,
            group_patterns: [0; 4], // Each group starts on pattern 0
            is_playing: false,
            is_recording: false,
            tempo: 120,
            last_tick: Instant::now(),
            selected_pad: None,
            flashing_pads: Vec::new(),
            flash_timer: Instant::now(),
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
                    self.group_patterns[self.current_group],
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
        self.sequencer.clear_pattern(self.current_group, self.group_patterns[self.current_group]);
    }

    pub fn next_group(&mut self) {
        self.current_group = (self.current_group + 1) % 4;
    }

    pub fn prev_group(&mut self) {
        self.current_group = if self.current_group == 0 { 3 } else { self.current_group - 1 };
    }

    pub fn next_pattern(&mut self) {
        self.group_patterns[self.current_group] = (self.group_patterns[self.current_group] + 1) % 99;
        self.sequencer.set_active_pattern(self.current_group, self.group_patterns[self.current_group]);
    }

    pub fn prev_pattern(&mut self) {
        let current_pattern = self.group_patterns[self.current_group];
        self.group_patterns[self.current_group] = if current_pattern == 0 { 98 } else { current_pattern - 1 };
        self.sequencer.set_active_pattern(self.current_group, self.group_patterns[self.current_group]);
    }

    pub fn adjust_tempo(&mut self, delta: i32) {
        let new_tempo = (self.tempo as i32 + delta).clamp(60, 300) as u32;
        self.tempo = new_tempo;
    }

    pub fn tick(&mut self) {
        let now = Instant::now();
        
        // Clear flashing pads after flash duration (150ms)
        if now.duration_since(self.flash_timer) >= std::time::Duration::from_millis(150) {
            self.flashing_pads.clear();
        }
        
        if self.is_playing {
            let elapsed = now.duration_since(self.last_tick);
            let tick_duration = std::time::Duration::from_millis(60000 / (self.tempo * 4) as u64);
            
            if elapsed >= tick_duration {
                self.last_tick = now;
                
                // Get hits for current position
                let hits = self.sequencer.tick(self.tempo);
                
                // Clear previous flashing pads and set new ones
                self.flashing_pads.clear();
                self.flash_timer = now;
                
                // Play all hits and add to flashing pads
                for (group, pad) in hits {
                    if let Some(sample) = self.sample_bank.get_sample(group, pad) {
                        self.audio_engine.play_sample(sample);
                    }
                    self.flashing_pads.push((group, pad));
                }
            }
        }
    }

    pub fn get_pattern_grid(&self) -> Vec<Vec<bool>> {
        self.sequencer.get_pattern_grid(self.current_group, self.group_patterns[self.current_group])
    }
    
    pub fn get_current_pattern(&self) -> usize {
        self.group_patterns[self.current_group]
    }

    pub fn get_current_step(&self) -> usize {
        self.sequencer.get_current_step()
    }
}