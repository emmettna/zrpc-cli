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
use crate::grpc_client;
use crate::grpc_request_dsl;
use ui_states::*;
use unicode_width::UnicodeWidthStr;
use grpc_request_dsl::*;

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app_state: AppState) -> io::Result<()> {
    fn handle_service_list_request(state: &mut AppState) {
        let service_request = state.request_state.service_request.clone();
        let services = grpc_client::request_service_list(&service_request.host, &service_request.port);
        let messages = services.clone().iter().enumerate().map(|(i, s)| format!("[{}] {}", i, s.0)).collect::<Vec<String>>();
        let mut m = "".to_owned();
        if messages.len() <= 0 {
            m.push_str("Service not found. Check if Host and Port are correct")
        } else {
            state.request_state.next();
            state.request_state.next();
            state.request_state.next();
            m.push_str("Select Service\n");
            m.push_str(&messages.join("\n"));
            state.result = Some(services.iter().map(|s| s.0.clone()).collect());
        }
        state.update_instruction_messages(m.as_str())
    }

    fn handle_function_list_request(state: &mut AppState) {
        let r = state.request_state.service_request.clone();
        let methods = grpc_client::request_function_list(&r.host, &r.port, &r.service_name);
        let messages = methods.clone().iter().enumerate().map(|(i, s)| format!("[{}] {}", i, s.0)).collect::<Vec<String>>();
        let mut m = "".to_owned();
        if messages.len() <= 0 {
            m.push_str("Method not found. Check if Host, Port and Service are correct")
        } else {
            state.request_state.next();
            state.request_state.next();
            state.request_state.next();
            state.request_state.next();
            m.push_str("Method Service\n");
            m.push_str(&messages.join("\n"));
            state.result = Some(methods.iter().map(|s| s.0.clone()).collect());
        }
        state.debug("Getting here");
        state.update_instruction_messages(m.as_str())
    }

    fn handle_send_request(state: &mut AppState) {
        let r = state.request_state.service_request.clone();
        let result = grpc_client::request(&r);
        match result {
            Ok(v) => {
                let splitted = v.split("\n").collect::<Vec<&str>>().iter().map(|&s|s.to_owned()).collect();
                state.set_result(splitted)
            },
            Err(e) => state.update_instruction_messages(e.as_str())
        }
    }


    loop {
        match app_state.command {
            StateCommands::SendServiceListRequest => {
                if app_state.is_service_names_requestable() {
                    handle_service_list_request(&mut app_state);
                    app_state.next_command();
                }
            },
            StateCommands::SendFunctionListRequest => {
                if app_state.is_service_methods_requestable() {
                    handle_function_list_request(&mut app_state);
                    app_state.next_command();
                }
            },
            StateCommands::SetBody => {
                app_state.request_state.select(4);
            },
            StateCommands::Send => {
                handle_send_request(&mut app_state);
                app_state.next_command();
            }
            _ => {}
        }
        terminal.draw(|frame|ui(frame, &mut app_state))?;

        // Select field -> Edit -> Send -> Get result
        if let Event::Key(key) = event::read()? {
            if SelectedField::NotSelected == app_state.request_state.selected_field {
                app_state.update_instruction_messages("Select field to modify then Enter. OR Enter to send request");
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('e') | KeyCode::Enter => app_state.update_instruction_messages("Field is not selected. Select field to modify"),
                    KeyCode::Down => app_state.request_state.next(),
                    KeyCode::Up =>  app_state.request_state.prev(),
                    _ => {}
                }
            } else {
                // app_state.update_instruction_messages(
                //     "Press `Enter`(or `e`) to edit Or `q` to exit"
                // );
                app_state.debug(format!("state: {:?}, {:?}\n{:?}", &app_state.result, &app_state.command, &app_state.request_state.selected_field).as_str());
                match app_state.input_mode {
                    InputMode::Normal =>
                        match key.code {
                            KeyCode::Char('q')                  => return Ok(()),
                            KeyCode::Esc | KeyCode::Left        => app_state.request_state.unselect(),
                            KeyCode::Char('e') | KeyCode::Enter => app_state.update_input_mode(InputMode::Editing),
                            KeyCode::Down                       => app_state.request_state.next(),
                            KeyCode::Up                         =>  app_state.request_state.prev(),
                            _ => {}
                        },
                    InputMode::Editing => match key.code {
                        KeyCode::Enter => {
                            match app_state.command {
                                StateCommands::SetPort => {
                                    let user_input: String = app_state.input.drain(..).collect();
                                    app_state.input_messages.push(user_input.clone());
                                    app_state.update_field(user_input);
                                    app_state.next_command();
                                },
                                StateCommands::SelectService => {
                                    let user_input: String = app_state.input.drain(..).collect();
                                    app_state.input_messages.push(user_input.clone());
                                    app_state.update_field(user_input);
                                    app_state.next_command();
                                }
                                StateCommands::SelectFunction => {
                                    let user_input: String = app_state.input.drain(..).collect();
                                    app_state.input_messages.push(user_input.clone());
                                    app_state.update_field(user_input);
                                    app_state.next_command();
                                },
                                StateCommands::SetBody => {
                                    let user_input: String = app_state.input.drain(..).collect();
                                    app_state.input_messages.push(user_input.clone());
                                    app_state.update_field(user_input);
                                    app_state.next_command();
                                },
                                // StateCommands::Send => StateCommands::Sent,
                                _ => {}
                            }
                            // app_state.update_input_mode(InputMode::Normal)
                        }
                        KeyCode::Char(c) => app_state.input.push(c),
                        KeyCode::Backspace => { app_state.input.pop(); }
                        KeyCode::Esc => app_state.update_input_mode(InputMode::Normal),
                        _ => {}
                    }
                    // TODO: Fix
                    InputMode::MultilineEditing => match key.code {
                        KeyCode::Enter => app_state.input_messages.push(app_state.input.drain(..).collect()),
                        KeyCode::Char(c) => app_state.input.push(c),
                        KeyCode::Backspace => { app_state.input.pop(); }
                        KeyCode::Esc => app_state.input_mode = InputMode::Normal,
                        _ => {}
                    }

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
        .constraints([Constraint::Min(10), Constraint::Percentage(90)].as_ref())
        .split(f.size());

    let top_chunk =
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[0]);

    let bottom_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[1]);

    let bottom_left = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(bottom_chunks[0]);

    let bottom_right = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(bottom_chunks[1]);

    // Request Box
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

    // Debugging Box
    let debugging_message: Vec<ListItem> = app.debugging.split("\n").map(|m| {
        let content = vec![Spans::from(m)];
        ListItem::new(content)
    }).collect();
    let block = Block::default()
        .title("Debugging Messages")
        .border_style(Style::default().fg(Color::Cyan))
        .borders(Borders::LEFT | Borders::RIGHT)
        .border_type(BorderType::Double);
    let debugging_messages = List::new(debugging_message).block(block);
    f.render_widget(debugging_messages, top_chunk[1]);

    // Result Box

    let result_message: Vec<ListItem> =
        match &app.result {
            Some(message) => {
                message.iter().map(|m| {
                    let content = vec![Spans::from(m.as_str())];
                    ListItem::new(content)
                }).collect()
            },
            None => vec![]
        };
    let messages = List::new(result_message).block(Block::default().title("Result").borders(Borders::ALL));
    f.render_widget(messages, bottom_right[0]);

    // Instruction Box
    let instruction_messages: Vec<ListItem> = app.instruction_messages.split("\n").map(|m| {
        let content = vec![Spans::from(m)];
        ListItem::new(content)
    }).collect();
    let messages = List::new(instruction_messages).block(Block::default().title("Instruction Messages").borders(Borders::ALL));
    f.render_widget(messages, bottom_left[0]);

    // Input Box
    match app.input_mode {
        InputMode::Normal => {}
        InputMode::Editing | InputMode::MultilineEditing => {
            // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
            f.set_cursor(
                // Put cursor past the end of the input text
                bottom_left[1].x + app.input.width() as u16 + 1,
                // Move one line down, from the border to the input line
                bottom_left[1].y + 1,
            )
        }
    }
    let block = Block::default().title("User Input").borders(Borders::ALL);
    let input = Paragraph::new(app.input.as_ref())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing | InputMode::MultilineEditing => Style::default().fg(Color::Yellow),
        }).block(block);
    f.render_widget(input, bottom_left[1]);

}

