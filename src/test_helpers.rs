#[cfg(test)]
pub mod tests {
    use crate::screen::{ConsoleScreen, Screen};

    pub struct MockScreen {
        console_screen: ConsoleScreen,
    }

    impl MockScreen {
        pub const fn new() -> MockScreen {
            MockScreen {
                console_screen: ConsoleScreen::new(),
            }
        }
    }

    impl Screen for MockScreen {
        fn clear(&mut self) {
            self.console_screen.clear();
        }

        fn clear_msg(&mut self) {
            self.console_screen.clear_msg();
        }

        fn msg(&mut self, msg: &str) {
            self.console_screen.msg(msg);
        }

        fn status(&mut self, msg: &str) {
            self.console_screen.status(msg);
        }

        fn move_cursor(&mut self, y: usize, x: usize) {
            self.console_screen.move_cursor(y, x);
        }

        fn writ_char(&mut self, c: char) {
            self.console_screen.writ_char(c);
        }

        fn write(&mut self, msg: &str) {
            self.console_screen.write(msg);
        }

        fn draw(&self) {
            self.console_screen.draw();
        }
    }
}