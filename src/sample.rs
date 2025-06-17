use std::collections::HashMap;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SampleMapping {
    pub group: usize,
    pub pad: usize,
    pub file: String,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SampleConfig {
    pub mappings: Vec<SampleMapping>,
}

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
        // First try to load from JSON config
        if self.load_from_config().is_ok() {
            eprintln!("Loaded samples from config file");
            return;
        }
        
        // Fallback to directory scanning
        eprintln!("No config file found, scanning directories...");
        
        // Try to load samples from the samples directory
        let samples_dir = "samples";
        let group_dirs = ["drums", "bass", "lead", "vocal"];
        
        for (group_idx, group_dir) in group_dirs.iter().enumerate() {
            let group_path = format!("{}/{}", samples_dir, group_dir);
            
            // Try to load up to 16 samples from each group directory
            if let Ok(entries) = std::fs::read_dir(&group_path) {
                let mut files = Vec::new();
                
                // Collect all audio files
                for entry in entries.flatten() {
                    let path = entry.path();
                    if let Some(extension) = path.extension() {
                        let ext = extension.to_string_lossy().to_lowercase();
                        if matches!(ext.as_str(), "wav" | "mp3" | "flac" | "ogg") {
                            files.push(path);
                        }
                    }
                }
                
                // Sort files to ensure consistent loading order
                files.sort();
                
                // Load samples with intelligent pad assignment
                for path in files {
                    if let Some(path_str) = path.to_str() {
                        let file_stem = path.file_stem()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .to_lowercase();
                        
                        // Try to extract pad number from filename
                        let target_pad = self.extract_pad_from_filename(&file_stem);
                        
                        if let Some(pad_idx) = target_pad {
                            // Load to specific pad if specified in filename
                            if pad_idx < 16 && !self.samples.contains_key(&(group_idx, pad_idx)) {
                                if let Err(e) = self.load_sample(group_idx, pad_idx, path_str) {
                                    eprintln!("Failed to load sample {}: {}", path_str, e);
                                } else {
                                    eprintln!("Loaded sample: {} -> Group {} Pad {} (from filename)", path_str, group_idx, pad_idx);
                                }
                            }
                        } else {
                            // Load to next available pad
                            for pad_idx in 0..16 {
                                if !self.samples.contains_key(&(group_idx, pad_idx)) {
                                    if let Err(e) = self.load_sample(group_idx, pad_idx, path_str) {
                                        eprintln!("Failed to load sample {}: {}", path_str, e);
                                    } else {
                                        eprintln!("Loaded sample: {} -> Group {} Pad {} (auto-assigned)", path_str, group_idx, pad_idx);
                                    }
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
        
        // Create placeholder names for empty pads
        self.create_placeholder_names();
    }
    
    fn create_placeholder_names(&mut self) {
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
                // Only add names if we don't already have a sample loaded
                if !self.samples.contains_key(&(group, pad)) {
                    self.sample_names.insert((group, pad), name.to_string());
                }
            }
        }
    }

    pub fn load_sample(&mut self, group: usize, pad: usize, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Check if file exists
        if !std::path::Path::new(path).exists() {
            return Err(format!("Sample file not found: {}", path).into());
        }

        // Read the entire file into memory
        let sample_data = std::fs::read(path)?;
        
        // Verify it's a valid audio file by checking the header
        if sample_data.len() < 12 {
            return Err("Invalid audio file: too small".into());
        }
        
        // Basic format validation (check for common audio file headers)
        let is_wav = sample_data.starts_with(b"RIFF") && sample_data[8..12] == *b"WAVE";
        let is_mp3 = sample_data.starts_with(b"ID3") || 
                    (sample_data.len() > 2 && sample_data[0] == 0xFF && (sample_data[1] & 0xE0) == 0xE0);
        let is_flac = sample_data.starts_with(b"fLaC");
        let is_ogg = sample_data.starts_with(b"OggS");
        
        if !is_wav && !is_mp3 && !is_flac && !is_ogg {
            return Err("Unsupported audio format. Please use WAV, MP3, FLAC, or OGG files.".into());
        }
        
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
    
    pub fn create_samples_directory() -> Result<(), Box<dyn std::error::Error>> {
        let samples_dir = "samples";
        let group_dirs = ["drums", "bass", "lead", "vocal"];
        
        std::fs::create_dir_all(samples_dir)?;
        
        for group_dir in &group_dirs {
            let group_path = format!("{}/{}", samples_dir, group_dir);
            std::fs::create_dir_all(&group_path)?;
        }
        
        println!("Created samples directory structure:");
        println!("samples/");
        for group_dir in &group_dirs {
            println!("  {}/", group_dir);
        }
        println!("\nPlace your audio files in the appropriate directories and restart the app.");
        
        Ok(())
    }
    
    pub fn generate_simple_kick(&self) -> Vec<u8> {
        // Generate a simple kick drum sound as WAV data
        let sample_rate = 44100;
        let duration = 0.5; // 500ms
        let samples = (sample_rate as f32 * duration) as usize;
        
        let mut audio_data = Vec::new();
        
        // Generate kick drum sound (low frequency sweep + click)
        for i in 0..samples {
            let t = i as f32 / sample_rate as f32;
            let envelope = (-t * 8.0).exp(); // Exponential decay
            
            // Low frequency component (kick)
            let kick_freq = 60.0 * (1.0 - t * 0.8); // Frequency sweep down
            let kick = (t * kick_freq * 2.0 * std::f32::consts::PI).sin();
            
            // Click component
            let click = if t < 0.01 { 
                (t * 5000.0 * 2.0 * std::f32::consts::PI).sin() * 0.3 
            } else { 
                0.0 
            };
            
            let sample = (kick + click) * envelope * 0.7;
            let sample_i16 = (sample * 32767.0) as i16;
            
            audio_data.extend_from_slice(&sample_i16.to_le_bytes());
        }
        
        // Create WAV header
        let mut wav_data = Vec::new();
        
        // RIFF header
        wav_data.extend_from_slice(b"RIFF");
        wav_data.extend_from_slice(&((audio_data.len() + 36) as u32).to_le_bytes());
        wav_data.extend_from_slice(b"WAVE");
        
        // fmt chunk
        wav_data.extend_from_slice(b"fmt ");
        wav_data.extend_from_slice(&16u32.to_le_bytes()); // chunk size
        wav_data.extend_from_slice(&1u16.to_le_bytes()); // audio format (PCM)
        wav_data.extend_from_slice(&1u16.to_le_bytes()); // num channels
        wav_data.extend_from_slice(&(sample_rate as u32).to_le_bytes()); // sample rate
        wav_data.extend_from_slice(&(sample_rate as u32 * 2).to_le_bytes()); // byte rate
        wav_data.extend_from_slice(&2u16.to_le_bytes()); // block align
        wav_data.extend_from_slice(&16u16.to_le_bytes()); // bits per sample
        
        // data chunk
        wav_data.extend_from_slice(b"data");
        wav_data.extend_from_slice(&(audio_data.len() as u32).to_le_bytes());
        wav_data.extend_from_slice(&audio_data);
        
        wav_data
    }
    
    fn extract_pad_from_filename(&self, filename: &str) -> Option<usize> {
        // Extract pad number from various naming conventions:
        // "kick_01.wav" -> pad 1
        // "snare-05.wav" -> pad 5  
        // "hihat.12.wav" -> pad 12
        // "01_kick.wav" -> pad 1
        // "pad00.wav" -> pad 0
        // "p15_crash.wav" -> pad 15
        
        // Look for patterns: number at start, end, or after common separators
        // Order matters - more specific patterns first
        let patterns = [
            r"^pad(\d{1,2})",            // pad01, pad15
            r"^p(\d{1,2})[_\-]",         // p01_kick, p15-crash
            r"^(\d{1,2})$",              // 12.wav (just number + extension)
            r"^(\d{1,2})[_\-]",          // 01_kick, 12-snare (number + separator + text)
            r"[_\-\.](\d{1,2})$",        // kick_01, snare-12, hihat.05  
            r"[_\-\.](\d{1,2})[_\-\.]",  // kick_01_loop, snare-12-hard
        ];
        
        for pattern in &patterns {
            if let Ok(regex) = Regex::new(pattern) {
                if let Some(captures) = regex.captures(filename) {
                    if let Some(num_str) = captures.get(1) {
                        if let Ok(pad_num) = num_str.as_str().parse::<usize>() {
                            // Handle both 0-based (0-15) and 1-based (1-16) numbering
                            if pad_num >= 1 && pad_num <= 16 {
                                // 1-based: convert to 0-based
                                let target_pad = pad_num - 1;
                                return Some(target_pad);
                            } else if pad_num <= 15 {
                                // 0-based: use as-is  
                                return Some(pad_num);
                            }
                        }
                    }
                }
            }
        }
        
        None
    }
    
    fn load_from_config(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = "samples/config.json";
        
        if !std::path::Path::new(config_path).exists() {
            return Err("Config file not found".into());
        }
        
        let config_content = std::fs::read_to_string(config_path)?;
        let config: SampleConfig = serde_json::from_str(&config_content)?;
        
        for mapping in config.mappings {
            if mapping.group < 4 && mapping.pad < 16 {
                let full_path = if mapping.file.starts_with('/') {
                    mapping.file.clone()
                } else {
                    format!("samples/{}", mapping.file)
                };
                
                match self.load_sample(mapping.group, mapping.pad, &full_path) {
                    Ok(_) => {
                        eprintln!("Loaded: {} -> Group {} Pad {} (from config)", 
                                 full_path, mapping.group, mapping.pad);
                        
                        // Override name if specified in config
                        if let Some(name) = mapping.name {
                            self.sample_names.insert((mapping.group, mapping.pad), name);
                        }
                    }
                    Err(e) => eprintln!("Failed to load {}: {}", full_path, e),
                }
            } else {
                eprintln!("Invalid mapping: group {} pad {} (must be group 0-3, pad 0-15)", 
                         mapping.group, mapping.pad);
            }
        }
        
        Ok(())
    }
    
    pub fn generate_example_config() -> Result<(), Box<dyn std::error::Error>> {
        let example_config = SampleConfig {
            mappings: vec![
                SampleMapping {
                    group: 0,
                    pad: 0,
                    file: "drums/kick.wav".to_string(),
                    name: Some("Kick".to_string()),
                },
                SampleMapping {
                    group: 0,
                    pad: 1,
                    file: "drums/snare.wav".to_string(),
                    name: Some("Snare".to_string()),
                },
                SampleMapping {
                    group: 0,
                    pad: 2,
                    file: "drums/hihat.wav".to_string(),
                    name: Some("Hi-Hat".to_string()),
                },
                SampleMapping {
                    group: 1,
                    pad: 0,
                    file: "bass/bass01.wav".to_string(),
                    name: Some("Bass 1".to_string()),
                },
            ],
        };
        
        let config_json = serde_json::to_string_pretty(&example_config)?;
        std::fs::write("samples/config.example.json", config_json)?;
        
        println!("Generated example config at samples/config.example.json");
        println!("Rename to config.json and edit to use custom mappings");
        
        Ok(())
    }
}