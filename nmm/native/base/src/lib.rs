#![allow(non_snake_case)]
#![allow(dead_code)]

// ToString Needs to be in scope, do not believe the linter's lies.
use std::string::ToString;
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use strum_macros::{AsRefStr, Display};

use enum_map::{enum_map, Enum, EnumMap};

mod util;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Display)]
pub enum Player {
    PlayerOne,
    PlayerTwo,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash, Eq, Display, Enum)]
enum XCoord {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash, Eq, Display, Enum)]
enum YCoord {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Display, Enum, AsRefStr)]
enum MillXY {
    A1,
    A4,
    A7,
    B2,
    B6,
    C3,
    C5,
    D1,
    D5,
    E3,
    E4,
    F2,
    G1,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash, Eq)]
pub struct Coord(XCoord, YCoord);

type Position = Option<Rc<RefCell<PositionNode>>>;

#[derive(Debug, Clone, PartialEq)]
pub struct AdjacentPositionList(HashMap<Coord, PositionList>);

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct PositionNode {
    data: Coord,
    next: Position,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct PositionList {
    head: Position,
    tail: Position,
    pub length: u64,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct Mill {
    owner: Player,
    pieces: [Coord; 3],
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PositionStatus(bool, Option<Player>);

#[derive(Debug, Clone, PartialEq)]
pub struct MillMap(EnumMap<MillXY, ([Coord; 3], Option<[Coord; 3]>)>);

#[derive(Debug, Clone, PartialEq)]
pub struct Board(HashMap<Coord, PositionStatus>);

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Display)]
pub enum Handle {
    Ok,
    Err,
}

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Display)]
pub enum PollError {
    CantAttack,
    InvalidPosition,
    NotPlayersTurn,
    AttackingSelf,
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

// TODO: Make top level GameResult type that wraps GameState and custom GameErr types?
#[derive(Debug, Clone, PartialEq)]
pub struct GameState {
    handle: Handle,
    trigger: Trigger,
    board: Board,
    adj_positions: AdjacentPositionList,
    mills: MillMap,
    curr_mills: Vec<Mill>,
    p1_count: (u32, u32),
    p2_count: (u32, u32),
    switch_move: bool,
}

#[derive(Debug, Clone, PartialEq, Display, Copy)]
pub enum Agent {
    Human,
    Auto,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct GameOpts {
    user: Option<Player>,
    opponent: Option<Player>,
    agent: Option<Agent>,
    sender: Option<Player>,
    position: Option<Coord>,
}

#[derive(Debug, Clone, PartialEq)]
struct ActionResult {
    sender: Player,
    board: Board,
    position: Coord,
    // Just want to get this going, will impl later.
    trigger: Trigger,
    handle: Handle,
}

#[derive(Debug, Clone, Copy)]
struct Game {
    user: Player,
    opponent: Player,
    last_turn: Player,
    turns: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Manager {
    state: GameState,
    history: Vec<ActionResult>,
    settings: GameOpts,
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
            'G' => XCoord::G,
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

    pub fn from(player: Player) -> Self {
        PositionStatus(true, Some(player))
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
    fn occupied(&self) -> bool {
        self.0
    }

    fn player(&self) -> Option<Player> {
        self.1
    }

    fn as_tuple(&self) -> (bool, Option<Player>) {
        (self.0, self.1)
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

    pub fn from_str(s: &str) -> Coord {
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

impl MillXY {
    fn from_coord(c: Coord) -> MillXY {
        match c {
            Coord(XCoord::A, YCoord::One) => MillXY::A1,
            Coord(XCoord::A, YCoord::Four) => MillXY::A4,
            Coord(XCoord::A, YCoord::Seven) => MillXY::A7,
            Coord(XCoord::B, YCoord::Two) => MillXY::B2,
            Coord(XCoord::B, YCoord::Six) => MillXY::B6,
            Coord(XCoord::C, YCoord::Three) => MillXY::C3,
            Coord(XCoord::C, YCoord::Five) => MillXY::C5,
            Coord(XCoord::D, YCoord::One) => MillXY::D1,
            Coord(XCoord::D, YCoord::Five) => MillXY::D5,
            Coord(XCoord::E, YCoord::Three) => MillXY::E3,
            Coord(XCoord::E, YCoord::Four) => MillXY::E4,
            Coord(XCoord::F, YCoord::Two) => MillXY::F2,
            Coord(XCoord::G, YCoord::One) => MillXY::G1,
            _ => panic!("Invalid Coord passed to MillXY::from_coord()"),
        }
    }
}

impl PositionNode {
    fn new(xy: Coord, next: Position) -> Rc<RefCell<PositionNode>> {
        Rc::new(RefCell::new(PositionNode {
            data: xy,
            next: next,
        }))
    }
}

impl PositionList {
    fn new_empty() -> Self {
        PositionList {
            head: None,
            tail: None,
            length: 0,
        }
    }
    fn new_from_vec(head: Coord, ls: Vec<Coord>) -> Self {
        let mut pl = PositionList::new_empty();
        pl.append(head);

        // shoutout to rust iters for not blowing up on empty.
        for c in &ls[1..] {
            pl.append(*c)
        }
        pl
    }

    fn append(&mut self, pos: Coord) {
        let new_pos = PositionNode::new(pos, None);
        match self.tail.take() {
            Some(p) => p.borrow_mut().next = Some(new_pos.clone()),
            _ => self.head = Some(new_pos.clone()),
        };
        self.length += 1;
        self.tail = Some(new_pos);
    }

    // get_head(&self) -> & {
    //     unimplemented!()
    // }
}

pub struct AdjacencyIterator {
    current: Position,
}
impl AdjacencyIterator {
    fn new(start: Position) -> AdjacencyIterator {
        AdjacencyIterator { current: start }
    }
}

impl Iterator for AdjacencyIterator {
    type Item = PositionNode;

    fn next(&mut self) -> Option<Self::Item> {
        let curr = &self.current;
        let mut result = None;
        self.current = match curr {
            Some(ref curr) => {
                let curr = curr.borrow();
                result = Some(curr.clone());
                curr.next.clone()
            }
            _ => None,
        };

        result
    }
}

impl IntoIterator for PositionList {
    type Item = PositionNode;
    type IntoIter = AdjacencyIterator;

    fn into_iter(self) -> Self::IntoIter {
        AdjacencyIterator { current: self.head }
    }
}

impl<'a> IntoIterator for &'a AdjacentPositionList {
    type Item = (&'a Coord, &'a PositionList);
    type IntoIter = ::std::collections::hash_map::Iter<'a, Coord, PositionList>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl AdjacentPositionList {
    fn get(&self, xy: &Coord) -> Option<&PositionList> {
        match &self.0.get(xy) {
            Some(pl) => Some(pl),
            None => None,
        }
    }
}

impl Default for AdjacentPositionList {
    fn default() -> Self {
        let mut apl: HashMap<Coord, PositionList> = HashMap::new();

        let vec_of_vec_coords = util::get_adjacency_vec("default.txt");
        for coord_vec in vec_of_vec_coords {
            let xy = match coord_vec.first() {
                Some(c) => c,
                None => panic!("Empty vec in Vec<Vec<Coord>> returned by `get_adjacency_vec`"),
            };

            apl.insert(*xy, PositionList::new_from_vec(*xy, coord_vec));
        }
        AdjacentPositionList(apl)
    }
}

impl Default for MillMap {
    #[rustfmt::skip]
    fn default() -> Self {
        MillMap(
            enum_map! {
            // It doesn't matter what order the mills are inserted in the tuple, but I organize this
            // nasty declaration with the row mill being first when a MillXY is assoc with 2 mills.
            // Otherwise, the non-option array is whatever mill a MillXY assoc with, whether row
            // or col
            MillXY::A1 => ([Coord::from_str("A1"), Coord::from_str("D1"), Coord::from_str("G1")],
                           Some([Coord::from_str("A1"), Coord::from_str("A4"), Coord::from_str("A7")])),
            MillXY::A4 => ([Coord::from_str("A4"), Coord::from_str("B4"), Coord::from_str("C4")], None),
            MillXY::A7 => ([Coord::from_str("A7"), Coord::from_str("D7"), Coord::from_str("G7")], None),
            MillXY::B2 => ([Coord::from_str("B2"), Coord::from_str("D2"), Coord::from_str("F2")],
                           Some([Coord::from_str("B2"), Coord::from_str("B4"), Coord::from_str("B6")])),
            MillXY::B6 => ([Coord::from_str("B6"), Coord::from_str("D6"), Coord::from_str("F6")], None),
            MillXY::C3 => ([Coord::from_str("C3"), Coord::from_str("D3"), Coord::from_str("E3")],
                           Some([Coord::from_str("C3"), Coord::from_str("C4"), Coord::from_str("C5")])),
            MillXY::C5 => ([Coord::from_str("C5"), Coord::from_str("D5"), Coord::from_str("E5")], None),
            MillXY::D1 => ([Coord::from_str("D1"), Coord::from_str("D2"), Coord::from_str("D3")], None),
            MillXY::D5 => ([Coord::from_str("D5"), Coord::from_str("D6"), Coord::from_str("D7")], None),
            MillXY::E3 => ([Coord::from_str("E3"), Coord::from_str("E4"), Coord::from_str("E5")], None),
            MillXY::E4 => ([Coord::from_str("E4"), Coord::from_str("F4"), Coord::from_str("G4")], None),
            MillXY::F2 => ([Coord::from_str("F2"), Coord::from_str("F4"), Coord::from_str("F6")], None),
            MillXY::G1 => ([Coord::from_str("G1"), Coord::from_str("G4"), Coord::from_str("G7")], None),
        })
    }
}

impl MillXY {
    fn as_coord(&self) -> Coord {
        Coord::from_str(self.as_ref())
    }
}

impl MillMap {
    fn detect_mills(&self, player_pos: &Vec<Coord>) -> Vec<[Coord; 3]> {
        let mut player_mills = vec![];

        for (xy, mills) in self.0.iter() {
            if player_pos.contains(&Coord::from_str(xy.as_ref())) {
                let _ = match mills {
                    (mill, Some(opt_mill)) => {
                        if mill.iter().all(|c| player_pos.contains(c)) {
                            player_mills.push(*mill);
                        }
                        if opt_mill.iter().all(|c| player_pos.contains(c)) {
                            player_mills.push(*opt_mill);
                        }
                    }
                    (mill, None) => {
                        if mill.iter().all(|c| player_pos.contains(c)) {
                            player_mills.push(*mill);
                        }
                    }
                };
            }
        }
        player_mills
    }
}

impl Board {
    pub fn new() -> Self {
        Board::default()
    }

    fn update(&mut self, k: Coord, v: PositionStatus) {
        self.0.insert(k, v);
    }

    pub fn len(&self) -> u32 {
        self.0.len() as u32
    }

    pub fn get(&self, xy: &Coord) -> Option<&PositionStatus> {
        self.0.get(xy)
    }
    pub fn set(&mut self, xy: Coord, player: Player) -> Option<PositionStatus> {
        self.0.insert(xy, PositionStatus::from(player))
    }

    fn unset(&mut self, xy: &Coord) {
        // TODO: Technically returns an Option of the old value. Do I care to handle it? I don't
        // think so.
        self.0.insert(*xy, PositionStatus::new());
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

impl<'a> IntoIterator for &'a Board {
    type Item = (&'a Coord, &'a PositionStatus);
    type IntoIter = ::std::collections::hash_map::Iter<'a, Coord, PositionStatus>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl Default for Board {
    fn default() -> Self {
        // TODO: maybe move out coordinates/position types into own module and declare a static
        //       vector for each major board variation (3, 6, 9, 12) for sake of keeping both
        //       `new_from_n(n)` and `default()` from having such noisy declarations.
        #[rustfmt::skip]
        let valid_positions: HashMap<Coord, PositionStatus> = [
            // A => 1, 4, 7
            (Coord::new(XCoord::A, YCoord::One), PositionStatus::default()),
            (Coord::new(XCoord::A, YCoord::Four), PositionStatus::default()),
            (Coord::new(XCoord::A, YCoord::Seven), PositionStatus::default()),
            // B => 2, 4, 6
            (Coord::new(XCoord::B, YCoord::Two), PositionStatus::default()),
            (Coord::new(XCoord::B, YCoord::Four), PositionStatus::default()),
            (Coord::new(XCoord::B, YCoord::Six),PositionStatus::default()),
            // C => 3, 4, 5
            (Coord::new(XCoord::C, YCoord::Three), PositionStatus::default()),
            (Coord::new(XCoord::C, YCoord::Four), PositionStatus::default()),
            (Coord::new(XCoord::C, YCoord::Five), PositionStatus::default()),
            // D => 1, 2, 3, 5, 6, 7
            (Coord::new(XCoord::D, YCoord::One), PositionStatus::default()),
            (Coord::new(XCoord::D, YCoord::Two), PositionStatus::default()),
            (Coord::new(XCoord::D, YCoord::Three), PositionStatus::default()),
            (Coord::new(XCoord::D, YCoord::Five), PositionStatus::default()),
            (Coord::new(XCoord::D, YCoord::Six), PositionStatus::default()),
            (Coord::new(XCoord::D, YCoord::Seven), PositionStatus::default()),
            // E => 3, 4, 5
            (Coord::new(XCoord::E, YCoord::Three), PositionStatus::default()),
            (Coord::new(XCoord::E, YCoord::Four), PositionStatus::default()),
            (Coord::new(XCoord::E, YCoord::Five), PositionStatus::default()),
            // F => 2, 4, 6
            (Coord::new(XCoord::F, YCoord::Two), PositionStatus::default()),
            (Coord::new(XCoord::F, YCoord::Four), PositionStatus::default()),
            (Coord::new(XCoord::F, YCoord::Six), PositionStatus::default()),
            // G => 1, 4, 7
            (Coord::new(XCoord::G, YCoord::One), PositionStatus::default()),
            (Coord::new(XCoord::G, YCoord::Four), PositionStatus::default()),
            (Coord::new(XCoord::G, YCoord::Seven), PositionStatus::default()),
        ]
        .iter()
        .cloned()
        .collect();

        Board(valid_positions)
    }
}

impl ActionResult {
    fn new(
        sender: Player,
        board: Board,
        position: Coord,
        handle: Handle,
        trigger: Trigger,
    ) -> Self {
        ActionResult {
            sender: sender,
            board: board,
            position: position,
            handle: handle,
            trigger: trigger,
        }
    }
    fn get_trigger(&self) -> Trigger {
        self.trigger
    }

    fn get_player(&self) -> Player {
        self.sender
    }
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            handle: Handle::Ok,
            trigger: Trigger::None,
            board: Board::default(),
            adj_positions: AdjacentPositionList::default(),
            mills: MillMap::default(),
            curr_mills: vec![],
            p1_count: (0, 9),
            p2_count: (0, 9),
            switch_move: false,
        }
    }

    fn set_state(&mut self, handle: Handle, trigger: Trigger) {
        self.handle = handle;
        self.trigger = trigger;
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

    fn get_player_pieces(&self) -> ((Player, u32), (Player, u32)) {
        let mut p1 = 0;
        let mut p2 = 0;

        for (_, pos) in (&self.board).into_iter() {
            if pos.occupied() {
                match pos.1.as_ref() {
                    Some(Player::PlayerOne) => p1 += 1,
                    Some(Player::PlayerTwo) => p2 += 1,
                    None => continue,
                }
            }
        }

        ((Player::PlayerOne, p1), (Player::PlayerTwo, p2))
    }

    fn set_switch(&mut self, b: bool) {
        self.switch_move = b;
    }

    fn update_mills(&mut self) {
        let mut p1_positions: Vec<Coord> = vec![];
        let mut p1_mills = vec![];
        let mut p2_positions: Vec<Coord> = vec![];
        let mut p2_mills = vec![];

        for (xy, pos) in &self.board {
            match (*pos).as_tuple() {
                (true, Some(p)) if p == Player::PlayerOne => p1_positions.push(*xy),
                (true, Some(_)) => p2_positions.push(*xy),
                (true, _) => panic!("matched PositionStatus true with None in find_mills"),
                (false, _) => continue,
            }
        }
        if p1_positions.len() > 2 {
            for mill in self.mills.detect_mills(&p1_positions) {
                p1_mills.push(Mill {
                    owner: Player::PlayerOne,
                    pieces: mill,
                })
            }
        }
        if p2_positions.len() > 2 {
            for mill in self.mills.detect_mills(&p2_positions) {
                p2_mills.push(Mill {
                    owner: Player::PlayerOne,
                    pieces: mill,
                })
            }
        }

        p1_mills.append(&mut p2_mills);
        self.curr_mills.clear();
        self.curr_mills.append(&mut p1_mills);
    }

    fn player_mills(&self, player: &Player) -> Vec<Mill> {
        self.curr_mills
            .clone()
            .into_iter()
            .filter(|mill| mill.owner == *player)
            .collect()
    }
}

impl GameOpts {
    pub fn new() -> Self {
        GameOpts {
            user: None,
            opponent: None,
            agent: None,
            sender: None,
            position: None,
        }
    }
    pub fn new_game_opt(user: u32, opp: u32, agent: Agent) -> Self {
        let user = match user {
            1 => Some(Player::PlayerOne),
            2 => Some(Player::PlayerTwo),
            _ => unreachable!(),
        };
        let opp = match opp {
            1 => Some(Player::PlayerOne),
            2 => Some(Player::PlayerTwo),
            _ => unreachable!(),
        };
        GameOpts {
            user: user,
            opponent: opp,
            agent: Some(agent),
            sender: None,
            position: None,
        }
    }

    pub fn new_piece_opt(sender: u32, position: Coord) -> Self {
        let sender = match sender {
            1 => Some(Player::PlayerOne),
            2 => Some(Player::PlayerTwo),
            _ => unreachable!(),
        };

        GameOpts {
            user: None,
            opponent: None,
            agent: None,
            sender: sender,
            position: Some(position),
        }
    }
}

impl Manager {
    pub fn new() -> Self {
        Manager {
            state: GameState::new(),
            history: vec![],
            settings: GameOpts::new(),
        }
    }

    pub fn new_opts(&mut self, game_opts: GameOpts) {
        self.settings = game_opts;
    }

    pub fn poll(&mut self, opts: GameOpts) -> (Handle, Trigger, Board) {
        // Grabbing this in case of failed poll operation to return.
        let prev_board = self.get_board();
        // Get what we need out of the GameOpts struct.
        let (move_coord, curr_player) = self.setup(&opts);

        // Actual poll logic starts here.
        self.validate(&move_coord, &curr_player);
        // TODO: generate action result, whether in validate, a method called within validate, etc
        // TODO: Add action result to history after deciding where to generate it
        self.get_curr_state()
    }

    fn setup(&mut self, opts: &GameOpts) -> (Coord, Player) {
        let curr_player = match opts.sender {
            Some(Player::PlayerOne) => Player::PlayerOne,
            Some(Player::PlayerTwo) => Player::PlayerTwo,
            None => panic!("Poll called without a `sender` value in GameOpts struct"),
        };
        let move_coord = match opts.position {
            Some(c) => c,
            None => panic!("Poll called without a `position` value in GameOpts struct"),
        };
        (move_coord, curr_player)
    }

    fn validate(&mut self, move_coord: &Coord, curr_player: &Player) {
        if self.is_switch() && Some(*curr_player) != self.get_prev_player() {
            // TODO: Generate unsuccessful poll game results
            self.failed_poll(PollError::NotPlayersTurn);
            // Leave
            return;
        }

        match self.get_position(move_coord) {
            Some(p) => match p.as_tuple() {
                (true, Some(_p)) if _p == *curr_player => self.move_out(move_coord, curr_player),
                (true, Some(_p)) => self.move_attack(move_coord, curr_player),
                (true, _) => panic!(
                    "Invalid game state: PositionStatus of (true, None) matched in fn validate"
                ),
                (false, None) => self.move_into(move_coord, curr_player),
                (false, _) => panic!(
                    "Invalid game state: PositionStatus of (false, Some(_)) matched in fn `validate`"
                ),
            },
            None => panic!("Invalid game state: `position` value that does not exist matched in fn `validate`")
        };
    }

    fn failed_poll(&mut self, cause: PollError) {
        match cause {
            // TODO: Integrate error type into ActionResult/Returned values to exported module?
            // PollError::AttackingSelf => ,
            // PollError::CantAttack => ,
            // PollError::InvalidPosition => ,
            // PollError::NotPlayersTurn => ,
            _ => self.state.set_state(Handle::Err, self.get_prev_trigger()),
        }
    }
        unimplemented!()
    }

    fn move_out(&mut self, xy: &Coord, curr_player: &Player) {
        self.unset_position(xy);
        self.set_switch(true);
        // TODO: set_actionresult again
    }

    fn move_into(&mut self, xy: &Coord, curr_player: &Player) {
        let prev_trig = self.get_prev_trigger();
        if (prev_trig == Trigger::None || prev_trig == Trigger::Placement)
            && self.player_pieces_set(curr_player) < 9
        {
            self.inc_player_pieces_set(curr_player);
        }

        let prev_mills = self.get_player_mills(curr_player);

        self.set_position(*xy, *curr_player);

        let updated_mills = self.get_player_mills(curr_player);
        // NOTE: New mill => ability to take an attack immediately after, w/o changing turns.
        // Current solution: compare previous mills to mills after setting piece.
        // If not the same AND <= old, then a new mill was not formed if my logic about gameboard
        // state is right.
        if self.is_switch()
            && prev_mills != updated_mills
            && updated_mills.len() <= prev_mills.len()
        {
            // && does not have a new mill ^^^ (??)
            self.set_switch(false);
        }
        // TODO: set_actionresult
    }

    fn move_attack(&mut self, xy: &Coord, curr_player: &Player) {
        unimplemented!()
    }

    fn update_mills(&mut self) {
        self.state.update_mills()
    }

    fn get_player_mills(&self, player: &Player) -> Vec<Mill> {
        self.state.player_mills(player)
    }

    fn has_mill(&self, attacker: &Player) -> bool {
        unimplemented!()
    }

    fn update_board(&mut self, player: &Player, pos: &Coord) {
        // TODO: Should I move the setter one level up into state instead of reaching all the
        // way into Board? I hate OOP! This shouldn't be a thing I need to even consider! Why
        // bundle all this state needlessly! Whatever! Figure out what is less horrificaly ugly!
        self.state.board.update(*pos, PositionStatus::from(*player))
    }

    pub fn get_curr_state(&self) -> (Handle, Trigger, Board) {
        (
            self.state.get_handle(),
            self.state.get_trigger(),
            self.state.get_board(),
        )
    }

    pub fn get_board(&self) -> Board {
        self.state.get_board()
    }

    fn get_position(&self, xy: &Coord) -> Option<&PositionStatus> {
        self.state.board.get(xy)
    }

    fn set_position(&mut self, xy: Coord, player: Player) -> Option<PositionStatus> {
        self.state.board.set(xy, player)
    }

    fn unset_position(&mut self, xy: &Coord) {
        self.state.board.unset(xy)
    }

    fn get_settings(&self) -> GameOpts {
        self.settings
    }

    fn get_prev_trigger(&self) -> Trigger {
        match self.history.last() {
            Some(t) => t.get_trigger(),
            None => Trigger::None,
        }
    }

    fn get_prev_player(&self) -> Option<Player> {
        match self.history.last() {
            Some(p) => Some(p.get_player()),
            None => None,
        }
    }

    fn player_pieces_set(&self, player: &Player) -> u32 {
        let (count, _) = match player {
            &Player::PlayerOne => self.state.p1_count,
            &Player::PlayerTwo => self.state.p2_count,
        };
        count
    }

    fn inc_player_pieces_set(&mut self, player: &Player) {
        match player {
            &Player::PlayerOne => self.state.p1_count.0 += 1,
            &Player::PlayerTwo => self.state.p2_count.0 += 1,
        };
    }

    fn dec_player_pieces_set(&mut self, player: &Player) {
        match player {
            &Player::PlayerOne => self.state.p1_count.0 -= 1,
            &Player::PlayerTwo => self.state.p2_count.0 -= 1,
        };
    }

    fn set_switch(&mut self, b: bool) {
        self.state.set_switch(b)
    }

    fn is_switch(&self) -> bool {
        self.state.switch_move
    }
}

#[cfg(test)]
mod base_tests {

    #[test]
    fn test_X_from_char() {
        use super::XCoord;

        assert_eq!(XCoord::from_char('A'), XCoord::A);
        assert_eq!(XCoord::from_char('G'), XCoord::G);
    }

    #[test]
    fn test_Y_from_char() {
        use super::YCoord;

        assert_eq!(YCoord::from_char('1'), YCoord::One);
        assert_eq!(YCoord::from_char('7'), YCoord::Seven);
    }

    #[test]
    fn test_coord_from_str() {
        use super::{Coord, XCoord, YCoord};

        assert_eq!(Coord::from_str("A1"), Coord::new(XCoord::A, YCoord::One));
        assert_eq!(Coord::from_str("G7"), Coord::new(XCoord::G, YCoord::Seven));
    }

    #[test]
    fn test_coord_as_str() {
        use super::{Coord, XCoord, YCoord};

        assert_eq!(
            Coord::new(XCoord::A, YCoord::One).as_string(),
            String::from("A1")
        );
        assert_eq!(
            Coord::new(XCoord::G, YCoord::Seven).as_string(),
            String::from("G7")
        );
    }

    #[test]
    fn test_board_len() {
        use super::Board;

        assert_eq!(Board::new().len(), 24u32);
    }

    #[test]
    fn test_manager_new_get_curr_state() {
        use super::{Agent, Board, GameOpts, Handle, Manager, Trigger};

        assert_eq!(
            Manager::new().get_curr_state(),
            (Handle::Ok, Trigger::None, Board::new())
        );
    }

    #[test]
    fn test_manager_new_opts() {
        use super::{Agent, Board, GameOpts, Handle, Manager, Trigger};

        let opts = GameOpts::new_game_opt(1, 2, Agent::Human);
        let mut mngr = Manager::new();
        mngr.new_opts(opts.clone());
        assert_eq!(mngr.get_settings(), opts)
    }

    #[test]
    fn test_manager_new_get_board() {
        use super::{Agent, Board, Manager};
        assert_eq!(Manager::new().get_board(), Board::new());
    }

    #[test]
    fn test_get_player_pieces() {
        use super::{Board, GameState};
        let gs = GameState::new();

        let ((p1, p1_pieces), (p2, p2_pieces)) = gs.get_player_pieces();
        assert_eq!(p1_pieces, 0);
        assert_eq!(p2_pieces, 0);
    }

    #[test]
    fn test_update_board() {
        use super::{Board, Coord, GameOpts, Manager, Player, PositionStatus};

        let mut mngr = Manager::new();
        mngr.set_position(Coord::from_str("A1"), Player::PlayerOne);
        assert_eq!(
            Some(&PositionStatus(true, Some(Player::PlayerOne))),
            mngr.get_position(&Coord::from_str("A1"))
        )
    }

    #[test]
    fn test_basic_validate() {
        use super::{Coord, GameOpts, GameState, Manager, Player, PositionStatus};

        let gs = GameOpts::new_piece_opt(1, Coord::from_str("A1"));
        let mut mngr = Manager::new();
        mngr.validate(
            gs.position.as_ref().expect("A1"),
            gs.sender.as_ref().expect("Player1"),
        );
        // TODO: Actual assertion
        assert_eq!(
            mngr.get_position(&Coord::from_str("A1")),
            Some(&PositionStatus::from(Player::PlayerOne))
        )
    }

    #[test]
    fn test_inc_player_piece() {
        use super::{GameState, Manager, Player};
        let mut mngr = Manager::new();
        mngr.inc_player_pieces_set(&Player::PlayerOne);
        mngr.inc_player_pieces_set(&Player::PlayerTwo);
        mngr.inc_player_pieces_set(&Player::PlayerTwo);

        assert_eq!(mngr.player_pieces_set(&Player::PlayerOne), 1);
        assert_eq!(mngr.player_pieces_set(&Player::PlayerTwo), 2)
    }

    #[test]
    fn test_unset_position() {
        use super::{Coord, GameState, Manager, Player, PositionStatus};

        let mut mgr = Manager::new();
        mgr.set_position(Coord::from_str("A1"), Player::PlayerOne);
        assert_eq!(
            mgr.get_position(&Coord::from_str("A1")),
            Some(&PositionStatus::from(Player::PlayerOne))
        );
        mgr.unset_position(&Coord::from_str("A1"));
        assert_eq!(
            mgr.get_position(&Coord::from_str("A1")),
            Some(&PositionStatus::new())
        )
    }

    #[test]
    fn test_incomplete_switch() {
        // TODO test where switch = true but it's a different player than previous turn.
    }

    #[test]
    fn test_adjacency_position_list_default() {
        use super::{AdjacentPositionList, Coord, Position, PositionList, PositionNode};
        use crate::util::get_adjacency_vec;

        let apl = AdjacentPositionList::default();

        let a1_list = apl.get(&Coord::from_str("A1")).unwrap();

        let g7_list = apl.get(&Coord::from_str("G7")).unwrap();

        assert_eq!(
            a1_list.head.as_ref().unwrap().as_ref().borrow().data,
            Coord::from_str("A1")
        );
        assert_eq!(
            g7_list.head.as_ref().unwrap().as_ref().borrow().data,
            Coord::from_str("G7")
        );
    }

    #[test]
    fn test_AdjPosList_PosList_iterators() {
        use super::{
            AdjacencyIterator, AdjacentPositionList, Coord, Position, PositionList, PositionNode,
        };

        let apl = AdjacentPositionList::default();

        let a1 = Coord::from_str("A1");
        for (xy, pos) in &apl {
            if *xy == a1 {
                assert_eq!(
                    pos.clone().into_iter().next().unwrap().data,
                    Coord::from_str("A1")
                )
            }
        }
    }
}
