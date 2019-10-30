#![allow(non_snake_case)]
#![allow(dead_code)]

use std::{cell::RefCell, collections::HashMap, rc::Rc};
use strum_macros::Display;

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
#[derive(Debug, Clone)]
struct Board(HashMap<Coord, bool>);

#[derive(Debug, Clone)]
struct GameState {
    board: Board,
    mills: Vec<Mill>,
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
