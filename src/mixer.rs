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