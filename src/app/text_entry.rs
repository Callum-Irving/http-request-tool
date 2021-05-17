use tui::{
    style::{Color, Style},
    text::Spans,
    widgets::{Block, Borders, Paragraph},
};

pub struct TextEntry {
    cursor_pos: usize,
    text: String,
    newlines_allowed: bool,
}

// Request text entry
impl TextEntry {
    pub fn new(text: String, newlines_allowed: bool) -> TextEntry {
        TextEntry {
            cursor_pos: text.len(),
            text,
            newlines_allowed,
        }
    }

    pub fn get_widget(&self, colour: Color) -> Paragraph {
        let lines: Vec<Spans> = self
            .text
            .split('\n')
            .collect::<Vec<&str>>()
            .iter()
            .map(|s| Spans::from(s.clone()))
            .collect();
        Paragraph::new(lines).block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(colour)),
        )
    }

    // Text entry
    pub fn input_char(&mut self, c: char) {
        // Insert the character at cursor pos
        if c == '\n' && !self.newlines_allowed {
            return;
        }

        if self.cursor_pos >= self.text.len() {
            self.text.push(c);
        } else {
            self.text.insert(self.cursor_pos, c);
        }
        self.cursor_pos += 1;
    }

    pub fn input_tab(&mut self) {
        self.text.insert_str(self.cursor_pos, "  ");
        self.cursor_pos += 2;
    }

    pub fn backspace(&mut self) {
        // Delete character before cursor
        if self.cursor_pos < 1 {
            return;
        }
        self.text.remove(self.cursor_pos - 1);
        self.cursor_pos -= 1;
    }

    pub fn cursor_left(&mut self) {
        self.cursor_pos -= (self.cursor_pos > 0) as usize;
    }

    pub fn cursor_right(&mut self) {
        self.cursor_pos += (self.cursor_pos < self.text.len()) as usize;
    }

    pub fn cursor_up(&mut self) {
        // Go back 2 \n's
        // Go forward the amount of chars you were from the fist \n
    }

    pub fn cursor_down(&mut self) {}

    pub fn get_cursor_xy(&self) -> (u16, u16) {
        let chars = self.text.chars().take(self.cursor_pos);

        let mut x: u16 = 0;
        chars.clone().for_each(|c| {
            if c == '\n' {
                x = 0;
            } else {
                x += 1;
            }
        });

        let y = chars.filter(|c| c == &'\n').count() as u16;
        // x = number of chars after last \n
        (x, y)
    }
}
