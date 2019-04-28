use structopt::clap::arg_enum;
use termion::color;

use std::num::ParseIntError;
use std::str::FromStr;

fn parse_fps(src: &str) -> Result<u32, ParseIntError> {
    let res = u32::from_str(src)?;

    if res > 0 {
        Ok(res)
    } else {
        panic!("fps must be greater than zero!")
    }
}

arg_enum! {
    #[derive(Debug)]
    pub enum Color {
        Black,
        Blue,
        Cyan,
        Green,
        LightBlack,
        LightBlue,
        LightCyan,
        LightGreen,
        LightMagenta,
        LightRed,
        LightWhite,
        LightYellow,
        Magenta,
        Red,
        White,
        Yellow,
    }
}

impl Color {
    pub fn to_color_str(&self) -> String {
        match self {
            Color::Black => format!("{}", color::Fg(color::Black)),
            Color::Blue => format!("{}", color::Fg(color::Blue)),
            Color::Cyan => format!("{}", color::Fg(color::Cyan)),
            Color::Green => format!("{}", color::Fg(color::Green)),
            Color::LightBlack => format!("{}", color::Fg(color::LightBlack)),
            Color::LightBlue => format!("{}", color::Fg(color::LightBlue)),
            Color::LightCyan => format!("{}", color::Fg(color::LightCyan)),
            Color::LightGreen => format!("{}", color::Fg(color::LightGreen)),
            Color::LightMagenta => format!("{}", color::Fg(color::LightMagenta)),
            Color::LightRed => format!("{}", color::Fg(color::LightRed)),
            Color::LightWhite => format!("{}", color::Fg(color::LightWhite)),
            Color::LightYellow => format!("{}", color::Fg(color::LightYellow)),
            Color::Magenta => format!("{}", color::Fg(color::Magenta)),
            Color::Red => format!("{}", color::Fg(color::Red)),
            Color::White => format!("{}", color::Fg(color::White)),
            Color::Yellow => format!("{}", color::Fg(color::Yellow)),
        }
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
        default_value = "Blue",
        short = "c",
        long = "color",
        help = "ANSI color to draw the rain in",
        raw(possible_values = "&Color::variants()", case_insensitive = "true")
    )]
    pub color: Color,
    #[structopt(
        default_value = "10.0",
        short = "r",
        long = "rate",
        help = "average drops per second to generate"
    )]
    pub rate: f64,
    #[structopt(
        default_value = "100",
        short = "m",
        long = "max",
        help = "maximum number of drops to render at one time"
    )]
    pub max: i32,
    #[structopt(
        default_value = "10",
        parse(try_from_str = "parse_fps"),
        short = "f",
        long = "fps",
        help = "rendered frames per second (may be limited by terminal draw speed)"
    )]
    pub fps: u32,
}
