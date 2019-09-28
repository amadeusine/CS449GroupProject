use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum Players {
    PlayerOne,
    PlayerTwo,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum XCoord {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum YCoord {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Coord(XCoord, YCoord);

type Peer = Option<Rc<RefCell<Piece>>>;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct Piece {
    position: Coord,
    peers: Vec<Peer>,
    occupied: Option<Players>,
}

fn main() {
    println!("Hello, world!");
}
