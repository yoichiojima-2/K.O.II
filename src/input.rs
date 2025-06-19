use crossterm::event::{KeyCode, KeyModifiers};
use std::collections::HashMap;
use crate::command::Command;
use crate::config::Config;
use crate::error::Result;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeyBinding {
    pub code: KeyCode,
    pub modifiers: KeyModifiers,
}

impl KeyBinding {
    pub fn new(code: KeyCode) -> Self {
        Self {
            code,
            modifiers: KeyModifiers::empty(),
        }
    }

    pub fn with_modifiers(code: KeyCode, modifiers: KeyModifiers) -> Self {
        Self { code, modifiers }
    }
}

pub struct InputMapper {
    bindings: HashMap<KeyBinding, Command>,
}

impl Default for InputMapper {
    fn default() -> Self {
        Self::from_default_bindings()
    }
}

impl InputMapper {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn from_config(config: &Config) -> Result<Self> {
        let mut bindings = HashMap::new();
        
        // Transport controls
        if let Some(key) = config.parse_key_code(&config.key_bindings.transport.play_stop) {
            bindings.insert(KeyBinding::new(key), Command::TogglePlayback);
        }
        if let Some(key) = config.parse_key_code(&config.key_bindings.transport.record) {
            bindings.insert(KeyBinding::new(key), Command::ToggleRecording);
        }
        if let Some(key) = config.parse_key_code(&config.key_bindings.transport.clear) {
            bindings.insert(KeyBinding::new(key), Command::ClearPattern);
        }
        
        // Navigation
        if let Some(key) = config.parse_key_code(&config.key_bindings.navigation.next_group) {
            bindings.insert(KeyBinding::new(key), Command::NextGroup);
        }
        if let Some(key) = config.parse_key_code(&config.key_bindings.navigation.prev_group) {
            bindings.insert(KeyBinding::new(key), Command::PrevGroup);
        }
        if let Some(key) = config.parse_key_code(&config.key_bindings.navigation.next_pattern) {
            bindings.insert(KeyBinding::new(key), Command::NextPattern);
        }
        if let Some(key) = config.parse_key_code(&config.key_bindings.navigation.prev_pattern) {
            bindings.insert(KeyBinding::new(key), Command::PrevPattern);
        }
        if let Some(key) = config.parse_key_code(&config.key_bindings.navigation.tempo_up) {
            bindings.insert(KeyBinding::new(key), Command::IncreaseTempo(5));
        }
        if let Some(key) = config.parse_key_code(&config.key_bindings.navigation.tempo_down) {
            bindings.insert(KeyBinding::new(key), Command::DecreaseTempo(5));
        }
        
        // Volume controls
        if let Some(key) = config.parse_key_code(&config.key_bindings.volume.master_up) {
            bindings.insert(KeyBinding::new(key), Command::AdjustMasterVolume(0.05));
        }
        if let Some(key) = config.parse_key_code(&config.key_bindings.volume.master_down) {
            bindings.insert(KeyBinding::new(key), Command::AdjustMasterVolume(-0.05));
        }
        if let Some(key) = config.parse_key_code(&config.key_bindings.volume.master_mute) {
            bindings.insert(KeyBinding::new(key), Command::ToggleMasterMute);
        }
        
        // Group volume and mute controls
        for (i, key_str) in config.key_bindings.volume.group_up.iter().enumerate() {
            if let Some(key) = config.parse_key_code(key_str) {
                bindings.insert(KeyBinding::new(key), Command::AdjustGroupVolume(i, 0.05));
            }
        }
        for (i, key_str) in config.key_bindings.volume.group_down.iter().enumerate() {
            if let Some(key) = config.parse_key_code(key_str) {
                bindings.insert(KeyBinding::new(key), Command::AdjustGroupVolume(i, -0.05));
            }
        }
        for (i, key_str) in config.key_bindings.volume.group_mute.iter().enumerate() {
            if let Some(key) = config.parse_key_code(key_str) {
                bindings.insert(KeyBinding::new(key), Command::ToggleGroupMute(i));
            }
        }
        
        // Pad triggers
        for (key_str, &pad) in &config.key_bindings.pads {
            if let Some(key) = config.parse_key_code(key_str) {
                bindings.insert(KeyBinding::new(key), Command::TriggerPad(pad));
            }
        }
        
        // Application
        bindings.insert(KeyBinding::new(KeyCode::Esc), Command::Quit);
        
        Ok(Self { bindings })
    }
    
