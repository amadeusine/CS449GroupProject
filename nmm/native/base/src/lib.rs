#![allow(non_snake_case)]
#![allow(dead_code)]

// ToString Needs to be in scope, do not believe the linter's lies.
use std::string::ToString;
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use strum_macros::Display;

mod util;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Display)]
pub enum Player {
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

type AdjacentPosition = Option<Rc<RefCell<PositionNode>>>;

#[derive(Debug, Clone, PartialEq)]
pub struct AdjacentPositionList(HashMap<Coord, PositionList>);

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct PositionNode {
    data: Coord,
    next: AdjacentPosition,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct PositionList {
    head: AdjacentPosition,
    tail: AdjacentPosition,
    pub length: u64,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Position {
    position: Coord,
    peers: Vec<AdjacentPosition>,
    occupied: Option<Player>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct Mill {
    Owner: Player,
    Pieces: (Position, Position, Position),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PositionStatus(bool, Option<Player>);

#[derive(Debug, Clone, PartialEq)]
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

// TODO: Make top level GameResult type that wraps GameState and custom GameErr types?
#[derive(Debug, Clone, PartialEq)]
pub struct GameState {
    handle: Handle,
    trigger: Trigger,
    board: Board,
    mills: Vec<Mill>,
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

#[derive(Debug, Clone, Copy, PartialEq)]
struct ActionResult {
    sender: Player,
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

impl PositionNode {
    fn new(xy: Coord, next: AdjacentPosition) -> Rc<RefCell<PositionNode>> {
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
    current: AdjacentPosition,
}
impl AdjacencyIterator {
    fn new(start: AdjacentPosition) -> AdjacencyIterator {
        AdjacencyIterator { current: start }
    }
}

impl Iterator for AdjacencyIterator {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        let curr = &self.current;
        let mut result = None;
        self.current = match curr {
            Some(ref curr) => {
                let curr = curr.borrow();
                result = Some(curr.data.clone());

                curr.next.clone()
            }
            _ => None,
        };
        result
    }
}

impl AdjacentPositionList {
    fn get(&self, xy: &Coord) -> Option<&PositionList> {
        match self.0.get(xy) {
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
    fn new(sender: Player, position: Coord, handle: Handle, trigger: Trigger) -> Self {
        ActionResult {
            sender: sender,
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
            mills: vec![],
            p1_count: (0, 9),
            p2_count: (0, 9),
            switch_move: false,
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

    fn get_player_pieces(&self) -> ((Player, u32), (Player, u32)) {
        let mut p1 = 0;
        let mut p2 = 0;
        // I don't know why this works but
        // for (_, pos) in &self.board.into_iter() doesn't. Should ask someone abou this.
        let board = &self.board;
        for (_, pos) in board.into_iter() {
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
        (Handle::Ok, Trigger::None, Board::default())
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
            unimplemented!()
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
        self.update_board(&curr_player, &move_coord);
    }

    fn failed_poll(&self) -> (Handle, Trigger, Board) {
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

        self.set_position(*xy, *curr_player);
        // TODO: Should we check for new mills here?
        // TODO: New mill => ability to take an attack immediately after, w/o changing turns.
        // TODO: set_actionresult
    }

    fn move_attack(&mut self, xy: &Coord, curr_player: &Player) {
        unimplemented!()
    }

    fn find_mills(&mut self) {
        unimplemented!()
    }

    fn has_mill(&mut self) -> bool {
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
        use super::{AdjacentPosition, AdjacentPositionList, Coord, PositionList, PositionNode};
        use crate::util::get_adjacency_vec;

        let apl = AdjacentPositionList::default();

        // let a1_list = apl.0.get(&Coord::from_str("A1")).unwrap();
        let a1_list = apl.get(&Coord::from_str("A1")).unwrap();

        // let g7_list = apl.get(&Coord::from_str("G7")).unwrap();

        assert_eq!(
            a1_list.head.as_ref().unwrap().as_ref().borrow().data,
            Coord::from_str("A1")
        );
        // assert_eq!(
        //     g7_list.head.as_ref().unwrap().as_ref().borrow().data,
        //     Coord::from_str("G7")
        // )
    }
}
