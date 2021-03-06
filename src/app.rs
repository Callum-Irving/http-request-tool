mod tab_select;
mod text_entry;

use crate::{ui_graph, ui_graph::pane_identifiers::*};
use petgraph::{graph::Graph, visit::EdgeRef};
use serde_json::Value;
use std::error::Error;
use std::io::Stdout;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    symbols,
    text::Spans,
    widgets::{Block, Borders, Paragraph, Tabs},
    Terminal,
};

use self::{tab_select::TabSelect, text_entry::TextEntry};

#[derive(PartialEq)]
pub enum InputMode {
    Navigation,
    TabSelect,
    Entry,
    EndpointEntry,
    BodyHeaderSelect,
    MethodSelect,
    ResponseSelect,
}

pub struct App {
    tabs: Vec<usize>, // This vector holds IDs, not names
    current_tab: usize,
    pub input_mode: InputMode,
    response_body: String,
    current_pane: petgraph::graph::NodeIndex,
    ui: Graph<usize, usize>,
    endpoint_widget: TextEntry,
    request_widget: TextEntry,
    method_select_widget: TabSelect,
    widget_styles: [Color; 8],
}

impl App {
    pub fn new() -> App {
        let graph = ui_graph::init_ui_graph();
        let tabs_pane = graph.node_indices().find(|node| graph[*node] == 0).unwrap();
        let mut widget_styles: [Color; 8] = [Color::Rgb(255, 255, 255); 8];
        widget_styles[0] = Color::Yellow;
        App {
            tabs: vec![0],
            current_tab: 0,
            input_mode: InputMode::Navigation,
            response_body: String::new(),
            current_pane: tabs_pane,
            ui: graph,
            endpoint_widget: TextEntry::new("http://".to_string(), false),
            request_widget: TextEntry::new("".to_string(), true),
            method_select_widget: TabSelect::new(
                vec![
                    "GET".to_string(),
                    "POST".to_string(),
                    "PUT".to_string(),
                    "DELETE".to_string(),
                ],
                "".to_string(),
                Color::Magenta,
            ),
            widget_styles,
        }
    }

