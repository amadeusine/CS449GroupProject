#![allow(non_snake_case)]
#![allow(dead_code)]

use std::{cell::RefCell, collections::HashMap, rc::Rc};
use strum_macros::Display;

mod util;

pub fn base_hello() -> String {
    String::from("Hello from base")
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Display)]
enum Player {
    PlayerOne,
    PlayerTwo,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash, Eq, Display)]
enum XCoord {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash, Eq)]
enum YCoord {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash, Eq)]
struct Coord(XCoord, YCoord);

type Adjacents = Option<Rc<RefCell<Position>>>;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct Position {
    position: Coord,
    peers: Vec<Adjacents>,
    occupied: Option<Player>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct Mill {
    Owner: Player,
    Pieces: (Position, Position, Position),
}

type PositionStatus = (bool, Option<Player>);

#[derive(Debug, Clone)]
pub struct Board(HashMap<Coord, PositionStatus>);

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Display)]
pub enum Handle {
    Ok,
    Err,
}

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Display)]
pub enum Trigger {
    None,
    Placement,
    Elimination,
    Flying,
    End,
    Win,
    Lose,
}

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Display)]
pub enum Action {
    Menu,
    Piece,
}

// TODO: Make top level GameResult type that wraps GameState and custom GameErr types?
#[derive(Debug, Clone)]
struct GameState {
    handle: Handle,
    trigger: Trigger,
    board: Board,
    mills: Vec<Mill>,
}

#[derive(Debug, Clone)]
enum Agent {
    Human,
    Auto,
}

#[derive(Debug, Clone)]
pub struct GameOpts {
    user: Option<Player>,
    opponent: Option<Player>,
    agent: Option<Agent>,
    sender: Option<Player>,
    Position: Option<Coord>,
}

#[derive(Debug, Clone)]
pub struct Manager {
    state: GameState,
}

impl XCoord {
    fn from_char(s: char) -> XCoord {
        match s {
            'A' => XCoord::A,
            'B' => XCoord::B,
            'C' => XCoord::C,
            'D' => XCoord::D,
            'E' => XCoord::E,
            'F' => XCoord::F,
            'G' => XCoord::E,
            e => panic!("Invalid XCoord Character: {:?}", e),
        }
    }
}

impl YCoord {
    fn from_char(s: char) -> YCoord {
        match s {
            '1' => YCoord::One,
            '2' => YCoord::Two,
            '3' => YCoord::Three,
            '4' => YCoord::Four,
            '5' => YCoord::Five,
            '6' => YCoord::Six,
            '7' => YCoord::Seven,
            e => panic!("Invalid YCoord Character: {:?}", e),
        }
    }
}

impl Coord {
    fn new(x: XCoord, y: YCoord) -> Self {
        Coord(x, y)
    }
    fn from_str(s: &str) -> Coord {
        let mut chars = s.chars();
        if let Some(x) = chars.next() {
            if let Some(y) = chars.next() {
                return Coord::new(XCoord::from_char(x), YCoord::from_char(y));
            } else {
                panic!("Invalid Coordinate: Missing Y");
            }
        } else {
            panic!("Invalid Coordinate: Missing X");
        }
    }
}

impl Board {
    pub fn new() -> Self {
        Board::default()
    }
}

impl Default for Board {
    fn default() -> Self {
        // TODO: maybe move out coordinates/position types into own module and declare a static
        //       vector for each major board variation (3, 6, 9, 12) for sake of keeping both
        //       `new_from_n(n)` and `default()` from having such noisy declarations.
        let valid_positions: HashMap<Coord, PositionStatus> = [
            // A => 1, 4, 7
            (Coord::new(XCoord::A, YCoord::One), (false, None)),
            (Coord::new(XCoord::A, YCoord::Four), (false, None)),
            (Coord::new(XCoord::A, YCoord::Seven), (false, None)),
            // B => 2, 4, 6
            (Coord::new(XCoord::B, YCoord::Two), (false, None)),
            (Coord::new(XCoord::B, YCoord::Four), (false, None)),
            (Coord::new(XCoord::B, YCoord::Six), (false, None)),
            // C => 3, 4, 5
            (Coord::new(XCoord::C, YCoord::Three), (false, None)),
            (Coord::new(XCoord::C, YCoord::Four), (false, None)),
            (Coord::new(XCoord::C, YCoord::Five), (false, None)),
            // D => 1, 2, 3, 5, 6, 7
            (Coord::new(XCoord::D, YCoord::One), (false, None)),
            (Coord::new(XCoord::D, YCoord::Two), (false, None)),
            (Coord::new(XCoord::D, YCoord::Three), (false, None)),
            (Coord::new(XCoord::D, YCoord::Five), (false, None)),
            (Coord::new(XCoord::D, YCoord::Six), (false, None)),
            (Coord::new(XCoord::D, YCoord::Seven), (false, None)),
            // E => 3, 4, 5
            (Coord::new(XCoord::E, YCoord::Three), (false, None)),
            (Coord::new(XCoord::E, YCoord::Four), (false, None)),
            (Coord::new(XCoord::E, YCoord::Five), (false, None)),
            // F => 2, 4, 6
            (Coord::new(XCoord::F, YCoord::Two), (false, None)),
            (Coord::new(XCoord::F, YCoord::Four), (false, None)),
            (Coord::new(XCoord::F, YCoord::Six), (false, None)),
            // G => 1, 4, 7
            (Coord::new(XCoord::G, YCoord::One), (false, None)),
            (Coord::new(XCoord::G, YCoord::Four), (false, None)),
            (Coord::new(XCoord::G, YCoord::Seven), (false, None)),
        ]
        .iter()
        .cloned()
        .collect();

        Board(valid_positions)
    }
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            handle: Handle::Ok,
            trigger: Trigger::None,
            board: Board::default(),
            mills: vec![],
        }
    }
}

impl Manager {
    pub fn new() -> Self {
        Manager {
            state: GameState::new(),
        }
    }

    pub fn poll(act: Action, opts: GameOpts) -> Board {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
