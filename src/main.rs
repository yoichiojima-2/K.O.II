mod app;
mod audio;
mod mixer;
mod ui;
mod sequencer;
mod sample;
mod command;
mod input;
mod error;
mod state;
mod config;
mod audio_manager;

use std::io;
use std::time::Duration;

use crossterm::{
    event::{self, Event, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

use app::App;
use sample::SampleBank;
use command::Command;
use input::{InputMapper, KeyBinding};
use error::{AppError, Result};
use config::Config;

fn main() -> Result<()> {
    // Check for command line arguments
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() > 1 {
        match args[1].as_str() {
            "generate-config" => {
                SampleBank::generate_example_config()
                    .map_err(|e| AppError::Config(e.to_string()))?;
                Config::generate_example()?;
                return Ok(());
            }
            "help" | "--help" | "-h" => {
                print_help();
                return Ok(());
            }
            _ => {
                eprintln!("Unknown command: {}", args[1]);
                eprintln!("Run 'cargo run help' for usage information");
                return Ok(());
            }
        }
    }
    
    // Initialize the application
    let app = initialize_app()?;
    
    // Run the terminal UI
    run_terminal(app)?;
    
    Ok(())
}

fn print_help() {
    println!("K.O.II Terminal - Drum Machine/Sequencer");
    println!();
    println!("Usage:");
    println!("  cargo run                  - Start the application");
    println!("  cargo run generate-config  - Generate example config file");
    println!("  cargo run help             - Show this help");
}

fn initialize_app() -> Result<App> {
    println!("Initializing application...");
    let app = App::with_audio_test()?;
    println!("Application initialized successfully!");
    Ok(app)
}

fn run_terminal(app: App) -> Result<()> {
    // Load configuration
    let config = Config::load()?;
    
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    // Create input mapper from config
    let input_mapper = InputMapper::from_config(&config)
        .unwrap_or_else(|_| InputMapper::new());
    
    // Run the app
    let res = run_app(&mut terminal, app, input_mapper, config);
    
    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    
    res
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    input_mapper: InputMapper,
    config: Config,
) -> Result<()> {
    loop {
        terminal.draw(|f| ui::draw(f, &app))?;
        
        if crossterm::event::poll(Duration::from_millis(config.ui.tick_interval_ms))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    let binding = KeyBinding::with_modifiers(key.code, key.modifiers);
                    
                    if let Some(command) = input_mapper.get_command(&binding) {
                        if matches!(command, Command::Quit) {
                            return Ok(());
                        }
                        
                        if let Err(e) = command.execute(&mut app) {
                            eprintln!("Command error: {}", e);
                        }
                    }
                }
            }
        }
        
        app.tick();
    }
}
