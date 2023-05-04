use crate::core::screen::{Screen, SCREEN_WIDTH, SCREEN_HEIGHT};

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

    fn clear_message(&mut self) {
        for i in 0..SCREEN_WIDTH {
            self.screen[0][i] = ' ';
        }
    }

    // Display a message at the top of the screen.
    /*int msg(char *fmt, ...) {
        va_list args;

        /*
         * if the string is "", just clear the line
         */
        if (*fmt == '\0')
        {
	    move(0, 0);
	    clrtoeol();
	    mpos = 0;
	    return ~ESCAPE;
        }
        /*
         * otherwise add to the message and flush it out
         */
        va_start(args, fmt);
        doadd(fmt, args);
        va_end(args);
        return endmsg();
    }*/
    fn show_message(&mut self, msg: &str) {
        self.clear_message();
        for (i, c) in msg.chars().enumerate() {
            self.screen[0][i] = c;
        }
    }

    // Display the important stats line.  Keep the cursor where it was.
    /*void
    status()
    {
        register int oy, ox, temp;
        static int hpwidth = 0;
        static int s_hungry = 0;
        static int s_lvl = 0;
        static int s_pur = -1;
        static int s_hp = 0;
        static int s_arm = 0;
        static uint s_str = 0;
        static int s_exp = 0;
        static char *state_name[] =
        {
	    "", "Hungry", "Weak", "Faint"
        };

        /*
         * If nothing has changed since the last status, don't
         * bother.
         */
        temp = (cur_armor != NULL ? cur_armor->o_arm : pstats.s_arm);
        if (s_hp == pstats.s_hpt && s_exp == pstats.s_exp && s_pur == purse
	    && s_arm == temp && s_str == pstats.s_str && s_lvl == level
	    && s_hungry == hungry_state
	    )
	        return;

        s_arm = temp;

        getyx(stdscr, oy, ox);
        if (s_hp != max_hp)
        {
	    temp = max_hp;
	    s_hp = max_hp;
	    for (hpwidth = 0; temp; hpwidth++)
	        temp /= 10;
        }

        /*
         * Save current status
         */
        s_lvl = level;
        s_pur = purse;
        s_hp = pstats.s_hpt;
        s_str = pstats.s_str;
        s_exp = pstats.s_exp; 
        s_hungry = hungry_state;

	    move(STATLINE, 0);
                    
            printw("Level: %d  Gold: %-5d  Hp: %*d(%*d)  Str: %2d(%d)  Arm: %-2d  Exp: %d/%d  %s",
	        level, purse, hpwidth, pstats.s_hpt, hpwidth, max_hp, pstats.s_str,
	        max_stats.s_str, 10 - s_arm, pstats.s_lvl, pstats.s_exp,
	        state_name[hungry_state]);

        clrtoeol();
        move(oy, ox);
    }*/
    fn show_status(&mut self, msg: &str) {
        for (i, c) in msg.chars().enumerate() {
            self.screen[SCREEN_HEIGHT - 1][i] = c;
        }
    }

    fn move_cursor(&mut self, y: usize, x: usize) {
        self.cursor = (y, x);
    }

    fn write_char(&mut self, value: char) {
        self.screen[self.cursor.0][self.cursor.1] = value;
    }

    fn write(&mut self, value: &str) {
        for (_, character) in value.chars().enumerate() {
            if character == '\n' {
                self.move_cursor(self.cursor.0 + 1, 0);
            } else if self.cursor.1 > SCREEN_WIDTH - 1 {
                self.move_cursor(self.cursor.0 + 1, 0);
                self.write_char(character);
            } else {
                self.move_cursor(self.cursor.0, self.cursor.1 + 1);
                self.write_char(character);
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