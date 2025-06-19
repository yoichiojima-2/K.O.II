use std::collections::HashMap;

pub const STEPS_PER_PATTERN: usize = 16;
pub const MAX_PATTERNS: usize = 99;
pub const MAX_GROUPS: usize = 4;
pub const PADS_PER_GROUP: usize = 16;

#[derive(Debug, Clone)]
pub struct Pattern {
    pub steps: Vec<Vec<bool>>, // steps[pad][step]
    pub length: usize,
}

impl Pattern {
    pub fn new() -> Self {
        Self {
            steps: vec![vec![false; STEPS_PER_PATTERN]; PADS_PER_GROUP],
            length: STEPS_PER_PATTERN,
        }
    }

    pub fn clear(&mut self) {
        for pad in &mut self.steps {
            pad.fill(false);
        }
    }

    pub fn set_hit(&mut self, pad: usize, step: usize, value: bool) {
        if pad < PADS_PER_GROUP && step < self.length {
            self.steps[pad][step] = value;
        }
    }

    pub fn get_hits_at_step(&self, step: usize) -> Vec<usize> {
        let mut hits = Vec::new();
        if step < self.length {
            for (pad, steps) in self.steps.iter().enumerate() {
                if steps[step] {
                    hits.push(pad);
                }
            }
        }
        hits
    }
}

pub struct Sequencer {
    patterns: HashMap<(usize, usize), Pattern>, // (group, pattern_idx) -> Pattern
    current_step: usize,
    active_patterns: [usize; MAX_GROUPS], // Pattern index for each group
}

impl Sequencer {
    pub fn new() -> Self {
        Self {
            patterns: HashMap::new(),
            current_step: 0,
            active_patterns: [0; MAX_GROUPS],
        }
    }

    pub fn tick(&mut self, _tempo: u32) -> Vec<(usize, usize)> {
        let mut all_hits = Vec::new();
        let current_step = self.current_step;
        
        // Collect hits from all active patterns
        for group in 0..MAX_GROUPS {
            let pattern_idx = self.active_patterns[group];
            let pattern = self.get_or_create_pattern(group, pattern_idx);
            
            let hits = pattern.get_hits_at_step(current_step);
            for pad in hits {
                all_hits.push((group, pad));
            }
        }
        
        // Advance step
        self.current_step = (self.current_step + 1) % STEPS_PER_PATTERN;
        
        all_hits
    }

    pub fn record_hit(&mut self, group: usize, pattern_idx: usize, pad: usize) {
        let current_step = self.current_step;
        let pattern = self.get_or_create_pattern_mut(group, pattern_idx);
        pattern.set_hit(pad, current_step, true);
    }

    pub fn clear_pattern(&mut self, group: usize, pattern_idx: usize) {
        if let Some(pattern) = self.patterns.get_mut(&(group, pattern_idx)) {
            pattern.clear();
        }
    }

    pub fn get_pattern_grid(&self, group: usize, pattern_idx: usize) -> Vec<Vec<bool>> {
        if let Some(pattern) = self.patterns.get(&(group, pattern_idx)) {
            pattern.steps.clone()
        } else {
            vec![vec![false; STEPS_PER_PATTERN]; PADS_PER_GROUP]
        }
    }

    pub fn get_current_step(&self) -> usize {
        self.current_step
    }

    pub fn reset_position(&mut self) {
        self.current_step = 0;
    }

    pub fn set_active_pattern(&mut self, group: usize, pattern_idx: usize) {
        if group < MAX_GROUPS && pattern_idx < MAX_PATTERNS {
            self.active_patterns[group] = pattern_idx;
        }
    }

    fn get_or_create_pattern(&mut self, group: usize, pattern_idx: usize) -> &Pattern {
        self.patterns.entry((group, pattern_idx))
            .or_insert_with(Pattern::new)
    }

