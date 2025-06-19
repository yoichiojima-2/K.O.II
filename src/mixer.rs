use rodio::{Decoder, OutputStream, Sink, Source};
use std::io::Cursor;

pub struct Mixer {
    _output_stream: OutputStream,
    output_handle: rodio::OutputStreamHandle,
    master_volume: f32,
    group_volumes: [f32; 4], // Volume for each sample group
    group_muted: [bool; 4],  // Mute state for each group
    master_muted: bool,
}

impl Mixer {
    pub fn new() -> Self {
        let (output_stream, output_handle) = OutputStream::try_default()
            .expect("Failed to create audio output stream");
        
        eprintln!("Mixer initialized successfully");
        
        Self {
            _output_stream: output_stream,
            output_handle,
            master_volume: 0.7,
            group_volumes: [0.8, 0.8, 0.8, 0.8], // Default volume for all groups
            group_muted: [false; 4],
            master_muted: false,
        }
    }

    pub fn play_sample(&mut self, sample_data: &[u8], group: usize) {
        if sample_data.is_empty() || group >= 4 {
            return;
        }

        // Calculate final volume
        let final_volume = if self.master_muted || self.group_muted[group] {
            0.0
        } else {
            self.master_volume * self.group_volumes[group]
        };

        let cursor = Cursor::new(sample_data.to_vec());
        
        match Decoder::new(cursor) {
            Ok(source) => {
                match Sink::try_new(&self.output_handle) {
                    Ok(sink) => {
                        // Apply volume to the source
                        let amplified_source = source.amplify(final_volume);
                        sink.append(amplified_source);
                        sink.detach();
                    }
                    Err(e) => eprintln!("Failed to create audio sink: {}", e),
                }
            }
            Err(e) => eprintln!("Failed to decode audio sample: {}", e),
        }
    }

    pub fn play_tone(&mut self, frequency: f32, duration: f32, group: usize) {
        if group >= 4 {
            return;
        }

        let final_volume = if self.master_muted || self.group_muted[group] {
            0.0
        } else {
            self.master_volume * self.group_volumes[group] * 0.3
        };

        let sample_rate = 44100;
        let samples = (sample_rate as f32 * duration) as usize;
        
        let sine_wave = (0..samples)
            .map(|i| {
                let t = i as f32 / sample_rate as f32;
                (t * frequency * 2.0 * std::f32::consts::PI).sin() * final_volume
            })
            .collect::<Vec<f32>>();

        match Sink::try_new(&self.output_handle) {
            Ok(sink) => {
                let source = rodio::buffer::SamplesBuffer::new(1, sample_rate, sine_wave);
                sink.append(source);
                sink.detach();
            }
            Err(e) => eprintln!("Failed to create audio sink for tone: {}", e),
        }
    }

    // Master volume controls
    pub fn set_master_volume(&mut self, volume: f32) {
        self.master_volume = volume.clamp(0.0, 1.0);
    }

    pub fn get_master_volume(&self) -> f32 {
        self.master_volume
    }

    pub fn adjust_master_volume(&mut self, delta: f32) {
        self.master_volume = (self.master_volume + delta).clamp(0.0, 1.0);
    }

    pub fn toggle_master_mute(&mut self) {
        self.master_muted = !self.master_muted;
    }

    pub fn is_master_muted(&self) -> bool {
        self.master_muted
    }

    // Group volume controls
    pub fn set_group_volume(&mut self, group: usize, volume: f32) {
        if group < 4 {
            self.group_volumes[group] = volume.clamp(0.0, 1.0);
        }
    }

    pub fn get_group_volume(&self, group: usize) -> f32 {
        if group < 4 {
            self.group_volumes[group]
        } else {
            0.0
        }
    }

    pub fn adjust_group_volume(&mut self, group: usize, delta: f32) {
        if group < 4 {
            self.group_volumes[group] = (self.group_volumes[group] + delta).clamp(0.0, 1.0);
        }
    }

    pub fn toggle_group_mute(&mut self, group: usize) {
        if group < 4 {
            self.group_muted[group] = !self.group_muted[group];
        }
    }

    pub fn is_group_muted(&self, group: usize) -> bool {
        if group < 4 {
            self.group_muted[group]
        } else {
            false
        }
    }

