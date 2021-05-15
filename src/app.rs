use std::error::Error;
use std::io::Stdout;
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    symbols,
    text::{Span, Spans},
    widgets::{
        Block, BorderType, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table, Tabs,
    },
    Terminal,
};

pub struct App {
    tabs: Vec<String>,
    current_tab: usize,
}

impl App {
    pub fn new() -> App {
        App {
            tabs: vec!["1".to_string(), "2".to_string()],
            current_tab: 0,
        }
    }

    pub fn draw(
        &self,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    ) -> Result<(), Box<dyn Error>> {
        terminal.draw(|frame| {
            let size = frame.size();

            let tab_names = self
                .tabs
                .iter()
                .map(|f| {
                    let value = f.clone();
                    Spans::from(value)
                })
                .collect();
            let tabs = Tabs::new(tab_names)
                .block(Block::default().title("Tabs").borders(Borders::ALL))
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().fg(Color::Yellow))
                .divider(symbols::line::VERTICAL);

            frame.render_widget(tabs, size);
        })?;
        Ok(())
    }

    pub fn enter(&mut self) {}

    pub fn left(&mut self) {}

    pub fn down(&mut self) {}

    pub fn up(&mut self) {}

    pub fn right(&mut self) {}

    pub fn exit(&mut self) {}
}
