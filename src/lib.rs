extern crate rand;
#[macro_use]
extern crate structopt;
extern crate termion;

use rand::distributions::{Distribution, Poisson};
use rand::Rng;
use std::cmp;
use std::convert::TryInto;
use std::io;
use std::io::prelude::*;
use std::num::ParseFloatError;
use std::str::FromStr;
use std::{thread, time};
use termion::color;
use termion::screen::AlternateScreen;
use termion::terminal_size;

fn parse_fps(src: &str) -> Result<f64, ParseFloatError> {
    let res = f64::from_str(src)?;

    if res > 0.0 {
        Ok(res)
    } else {
        panic!("fps must be greater than zero!")
    }
}

#[derive(StructOpt)]
#[structopt(
    name = "rain",
    about = "Display rain on the terminal",
    raw(setting = "structopt::clap::AppSettings::ColoredHelp")
)]
pub struct Opt {
    #[structopt(
        default_value = "blue",
        short = "c",
        long = "color",
        help = "color to draw the rain (black, blue, cyan, green, magenta, red, white, yellow)"
    )]
    color: String,
    #[structopt(
        default_value = "10.0",
        short = "r",
        long = "rate",
        help = "average drops per second to generate"
    )]
    rate: f64,
    #[structopt(
        default_value = "100",
        short = "m",
        long = "max",
        help = "maximum number of drops to render at one time"
    )]
    max: i32,
    #[structopt(
        default_value = "10",
        parse(try_from_str = "parse_fps"),
        short = "f",
        long = "fps",
        help = "rendered frames per second (may be limited by terminal draw speed)"
    )]
    fps: f64,
}

#[derive(Debug)]
struct Coordinate {
    x: u16,
    y: u16,
}

#[derive(Debug)]
struct RelativeCoordinate {
    x: i8,
    y: i8,
}

#[derive(Debug)]
struct Drawchar {
    coord: RelativeCoordinate,
    char: &'static str,
}

impl Drawchar {
    fn new(x: i8, y: i8, c: &'static str) -> Drawchar {
        Drawchar {
            coord: RelativeCoordinate { x, y },
            char: c,
        }
    }
}

#[derive(Debug)]
struct Raindrop {
    state: usize,
    coord: Coordinate,
}

impl Raindrop {
    fn new(x: u16, y: u16) -> Raindrop {
        Raindrop {
            state: 0,
            coord: Coordinate { x, y },
        }
    }

    fn draw(&self, screen: &mut dyn std::io::Write) {
        for drawchar in &Raindrop::states()[self.state] {
            let newx: u16 = (self.coord.x as i32 + drawchar.coord.x as i32)
                .try_into()
                .unwrap();
            let newy: u16 = (self.coord.y as i32 + drawchar.coord.y as i32)
                .try_into()
                .unwrap();
            write!(
                screen,
                "{}{}",
                termion::cursor::Goto(newx, newy),
                drawchar.char
            )
            .unwrap();
        }
    }

    fn increment(&mut self) {
        self.state = cmp::min(self.state + 1, Raindrop::states().len());
    }

    fn is_done(&self) -> bool {
        return self.state >= Raindrop::states().len();
    }

    fn states() -> Vec<Vec<Drawchar>> {
        vec![
            vec![Drawchar::new(0, 0, ".")],
            vec![Drawchar::new(0, 0, "o")],
            vec![Drawchar::new(0, 0, "O")],
            vec![
                Drawchar::new(0, -1, "-"),
                Drawchar::new(-1, 0, "|"),
                Drawchar::new(0, 0, "."),
                Drawchar::new(1, 0, "|"),
                Drawchar::new(0, 1, "-"),
            ],
            vec![
                Drawchar::new(0, -2, "-"),
                Drawchar::new(-1, -1, "/"),
                Drawchar::new(1, -1, "\\"),
                Drawchar::new(-2, 0, "|"),
                Drawchar::new(0, 0, "O"),
                Drawchar::new(2, 0, "|"),
                Drawchar::new(-1, 1, "\\"),
                Drawchar::new(1, 1, "/"),
                Drawchar::new(0, 2, "-"),
            ],
        ]
    }

    const MAX_X: u16 = 2;
    const MAX_Y: u16 = 2;
}

pub fn draw_rain(opts: &Opt) {
    let (x_max, y_max) = terminal_size().unwrap();
    let mut rng = rand::thread_rng();
    let mut screen = AlternateScreen::from(io::stdout());

    print!("{}", termion::cursor::Hide);

    let mut drops: Vec<Raindrop> = Vec::new();

    let millis = (((1.0 / opts.fps) * 1000.0) as u64).try_into().unwrap();

    let adjusted_rate = opts.rate * (millis as f64 / 1000.0);

    let poi = Poisson::new(adjusted_rate);

    loop {
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

        match opts.color.as_str() {
            "black" => write!(screen, "{}", color::Fg(color::Black)).unwrap(),
            "blue" => write!(screen, "{}", color::Fg(color::Blue)).unwrap(),
            "cyan" => write!(screen, "{}", color::Fg(color::Cyan)).unwrap(),
            "green" => write!(screen, "{}", color::Fg(color::Green)).unwrap(),
            "magenta" => write!(screen, "{}", color::Fg(color::Magenta)).unwrap(),
            "red" => write!(screen, "{}", color::Fg(color::Red)).unwrap(),
            "white" => write!(screen, "{}", color::Fg(color::White)).unwrap(),
            "yellow" => write!(screen, "{}", color::Fg(color::Yellow)).unwrap(),
            _ => panic!("could not parse color"),
        }

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
        thread::sleep(time::Duration::from_millis(millis));
    }
}
