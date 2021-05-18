use tui::{
    style::{Color, Style},
    symbols,
    text::Spans,
    widgets::{Block, Borders, Tabs},
};

pub struct TabSelect {
    tabs: Vec<String>,
    current_tab: usize,
    title: String,
    highlighted_color: Color,
}

impl TabSelect {
    pub fn new(tabs: Vec<String>, title: String, highlighted_color: Color) -> TabSelect {
        TabSelect {
            tabs,
            current_tab: 0,
            title,
            highlighted_color,
        }
    }

    //pub fn append_tab(&mut self, title: String) {
    //self.tabs.push(title);
    //}

    //pub fn remove_tab(&mut self) {
    //if self.tabs.len() > 1 {
    //self.tabs.remove(self.current_tab);
    //}
    //}

    pub fn move_left(&mut self) {
        self.current_tab -= (self.current_tab > 0) as usize;
    }

    pub fn move_right(&mut self) {
        self.current_tab += (self.current_tab < self.tabs.len() - 1) as usize;
    }

    pub fn get_widget(&self, color: Color) -> Tabs {
        let titles = self.tabs.iter().map(|s| Spans::from(s.clone())).collect();
        Tabs::new(titles)
            .block(
                Block::default()
                    .title(self.title.clone())
                    .style(Style::default().fg(color))
                    .borders(Borders::ALL),
            )
            .select(self.current_tab)
            .highlight_style(Style::default().fg(self.highlighted_color))
            .divider(symbols::line::VERTICAL)
    }

    pub fn get_current_tab(&self) -> String {
        self.tabs[self.current_tab].clone()
    }
}
