mod app;
mod audio;
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

fn main() -> Result<(), Box<dyn Error>> {
    // Create app first to check if that's the issue
    println!("Creating app...");
    let app = App::new();
    println!("App created successfully!");

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
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                        KeyCode::Char(' ') => app.toggle_playback(),
                        KeyCode::Char('r') => app.toggle_recording(),
                        KeyCode::Char('c') => app.clear_pattern(),
                        KeyCode::Tab => app.next_group(),
                        KeyCode::BackTab => app.prev_group(),
                        KeyCode::Char(c) if c.is_digit(10) => {
                            if let Some(pad) = c.to_digit(10) {
                                app.trigger_pad(pad as usize);
                            }
                        }
                        KeyCode::Char(c) if c.is_alphabetic() => {
                            let pad_map = "qwertyuiopasdfghjklzxcvbnm";
                            if let Some(idx) = pad_map.find(c) {
                                app.trigger_pad(idx);
                            }
                        }
                        KeyCode::Up => app.adjust_tempo(5),
                        KeyCode::Down => app.adjust_tempo(-5),
                        KeyCode::Left => app.prev_pattern(),
                        KeyCode::Right => app.next_pattern(),
                        _ => {}
                    }
                }
            }
        }

        app.tick();
    }
}
