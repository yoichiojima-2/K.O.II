use crate::error::{AppError, Result};
use crate::sample::SampleBank;
use crate::mixer::Mixer;

pub struct AudioManager {
    pub mixer: Mixer,
    pub sample_bank: SampleBank,
}

impl AudioManager {
    pub fn new() -> Result<Self> {
        let mixer = Mixer::new();
        let mut sample_bank = SampleBank::new();
        
        // Load default samples
        if let Err(e) = sample_bank.load_defaults() {
            eprintln!("Warning: Failed to load samples: {}", e);
        }
        
        Ok(Self {
            mixer,
            sample_bank,
        })
    }
    
    pub fn test_audio(&mut self) -> Result<()> {
        if let Some(kick_sample) = self.sample_bank.get_sample(0, 0) {
            println!("Testing built-in kick drum...");
            self.mixer.play_sample(kick_sample, 0);
            std::thread::sleep(std::time::Duration::from_millis(1000));
            println!("Audio test complete!");
            Ok(())
        } else {
            Err(AppError::Audio("No kick drum sample found for testing".to_string()))
        }
    }
    
    pub fn validate_audio_system(&self) -> Result<()> {
        // Basic validation that audio system is ready
        println!("Validating audio system...");
        
        // Check if we have at least one sample loaded
        let has_samples = (0..4).any(|group| {
            (0..16).any(|pad| self.sample_bank.has_sample(group, pad))
        });
        
        if !has_samples {
            return Err(AppError::Audio("No samples loaded in any group".to_string()));
        }
        
        println!("Audio system validation complete!");
        Ok(())
    }
}

impl Default for AudioManager {
    fn default() -> Self {
        Self::new().unwrap_or_else(|e| {
            eprintln!("Failed to initialize audio manager: {}", e);
            Self {
                mixer: Mixer::new(),
                sample_bank: SampleBank::new(),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_manager_creation() {
        let result = AudioManager::new();
        assert!(result.is_ok());
    }

    #[test]
    fn test_audio_validation() {
        let audio_manager = AudioManager::new().unwrap();
        
        // Should pass validation since we load default samples
        let result = audio_manager.validate_audio_system();
        assert!(result.is_ok());
    }
}