    fn get_or_create_pattern_mut(&mut self, group: usize, pattern_idx: usize) -> &mut Pattern {
        self.patterns.entry((group, pattern_idx))
            .or_insert_with(Pattern::new)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_creation() {
        let pattern = Pattern::new();
        assert_eq!(pattern.steps.len(), PADS_PER_GROUP);
        assert_eq!(pattern.length, STEPS_PER_PATTERN);
        
        // All steps should be false initially
        for pad_steps in &pattern.steps {
            assert_eq!(pad_steps.len(), STEPS_PER_PATTERN);
            for &step in pad_steps {
                assert!(!step);
            }
        }
    }

    #[test]
    fn test_pattern_set_hit() {
        let mut pattern = Pattern::new();
        
        // Test valid set
        pattern.set_hit(0, 0, true);
        assert!(pattern.steps[0][0]);
        
        pattern.set_hit(5, 10, true);
        assert!(pattern.steps[5][10]);
        
        // Test clearing
        pattern.set_hit(0, 0, false);
        assert!(!pattern.steps[0][0]);
        
        // Test bounds checking
        pattern.set_hit(99, 0, true); // Invalid pad
        pattern.set_hit(0, 99, true); // Invalid step
        // Should not panic
    }

    #[test]
    fn test_pattern_get_hits_at_step() {
        let mut pattern = Pattern::new();
        
        // Test empty step
        let hits = pattern.get_hits_at_step(0);
        assert!(hits.is_empty());
        
        // Add some hits
        pattern.set_hit(0, 0, true);
        pattern.set_hit(5, 0, true);
        pattern.set_hit(10, 0, true);
        
        let hits = pattern.get_hits_at_step(0);
        assert_eq!(hits.len(), 3);
        assert!(hits.contains(&0));
        assert!(hits.contains(&5));
        assert!(hits.contains(&10));
        
        // Test different step
        let hits = pattern.get_hits_at_step(1);
        assert!(hits.is_empty());
        
        // Test out of bounds
        let hits = pattern.get_hits_at_step(99);
        assert!(hits.is_empty());
    }

    #[test]
    fn test_pattern_clear() {
        let mut pattern = Pattern::new();
        
        // Set some hits
        pattern.set_hit(0, 0, true);
        pattern.set_hit(5, 10, true);
        pattern.set_hit(15, 15, true);
        
        // Verify they're set
        assert!(pattern.steps[0][0]);
        assert!(pattern.steps[5][10]);
        assert!(pattern.steps[15][15]);
        
        // Clear pattern
        pattern.clear();
        
        // Verify all hits are cleared
        for pad_steps in &pattern.steps {
            for &step in pad_steps {
                assert!(!step);
            }
        }
    }

    #[test]
    fn test_sequencer_creation() {
        let sequencer = Sequencer::new();
        assert_eq!(sequencer.current_step, 0);
        assert_eq!(sequencer.active_patterns, [0; MAX_GROUPS]);
        assert!(sequencer.patterns.is_empty());
    }

    #[test]
    fn test_sequencer_tick() {
        let mut sequencer = Sequencer::new();
        
        // Test empty sequencer
        let hits = sequencer.tick(120);
        assert!(hits.is_empty());
        assert_eq!(sequencer.current_step, 1);
        
        // Test wrap around
        sequencer.current_step = STEPS_PER_PATTERN - 1;
        sequencer.tick(120);
        assert_eq!(sequencer.current_step, 0);
    }

    #[test]
    fn test_sequencer_record_hit() {
        let mut sequencer = Sequencer::new();
        
        // Record a hit
        sequencer.record_hit(0, 0, 5);
        
        // The pattern should be created and the hit recorded at current step
        let pattern_grid = sequencer.get_pattern_grid(0, 0);
        assert!(pattern_grid[5][0]); // pad 5, step 0 (current_step is 0)
        
        // Advance step and record another hit
        sequencer.current_step = 5;
        sequencer.record_hit(0, 0, 10);
        
        let pattern_grid = sequencer.get_pattern_grid(0, 0);
        assert!(pattern_grid[10][5]); // pad 10, step 5
    }

    #[test]
    fn test_sequencer_clear_pattern() {
        let mut sequencer = Sequencer::new();
        
        // Record some hits
        sequencer.record_hit(0, 0, 5);
        sequencer.current_step = 3;
        sequencer.record_hit(0, 0, 10);
        
        // Verify hits are recorded
        let pattern_grid = sequencer.get_pattern_grid(0, 0);
        assert!(pattern_grid[5][0]);
        assert!(pattern_grid[10][3]);
        
        // Clear pattern
        sequencer.clear_pattern(0, 0);
        
        // Verify pattern is cleared
        let pattern_grid = sequencer.get_pattern_grid(0, 0);
        for pad_steps in pattern_grid {
            for step in pad_steps {
                assert!(!step);
            }
        }
        
        // Test clearing non-existent pattern (should not panic)
        sequencer.clear_pattern(1, 50);
    }

    #[test]
    fn test_sequencer_pattern_grid() {
        let sequencer = Sequencer::new();
        
        // Test empty pattern
        let grid = sequencer.get_pattern_grid(0, 0);
        assert_eq!(grid.len(), PADS_PER_GROUP);
        assert_eq!(grid[0].len(), STEPS_PER_PATTERN);
        
        // Test non-existent pattern returns empty grid
        let grid = sequencer.get_pattern_grid(3, 99);
        assert_eq!(grid.len(), PADS_PER_GROUP);
        for pad_steps in grid {
            for step in pad_steps {
                assert!(!step);
            }
        }
    }

    #[test]
    fn test_sequencer_active_patterns() {
        let mut sequencer = Sequencer::new();
        
        // Test setting active patterns
        sequencer.set_active_pattern(0, 5);
        assert_eq!(sequencer.active_patterns[0], 5);
        
        sequencer.set_active_pattern(3, 98);
        assert_eq!(sequencer.active_patterns[3], 98);
        
        // Test bounds checking
        sequencer.set_active_pattern(99, 0); // Invalid group
        sequencer.set_active_pattern(0, 999); // Invalid pattern
        // Should not panic, values should remain unchanged
        assert_eq!(sequencer.active_patterns[0], 5);
    }

    #[test]
    fn test_sequencer_reset_position() {
        let mut sequencer = Sequencer::new();
        
        sequencer.current_step = 10;
        sequencer.reset_position();
        assert_eq!(sequencer.current_step, 0);
    }

    #[test]
    fn test_sequencer_current_step() {
        let mut sequencer = Sequencer::new();
        
        assert_eq!(sequencer.get_current_step(), 0);
        
        sequencer.current_step = 5;
        assert_eq!(sequencer.get_current_step(), 5);
    }

    #[test]
    fn test_sequencer_multi_group_playback() {
        let mut sequencer = Sequencer::new();
        
        // Set up patterns for different groups
        sequencer.record_hit(0, 0, 0); // Group 0, pattern 0, pad 0 at step 0
        sequencer.record_hit(1, 0, 5); // Group 1, pattern 0, pad 5 at step 0
        sequencer.record_hit(2, 1, 10); // Group 2, pattern 1, pad 10 at step 0
        
        // Set active patterns
        sequencer.set_active_pattern(0, 0);
        sequencer.set_active_pattern(1, 0);
        sequencer.set_active_pattern(2, 1);
        
        // Reset to step 0 and tick
        sequencer.reset_position();
        let hits = sequencer.tick(120);
        
        // Should get hits from groups 0, 1, and 2
        assert_eq!(hits.len(), 3);
        assert!(hits.contains(&(0, 0)));
        assert!(hits.contains(&(1, 5)));
        assert!(hits.contains(&(2, 10)));
    }

    #[test]
    fn test_constants() {
        assert_eq!(STEPS_PER_PATTERN, 16);
        assert_eq!(MAX_PATTERNS, 99);
        assert_eq!(MAX_GROUPS, 4);
        assert_eq!(PADS_PER_GROUP, 16);
    }
}