    pub fn draw(
        &self,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    ) -> Result<(), Box<dyn Error>> {
        terminal.draw(|frame| {
            let size = frame.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Length(3), Constraint::Min(2)].as_ref())
                .split(size);

            // TODO: Implement tab selection
            let tab_names = (1..self.tabs.len() + 1)
                .map(|num| Spans::from(num.to_string()))
                .collect();
            let tabs = Tabs::new(tab_names)
                .block(
                    Block::default()
                        .title("Tabs")
                        .style(Style::default().fg(self.widget_styles[PANE_TABS]))
                        .borders(Borders::ALL),
                )
                .select(self.current_tab)
                .highlight_style(Style::default().fg(Color::Magenta))
                .divider(symbols::line::VERTICAL);

            // Body layout
            let body_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
                .split(chunks[1]);

            // ===== REQUEST BLOCK LAYOUT =====

            let request_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Min(2),
                    Constraint::Length(3),
                ])
                .split(body_layout[0]);

            let endpoint_entry = self
                .endpoint_widget
                .get_widget(self.widget_styles[PANE_ENDPOINT]);

            let body_header_options = vec![
                Spans::from("BODY"),
                Spans::from("HEADER"),
                Spans::from("QUERY"),
            ];
            let body_header_select = Tabs::new(body_header_options)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().fg(self.widget_styles[PANE_BODY_HEADER_SELECT])),
                )
                .divider(symbols::line::VERTICAL);

            // TODO: Separate widget for body/header/query
            let request_entry = self
                .request_widget
                .get_widget(self.widget_styles[PANE_REQUEST_ENTRY]);

            let request_bottom_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Min(1), Constraint::Length(6)])
                .split(request_layout[3]);

            let method_select = self
                .method_select_widget
                .get_widget(self.widget_styles[PANE_METHOD_SELECT]);

            let send_button = Paragraph::new(Spans::from("SEND")).block(
                Block::default()
                    .style(Style::default().fg(self.widget_styles[PANE_SEND_BUTTON]))
                    .borders(Borders::ALL),
            );

            // ===== RESPONSE BLOCK LAYOUT =====

            let response_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(1)])
                .split(body_layout[1]);

            let response_tabs_names = vec![Spans::from("BODY"), Spans::from("HEADER")];
            let response_tabs = Tabs::new(response_tabs_names)
                .block(Block::default().borders(Borders::ALL))
                .style(Style::default().fg(self.widget_styles[PANE_RESPONSE_TABS]))
                .divider(symbols::line::VERTICAL);

            let response_lines: Vec<Spans> = self
                .response_body
                .split('\n')
                .collect::<Vec<&str>>()
                .iter()
                .map(|s| Spans::from(s.clone()))
                .collect();
            let response_paragraph = Paragraph::new(response_lines).block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(self.widget_styles[PANE_RESPONSE_TEXT])),
            );

            // ===== CURSOR DRAWING =====

            match self.input_mode {
                InputMode::Entry => {
                    let (x, y) = self.request_widget.get_cursor_xy();
                    frame.set_cursor(request_layout[2].x + x + 1, request_layout[2].y + y + 1)
                }
                InputMode::EndpointEntry => {
                    let (x, _) = self.endpoint_widget.get_cursor_xy();
                    frame.set_cursor(request_layout[0].x + x + 1, request_layout[0].y + 1)
                }
                _ => {}
            }

            frame.render_widget(tabs, chunks[0]);
            frame.render_widget(response_tabs, response_layout[0]);
            frame.render_widget(response_paragraph, response_layout[1]);
            frame.render_widget(endpoint_entry, request_layout[0]);
            frame.render_widget(body_header_select, request_layout[1]);
            frame.render_widget(method_select, request_bottom_layout[0]);
            frame.render_widget(send_button, request_bottom_layout[1]);
            frame.render_widget(request_entry, request_layout[2]);
        })?;
        Ok(())
    }

    // Tab navigation
    pub fn tab_left(&mut self) {
        self.current_tab -= (self.current_tab > 0) as usize;
    }
    pub fn tab_right(&mut self) {
        self.current_tab += 1;
        if self.current_tab >= self.tabs.len() {
            let mut i: usize = 0;
            loop {
                if !self.tabs.contains(&i) {
                    self.tabs.push(i);
                    break;
                }
                i += 1;
            }
        }
    }
    pub fn tab_delete(&mut self) {
        self.tabs.remove(self.current_tab);
        if self.tabs.is_empty() {
            self.tabs.push(0);
        }
        if self.current_tab >= self.tabs.len() {
            self.current_tab -= 1;
        }
    }

    // Cleanly exit
    pub fn exit(&mut self) {}
}

// Navigation functions
impl App {
    pub fn escape(&mut self) {
        let edges = self.ui.edges(self.current_pane);
        for edge in edges {
            if *edge.weight() == 6 {
                self.widget_styles[self.ui[self.current_pane]] = Color::Rgb(255, 255, 255);
                self.current_pane = edge.target();
                self.widget_styles[self.ui[self.current_pane]] = Color::Yellow;
            }
        }
    }

    pub fn left(&mut self) {
        let edges = self.ui.edges(self.current_pane);
        for edge in edges {
            if *edge.weight() == 1 {
                self.widget_styles[self.ui[self.current_pane]] = Color::Rgb(255, 255, 255);
                self.current_pane = edge.target();
                self.widget_styles[self.ui[self.current_pane]] = Color::Yellow;
            }
        }
    }

    pub fn right(&mut self) {
        let edges = self.ui.edges(self.current_pane);
        for edge in edges {
            if *edge.weight() == 2 {
                self.widget_styles[self.ui[self.current_pane]] = Color::Rgb(255, 255, 255);
                self.current_pane = edge.target();
                self.widget_styles[self.ui[self.current_pane]] = Color::Yellow;
            }
        }
    }

    pub fn up(&mut self) {
        let edges = self.ui.edges(self.current_pane);
        for edge in edges {
            if *edge.weight() == 3 {
                self.widget_styles[self.ui[self.current_pane]] = Color::Rgb(255, 255, 255);
                self.current_pane = edge.target();
                self.widget_styles[self.ui[self.current_pane]] = Color::Yellow;
            }
        }
    }

