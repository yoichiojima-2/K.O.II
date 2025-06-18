use crate::app::App;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{
        Block, Borders, Cell, Paragraph, Row, Table, Tabs,
    },
    Frame,
};

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Min(10),    // Main content
            Constraint::Length(3),  // Footer
        ])
        .split(f.area());

    draw_header(f, chunks[0], app);
    draw_main_content(f, chunks[1], app);
    draw_footer(f, chunks[2], app);
}

fn draw_header(f: &mut Frame, area: Rect, app: &App) {
    let group_names = ["DRUMS", "BASS", "LEAD", "VOCAL"];
    let titles: Vec<Line> = group_names
        .iter()
        .enumerate()
        .map(|(i, name)| {
            let group_color = match i {
                0 => Color::Rgb(100, 150, 150), // DRUMS - muted teal
                1 => Color::Rgb(100, 100, 150), // BASS - muted blue  
                2 => Color::Rgb(150, 100, 150), // LEAD - muted purple
                3 => Color::Rgb(150, 150, 100), // VOCAL - muted gold
                _ => Color::DarkGray,
            };
            
            if i == app.current_group {
                Line::from(Span::styled(
                    format!(" {} ", name),
                    Style::default()
                        .fg(Color::Black)
                        .bg(group_color)
                        .add_modifier(Modifier::BOLD),
                ))
            } else {
                Line::from(Span::styled(
                    format!(" {} ", name),
                    Style::default().fg(group_color),
                ))
            }
        })
        .collect();

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("K.O.II Terminal"))
        .select(app.current_group)
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Cyan));

    f.render_widget(tabs, area);
}

fn draw_main_content(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50), // Pad grid
            Constraint::Percentage(25), // Pattern view
            Constraint::Percentage(25), // Mixer
        ])
        .split(area);

    draw_pad_grid(f, chunks[0], app);
    draw_pattern_view(f, chunks[1], app);
    draw_mixer(f, chunks[2], app);
}

fn draw_pad_grid(f: &mut Frame, area: Rect, app: &App) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(25); 4])
        .split(area);

    for (row_idx, row_area) in rows.iter().enumerate() {
        let cols = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(25); 4])
            .split(*row_area);

        for (col_idx, col_area) in cols.iter().enumerate() {
            let pad_idx = row_idx * 4 + col_idx;
            let is_selected = app.selected_pad == Some(pad_idx);
            let is_flashing = app.flashing_pads.contains(&(app.current_group, pad_idx));
            
            let sample_name = app.sample_bank
                .get_sample_name(app.current_group, pad_idx)
                .unwrap_or("Empty");

            let key_hint = match pad_idx {
                0 => "7".to_string(),
                1 => "8".to_string(),
                2 => "9".to_string(),
                3 => "0".to_string(),
                4 => "U".to_string(),
                5 => "I".to_string(),
                6 => "O".to_string(),
                7 => "P".to_string(),
                8 => "J".to_string(),
                9 => "K".to_string(),
                10 => "L".to_string(),
                11 => ";".to_string(),
                12 => "M".to_string(),
                13 => ",".to_string(),
                14 => ".".to_string(),
                15 => "/".to_string(),
                _ => "?".to_string(),
            };

            let block_style = if is_flashing {
                Style::default()
                    .fg(Color::White)
                    .bg(Color::Magenta)
                    .add_modifier(Modifier::BOLD)
            } else if is_selected {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Cyan)
            } else if app.sample_bank.has_sample(app.current_group, pad_idx) {
                Style::default()
                    .fg(Color::Cyan)
                    .bg(Color::DarkGray)
            } else {
                Style::default()
                    .fg(Color::DarkGray)
                    .bg(Color::Black)
            };

            let pad_block = Block::default()
                .borders(Borders::ALL)
                .style(block_style);

            let text = Text::from(vec![
                Line::from(Span::styled(
                    format!("[{}]", key_hint),
                    Style::default().add_modifier(Modifier::BOLD),
                )),
                Line::from(""),
                Line::from(Span::styled(
                    sample_name,
                    Style::default(),
                )),
            ]);

            let paragraph = Paragraph::new(text)
                .block(pad_block)
                .alignment(Alignment::Center);

            f.render_widget(paragraph, *col_area);
        }
    }
}

