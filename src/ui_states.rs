use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};
use crate::commands::Commands;
use crate::util;
use std::ops::Index;
use clap::command;
use tonic::codegen::Service;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Corner, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame, Terminal,
};
use crate::grpc_request_dsl::*;
use unicode_width::UnicodeWidthStr;
use crate::grpc_client::request;

#[derive(Debug)]
pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn select(&mut self, n: usize) { self.state.select(Some(n))}

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}

#[derive(Debug, PartialEq)]
pub enum SelectedField {
    NotSelected,
    Host,
    Port,
    ServiceName,
    FunctionName,
    Body
}

impl SelectedField {
    pub fn select_by_index(i: usize) -> SelectedField {
        match i {
            // 1 => Self::NotSelected,
            0 => Self::Host,
            1 => Self::Port,
            2 => Self::ServiceName,
            3 => Self::FunctionName,
            4 => Self::Body,
            i => Self::NotSelected
        }
    }
}

#[derive(Debug)]
pub struct ServiceRequestState {
    pub service_request: ServiceRequest,
    pub request_fields: StatefulList<String>,
    pub selected_field: SelectedField,
}

impl ServiceRequestState {
    fn mk_request_fields(service_request: &ServiceRequest) -> StatefulList<String> {
        let fields = service_request.pretty_string().split("\n").into_iter().map(|s|String::from(s)).collect::<Vec<String>>();
        StatefulList::with_items(fields)
    }

    pub fn new(service_request: ServiceRequest) -> ServiceRequestState {
        let fields = Self::mk_request_fields(&service_request);
        ServiceRequestState {
            service_request,
            request_fields: fields,
            selected_field: SelectedField::NotSelected,
        }
    }

    fn update_fields(&mut self) {
        self.request_fields = Self::mk_request_fields(&self.service_request);
    }

    pub fn unselect(&mut self) {
        self.request_fields.unselect();
        self.selected_field = SelectedField::NotSelected
    }

    // pub fn selected(&self) -> Option<usize> { self.selected_field }

    pub fn prev(&mut self) {
        self.request_fields.previous();
        let field = match self.request_fields.state.selected() {
            Some(v) => SelectedField::select_by_index(v),
            None => SelectedField::NotSelected
        };
        self.selected_field = field
    }

    pub fn select(&mut self, n: usize) {
        self.request_fields.select(n);
        self.selected_field = SelectedField::select_by_index(n);

    }

    pub fn next(&mut self) {
        self.request_fields.next();
        let field = match self.request_fields.state.selected() {
            Some(v) => SelectedField::select_by_index(v),
            None => SelectedField::NotSelected
        };
        self.selected_field = field
    }
}
#[derive(Debug, PartialEq)]
pub enum StateCommands {
    SetPort,
    SendServiceListRequest,
    SelectService,
    SendFunctionListRequest,
    SelectFunction,
    SetBody,
    Send,
    Sent
}


#[derive(Debug, PartialEq)]
pub enum InputMode {
    Normal,
    Editing,
    MultilineEditing
}
#[derive(Debug)]
pub struct AppState {
    pub request_state: ServiceRequestState,
    pub instruction_messages: String,
    pub input: String,
    pub input_mode: InputMode,
    pub input_messages: Vec<String>,
    pub result: Option<Vec<String>>,
    pub command: StateCommands,
    pub debugging: String
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            request_state: ServiceRequestState::new(ServiceRequest::default()),
            instruction_messages: String::new(),
            input: String::new(),
            input_mode: InputMode::Normal,
            input_messages: Vec::new(),
            result: None,
            command: StateCommands::SetPort,
            debugging: String::new()
        }
    }

    pub fn set_result(&mut self, m: Vec<String>) {
        self.result = Some(m)
    }

    pub fn next_command(&mut self) {
        self.command = match self.command {
            StateCommands::SetPort => StateCommands::SendServiceListRequest,
            StateCommands::SendServiceListRequest => StateCommands::SelectService,
            StateCommands::SelectService => StateCommands::SendFunctionListRequest,
            StateCommands::SendFunctionListRequest => StateCommands::SelectFunction,
            StateCommands::SelectFunction => StateCommands::SetBody,
            StateCommands::SetBody => StateCommands::Send,
            StateCommands::Send => StateCommands::Sent,
            _ => StateCommands::Sent
        }
    }

    pub fn is_service_names_requestable(&self) -> bool {
        let r = &self.request_state.service_request;
        &r.service_name.0.is_empty() & !(&r.host.0.is_empty())
    }


    pub fn is_service_methods_requestable(&self) -> bool {
        let r = &self.request_state.service_request;
        !(&r.host.0.is_empty()) & !(&r.service_name.0.is_empty())
    }

    pub fn debug(&mut self, s: &str) {
        self.debugging = String::from(s)
    }

    pub fn update_instruction_messages(&mut self, messages: &str) {
        self.instruction_messages = String::from(messages)
    }

    pub fn update_input_mode(&mut self, mode: InputMode) {
        self.input_mode = mode
    }

    pub fn update_field(&mut self, user_input: String) {
        match self.request_state.selected_field {
            SelectedField::Host => self.request_state.service_request.update_host(Host::from(user_input).unwrap()), // Validation check needed
            SelectedField::Port => self.request_state.service_request.update_port(Port::from(user_input).unwrap()), // Validation check needed
            SelectedField::ServiceName => {
                if let Some(v) = &self.result {
                    let index = util::parse_usize(user_input, &5).unwrap(); // change max bound
                    let new_value = v.index(index).clone();
                    self.request_state.service_request.update_service(ServiceName::from(new_value.as_str()))
                }
            }
            SelectedField::FunctionName => {
                if let Some(v) = &self.result {
                    let index = util::parse_usize(user_input, &5).unwrap(); // change max bound
                    let new_value = v.index(index).clone();
                    self.request_state.service_request.update_function(ServiceFunction::from(new_value.as_str()))
                }
            }
            SelectedField::Body => {
                self.request_state.service_request.body = RequestBody::from(user_input.as_str())
            }
            _ => ()
        };
        self.request_state.update_fields()
    }

    pub fn apply_input(&mut self) {
        let line: String = self.input.drain(..).collect();
        self.input_messages.push(line.clone());
        // match self.request_state.selected_field {
        //     SelectedField::NotSelected => unreachable!(),
        //     SelectedField::Host => self.request_state.request_fields,
        //     SelectedField::Port => ,
        //     SelectedField::ServiceName => ,
        //     SelectedField::FunctionName => ,
        //     SelectedField::Body => ,
        // }
    }
}
