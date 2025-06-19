use crate::mixer::Mixer;
use crate::sequencer::Sequencer;
use crate::sample::SampleBank;
use crate::state::{AppState, UIState};
use crate::audio_manager::AudioManager;
use crate::error::Result;

pub struct App {
    pub mixer: Mixer,
    pub sequencer: Sequencer,
    pub sample_bank: SampleBank,
    pub state: AppState,
    pub ui_state: UIState,
}

impl App {
    pub fn new() -> Result<Self> {
        let audio_manager = AudioManager::new()?;
        
        let mut sequencer = Sequencer::new();
        
        // Initialize all groups to use pattern 0
        for group in 0..4 {
            sequencer.set_active_pattern(group, 0);
        }
        
        Ok(Self {
            mixer: audio_manager.mixer,
            sequencer,
            sample_bank: audio_manager.sample_bank,
            state: AppState::new(),
            ui_state: UIState::new(),
        })
    }
    
    pub fn with_audio_test() -> Result<Self> {
        let mut audio_manager = AudioManager::new()?;
        audio_manager.test_audio()?;
        audio_manager.validate_audio_system()?;
        
        let mut sequencer = Sequencer::new();
        
        // Initialize all groups to use pattern 0
        for group in 0..4 {
            sequencer.set_active_pattern(group, 0);
        }
        
        Ok(Self {
            mixer: audio_manager.mixer,
            sequencer,
            sample_bank: audio_manager.sample_bank,
            state: AppState::new(),
            ui_state: UIState::new(),
        })
    }

    pub fn trigger_pad(&mut self, pad: usize) {
        if pad < 16 {
            // Play the sample only if not recording
            if !self.state.is_recording {
                if let Some(sample) = self.sample_bank.get_sample(self.state.current_group, pad) {
                    self.mixer.play_sample(sample, self.state.current_group);
                }
            }
            
            // Record if recording
            if self.state.is_recording && self.state.is_playing {
                self.sequencer.record_hit(
                    self.state.current_group,
                    self.state.group_patterns[self.state.current_group],
                    pad,
                );
            }
            
            self.ui_state.select_pad(pad);
        }
    }

    pub fn toggle_playback(&mut self) {
        self.state.toggle_playback();
        if self.state.is_playing {
            self.sequencer.reset_position();
        }
    }

    pub fn toggle_recording(&mut self) {
        self.state.toggle_recording();
    }

    pub fn clear_pattern(&mut self) {
        self.sequencer.clear_pattern(self.state.current_group, self.state.group_patterns[self.state.current_group]);
    }

    pub fn next_group(&mut self) {
        self.state.next_group();
    }

    pub fn prev_group(&mut self) {
        self.state.prev_group();
    }

    pub fn next_pattern(&mut self) {
        self.state.next_pattern();
        self.sequencer.set_active_pattern(self.state.current_group, self.state.group_patterns[self.state.current_group]);
    }

    pub fn prev_pattern(&mut self) {
        self.state.prev_pattern();
        self.sequencer.set_active_pattern(self.state.current_group, self.state.group_patterns[self.state.current_group]);
    }

    pub fn adjust_tempo(&mut self, delta: i32) {
        self.state.adjust_tempo(delta);
    }

    pub fn tick(&mut self) {
        // Update UI state
        self.ui_state.update_flash();
        
        if self.state.should_tick() {
            self.state.update_tick_time();
            
            // Get hits for current position
            let hits = self.sequencer.tick(self.state.tempo);
            
            // Start flash for new hits
            self.ui_state.start_flash(hits.clone());
            
            // Play all hits
            for (group, pad) in hits {
                if let Some(sample) = self.sample_bank.get_sample(group, pad) {
                    self.mixer.play_sample(sample, group);
                }
            }
        }
    }

    pub fn get_pattern_grid(&self) -> Vec<Vec<bool>> {
        self.sequencer.get_pattern_grid(self.state.current_group, self.state.group_patterns[self.state.current_group])
    }
    
    pub fn get_current_pattern(&self) -> usize {
        self.state.get_current_pattern()
    }

    pub fn get_current_step(&self) -> usize {
        self.sequencer.get_current_step()
    }
    
