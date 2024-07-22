use std::io::{stdout, Stdout, Write};

use termion::raw::{IntoRawMode, RawTerminal};

pub const SCREEN_HEIGHT: usize = 32; // MAXLINES
pub const SCREEN_WIDTH: usize = 80; // MAXCOLS
// pub const SIZE_OF_MORE: usize = 8; // "--More--", more no longer used
// pub const MAX_MESSAGE_LENGTH: usize = SCREEN_WIDTH - SIZE_OF_MORE; // MAXMSG, no max message length, will wrap instead

// Wrapper for terminal screen
pub struct GameScreen {
    // stdout: AlternateScreen<RawTerminal<Stdout>>,
    pub stdout: RawTerminal<Stdout>,
}

impl GameScreen {
    pub fn new() -> Self {
        //let mut stdout = stdout().into_raw_mode().unwrap().into_alternate_screen().unwrap();
        let mut stdout = stdout().into_raw_mode().unwrap();
        write!(stdout, "{}", termion::cursor::Hide).unwrap();
        stdout.flush().unwrap();
        GameScreen { stdout }
    }
}

impl Drop for GameScreen {
    fn drop(&mut self) {
        write!(self.stdout, "{}", termion::cursor::Show).unwrap();
        // write!(self.stdout, "{}", ToMainScreen).unwrap();
        self.stdout.flush().unwrap();
    }
}