fn draw_pattern_view(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Pattern info
            Constraint::Min(5),     // Step sequencer
        ])
        .split(area);

    // Pattern info  
    let group_names = ["DRUMS", "BASS", "LEAD", "VOCAL"];
    let pattern_info = Paragraph::new(format!(
        "{} Pattern: {:02}\nStep: {:02}/16",
        group_names[app.current_group],
        app.get_current_pattern() + 1,
        app.get_current_step() + 1
    ))
    .block(Block::default().borders(Borders::ALL).title("Pattern"));
    f.render_widget(pattern_info, chunks[0]);

    // Step sequencer grid
    let pattern_grid = app.get_pattern_grid();
    let current_step = app.get_current_step();

    // Create header with step numbers
    let mut header_cells = vec![Cell::from("Pad")];
    for i in 0..16 {
        let step_style = if i == current_step && app.is_playing {
            Style::default().fg(Color::Black).bg(Color::White)
        } else {
            Style::default().fg(Color::White)
        };
        header_cells.push(Cell::from(format!("{:2}", i + 1)).style(step_style));
    }
    let header = Row::new(header_cells).style(Style::default().fg(Color::White));

    // Create rows for each pad
    let mut rows = vec![header];
    for (pad_idx, pad_steps) in pattern_grid.iter().enumerate().take(16) {
        let mut cells = vec![Cell::from(format!("{:2}", pad_idx))];
        for (step_idx, &has_hit) in pad_steps.iter().enumerate() {
            let cell_content = if has_hit { "●" } else { "·" };
            let group_color = match app.current_group {
                0 => Color::Rgb(100, 150, 150), // DRUMS - muted teal
                1 => Color::Rgb(100, 100, 150), // BASS - muted blue  
                2 => Color::Rgb(150, 100, 150), // LEAD - muted purple
                3 => Color::Rgb(150, 150, 100), // VOCAL - muted gold
                _ => Color::DarkGray,
            };
            
            let cell_style = if step_idx == current_step && app.is_playing {
                Style::default().fg(Color::Black).bg(Color::White)
            } else if has_hit {
                Style::default().fg(group_color)
            } else {
                Style::default().fg(Color::DarkGray)
            };
            cells.push(Cell::from(cell_content).style(cell_style));
        }
        rows.push(Row::new(cells));
    }

    let widths = [Constraint::Length(3)]
        .iter()
        .chain([Constraint::Length(3); 16].iter())
        .cloned()
        .collect::<Vec<_>>();

    let table = Table::new(rows, widths)
        .block(Block::default().borders(Borders::ALL).title("Sequencer"))
        .column_spacing(0);

    f.render_widget(table, chunks[1]);
}

fn draw_mixer(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),   // Master volume
            Constraint::Min(8),      // Group volumes
        ])
        .split(area);

    // Master volume section
    let master_vol = (app.get_master_volume() * 100.0) as u8;
    let master_bar = create_volume_bar(master_vol, app.is_master_muted());
    let master_text = format!(
        "MASTER: {}%\n{}\n{}", 
        master_vol,
        master_bar,
        if app.is_master_muted() { "[MUTED]" } else { "" }
    );
    
    let master_style = if app.is_master_muted() {
        Style::default().fg(Color::Red)
    } else {
        Style::default().fg(Color::White)
    };
    
    let master_block = Paragraph::new(master_text)
        .block(Block::default().borders(Borders::ALL).title("Master (=/- M)"))
        .style(master_style)
        .alignment(Alignment::Center);
    f.render_widget(master_block, chunks[0]);

    // Group volumes
    let group_names = ["DRUMS", "BASS", "LEAD", "VOCAL"];
    let group_keys = ["1/! F1", "2/@ F2", "3/# F3", "4/$ F4"];
    let group_colors = [
        Color::Rgb(100, 150, 150), // DRUMS - muted teal
        Color::Rgb(100, 100, 150), // BASS - muted blue  
        Color::Rgb(150, 100, 150), // LEAD - muted purple
        Color::Rgb(150, 150, 100), // VOCAL - muted gold
    ];
    
    let group_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(25); 4])
        .split(chunks[1]);

    for (i, (name, keys)) in group_names.iter().zip(group_keys.iter()).enumerate() {
        let vol = (app.get_group_volume(i) * 100.0) as u8;
        let bar = create_volume_bar(vol, app.is_group_muted(i));
        let text = format!("{}: {}%\n{}", name, vol, bar);
        
        let style = if app.is_group_muted(i) {
            Style::default().fg(Color::Red)
        } else if i == app.current_group {
            Style::default().fg(group_colors[i]).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(group_colors[i])
        };
        
        let mute_indicator = if app.is_group_muted(i) { " [MUTED]" } else { "" };
        let block_title = format!("{}{}", keys, mute_indicator);
        
        let group_block = Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL).title(block_title))
            .style(style)
            .alignment(Alignment::Center);
        f.render_widget(group_block, group_chunks[i]);
    }
}

fn create_volume_bar(volume: u8, is_muted: bool) -> String {
    if is_muted {
        "■■■■■■■■■■".to_string()
    } else {
        let filled = (volume / 10) as usize;
        let empty = 10 - filled;
        "█".repeat(filled) + &"░".repeat(empty)
    }
}

fn draw_footer(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25), // Transport
            Constraint::Percentage(25), // Tempo
            Constraint::Percentage(50), // Help
        ])
        .split(area);

    // Transport controls
    let transport_text = format!(
        "{}  {}",
        if app.is_playing { "⏸ PLAYING" } else { "⏵ STOPPED" },
        if app.is_recording { "● REC" } else { "○" }
    );
    let transport = Paragraph::new(transport_text)
        .block(Block::default().borders(Borders::ALL).title("Transport"))
        .style(if app.is_playing {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default().fg(Color::White)
        });
    f.render_widget(transport, chunks[0]);

    // Tempo
    let tempo = Paragraph::new(format!("{} BPM", app.tempo))
        .block(Block::default().borders(Borders::ALL).title("Tempo"))
        .alignment(Alignment::Center);
    f.render_widget(tempo, chunks[1]);

    // Help
    let help_text = "SPACE:Play/Stop | R:Record | C:Clear | TAB:Groups | ←→:Patterns | ↑↓:Tempo | =/−:Master Vol | M:Master Mute | 1-4/!@#$:Group Vol | F1-F4:Group Mute | ESC:Quit";
    let help = Paragraph::new(help_text)
        .block(Block::default().borders(Borders::ALL).title("Controls"))
        .style(Style::default().fg(Color::DarkGray));
    f.render_widget(help, chunks[2]);
}