    // Getter methods for UI
    pub fn get_current_group(&self) -> usize {
        self.state.current_group
    }
    
    pub fn get_selected_pad(&self) -> Option<usize> {
        self.ui_state.selected_pad
    }
    
    pub fn is_pad_flashing(&self, group: usize, pad: usize) -> bool {
        self.ui_state.is_pad_flashing(group, pad)
    }
    
    pub fn is_playing(&self) -> bool {
        self.state.is_playing
    }
    
    pub fn is_recording(&self) -> bool {
        self.state.is_recording
    }
    
    pub fn get_tempo(&self) -> u32 {
        self.state.tempo
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
        let app = App::new().unwrap();
        assert_eq!(app.state.current_group, 0);
        assert_eq!(app.state.group_patterns, [0; 4]);
        assert!(!app.state.is_playing);
        assert!(!app.state.is_recording);
        assert_eq!(app.state.tempo, 120);
        assert_eq!(app.ui_state.selected_pad, None);
        assert!(app.ui_state.flashing_pads.is_empty());
    }

    #[test]
    fn test_group_navigation() {
        let mut app = App::new().unwrap();
        
        // Test next group
        app.next_group();
        assert_eq!(app.state.current_group, 1);
        
        app.next_group();
        assert_eq!(app.state.current_group, 2);
        
        app.next_group();
        assert_eq!(app.state.current_group, 3);
        
        // Test wrap around
        app.next_group();
        assert_eq!(app.state.current_group, 0);
        
        // Test previous group
        app.prev_group();
        assert_eq!(app.state.current_group, 3);
        
        app.prev_group();
        assert_eq!(app.state.current_group, 2);
    }

    #[test]
    fn test_pattern_navigation() {
        let mut app = App::new().unwrap();
        
        // Test next pattern
        app.next_pattern();
        assert_eq!(app.state.group_patterns[0], 1);
        
        // Test wrap around (max is 99)
        app.state.group_patterns[0] = 98;
        app.next_pattern();
        assert_eq!(app.state.group_patterns[0], 0);
        
        // Test previous pattern
        app.prev_pattern();
        assert_eq!(app.state.group_patterns[0], 98);
        
        app.state.group_patterns[0] = 1;
        app.prev_pattern();
        assert_eq!(app.state.group_patterns[0], 0);
    }

    #[test]
    fn test_tempo_adjustment() {
        let mut app = App::new().unwrap();
        
        // Test increase tempo
        app.adjust_tempo(10);
        assert_eq!(app.state.tempo, 130);
        
        // Test decrease tempo
        app.adjust_tempo(-20);
        assert_eq!(app.state.tempo, 110);
        
        // Test tempo bounds
        app.adjust_tempo(-100);
        assert_eq!(app.state.tempo, 60); // Min tempo
        
        app.adjust_tempo(300);
        assert_eq!(app.state.tempo, 300); // Max tempo
        
        app.adjust_tempo(10);
        assert_eq!(app.state.tempo, 300); // Should not exceed max
    }

    #[test]
    fn test_playback_toggle() {
        let mut app = App::new().unwrap();
        
        assert!(!app.state.is_playing);
        app.toggle_playback();
        assert!(app.state.is_playing);
        app.toggle_playback();
        assert!(!app.state.is_playing);
    }

    #[test]
    fn test_recording_toggle() {
        let mut app = App::new().unwrap();
        
        assert!(!app.state.is_recording);
        app.toggle_recording();
        assert!(app.state.is_recording);
        app.toggle_recording();
        assert!(!app.state.is_recording);
    }

    #[test]
    fn test_pad_trigger() {
        let mut app = App::new().unwrap();
        
        // Test valid pad
        app.trigger_pad(5);
        assert_eq!(app.ui_state.selected_pad, Some(5));
        
        // Test invalid pad (should be ignored)
        app.trigger_pad(20);
        assert_eq!(app.ui_state.selected_pad, Some(5)); // Should remain unchanged
    }

    #[test]
    fn test_volume_controls() {
        let mut app = App::new().unwrap();
        
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
        let mut app = App::new().unwrap();
        
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