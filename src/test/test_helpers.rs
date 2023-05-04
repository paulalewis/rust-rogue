#[cfg(test)]
pub mod tests {
    use crate::{core::screen::Screen, ui::console_screen::ConsoleScreen};


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

        fn clear_message(&mut self) {
            self.console_screen.clear_message();
        }

        fn show_message(&mut self, msg: &str) {
            self.console_screen.show_message(msg);
        }

        fn show_status(&mut self, msg: &str) {
            self.console_screen.show_status(msg);
        }

        fn move_cursor(&mut self, y: usize, x: usize) {
            self.console_screen.move_cursor(y, x);
        }

        fn write_char(&mut self, c: char) {
            self.console_screen.write_char(c);
        }

        fn write(&mut self, msg: &str) {
            self.console_screen.write(msg);
        }

        fn draw(&self) {
            self.console_screen.draw();
        }
    }
}