    fn from_default_bindings() -> Self {
        let mut bindings = HashMap::new();
        
        // Transport controls
        bindings.insert(KeyBinding::new(KeyCode::Char(' ')), Command::TogglePlayback);
        bindings.insert(KeyBinding::new(KeyCode::Char('r')), Command::ToggleRecording);
        bindings.insert(KeyBinding::new(KeyCode::Char('c')), Command::ClearPattern);
        
        // Navigation
        bindings.insert(KeyBinding::new(KeyCode::Tab), Command::NextGroup);
        bindings.insert(KeyBinding::new(KeyCode::BackTab), Command::PrevGroup);
        bindings.insert(KeyBinding::new(KeyCode::Right), Command::NextPattern);
        bindings.insert(KeyBinding::new(KeyCode::Left), Command::PrevPattern);
        
        // Tempo
        bindings.insert(KeyBinding::new(KeyCode::Up), Command::IncreaseTempo(5));
        bindings.insert(KeyBinding::new(KeyCode::Down), Command::DecreaseTempo(5));
        
        // Volume controls
        bindings.insert(KeyBinding::new(KeyCode::Char('=')), Command::AdjustMasterVolume(0.05));
        bindings.insert(KeyBinding::new(KeyCode::Char('-')), Command::AdjustMasterVolume(-0.05));
        bindings.insert(KeyBinding::new(KeyCode::Char('M')), Command::ToggleMasterMute);
        
        // Group volume controls
        bindings.insert(KeyBinding::new(KeyCode::Char('1')), Command::AdjustGroupVolume(0, 0.05));
        bindings.insert(KeyBinding::new(KeyCode::Char('!')), Command::AdjustGroupVolume(0, -0.05));
        bindings.insert(KeyBinding::new(KeyCode::Char('2')), Command::AdjustGroupVolume(1, 0.05));
        bindings.insert(KeyBinding::new(KeyCode::Char('@')), Command::AdjustGroupVolume(1, -0.05));
        bindings.insert(KeyBinding::new(KeyCode::Char('3')), Command::AdjustGroupVolume(2, 0.05));
        bindings.insert(KeyBinding::new(KeyCode::Char('#')), Command::AdjustGroupVolume(2, -0.05));
        bindings.insert(KeyBinding::new(KeyCode::Char('4')), Command::AdjustGroupVolume(3, 0.05));
        bindings.insert(KeyBinding::new(KeyCode::Char('$')), Command::AdjustGroupVolume(3, -0.05));
        
        // Group mute controls
        bindings.insert(KeyBinding::new(KeyCode::F(1)), Command::ToggleGroupMute(0));
        bindings.insert(KeyBinding::new(KeyCode::F(2)), Command::ToggleGroupMute(1));
        bindings.insert(KeyBinding::new(KeyCode::F(3)), Command::ToggleGroupMute(2));
        bindings.insert(KeyBinding::new(KeyCode::F(4)), Command::ToggleGroupMute(3));
        
        // Pad triggers
        let pad_mappings = [
            ('7', 0), ('8', 1), ('9', 2), ('0', 3),
            ('u', 4), ('i', 5), ('o', 6), ('p', 7),
            ('j', 8), ('k', 9), ('l', 10), (';', 11),
            ('m', 12), (',', 13), ('.', 14), ('/', 15),
        ];
        
        for (key, pad) in pad_mappings {
            bindings.insert(KeyBinding::new(KeyCode::Char(key)), Command::TriggerPad(pad));
        }
        
        // Application
        bindings.insert(KeyBinding::new(KeyCode::Esc), Command::Quit);
        
        Self { bindings }
    }

    pub fn get_command(&self, key: &KeyBinding) -> Option<&Command> {
        self.bindings.get(key)
    }

    pub fn add_binding(&mut self, key: KeyBinding, command: Command) {
        self.bindings.insert(key, command);
    }

    pub fn remove_binding(&mut self, key: &KeyBinding) -> Option<Command> {
        self.bindings.remove(key)
    }

    pub fn get_bindings(&self) -> &HashMap<KeyBinding, Command> {
        &self.bindings
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_binding_creation() {
        let binding = KeyBinding::new(KeyCode::Char('a'));
        assert_eq!(binding.code, KeyCode::Char('a'));
        assert_eq!(binding.modifiers, KeyModifiers::empty());
        
        let binding = KeyBinding::with_modifiers(KeyCode::Char('c'), KeyModifiers::CONTROL);
        assert_eq!(binding.code, KeyCode::Char('c'));
        assert_eq!(binding.modifiers, KeyModifiers::CONTROL);
    }

    #[test]
    fn test_default_input_mapper() {
        let mapper = InputMapper::default();
        
        // Test some default bindings
        let space_key = KeyBinding::new(KeyCode::Char(' '));
        assert_eq!(mapper.get_command(&space_key), Some(&Command::TogglePlayback));
        
        let esc_key = KeyBinding::new(KeyCode::Esc);
        assert_eq!(mapper.get_command(&esc_key), Some(&Command::Quit));
        
        let pad_key = KeyBinding::new(KeyCode::Char('7'));
        assert_eq!(mapper.get_command(&pad_key), Some(&Command::TriggerPad(0)));
    }

    #[test]
    fn test_custom_bindings() {
        let mut mapper = InputMapper::new();
        
        // Add a custom binding
        let custom_key = KeyBinding::new(KeyCode::Char('x'));
        mapper.add_binding(custom_key.clone(), Command::TogglePlayback);
        assert_eq!(mapper.get_command(&custom_key), Some(&Command::TogglePlayback));
        
        // Remove a binding
        let removed = mapper.remove_binding(&custom_key);
        assert_eq!(removed, Some(Command::TogglePlayback));
        assert_eq!(mapper.get_command(&custom_key), None);
    }

    #[test]
    fn test_all_pad_mappings() {
        let mapper = InputMapper::default();
        
        let pad_keys = [
            ('7', 0), ('8', 1), ('9', 2), ('0', 3),
            ('u', 4), ('i', 5), ('o', 6), ('p', 7),
            ('j', 8), ('k', 9), ('l', 10), (';', 11),
            ('m', 12), (',', 13), ('.', 14), ('/', 15),
        ];
        
        for (key, expected_pad) in pad_keys {
            let binding = KeyBinding::new(KeyCode::Char(key));
            assert_eq!(mapper.get_command(&binding), Some(&Command::TriggerPad(expected_pad)));
        }
    }
}