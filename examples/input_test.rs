use std::io;

use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::event::{Event, KeyCode, KeyEvent, read};
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    enable_raw_mode()?;
    crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    loop {
        // Blocks until an `Event` is available
        let inp = read()?;
        println!("{:?}\r", inp);
        match inp {
            Event::Key(KeyEvent {
                code: KeyCode::Esc, ..
            }) => {
                break;
            }
            _ => {}
        }
    }
    crossterm::execute!(stdout, DisableMouseCapture, LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}
