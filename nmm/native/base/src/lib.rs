#![allow(non_snake_case)]
#![allow(dead_code)]

use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub fn base_hello() -> String {
    String::from("Hello from base")
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum Players {
    PlayerOne,
    PlayerTwo,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash, Eq)]
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

type Peer = Option<Rc<RefCell<Piece>>>;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct Piece {
    position: Coord,
    peers: Vec<Peer>,
    occupied: Option<Players>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct Mill(Piece, Piece, Piece);
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
