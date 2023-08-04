use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, ListItem},
    Frame, Terminal,
};

use tui::text::Spans;
use tui::widgets::{List, Paragraph};
use crate::ui_states;
use ui_states::*;
use unicode_width::UnicodeWidthStr;

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app_state: AppState) -> io::Result<()> {
    loop {
        terminal.draw(|frame|ui(frame, &mut app_state))?;

        if let Event::Key(key) = event::read()? {
            match app_state.input_mode {
                InputMode::Normal =>
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Char('e') => app_state.input_mode = InputMode::Editing,
                        // KeyCode::Left => app.items.unselect(),
                        KeyCode::Down => app_state.request_state.request_fields.next(),
                        KeyCode::Up =>  app_state.request_state.request_fields.previous(),
                        _ => {}
                    },
                InputMode::Editing => match key.code {
                    KeyCode::Enter => app_state.messages.push(app_state.input.drain(..).collect()),
                    KeyCode::Char(c) => app_state.input.push(c),
                    KeyCode::Backspace => { app_state.input.pop(); }
                    KeyCode::Esc => app_state.input_mode = InputMode::Normal,
                    _ => {}
                }

            }

        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut AppState) {
    let size = f.size();

    let block = Block::default()
        .borders(Borders::ALL)
        .title("ZRPC-CLI-TUI")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);
    f.render_widget(block, size);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(4)
        .constraints([Constraint::Min(7), Constraint::Percentage(45), Constraint::Percentage(45)].as_ref())
        .split(f.size());

    let top_chunk =
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[0]);

    let middle_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[1]);

    let list_item: Vec<ListItem> = app.request_state.request_fields.items
        .iter()
        .map(|s| {
            let v = vec![Spans::from(s.clone())];
            ListItem::new(v)
                .style(
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::White)
                )
        })
        .collect();

    let items = List::new(list_item)
        .block(Block::default().borders(Borders::ALL).title("Current Request Info"))
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");
    f.render_stateful_widget(items, top_chunk[0], &mut app.request_state.request_fields.state);

    let block = Block::default()
        .title(vec![
            Span::styled("Styled title", Style::default().fg(Color::White).bg(Color::Red).add_modifier(Modifier::BOLD)),
            Span::from(" background")
        ])
        .title_alignment(Alignment::Right);
    f.render_widget(block, middle_chunk[1]);

    let bottom_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[2]);

    let block = Block::default().title("Instruction Messages").borders(Borders::ALL);
    f.render_widget(block, middle_chunk[0]);

    let block = Block::default().title("User Input").borders(Borders::ALL);
    let input = Paragraph::new(app.input.as_ref())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        }).block(block);
    f.render_widget(input, bottom_chunks[0]);

    match app.input_mode {
        InputMode::Normal => {}
        InputMode::Editing => {
            // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
            f.set_cursor(
                // Put cursor past the end of the input text
                bottom_chunks[0].x + app.input.width() as u16 + 1,
                // Move one line down, from the border to the input line
                bottom_chunks[0].y + 1,
            )
        }
    }

    let block = Block::default()
        .title("With styled borders and doublefs borders")
        .border_style(Style::default().fg(Color::Cyan))
        .borders(Borders::LEFT | Borders::RIGHT)
        .border_type(BorderType::Double);
    f.render_widget(block, bottom_chunks[1])
}

