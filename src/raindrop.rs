use std::cmp;
use std::convert::TryInto;

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
pub struct Raindrop {
    state: usize,
    coord: Coordinate,
}

impl Raindrop {
    pub fn new(x: u16, y: u16) -> Raindrop {
        Raindrop {
            state: 0,
            coord: Coordinate { x, y },
        }
    }

    pub fn draw(&self, screen: &mut dyn std::io::Write) {
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

    pub fn increment(&mut self) {
        self.state = cmp::min(self.state + 1, Raindrop::states().len());
    }

    pub fn is_done(&self) -> bool {
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

    pub const MAX_X: u16 = 2;
    pub const MAX_Y: u16 = 2;
}
