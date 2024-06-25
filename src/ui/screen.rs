pub const SCREEN_HEIGHT: usize = 32; // MAXLINES
pub const SCREEN_WIDTH: usize = 80; // MAXCOLS
// pub const SIZE_OF_MORE: usize = 8; // "--More--"
// pub const MAX_MESSAGE_LENGTH: usize = SCREEN_WIDTH - SIZE_OF_MORE; // MAXMSG

/// abstract representation of 2d screen
pub trait Screen {
    fn move_cursor(&mut self, y: usize, x: usize);
    
    fn clear(&mut self);

    fn clear_at(&mut self, y: usize, x: usize) {
        self.move_cursor(y, x);
        self.clear();
    }

    fn clear_all(&mut self) {
        for i in 0..SCREEN_WIDTH {
            for j in 0..SCREEN_HEIGHT {
                self.clear_at(j, i);
            }
        }
    }
    
    fn write(&mut self, value: &str);
    
    fn write_at(&mut self, value: &str, y: usize, x: usize) {
        self.move_cursor(y, x);
        self.write(value);
    }
    
    fn write_char(&mut self, value: char);
    
    fn write_char_at(&mut self, value: char, y: usize, x: usize) {
        self.move_cursor(y, x);
        self.write_char(value);
    }
}
