use std::{error::Error, io, sync::mpsc::channel, thread};
use termion::{
    event::{Event, Key},
    input::{MouseTerminal, TermRead},
    raw::IntoRawMode,
    screen::AlternateScreen,
};

use tui::{
    backend::TermionBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Wrap},
    Terminal,
};

pub fn run() -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut frame = 0;
    let (tx, rx) = channel();

    thread::spawn(move || {
        let stdin = io::stdin();
        for evt in stdin.events() {
            tx.send(evt.unwrap()).unwrap();
        }
    });

    loop {
        frame += 1;
        terminal.draw(|f| {
            let size = f.size();
            let txt = format!("hi there. Frame: {}", frame);
            let para = Paragraph::new(txt.as_str());
            f.render_widget(para, size);
        })?;

        if let Ok(evt) = rx.try_recv() {
            match evt {
                Event::Key(Key::Char('q') | Key::Ctrl('c') | Key::Ctrl('d')) => break,
                _ => {}
            }
        }
    }

    Ok(())
}
