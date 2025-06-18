mod app;
mod audio;
mod mixer;
mod ui;
mod sequencer;
mod sample;

use std::error::Error;
use std::io;
use std::time::Duration;

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

use app::App;
use sample::SampleBank;

fn main() -> Result<(), Box<dyn Error>> {
    // Check for command line arguments
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() > 1 {
        match args[1].as_str() {
            "generate-config" => {
                SampleBank::generate_example_config()?;
                return Ok(());
            }
            "help" | "--help" | "-h" => {
                println!("K.O.II Terminal - Drum Machine/Sequencer");
                println!();
                println!("Usage:");
                println!("  cargo run                  - Start the application");
                println!("  cargo run generate-config  - Generate example config file");
                println!("  cargo run help             - Show this help");
                return Ok(());
            }
            _ => {
                eprintln!("Unknown command: {}", args[1]);
                eprintln!("Run 'cargo run help' for usage information");
                return Ok(());
            }
        }
    }
    
    // Create app first to check if that's the issue
    println!("Creating app...");
    let mut app = App::new();
    println!("App created successfully!");
    
    // Test audio by playing the built-in kick drum
    if let Some(kick_sample) = app.sample_bank.get_sample(0, 0) {
        println!("Testing built-in kick drum...");
        app.mixer.play_sample(kick_sample, 0);
        std::thread::sleep(std::time::Duration::from_millis(1000));
        println!("Audio test complete!");
    } else {
        println!("No kick drum sample found");
    }

    // Setup terminal
    println!("Setting up terminal...");
    enable_raw_mode().map_err(|e| {
        eprintln!("Failed to enable raw mode: {}", e);
        e
    })?;
    
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).map_err(|e| {
        disable_raw_mode().ok();
        eprintln!("Failed to enter alternate screen: {}", e);
        e
    })?;
    
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).map_err(|e| {
        disable_raw_mode().ok();
        eprintln!("Failed to create terminal: {}", e);
        e
    })?;
    
    println!("Terminal setup complete, starting app...");

    // Run the app
    let res = run_app(&mut terminal, app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::draw(f, &app))?;

        if crossterm::event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Esc => return Ok(()),
                        KeyCode::Char(' ') => app.toggle_playback(),
                        KeyCode::Char('r') => app.toggle_recording(),
                        KeyCode::Char('c') => app.clear_pattern(),
                        KeyCode::Tab => app.next_group(),
                        KeyCode::BackTab => app.prev_group(),
                        // Mixer controls
                        KeyCode::Char('=') => app.adjust_master_volume(0.05),
                        KeyCode::Char('-') => app.adjust_master_volume(-0.05),
                        KeyCode::Char('M') => app.toggle_master_mute(),
                        KeyCode::Char('1') => app.adjust_group_volume(0, 0.05),
                        KeyCode::Char('!') => app.adjust_group_volume(0, -0.05),
                        KeyCode::Char('2') => app.adjust_group_volume(1, 0.05),
                        KeyCode::Char('@') => app.adjust_group_volume(1, -0.05),
                        KeyCode::Char('3') => app.adjust_group_volume(2, 0.05),
                        KeyCode::Char('#') => app.adjust_group_volume(2, -0.05),
                        KeyCode::Char('4') => app.adjust_group_volume(3, 0.05),
                        KeyCode::Char('$') => app.adjust_group_volume(3, -0.05),
                        KeyCode::Char(c) => {
                            let pad_index = match c {
                                // 1st row: 7890 maps to pads 0,1,2,3 (left to right)
                                '7' => Some(0),
                                '8' => Some(1),
                                '9' => Some(2),
                                '0' => Some(3),
                                // 2nd row: uiop maps to pads 4,5,6,7 (left to right)
                                'u' => Some(4),
                                'i' => Some(5),
                                'o' => Some(6),
                                'p' => Some(7),
                                // 3rd row: jkl; maps to pads 8,9,10,11 (left to right)
                                'j' => Some(8),
                                'k' => Some(9),
                                'l' => Some(10),
                                ';' => Some(11),
                                // 4th row: m,./ maps to pads 12,13,14,15 (left to right)
                                'm' => Some(12),
                                ',' => Some(13),
                                '.' => Some(14),
                                '/' => Some(15),
                                _ => None,
                            };
                            
                            if let Some(pad) = pad_index {
                                app.trigger_pad(pad);
                            }
                        }
                        KeyCode::Up => app.adjust_tempo(5),
                        KeyCode::Down => app.adjust_tempo(-5),
                        KeyCode::Left => app.prev_pattern(),
                        KeyCode::Right => app.next_pattern(),
                        KeyCode::F(1) => app.toggle_group_mute(0),
                        KeyCode::F(2) => app.toggle_group_mute(1),
                        KeyCode::F(3) => app.toggle_group_mute(2),
                        KeyCode::F(4) => app.toggle_group_mute(3),
                        _ => {}
                    }
                }
            }
        }

        app.tick();
    }
}
