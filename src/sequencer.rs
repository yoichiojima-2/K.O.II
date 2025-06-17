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