    pub fn down(&mut self) {
        let edges = self.ui.edges(self.current_pane);
        for edge in edges {
            if *edge.weight() == 4 {
                self.widget_styles[self.ui[self.current_pane]] = Color::Rgb(255, 255, 255);
                self.current_pane = edge.target();
                self.widget_styles[self.ui[self.current_pane]] = Color::Yellow;
            }
        }
    }

    pub fn enter(&mut self) {
        let edges = self.ui.edges(self.current_pane);
        for edge in edges {
            if *edge.weight() == 5 {
                self.widget_styles[self.ui[self.current_pane]] = Color::Rgb(255, 255, 255);
                self.current_pane = edge.target();
                self.widget_styles[self.ui[self.current_pane]] = Color::Yellow;
                return;
            }
        }

        // Special actions for certain widgets
        match self.ui[self.current_pane] {
            PANE_TABS => {
                self.input_mode = InputMode::TabSelect;
                self.widget_styles[self.ui[self.current_pane]] = Color::Red;
            }
            PANE_ENDPOINT => {
                self.input_mode = InputMode::EndpointEntry;
                self.widget_styles[self.ui[self.current_pane]] = Color::Red;
            }
            PANE_BODY_HEADER_SELECT => {
                self.input_mode = InputMode::BodyHeaderSelect;
                self.widget_styles[self.ui[self.current_pane]] = Color::Red;
            }
            PANE_REQUEST_ENTRY => {
                self.input_mode = InputMode::Entry;
                self.widget_styles[self.ui[self.current_pane]] = Color::Red;
            }
            PANE_SEND_BUTTON => {
                self.send_request().unwrap();
            }
            PANE_METHOD_SELECT => {
                self.input_mode = InputMode::MethodSelect;
                self.widget_styles[self.ui[self.current_pane]] = Color::Red;
            }
            PANE_RESPONSE_TABS => {
                self.input_mode = InputMode::ResponseSelect;
                self.widget_styles[self.ui[self.current_pane]] = Color::Red;
            }
            _ => {}
        }
    }

    fn send_request(&mut self) -> Result<(), Box<dyn Error>> {
        let client = reqwest::blocking::Client::new();

        let body_string = self.request_widget.get_text();
        let body_json: Value = serde_json::from_str(&body_string)?;

        let method = self.method_select_widget.get_current_tab();
        let res = match method.as_str() {
            "GET" => client
                .get(self.endpoint_widget.get_text())
                .json(&body_json)
                .send()?
                .text()?,
            "POST" => client
                .post(self.endpoint_widget.get_text())
                .json(&body_json)
                .send()?
                .text()?,
            "PUT" => client
                .put(self.endpoint_widget.get_text())
                .json(&body_json)
                .send()?
                .text()?,
            "DELETE" => client
                .delete(self.endpoint_widget.get_text())
                .json(&body_json)
                .send()?
                .text()?,
            _ => "".to_string(),
        };

        self.response_body = res;
        Ok(())
    }
}

// Endpoint entry
impl App {
    pub fn endpoint_input_char(&mut self, c: char) {
        self.endpoint_widget.input_char(c);
    }

    pub fn endpoint_backspace(&mut self) {
        self.endpoint_widget.backspace();
    }

    // TODO: Implement left/right cursor movement
}

// Entry mode
impl App {
    pub fn exit_input(&mut self) {
        self.input_mode = InputMode::Navigation;
        self.widget_styles[self.ui[self.current_pane]] = Color::Yellow;
    }

    pub fn backspace(&mut self) {
        self.request_widget.backspace();
    }

    pub fn input_tab(&mut self) {
        self.request_widget.input_tab();
    }

    pub fn entry_left(&mut self) {
        self.request_widget.cursor_left();
    }

    pub fn entry_right(&mut self) {
        self.request_widget.cursor_right();
    }

    pub fn entry_up(&mut self) {
        self.request_widget.cursor_up();
    }

    pub fn entry_down(&mut self) {
        self.request_widget.cursor_down();
    }

    pub fn input_char(&mut self, c: char) {
        self.request_widget.input_char(c);
    }
}

// Method select
impl App {
    pub fn method_select_left(&mut self) {
        self.method_select_widget.move_left();
    }

    pub fn method_select_right(&mut self) {
        self.method_select_widget.move_right();
    }
}
