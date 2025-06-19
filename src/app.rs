use crate::mixer::Mixer;
use crate::sequencer::Sequencer;
use crate::sample::SampleBank;
use std::time::Instant;

pub struct App {
    pub mixer: Mixer,
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
        let mixer = Mixer::new();
        let mut sample_bank = SampleBank::new();
        
        // Load default samples
        sample_bank.load_defaults();
        
        let mut sequencer = Sequencer::new();
        
        // Initialize all groups to use pattern 0
        for group in 0..4 {
            sequencer.set_active_pattern(group, 0);
        }
        
        Self {
            mixer,
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
            // Play the sample only if not recording
            if !self.is_recording {
                if let Some(sample) = self.sample_bank.get_sample(self.current_group, pad) {
                    self.mixer.play_sample(sample, self.current_group);
                }
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
                        self.mixer.play_sample(sample, group);
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

    // Mixer control methods
    pub fn adjust_master_volume(&mut self, delta: f32) {
        self.mixer.adjust_master_volume(delta);
    }

    pub fn toggle_master_mute(&mut self) {
        self.mixer.toggle_master_mute();
    }

    pub fn adjust_group_volume(&mut self, group: usize, delta: f32) {
        self.mixer.adjust_group_volume(group, delta);
    }

    pub fn toggle_group_mute(&mut self, group: usize) {
        self.mixer.toggle_group_mute(group);
    }

    pub fn get_master_volume(&self) -> f32 {
        self.mixer.get_master_volume()
    }

    pub fn get_group_volume(&self, group: usize) -> f32 {
        self.mixer.get_group_volume(group)
    }

    pub fn is_master_muted(&self) -> bool {
        self.mixer.is_master_muted()
    }

    pub fn is_group_muted(&self, group: usize) -> bool {
        self.mixer.is_group_muted(group)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_creation() {
        let app = App::new();
        assert_eq!(app.current_group, 0);
        assert_eq!(app.group_patterns, [0; 4]);
        assert!(!app.is_playing);
        assert!(!app.is_recording);
        assert_eq!(app.tempo, 120);
        assert_eq!(app.selected_pad, None);
        assert!(app.flashing_pads.is_empty());
    }

    #[test]
    fn test_group_navigation() {
        let mut app = App::new();
        
        // Test next group
        app.next_group();
        assert_eq!(app.current_group, 1);
        
        app.next_group();
        assert_eq!(app.current_group, 2);
        
        app.next_group();
        assert_eq!(app.current_group, 3);
        
        // Test wrap around
        app.next_group();
        assert_eq!(app.current_group, 0);
        
        // Test previous group
        app.prev_group();
        assert_eq!(app.current_group, 3);
        
        app.prev_group();
        assert_eq!(app.current_group, 2);
    }

    #[test]
    fn test_pattern_navigation() {
        let mut app = App::new();
        
        // Test next pattern
        app.next_pattern();
        assert_eq!(app.group_patterns[0], 1);
        
        // Test wrap around (max is 99)
        app.group_patterns[0] = 98;
        app.next_pattern();
        assert_eq!(app.group_patterns[0], 0);
        
        // Test previous pattern
        app.prev_pattern();
        assert_eq!(app.group_patterns[0], 98);
        
        app.group_patterns[0] = 1;
        app.prev_pattern();
        assert_eq!(app.group_patterns[0], 0);
    }

    #[test]
    fn test_tempo_adjustment() {
        let mut app = App::new();
        
        // Test increase tempo
        app.adjust_tempo(10);
        assert_eq!(app.tempo, 130);
        
        // Test decrease tempo
        app.adjust_tempo(-20);
        assert_eq!(app.tempo, 110);
        
        // Test tempo bounds
        app.adjust_tempo(-100);
        assert_eq!(app.tempo, 60); // Min tempo
        
        app.adjust_tempo(300);
        assert_eq!(app.tempo, 300); // Max tempo
        
        app.adjust_tempo(10);
        assert_eq!(app.tempo, 300); // Should not exceed max
    }

    #[test]
    fn test_playback_toggle() {
        let mut app = App::new();
        
        assert!(!app.is_playing);
        app.toggle_playback();
        assert!(app.is_playing);
        app.toggle_playback();
        assert!(!app.is_playing);
    }

    #[test]
    fn test_recording_toggle() {
        let mut app = App::new();
        
        assert!(!app.is_recording);
        app.toggle_recording();
        assert!(app.is_recording);
        app.toggle_recording();
        assert!(!app.is_recording);
    }

    #[test]
    fn test_pad_trigger() {
        let mut app = App::new();
        
        // Test valid pad
        app.trigger_pad(5);
        assert_eq!(app.selected_pad, Some(5));
        
        // Test invalid pad (should be ignored)
        app.trigger_pad(20);
        assert_eq!(app.selected_pad, Some(5)); // Should remain unchanged
    }

    #[test]
    fn test_volume_controls() {
        let mut app = App::new();
        
        let initial_master = app.get_master_volume();
        app.adjust_master_volume(0.1);
        assert!((app.get_master_volume() - (initial_master + 0.1)).abs() < 0.001);
        
        app.adjust_master_volume(-0.2);
        assert!((app.get_master_volume() - (initial_master - 0.1)).abs() < 0.001);
        
        // Test group volume
        let initial_group = app.get_group_volume(0);
        app.adjust_group_volume(0, 0.05);
        assert!((app.get_group_volume(0) - (initial_group + 0.05)).abs() < 0.001);
    }

    #[test]
    fn test_mute_controls() {
        let mut app = App::new();
        
        // Test master mute
        assert!(!app.is_master_muted());
        app.toggle_master_mute();
        assert!(app.is_master_muted());
        app.toggle_master_mute();
        assert!(!app.is_master_muted());
        
        // Test group mute
        assert!(!app.is_group_muted(0));
        app.toggle_group_mute(0);
        assert!(app.is_group_muted(0));
        app.toggle_group_mute(0);
        assert!(!app.is_group_muted(0));
    }
}