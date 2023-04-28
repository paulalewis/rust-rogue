// abstract representation of what the user sees

pub const SCREEN_HEIGHT: usize = 32; // MAXLINES
pub const SCREEN_WIDTH: usize = 80; // MAXCOLS
pub const SIZE_OF_MORE: usize = 8; // "--More--"
pub const MAX_MESSAGE_LENGTH: usize = SCREEN_WIDTH - SIZE_OF_MORE; // MAXMSG

pub trait Screen {
    fn clear(&mut self);
    fn clear_msg(&mut self);
    fn msg(&mut self, msg: &str);
    fn status(&mut self, msg: &str);
    fn move_cursor(&mut self, y: usize, x: usize);
    fn writ_char(&mut self, c: char);
    fn write(&mut self, msg: &str);
    fn draw(&self);
}

#[derive(Copy, Clone, Debug)]
pub struct ConsoleScreen {
    screen: [[char; SCREEN_WIDTH]; SCREEN_HEIGHT],
    cursor: (usize, usize),
}

impl ConsoleScreen {
    pub const fn new() -> ConsoleScreen {
        ConsoleScreen {
            screen: [[' '; SCREEN_WIDTH]; SCREEN_HEIGHT],
            cursor: (0, 0),
        }
    }
}

impl Screen for ConsoleScreen {
    fn clear(&mut self) {
        for i in 0..SCREEN_HEIGHT {
            for j in 0..SCREEN_WIDTH {
                self.screen[i][j] = ' ';
            }
        }
    }

    fn clear_msg(&mut self) {
        for i in 0..SCREEN_WIDTH {
            self.screen[0][i] = ' ';
        }
    }

    // Display a message at the top of the screen.
    fn msg(&mut self, msg: &str) {
        for (i, c) in msg.chars().enumerate() {
            self.screen[0][i] = c;
        }
    }

    fn status(&mut self, msg: &str) {
        for (i, c) in msg.chars().enumerate() {
            self.screen[SCREEN_HEIGHT - 1][i] = c;
        }
    }

    fn move_cursor(&mut self, y: usize, x: usize) {
        self.cursor = (y, x);
    }

    fn writ_char(&mut self, c: char) {
        self.screen[self.cursor.0][self.cursor.1] = c;
    }

    fn write(&mut self, msg: &str) {
        for (_, c) in msg.chars().enumerate() {
            if c == '\n' {
                self.move_cursor(self.cursor.0 + 1, 0);
            } else if self.cursor.1 > SCREEN_WIDTH - 1 {
                self.move_cursor(self.cursor.0 + 1, 0);
                self.writ_char(c);
            } else {
                self.move_cursor(self.cursor.0, self.cursor.1 + 1);
                self.writ_char(c);
            }
        }
    }

    fn draw(&self) {
        for i in 0..SCREEN_HEIGHT {
            for j in 0..SCREEN_WIDTH {
                print!("{}", self.screen[i][j]);
            }
            println!();
        }
    }
}