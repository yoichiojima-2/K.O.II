use crate::app::App;

#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    // Transport
    TogglePlayback,
    ToggleRecording,
    ClearPattern,
    
    // Navigation
    NextGroup,
    PrevGroup,
    NextPattern,
    PrevPattern,
    
    // Tempo
    IncreaseTempo(i32),
    DecreaseTempo(i32),
    
    // Pad triggers
    TriggerPad(usize),
    
    // Volume controls
    AdjustMasterVolume(f32),
    ToggleMasterMute,
    AdjustGroupVolume(usize, f32),
    ToggleGroupMute(usize),
    
    // Application
    Quit,
}

impl Command {
    pub fn execute(&self, app: &mut App) -> Result<(), String> {
        match self {
            Command::TogglePlayback => {
                app.toggle_playback();
                Ok(())
            }
            Command::ToggleRecording => {
                app.toggle_recording();
                Ok(())
            }
            Command::ClearPattern => {
                app.clear_pattern();
                Ok(())
            }
            Command::NextGroup => {
                app.next_group();
                Ok(())
            }
            Command::PrevGroup => {
                app.prev_group();
                Ok(())
            }
            Command::NextPattern => {
                app.next_pattern();
                Ok(())
            }
            Command::PrevPattern => {
                app.prev_pattern();
                Ok(())
            }
            Command::IncreaseTempo(amount) => {
                app.adjust_tempo(*amount);
                Ok(())
            }
            Command::DecreaseTempo(amount) => {
                app.adjust_tempo(-amount);
                Ok(())
            }
            Command::TriggerPad(pad) => {
                if *pad >= 16 {
                    return Err(format!("Invalid pad index: {}", pad));
                }
                app.trigger_pad(*pad);
                Ok(())
            }
            Command::AdjustMasterVolume(delta) => {
                app.adjust_master_volume(*delta);
                Ok(())
            }
            Command::ToggleMasterMute => {
                app.toggle_master_mute();
                Ok(())
            }
            Command::AdjustGroupVolume(group, delta) => {
                if *group >= 4 {
                    return Err(format!("Invalid group index: {}", group));
                }
                app.adjust_group_volume(*group, *delta);
                Ok(())
            }
            Command::ToggleGroupMute(group) => {
                if *group >= 4 {
                    return Err(format!("Invalid group index: {}", group));
                }
                app.toggle_group_mute(*group);
                Ok(())
            }
            Command::Quit => Ok(()), // Handled by the main loop
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_creation() {
        let cmd = Command::TogglePlayback;
        assert_eq!(cmd, Command::TogglePlayback);
        
        let cmd = Command::TriggerPad(5);
        assert_eq!(cmd, Command::TriggerPad(5));
        
        let cmd = Command::AdjustMasterVolume(0.1);
        assert_eq!(cmd, Command::AdjustMasterVolume(0.1));
    }

    #[test]
    fn test_command_execution() {
        let mut app = App::new().unwrap();
        
        // Test toggle playback
        let initial_playing = app.is_playing();
        let cmd = Command::TogglePlayback;
        assert!(cmd.execute(&mut app).is_ok());
        assert_eq!(app.is_playing(), !initial_playing);
        
        // Test invalid pad
        let cmd = Command::TriggerPad(20);
        assert!(cmd.execute(&mut app).is_err());
        
        // Test valid pad
        let cmd = Command::TriggerPad(5);
        assert!(cmd.execute(&mut app).is_ok());
        
        // Test group navigation
        let initial_group = app.get_current_group();
        let cmd = Command::NextGroup;
        assert!(cmd.execute(&mut app).is_ok());
        assert_eq!(app.get_current_group(), (initial_group + 1) % 4);
    }

    #[test]
    fn test_volume_commands() {
        let mut app = App::new().unwrap();
        
        // Test master volume adjustment
        let initial_vol = app.get_master_volume();
        let cmd = Command::AdjustMasterVolume(0.1);
        assert!(cmd.execute(&mut app).is_ok());
        assert!((app.get_master_volume() - (initial_vol + 0.1)).abs() < 0.001);
        
        // Test group volume with valid index
        let cmd = Command::AdjustGroupVolume(0, 0.05);
        assert!(cmd.execute(&mut app).is_ok());
        
        // Test group volume with invalid index
        let cmd = Command::AdjustGroupVolume(5, 0.05);
        assert!(cmd.execute(&mut app).is_err());
        
        // Test mute toggle
        let initial_muted = app.is_master_muted();
        let cmd = Command::ToggleMasterMute;
        assert!(cmd.execute(&mut app).is_ok());
        assert_eq!(app.is_master_muted(), !initial_muted);
    }

    #[test]
    fn test_tempo_commands() {
        let mut app = App::new().unwrap();
        
        let initial_tempo = app.get_tempo();
        let cmd = Command::IncreaseTempo(10);
        assert!(cmd.execute(&mut app).is_ok());
        assert_eq!(app.get_tempo(), initial_tempo + 10);
        
        let cmd = Command::DecreaseTempo(5);
        assert!(cmd.execute(&mut app).is_ok());
        assert_eq!(app.get_tempo(), initial_tempo + 5);
    }
}