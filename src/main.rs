mod app;
use crate::app::App;
mod ui_graph;

use crossterm::{
    event,
    event::KeyCode,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::error::Error;
use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use tui::{backend::CrosstermBackend, Terminal};

enum EventType<I> {
    Input(I),
    Tick,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;

    // Input handling thread
    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).unwrap() {
                if let event::Event::Key(key) = event::read().unwrap() {
                    tx.send(EventType::Input(key)).unwrap();
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(EventType::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });

    // Initialize application
    let mut app = App::new();

    // Main drawing loop
    loop {
        // Draw UI
        app.draw(&mut terminal)?;

        // Input handling
        match rx.recv()? {
            EventType::Input(key) => match app.input_mode {
                app::InputMode::Navigation => match key.code {
                    KeyCode::Esc => app.escape(),
                    KeyCode::Char('q') => break,
                    KeyCode::Enter => app.enter(),
                    KeyCode::Char('h') | KeyCode::Left => app.left(),
                    KeyCode::Char('l') | KeyCode::Right => app.right(),
                    KeyCode::Char('k') | KeyCode::Up => app.up(),
                    KeyCode::Char('j') | KeyCode::Down => app.down(),
                    _ => {}
                },
                app::InputMode::Entry => match key.code {
                    KeyCode::Esc => app.exit_input(),
                    KeyCode::Enter => app.input_char('\n'), // TODO: try removing this line
                    KeyCode::Backspace => app.backspace(),
                    KeyCode::Tab => app.input_tab(),
                    KeyCode::Left => app.entry_left(),
                    KeyCode::Right => app.entry_right(),
                    KeyCode::Up => app.entry_up(),
                    KeyCode::Down => app.entry_down(),
                    KeyCode::Char(c) => app.input_char(c),
                    _ => {}
                },
                app::InputMode::TabSelect => match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Esc => app.exit_input(),
                    KeyCode::Char('h') | KeyCode::Left => app.tab_left(),
                    KeyCode::Char('l') | KeyCode::Right => app.tab_right(),
                    KeyCode::Backspace | KeyCode::Delete => app.tab_delete(),
                    _ => {}
                },
                app::InputMode::EndpointEntry => match key.code {
                    KeyCode::Esc => app.exit_input(),
                    KeyCode::Enter => app.exit_input(),
                    KeyCode::Backspace => app.endpoint_backspace(),
                    KeyCode::Char(c) => app.endpoint_input_char(c),
                    _ => {}
                },
                app::InputMode::BodyHeaderSelect => match key.code {
                    KeyCode::Esc => app.exit_input(),
                    _ => {}
                },
                app::InputMode::MethodSelect => match key.code {
                    KeyCode::Esc => app.exit_input(),
                    _ => {}
                },
                app::InputMode::ResponseSelect => match key.code {
                    KeyCode::Esc => app.exit_input(),
                    _ => {}
                },
            },
            EventType::Tick => {}
        }
    }

    // Cleanup and exit
    app.exit();
    disable_raw_mode()?;
    terminal.show_cursor()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
