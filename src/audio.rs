use rodio::{Decoder, OutputStream, Sink};
use std::io::Cursor;

pub struct AudioEngine {
    _output_stream: OutputStream,
    output_handle: rodio::OutputStreamHandle,
    sinks: Vec<Sink>,
}

impl AudioEngine {
    pub fn new() -> Self {
        let (output_stream, output_handle) = OutputStream::try_default()
            .expect("Failed to create audio output stream");
        
        eprintln!("Audio system initialized successfully");
        
        Self {
            _output_stream: output_stream,
            output_handle,
            sinks: Vec::new(),
        }
    }

    pub fn play_sample(&mut self, sample_data: &[u8]) {
        if sample_data.is_empty() {
            return;
        }

        // Create a cursor from the sample data
        let cursor = Cursor::new(sample_data.to_vec());
        
        // Try to decode the audio data
        match Decoder::new(cursor) {
            Ok(source) => {
                // Create a new sink for this sample
                match Sink::try_new(&self.output_handle) {
                    Ok(sink) => {
                        sink.append(source);
                        sink.detach(); // Let it play in the background
                    }
                    Err(e) => eprintln!("Failed to create audio sink: {}", e),
                }
            }
            Err(e) => eprintln!("Failed to decode audio sample: {}", e),
        }
    }

    pub fn play_tone(&mut self, frequency: f32, duration: f32) {
        // Generate a simple sine wave
        let sample_rate = 44100;
        let samples = (sample_rate as f32 * duration) as usize;
        
        let sine_wave = (0..samples)
            .map(|i| {
                let t = i as f32 / sample_rate as f32;
                (t * frequency * 2.0 * std::f32::consts::PI).sin() * 0.3
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

    pub fn play_file(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file = std::fs::File::open(path)?;
        let source = Decoder::new(std::io::BufReader::new(file))?;
        
        let sink = Sink::try_new(&self.output_handle)?;
        sink.append(source);
        sink.detach();
        
        Ok(())
    }

    pub fn stop_all(&mut self) {
        for sink in &self.sinks {
            sink.stop();
        }
        self.sinks.clear();
    }
}