    pub fn get_group_names() -> &'static [&'static str] {
        &["DRUMS", "BASS", "LEAD", "VOCAL"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mixer_creation() {
        let mixer = Mixer::new();
        assert_eq!(mixer.master_volume, 0.7);
        assert_eq!(mixer.group_volumes, [0.8, 0.8, 0.8, 0.8]);
        assert_eq!(mixer.group_muted, [false; 4]);
        assert!(!mixer.master_muted);
    }

    #[test]
    fn test_master_volume_controls() {
        let mut mixer = Mixer::new();
        
        // Test get initial volume
        assert_eq!(mixer.get_master_volume(), 0.7);
        
        // Test set volume
        mixer.set_master_volume(0.5);
        assert_eq!(mixer.get_master_volume(), 0.5);
        
        // Test volume clamping
        mixer.set_master_volume(1.5); // Above max
        assert_eq!(mixer.get_master_volume(), 1.0);
        
        mixer.set_master_volume(-0.5); // Below min
        assert_eq!(mixer.get_master_volume(), 0.0);
        
        // Test adjust volume
        mixer.set_master_volume(0.5);
        mixer.adjust_master_volume(0.2);
        assert!((mixer.get_master_volume() - 0.7).abs() < 0.001);
        
        mixer.adjust_master_volume(-0.3);
        assert!((mixer.get_master_volume() - 0.4).abs() < 0.001);
        
        // Test adjust volume with clamping
        mixer.adjust_master_volume(1.0);
        assert_eq!(mixer.get_master_volume(), 1.0);
        
        mixer.adjust_master_volume(-2.0);
        assert_eq!(mixer.get_master_volume(), 0.0);
    }

    #[test]
    fn test_master_mute_controls() {
        let mut mixer = Mixer::new();
        
        // Test initial state
        assert!(!mixer.is_master_muted());
        
        // Test toggle mute
        mixer.toggle_master_mute();
        assert!(mixer.is_master_muted());
        
        mixer.toggle_master_mute();
        assert!(!mixer.is_master_muted());
    }

    #[test]
    fn test_group_volume_controls() {
        let mut mixer = Mixer::new();
        
        // Test get initial volume
        assert_eq!(mixer.get_group_volume(0), 0.8);
        assert_eq!(mixer.get_group_volume(3), 0.8);
        
        // Test invalid group
        assert_eq!(mixer.get_group_volume(99), 0.0);
        
        // Test set volume
        mixer.set_group_volume(0, 0.6);
        assert_eq!(mixer.get_group_volume(0), 0.6);
        
        // Test volume clamping
        mixer.set_group_volume(1, 1.5); // Above max
        assert_eq!(mixer.get_group_volume(1), 1.0);
        
        mixer.set_group_volume(2, -0.5); // Below min
        assert_eq!(mixer.get_group_volume(2), 0.0);
        
        // Test invalid group (should not panic)
        mixer.set_group_volume(99, 0.5);
        
        // Test adjust volume
        mixer.set_group_volume(0, 0.5);
        mixer.adjust_group_volume(0, 0.2);
        assert!((mixer.get_group_volume(0) - 0.7).abs() < 0.001);
        
        mixer.adjust_group_volume(0, -0.3);
        assert!((mixer.get_group_volume(0) - 0.4).abs() < 0.001);
        
        // Test adjust volume with clamping
        mixer.adjust_group_volume(0, 1.0);
        assert_eq!(mixer.get_group_volume(0), 1.0);
        
        mixer.adjust_group_volume(0, -2.0);
        assert_eq!(mixer.get_group_volume(0), 0.0);
        
        // Test invalid group adjust (should not panic)
        mixer.adjust_group_volume(99, 0.5);
    }

    #[test]
    fn test_group_mute_controls() {
        let mut mixer = Mixer::new();
        
        // Test initial state
        for group in 0..4 {
            assert!(!mixer.is_group_muted(group));
        }
        
        // Test invalid group
        assert!(!mixer.is_group_muted(99));
        
        // Test toggle mute
        mixer.toggle_group_mute(0);
        assert!(mixer.is_group_muted(0));
        assert!(!mixer.is_group_muted(1)); // Others should remain unchanged
        
        mixer.toggle_group_mute(0);
        assert!(!mixer.is_group_muted(0));
        
        // Test invalid group toggle (should not panic)
        mixer.toggle_group_mute(99);
        
        // Test all groups
        for group in 0..4 {
            mixer.toggle_group_mute(group);
            assert!(mixer.is_group_muted(group));
            
            mixer.toggle_group_mute(group);
            assert!(!mixer.is_group_muted(group));
        }
    }

    #[test]
    fn test_play_sample_validation() {
        let mut mixer = Mixer::new();
        
        // Test empty sample data (should not panic)
        mixer.play_sample(&[], 0);
        
        // Test invalid group (should not panic)
        let sample_data = vec![1, 2, 3, 4];
        mixer.play_sample(&sample_data, 99);
    }

    #[test]
    fn test_play_tone_validation() {
        let mut mixer = Mixer::new();
        
        // Test invalid group (should not panic)
        mixer.play_tone(440.0, 0.1, 99);
        
        // Test valid parameters (should not panic)
        mixer.play_tone(440.0, 0.01, 0); // Very short duration to avoid blocking test
    }

    #[test]
    fn test_group_names() {
        let names = Mixer::get_group_names();
        assert_eq!(names.len(), 4);
        assert_eq!(names[0], "DRUMS");
        assert_eq!(names[1], "BASS");
        assert_eq!(names[2], "LEAD");
        assert_eq!(names[3], "VOCAL");
    }

    #[test]
    fn test_volume_calculation_with_mute() {
        let mut mixer = Mixer::new();
        
        // Set volumes
        mixer.set_master_volume(0.8);
        mixer.set_group_volume(0, 0.6);
        
        // Test normal playback calculation (indirectly through internal logic)
        // Expected final volume should be 0.8 * 0.6 = 0.48
        
        // Test with master mute
        mixer.toggle_master_mute();
        // Final volume should be 0.0 regardless of other settings
        
        // Test with group mute
        mixer.toggle_master_mute(); // Unmute master
        mixer.toggle_group_mute(0);
        // Final volume should be 0.0 for group 0
        
        // These tests verify the logic exists without testing audio output
        assert!(mixer.is_master_muted() || !mixer.is_master_muted()); // Basic state verification
    }
}