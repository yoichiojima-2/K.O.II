use std::collections::HashMap;

pub struct SampleBank {
    samples: HashMap<(usize, usize), Vec<u8>>, // (group, pad) -> sample data
    sample_names: HashMap<(usize, usize), String>,
}

impl SampleBank {
    pub fn new() -> Self {
        Self {
            samples: HashMap::new(),
            sample_names: HashMap::new(),
        }
    }

    pub fn load_defaults(&mut self) {
        // Load default drum samples for each group
        let drum_names = [
            "Kick", "Snare", "HiHat", "OpenHat",
            "Crash", "Ride", "Tom1", "Tom2",
            "Perc1", "Perc2", "Perc3", "Perc4",
            "FX1", "FX2", "FX3", "FX4"
        ];

        let bass_names = [
            "Bass1", "Bass2", "Sub1", "Sub2",
            "Pluck1", "Pluck2", "Saw1", "Saw2",
            "Sine1", "Sine2", "FM1", "FM2",
            "Noise1", "Noise2", "Sweep1", "Sweep2"
        ];

        let lead_names = [
            "Lead1", "Lead2", "Arp1", "Arp2",
            "Pad1", "Pad2", "Strings1", "Strings2",
            "Brass1", "Brass2", "Choir1", "Choir2",
            "Organ1", "Organ2", "Piano1", "Piano2"
        ];

        let vocal_names = [
            "Vocal1", "Vocal2", "Chop1", "Chop2",
            "Voice1", "Voice2", "Speak1", "Speak2",
            "Breath1", "Breath2", "Scratch1", "Scratch2",
            "Reverse1", "Reverse2", "Echo1", "Echo2"
        ];

        let group_names = [&drum_names, &bass_names, &lead_names, &vocal_names];

        for (group, names) in group_names.iter().enumerate() {
            for (pad, name) in names.iter().enumerate() {
                // For now, we'll just store empty sample data
                // In a real implementation, we'd load actual audio files
                self.samples.insert((group, pad), vec![]);
                self.sample_names.insert((group, pad), name.to_string());
            }
        }
    }

    pub fn load_sample(&mut self, group: usize, pad: usize, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        // In a real implementation, we'd load the audio file here
        // For now, we'll just create a placeholder
        let sample_data = vec![0u8; 1024]; // Placeholder sample data
        
        self.samples.insert((group, pad), sample_data);
        self.sample_names.insert((group, pad), 
            std::path::Path::new(path)
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string()
        );
        
        Ok(())
    }

    pub fn get_sample(&self, group: usize, pad: usize) -> Option<&[u8]> {
        self.samples.get(&(group, pad)).map(|v| v.as_slice())
    }

    pub fn get_sample_name(&self, group: usize, pad: usize) -> Option<&str> {
        self.sample_names.get(&(group, pad)).map(|s| s.as_str())
    }

    pub fn has_sample(&self, group: usize, pad: usize) -> bool {
        self.samples.contains_key(&(group, pad))
    }

    pub fn remove_sample(&mut self, group: usize, pad: usize) {
        self.samples.remove(&(group, pad));
        self.sample_names.remove(&(group, pad));
    }

    pub fn get_group_name(&self, group: usize) -> String {
        match group {
            0 => "DRUMS".to_string(),
            1 => "BASS".to_string(),
            2 => "LEAD".to_string(),
            3 => "VOCAL".to_string(),
            _ => format!("GROUP{}", group),
        }
    }
}