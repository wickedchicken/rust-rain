extern crate ctrlc;
extern crate fps_clock;
extern crate rand;
#[macro_use]
extern crate structopt;
extern crate termion;

use rand::distributions::{Distribution, Poisson};
use rand::Rng;
use std::io;
use std::io::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use termion::color;
use termion::screen::AlternateScreen;
use termion::terminal_size;

pub mod opts;

mod raindrop;

use raindrop::Raindrop;

pub fn draw_rain(opts: &opts::Opt) {
    let (x_max, y_max) = terminal_size().unwrap();
    let mut rng = rand::thread_rng();
    let mut screen = AlternateScreen::from(io::stdout());

    let mut fps = fps_clock::FpsClock::new(opts.fps);

    write!(screen, "{}", termion::cursor::Hide).unwrap();

    let mut drops: Vec<Raindrop> = Vec::new();

    let adjusted_rate = opts.rate / opts.fps as f64;

    let poi = Poisson::new(adjusted_rate);

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    let color_str = opts.color.to_color_str();

    while running.load(Ordering::SeqCst) {
        let number_of_new_drops = poi.sample(&mut rand::thread_rng());

        for _ in 0..number_of_new_drops {
            if drops.len() >= opts.max as usize {
                break;
            }
            let x = rng.gen_range(0 + Raindrop::MAX_X, x_max - (1 + Raindrop::MAX_X));
            let y = rng.gen_range(0 + Raindrop::MAX_Y, y_max - (1 + Raindrop::MAX_Y));

            drops.push(Raindrop::new(x, y));
        }

        let mut new_drops: Vec<Raindrop> = Vec::new();

        write!(screen, "{}", termion::clear::All).unwrap();
        write!(screen, "{}", color_str).unwrap();

        for mut drop in drops {
            drop.draw(&mut screen);
            drop.increment();

            if !drop.is_done() {
                new_drops.push(drop);
            }
        }
        write!(screen, "{}", color::Fg(color::Reset)).unwrap();

        drops = new_drops;

        screen.flush().unwrap();
        fps.tick();
    }

    write!(screen, "{}", termion::cursor::Show).unwrap();
}
