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
