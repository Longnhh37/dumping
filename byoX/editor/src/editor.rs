use std::io::Error;

use crossterm::event::{
    Event::{self, Key},
    KeyCode::Char,
    KeyEvent, KeyModifiers, read,
};

mod terminal;
use terminal::{Position, Terminal};

const PRG_NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub const fn default() -> Self {
        Self { should_quit: false }
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();

        let result = self.repl();

        Terminal::termintate().unwrap();

        result.unwrap();
    }

    fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;

            if self.should_quit {
                break;
            }

            let event = read()?;

            self.evaluate_event(&event);
        }
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => self.should_quit = true,
                _ => (),
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_cursor()?;

        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye.\r\n")?;
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor_to(Position { x: 0, y: 0 })?;
        }

        Terminal::show_cursor()?;
        Terminal::execute()?;

        Ok(())
    }

    fn draw_rows() -> Result<(), Error> {
        let height = Terminal::size()?.height;
        for current_row in 0..height {
            Terminal::clear_line()?;

            if current_row == height / 3 {
                Self::draw_welcome_message()?;
            } else {
                Self::draw_empty_row()?;
            }

            if current_row + 1 < height {
                Terminal::print("\r\n")?;
            }
        }

        Ok(())
    }

    fn draw_welcome_message() -> Result<(), Error> {
        let mut welcome_msg = format!("{PRG_NAME} editor -- version {VERSION}");
        let width = Terminal::size()?.width;
        let len = welcome_msg.len();
        let padding = (width.saturating_sub(len)) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));

        welcome_msg = format!("{spaces}{welcome_msg}");
        welcome_msg.truncate(width);

        Terminal::print(&welcome_msg)?;
        Ok(())
    }

    fn draw_empty_row() -> Result<(), Error> {
        Terminal::print("~")?;
        Ok(())
    }
}
