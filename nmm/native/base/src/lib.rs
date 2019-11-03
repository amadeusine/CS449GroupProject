#![allow(non_snake_case)]
#![allow(dead_code)]

// ToString Needs to be in scope, do not believe the linter's lies.
use neon::prelude::*;
use std::string::ToString;
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use strum_macros::Display;

mod util;

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

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash, Eq, Display)]
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
pub struct Coord(XCoord, YCoord);

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

#[derive(Debug, Clone, Copy)]
pub struct PositionStatus(bool, Option<Player>);

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
pub struct GameState {
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

impl PositionStatus {
    pub fn new() -> Self {
        PositionStatus::default()
    }
    pub fn as_string(self) -> String {
        if self.0 {
            if let Some(p) = self.1 {
                return format!("{}", p);
            } else {
                panic!("PositionStatus set to true, but Player is None")
            }
        } else {
            return format!("None");
        }
    }
}

impl Default for PositionStatus {
    fn default() -> Self {
        PositionStatus(false, None)
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
    pub fn as_string(self) -> String {
        format!("{}{}", self.0, self.1 as u32)
    }
}

impl Board {
    pub fn new() -> Self {
        Board::default()
    }

    fn add(&mut self, k: Coord, v: PositionStatus) {
        self.0.insert(k, v);
    }

    pub fn len(&self) -> u32 {
        self.0.len() as u32
    }
}

impl IntoIterator for Board {
    // on god, i think the trait impl requires the assoc type Item but Hashmap literlaly doesn't
    // use it because it's a tuple type lol
    type Item = (Coord, PositionStatus);
    type IntoIter = ::std::collections::hash_map::IntoIter<Coord, PositionStatus>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Default for Board {
    fn default() -> Self {
        // TODO: maybe move out coordinates/position types into own module and declare a static
        //       vector for each major board variation (3, 6, 9, 12) for sake of keeping both
        //       `new_from_n(n)` and `default()` from having such noisy declarations.
        let valid_positions: HashMap<Coord, PositionStatus> = [
            // A => 1, 4, 7
            (
                Coord::new(XCoord::A, YCoord::One),
                PositionStatus::default(),
            ),
            (
                Coord::new(XCoord::A, YCoord::Four),
                PositionStatus::default(),
            ),
            (
                Coord::new(XCoord::A, YCoord::Seven),
                PositionStatus::default(),
            ),
            // B => 2, 4, 6
            (
                Coord::new(XCoord::B, YCoord::Two),
                PositionStatus::default(),
            ),
            (
                Coord::new(XCoord::B, YCoord::Four),
                PositionStatus::default(),
            ),
            (
                Coord::new(XCoord::B, YCoord::Six),
                PositionStatus::default(),
            ),
            // C => 3, 4, 5
            (
                Coord::new(XCoord::C, YCoord::Three),
                PositionStatus::default(),
            ),
            (
                Coord::new(XCoord::C, YCoord::Four),
                PositionStatus::default(),
            ),
            (
                Coord::new(XCoord::C, YCoord::Five),
                PositionStatus::default(),
            ),
            // D => 1, 2, 3, 5, 6, 7
            (
                Coord::new(XCoord::D, YCoord::One),
                PositionStatus::default(),
            ),
            (
                Coord::new(XCoord::D, YCoord::Two),
                PositionStatus::default(),
            ),
            (
                Coord::new(XCoord::D, YCoord::Three),
                PositionStatus::default(),
            ),
            (
                Coord::new(XCoord::D, YCoord::Five),
                PositionStatus::default(),
            ),
            (
                Coord::new(XCoord::D, YCoord::Six),
                PositionStatus::default(),
            ),
            (
                Coord::new(XCoord::D, YCoord::Seven),
                PositionStatus::default(),
            ),
            // E => 3, 4, 5
            (
                Coord::new(XCoord::E, YCoord::Three),
                PositionStatus::default(),
            ),
            (
                Coord::new(XCoord::E, YCoord::Four),
                PositionStatus::default(),
            ),
            (
                Coord::new(XCoord::E, YCoord::Five),
                PositionStatus::default(),
            ),
            // F => 2, 4, 6
            (
                Coord::new(XCoord::F, YCoord::Two),
                PositionStatus::default(),
            ),
            (
                Coord::new(XCoord::F, YCoord::Four),
                PositionStatus::default(),
            ),
            (
                Coord::new(XCoord::F, YCoord::Six),
                PositionStatus::default(),
            ),
            // G => 1, 4, 7
            (
                Coord::new(XCoord::G, YCoord::One),
                PositionStatus::default(),
            ),
            (
                Coord::new(XCoord::G, YCoord::Four),
                PositionStatus::default(),
            ),
            (
                Coord::new(XCoord::G, YCoord::Seven),
                PositionStatus::default(),
            ),
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

    fn get_handle(&self) -> Handle {
        self.handle
    }

    fn get_trigger(&self) -> Trigger {
        self.trigger
    }

    fn get_board(&self) -> Board {
        self.board.clone()
    }
}

impl GameOpts {
    fn new() -> Self {
        unimplemented!()
    }
}

impl Manager {
    pub fn new() -> Self {
        Manager {
            state: GameState::new(),
        }
    }

    // poll() will eventually use Action and Opts together to figure out what game logic to compute
    // from the attempted move. For now, just trying to figure out what an okay "public" "API" would
    // look like when this gets exported into the node module. Main idea is that node/js interacts
    // exclusively through this `Manager` struct, which is getting exported as a Js Class and
    // has a limited set of methods that will compute the necessary logic on the game state hidden
    // within the exported rust module.
    // pub fn poll(&mut self, act: Action, opts: GameOpts) -> (Handle, Trigger, Board) {
    pub fn poll(&self) -> (Handle, Trigger, Board) {
        (
            self.state.get_handle(),
            self.state.get_trigger(),
            self.state.get_board(),
        )
    }

    pub fn conv_elem_type_to_action(action: JsString) -> Action {
        match action.value().as_ref() {
            "Menu" => Action::Menu,
            "Piece" => Action::Piece,
            e => panic!("Invalid value for ElementType: {:#?}", e),
        }
    }

    pub fn conv_menu_options(ctx: &mut FunctionContext, opts: &mut JsObject) -> (u32, u32) {
        let user = Manager::conv_player_option(ctx, opts, "user");
        let opponent = Manager::conv_player_option(ctx, opts, "opponent");
        (user, opponent)
        // unreachable!()
    }

    pub fn conv_player_option(ctx: &mut FunctionContext, opts: &mut JsObject, player: &str) -> u32 {
        match opts.get(ctx, player) {
            Ok(js_handle) if js_handle.is_a::<JsNumber>() => match js_handle.downcast::<JsNumber>()
            {
                Ok(num) => num.value() as u32,
                Err(e) => panic!("Failed to convert JsNumber: {:#?}", e),
            },
            Ok(_) => {
                // let js_handle = js_handle.upcast();
                panic!(
                    "Property \"{}\" did not contain a JsNumber",
                    String::from(player)
                )
            }
            Err(_) => panic!(
                "Could not get \"{}\" property from options object.",
                String::from(player)
            ),
        }
    }

    pub fn get_board(&self) -> Board {
        self.state.get_board()
    }

    pub fn get_action(act: &str) -> Action {
        match act {
            "Menu" => Action::Menu,
            "Piece" => Action::Piece,
            other => panic!("Invalid type passed as ElementType: {:#?}", other),
        };
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
