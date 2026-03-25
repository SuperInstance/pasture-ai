//! ratatui TUI stub for Superinstance Backend
//! Terminal interface for local management

use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, Event, KeyCode},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use std::io;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Main loop
    loop {
        terminal.draw(ui)?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Esc => break,
                    _ => {}
                }
            }
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;

    println!("Superinstance TUI exited. Use 'backend' binary for HTTP API.");
    Ok(())
}

fn ui(f: &mut Frame) {
    // Create layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.area());

    // Title
    let title = Paragraph::new("🐕 Superinstance Backend TUI")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Main content
    let content = Paragraph::new(vec![
        Line::raw("Status: Running"),
        Line::raw(""),
        Line::raw("Server: http://127.0.0.1:3001"),
        Line::raw(""),
        Line::styled("This is a TUI stub.", Style::default().fg(Color::Yellow)),
        Line::raw("Full terminal interface coming soon."),
        Line::raw(""),
        Line::raw("Features planned:"),
        Line::raw("  • Real-time log viewing"),
        Line::raw("  • Model management"),
        Line::raw("  • Breed configuration"),
        Line::raw("  • Chat interface"),
    ])
    .block(Block::default().borders(Borders::ALL).title("Status"));
    f.render_widget(content, chunks[1]);

    // Footer
    let footer = Paragraph::new("Press 'q' or ESC to quit")
        .style(Style::default().fg(Color::DarkGray))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[2]);
}
