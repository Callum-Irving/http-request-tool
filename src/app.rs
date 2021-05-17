use crate::ui_graph;
use petgraph::{graph::Graph, visit::EdgeRef};
use reqwest;
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

#[derive(PartialEq)]
pub enum InputMode {
    Navigation,
    TabSelect,
    Entry,
}

pub struct App {
    tabs: Vec<usize>, // This vector holds IDs, not names
    current_tab: usize,
    pub input_mode: InputMode,
    request_body: String,
    response_body: String,
    current_pane: petgraph::graph::NodeIndex,
    ui: Graph<usize, usize>,
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
            request_body: String::new(),
            response_body: String::new(),
            current_pane: tabs_pane,
            ui: graph,
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
                        .style(Style::default().fg(self.widget_styles[0]))
                        .borders(Borders::ALL),
                )
                .select(self.current_tab)
                .highlight_style(Style::default().fg(Color::Magenta))
                .divider(symbols::line::VERTICAL);

            // Body layout
            let body = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
                .split(chunks[1]);
            let request_block = Block::default()
                .title("Request")
                .style(Style::default().fg(self.widget_styles[1]))
                .borders(Borders::ALL);
            let response_block = Block::default()
                .title("Response")
                .style(Style::default().fg(self.widget_styles[2]))
                .borders(Borders::ALL);

            frame.render_widget(tabs, chunks[0]);
            frame.render_widget(response_block, body[1]);
            frame.render_widget(request_block, body[0]);
        })?;
        Ok(())
    }

    // Navigation keys
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

        // TODO: Trigger special action
        // Actions:
        // Tabs
        // Endpoint entry
        // JSON entry
        // Body/header/query tabs
        // Send button
        // Method select
        match self.ui[self.current_pane] {
            0 => {
                self.input_mode = InputMode::TabSelect;
                self.widget_styles[self.ui[self.current_pane]] = Color::Red;
            }
            3 => {}
            4 => {}
            5 => {}
            6 => {}
            7 => {}
            _ => {}
        }
    }

    // Navigation functions
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

    // Text entry
    pub fn input_char(&mut self, c: char) {}
    pub fn newline(&mut self) {}
    pub fn backspace(&mut self) {}
    pub fn exit_input(&mut self) {
        self.input_mode = InputMode::Navigation;
        self.widget_styles[self.ui[self.current_pane]] = Color::Yellow;
    }

    // Tab navigation
    pub fn tab_left(&mut self) {
        if self.current_tab > 0 {
            self.current_tab -= 1;
        }
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
        if self.tabs.len() <= 0 {
            self.tabs.push(0);
        }
        if self.current_tab >= self.tabs.len() {
            self.current_tab -= 1;
        }
    }

    // Cleanly exit
    pub fn exit(&mut self) {}

    // Open help menu
    pub fn help(&self) {}

    // Async functions to send http requests
    async fn http_get(&mut self) {
        // Get data from text boxes
        let endpoint = "http://google.com/";
        let body = "{ json: yes }";

        // Send request
        let client = reqwest::Client::new();
        let response = client.post(endpoint).body(body).send().await;

        // Change response field
        match response {
            Ok(response) => {
                // Everything worked
            }
            Err(_) => {
                // Display error message
            }
        }
    }
    async fn http_post(&self) {}
    async fn http_put(&self) {}
    async fn http_delete(&self) {}
}
