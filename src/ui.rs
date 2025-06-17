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
            if i == app.current_group {
                Line::from(Span::styled(
                    format!(" {} ", name),
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ))
            } else {
                Line::from(Span::styled(
                    format!(" {} ", name),
                    Style::default().fg(Color::Gray),
                ))
            }
        })
        .collect();

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("K.O.II Terminal"))
        .select(app.current_group)
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow));

    f.render_widget(tabs, area);
}

fn draw_main_content(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(60), // Pad grid
            Constraint::Percentage(40), // Pattern view
        ])
        .split(area);

    draw_pad_grid(f, chunks[0], app);
    draw_pattern_view(f, chunks[1], app);
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
            
            let sample_name = app.sample_bank
                .get_sample_name(app.current_group, pad_idx)
                .unwrap_or("Empty");

            let key_hint = match pad_idx {
                0..=9 => format!("{}", pad_idx),
                10 => "Q".to_string(),
                11 => "W".to_string(),
                12 => "E".to_string(),
                13 => "R".to_string(),
                14 => "T".to_string(),
                15 => "Y".to_string(),
                _ => "?".to_string(),
            };

            let block_style = if is_selected {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Yellow)
            } else if app.sample_bank.has_sample(app.current_group, pad_idx) {
                Style::default()
                    .fg(Color::Green)
                    .bg(Color::Black)
            } else {
                Style::default()
                    .fg(Color::Gray)
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
    let pattern_info = Paragraph::new(format!(
        "Pattern: {:02}\nStep: {:02}/16",
        app.current_pattern + 1,
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
            Style::default().fg(Color::Black).bg(Color::Yellow)
        } else {
            Style::default().fg(Color::White)
        };
        header_cells.push(Cell::from(format!("{:2}", i + 1)).style(step_style));
    }
    let header = Row::new(header_cells).style(Style::default().fg(Color::Yellow));

    // Create rows for each pad
    let mut rows = vec![header];
    for (pad_idx, pad_steps) in pattern_grid.iter().enumerate().take(8) {
        let mut cells = vec![Cell::from(format!("{:2}", pad_idx))];
        for (step_idx, &has_hit) in pad_steps.iter().enumerate() {
            let cell_content = if has_hit { "●" } else { "·" };
            let cell_style = if step_idx == current_step && app.is_playing {
                Style::default().fg(Color::Black).bg(Color::Yellow)
            } else if has_hit {
                Style::default().fg(Color::Green)
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
            Style::default().fg(Color::Green)
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
    let help_text = "SPACE:Play/Stop | R:Record | C:Clear | TAB:Groups | ←→:Patterns | ↑↓:Tempo | Q:Quit";
    let help = Paragraph::new(help_text)
        .block(Block::default().borders(Borders::ALL).title("Controls"))
        .style(Style::default().fg(Color::Gray));
    f.render_widget(help, chunks[2]);
}