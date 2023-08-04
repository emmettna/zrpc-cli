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
use tonic::codegen::Service;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Corner, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame, Terminal,
};
use crate::grpc_request_dsl::ServiceRequest;
use unicode_width::UnicodeWidthStr;

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

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}

pub struct ServiceRequestState {
    pub request_fields: StatefulList<String>,
    selected_field: Option<usize>, // total 5 fields -> index from 0 to 40
}

impl ServiceRequestState {
    pub fn new(service_request: ServiceRequest) -> ServiceRequestState {
        let fields = service_request.pretty_string().split("\n").into_iter().map(|s|String::from(s)).collect::<Vec<String>>();
        ServiceRequestState {
            request_fields: StatefulList::with_items(fields),
            selected_field: None,
        }
    }

    pub fn selected(&self) -> Option<usize> { self.selected_field }

    pub fn select_prev(&mut self) {
        if let Some(n) = self.selected_field {
            if n <= 0 {
                self.selected_field = None;
            } else {
                self.selected_field = Some(n - 1);
            }
        } else {
            self.selected_field = Some(4)
        }
    }

    pub fn select_next(&mut self) {
        if let Some(n) = self.selected_field {
            if n >= 4 {
                self.selected_field = None;
            } else {
                self.selected_field = Some(n + 1);
            }
        } else { self.selected_field = Some(0) }
    }
}

pub enum SelectedField {
    Host,
    Port,
    ServiceName,
    FunctionName,
    Body
}

pub enum InputMode {
    Normal,
    Editing,
}

pub struct AppState {
    pub request_state: ServiceRequestState,
    pub input: String,
    pub input_mode: InputMode,
    pub messages: Vec<String>,
    selected_field: Option<SelectedField>
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            request_state: ServiceRequestState::new(ServiceRequest::default()),
            input: String::new(),
            input_mode: InputMode::Normal,
            messages: Vec::new(),
            selected_field: None
        }
    }
}
