use std::time::Instant;

#[derive(Debug, Clone)]
pub struct UIState {
    pub selected_pad: Option<usize>,
    pub flashing_pads: Vec<(usize, usize)>, // (group, pad) pairs that are currently flashing
    pub flash_timer: Instant,
}

impl UIState {
    pub fn new() -> Self {
        Self {
            selected_pad: None,
            flashing_pads: Vec::new(),
            flash_timer: Instant::now(),
        }
    }

    pub fn select_pad(&mut self, pad: usize) {
        self.selected_pad = Some(pad);
    }

    pub fn clear_selection(&mut self) {
        self.selected_pad = None;
    }

    pub fn start_flash(&mut self, pads: Vec<(usize, usize)>) {
        self.flashing_pads = pads;
        self.flash_timer = Instant::now();
    }

    pub fn update_flash(&mut self) {
        if self.flash_timer.elapsed() >= std::time::Duration::from_millis(150) {
            self.flashing_pads.clear();
        }
    }

    pub fn is_pad_flashing(&self, group: usize, pad: usize) -> bool {
        self.flashing_pads.contains(&(group, pad))
    }
}

impl Default for UIState {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub current_group: usize,
    pub group_patterns: [usize; 4], // Each group has its own current pattern
    pub is_playing: bool,
    pub is_recording: bool,
    pub tempo: u32,
    pub last_tick: Instant,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            current_group: 0,
            group_patterns: [0; 4],
            is_playing: false,
            is_recording: false,
            tempo: 120,
            last_tick: Instant::now(),
        }
    }

    pub fn next_group(&mut self) {
        self.current_group = (self.current_group + 1) % 4;
    }

    pub fn prev_group(&mut self) {
        self.current_group = if self.current_group == 0 { 3 } else { self.current_group - 1 };
    }

    pub fn get_current_pattern(&self) -> usize {
        self.group_patterns[self.current_group]
    }

    pub fn set_current_pattern(&mut self, pattern: usize) {
        self.group_patterns[self.current_group] = pattern;
    }

    pub fn next_pattern(&mut self) {
        self.group_patterns[self.current_group] = (self.group_patterns[self.current_group] + 1) % 99;
    }

    pub fn prev_pattern(&mut self) {
        let current_pattern = self.group_patterns[self.current_group];
        self.group_patterns[self.current_group] = if current_pattern == 0 { 98 } else { current_pattern - 1 };
    }

    pub fn adjust_tempo(&mut self, delta: i32) {
        self.tempo = (self.tempo as i32 + delta).clamp(60, 300) as u32;
    }

    pub fn toggle_playback(&mut self) {
        self.is_playing = !self.is_playing;
    }

    pub fn toggle_recording(&mut self) {
        self.is_recording = !self.is_recording;
    }

    pub fn update_tick_time(&mut self) {
        self.last_tick = Instant::now();
    }

    pub fn should_tick(&self) -> bool {
        if !self.is_playing {
            return false;
        }
        
        let elapsed = Instant::now().duration_since(self.last_tick);
        let tick_duration = std::time::Duration::from_millis(60000 / (self.tempo * 4) as u64);
        elapsed >= tick_duration
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ui_state() {
        let mut state = UIState::new();
        
        // Test pad selection
        assert_eq!(state.selected_pad, None);
        state.select_pad(5);
        assert_eq!(state.selected_pad, Some(5));
        state.clear_selection();
        assert_eq!(state.selected_pad, None);
        
        // Test flashing
        assert!(state.flashing_pads.is_empty());
        state.start_flash(vec![(0, 1), (1, 2)]);
        assert_eq!(state.flashing_pads.len(), 2);
        assert!(state.is_pad_flashing(0, 1));
        assert!(state.is_pad_flashing(1, 2));
        assert!(!state.is_pad_flashing(0, 2));
        
        // Test flash clearing after timeout
        std::thread::sleep(std::time::Duration::from_millis(160));
        state.update_flash();
        assert!(state.flashing_pads.is_empty());
    }

    #[test]
    fn test_app_state_navigation() {
        let mut state = AppState::new();
        
        // Test group navigation
        assert_eq!(state.current_group, 0);
        state.next_group();
        assert_eq!(state.current_group, 1);
        state.next_group();
        assert_eq!(state.current_group, 2);
        state.next_group();
        assert_eq!(state.current_group, 3);
        state.next_group();
        assert_eq!(state.current_group, 0);
        
        state.prev_group();
        assert_eq!(state.current_group, 3);
        state.prev_group();
        assert_eq!(state.current_group, 2);
    }

    #[test]
    fn test_app_state_patterns() {
        let mut state = AppState::new();
        
        // Test pattern management
        assert_eq!(state.get_current_pattern(), 0);
        state.next_pattern();
        assert_eq!(state.get_current_pattern(), 1);
        
        state.set_current_pattern(50);
        assert_eq!(state.get_current_pattern(), 50);
        
        // Test wrap around
        state.set_current_pattern(98);
        state.next_pattern();
        assert_eq!(state.get_current_pattern(), 0);
        
        state.prev_pattern();
        assert_eq!(state.get_current_pattern(), 98);
    }

    #[test]
    fn test_app_state_tempo() {
        let mut state = AppState::new();
        
        assert_eq!(state.tempo, 120);
        state.adjust_tempo(10);
        assert_eq!(state.tempo, 130);
        state.adjust_tempo(-20);
        assert_eq!(state.tempo, 110);
        
        // Test bounds
        state.adjust_tempo(-100);
        assert_eq!(state.tempo, 60);
        state.adjust_tempo(300);
        assert_eq!(state.tempo, 300);
    }

    #[test]
    fn test_app_state_toggles() {
        let mut state = AppState::new();
        
        assert!(!state.is_playing);
        state.toggle_playback();
        assert!(state.is_playing);
        state.toggle_playback();
        assert!(!state.is_playing);
        
        assert!(!state.is_recording);
        state.toggle_recording();
        assert!(state.is_recording);
        state.toggle_recording();
        assert!(!state.is_recording);
    }
}