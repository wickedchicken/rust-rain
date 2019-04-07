extern crate rand;
extern crate termion;

use rand::Rng;
use std::io;
use std::io::prelude::*;
use std::{thread, time};
use termion::screen::AlternateScreen;
use termion::terminal_size;

pub fn print_termsize() {
    let (x_max, y_max) = terminal_size().unwrap();
    let mut rng = rand::thread_rng();
    let mut screen = AlternateScreen::from(io::stdout());

    print!("{}", termion::cursor::Hide);

    loop {
        let x = rng.gen_range(0, x_max - 1);
        let y = rng.gen_range(0, y_max - 1);

        write!(screen, "{}", termion::clear::All).unwrap();
        write!(screen, "{}X", termion::cursor::Goto(x, y)).unwrap();
        screen.flush().unwrap();
        thread::sleep(time::Duration::from_millis(100));
    }
}
