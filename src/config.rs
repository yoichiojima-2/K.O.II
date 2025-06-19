use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crossterm::event::KeyCode;
use crate::error::{AppError, Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub key_bindings: KeyBindingsConfig,
    pub audio: AudioConfig,
    pub ui: UIConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyBindingsConfig {
    pub transport: TransportKeys,
    pub navigation: NavigationKeys,
    pub volume: VolumeKeys,
    pub pads: HashMap<String, usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransportKeys {
    pub play_stop: String,
    pub record: String,
    pub clear: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NavigationKeys {
    pub next_group: String,
    pub prev_group: String,
    pub next_pattern: String,
    pub prev_pattern: String,
    pub tempo_up: String,
    pub tempo_down: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VolumeKeys {
    pub master_up: String,
    pub master_down: String,
    pub master_mute: String,
    pub group_up: Vec<String>,
    pub group_down: Vec<String>,
    pub group_mute: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AudioConfig {
    pub default_tempo: u32,
    pub sample_rate: u32,
    pub buffer_size: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UIConfig {
    pub flash_duration_ms: u64,
    pub tick_interval_ms: u64,
}

impl Default for Config {
    fn default() -> Self {
        let mut pads = HashMap::new();
        
        // Default pad mappings
        let pad_keys = [
            ("7", 0), ("8", 1), ("9", 2), ("0", 3),
            ("u", 4), ("i", 5), ("o", 6), ("p", 7),
            ("j", 8), ("k", 9), ("l", 10), (";", 11),
            ("m", 12), (",", 13), (".", 14), ("/", 15),
        ];
        
        for (key, pad) in pad_keys {
            pads.insert(key.to_string(), pad);
        }
        
        Self {
            key_bindings: KeyBindingsConfig {
                transport: TransportKeys {
                    play_stop: " ".to_string(),
                    record: "r".to_string(),
                    clear: "c".to_string(),
                },
                navigation: NavigationKeys {
                    next_group: "Tab".to_string(),
                    prev_group: "BackTab".to_string(),
                    next_pattern: "Right".to_string(),
                    prev_pattern: "Left".to_string(),
                    tempo_up: "Up".to_string(),
                    tempo_down: "Down".to_string(),
                },
                volume: VolumeKeys {
                    master_up: "=".to_string(),
                    master_down: "-".to_string(),
                    master_mute: "M".to_string(),
                    group_up: vec!["1".to_string(), "2".to_string(), "3".to_string(), "4".to_string()],
                    group_down: vec!["!".to_string(), "@".to_string(), "#".to_string(), "$".to_string()],
                    group_mute: vec!["F1".to_string(), "F2".to_string(), "F3".to_string(), "F4".to_string()],
                },
                pads,
            },
            audio: AudioConfig {
                default_tempo: 120,
                sample_rate: 44100,
                buffer_size: 1024,
            },
            ui: UIConfig {
                flash_duration_ms: 150,
                tick_interval_ms: 50,
            },
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = "config.toml";
        
        if !std::path::Path::new(config_path).exists() {
            // Use default config if file doesn't exist
            return Ok(Self::default());
        }
        
        let config_content = std::fs::read_to_string(config_path)
            .map_err(|e| AppError::Config(format!("Failed to read config file: {}", e)))?;
        
        toml::from_str(&config_content)
            .map_err(|e| AppError::Config(format!("Failed to parse config file: {}", e)))
    }
    
    pub fn save(&self) -> Result<()> {
        let config_content = toml::to_string_pretty(self)
            .map_err(|e| AppError::Config(format!("Failed to serialize config: {}", e)))?;
        
        std::fs::write("config.toml", config_content)
            .map_err(|e| AppError::Config(format!("Failed to write config file: {}", e)))
    }
    
    pub fn generate_example() -> Result<()> {
        let config = Self::default();
        let config_content = toml::to_string_pretty(&config)
            .map_err(|e| AppError::Config(format!("Failed to serialize config: {}", e)))?;
        
        std::fs::write("config.example.toml", config_content)
            .map_err(|e| AppError::Config(format!("Failed to write example config: {}", e)))?;
        
        println!("Generated example config at config.example.toml");
        println!("Rename to config.toml and edit to customize");
        
        Ok(())
    }
    
    pub fn parse_key_code(&self, key_str: &str) -> Option<KeyCode> {
        match key_str {
            " " => Some(KeyCode::Char(' ')),
            "Tab" => Some(KeyCode::Tab),
            "BackTab" => Some(KeyCode::BackTab),
            "Enter" => Some(KeyCode::Enter),
            "Esc" => Some(KeyCode::Esc),
            "Backspace" => Some(KeyCode::Backspace),
            "Left" => Some(KeyCode::Left),
            "Right" => Some(KeyCode::Right),
            "Up" => Some(KeyCode::Up),
            "Down" => Some(KeyCode::Down),
            "Home" => Some(KeyCode::Home),
            "End" => Some(KeyCode::End),
            "PageUp" => Some(KeyCode::PageUp),
            "PageDown" => Some(KeyCode::PageDown),
            "Delete" => Some(KeyCode::Delete),
            "Insert" => Some(KeyCode::Insert),
            s if s.starts_with('F') && s.len() > 1 => {
                s[1..].parse::<u8>().ok().map(KeyCode::F)
            }
            s if s.len() == 1 => {
                s.chars().next().map(KeyCode::Char)
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        
        assert_eq!(config.audio.default_tempo, 120);
        assert_eq!(config.ui.flash_duration_ms, 150);
        assert_eq!(config.key_bindings.transport.play_stop, " ");
        assert_eq!(config.key_bindings.pads.get("7"), Some(&0));
    }

    #[test]
    fn test_parse_key_code() {
        let config = Config::default();
        
        assert_eq!(config.parse_key_code(" "), Some(KeyCode::Char(' ')));
        assert_eq!(config.parse_key_code("Tab"), Some(KeyCode::Tab));
        assert_eq!(config.parse_key_code("F1"), Some(KeyCode::F(1)));
        assert_eq!(config.parse_key_code("a"), Some(KeyCode::Char('a')));
        assert_eq!(config.parse_key_code("invalid"), None);
    }

    #[test]
    fn test_save_and_load() {
        use std::fs;
        
        // Clean up any existing test file
        let _ = fs::remove_file("config.toml");
        
        // Save default config
        let config = Config::default();
        assert!(config.save().is_ok());
        
        // Load config
        let loaded = Config::load();
        assert!(loaded.is_ok());
        
        let loaded = loaded.unwrap();
        assert_eq!(loaded.audio.default_tempo, config.audio.default_tempo);
        assert_eq!(loaded.key_bindings.transport.play_stop, config.key_bindings.transport.play_stop);
        
        // Clean up
        let _ = fs::remove_file("config.toml");
    }
}