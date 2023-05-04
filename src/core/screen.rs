// abstract representation of what the user sees

pub const SCREEN_HEIGHT: usize = 32; // MAXLINES
pub const SCREEN_WIDTH: usize = 80; // MAXCOLS
pub const SIZE_OF_MORE: usize = 8; // "--More--"
pub const MAX_MESSAGE_LENGTH: usize = SCREEN_WIDTH - SIZE_OF_MORE; // MAXMSG

pub trait Screen {
    fn clear(&mut self);
    fn clear_message(&mut self);
    fn show_message(&mut self, msg: &str);
    fn show_status(&mut self, msg: &str);
    fn move_cursor(&mut self, y: usize, x: usize);
    fn write_char(&mut self, value: char);
    fn write(&mut self, value: &str);
    fn draw(&self);
}
