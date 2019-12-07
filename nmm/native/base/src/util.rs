use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

use crate::Coord;

pub fn get_adjacency_vec(game_layout: &str) -> Vec<Vec<Coord>> {
    match read_position_file(game_layout) {
        Ok(vec_str) => read_positions(vec_str),
        Err(e) => panic!("Error while reading position file: {:#?}", e),
    }
}

fn read_positions(lines: Vec<String>) -> Vec<Vec<Coord>> {
    let mut adjacent: Vec<Coord> = vec![];
    let mut adjList: Vec<Vec<Coord>> = vec![];
    for line in lines {
        for xy in line.split(" ") {
            adjacent.push(Coord::from_str(xy))
        }
        adjList.push(adjacent.clone());
        adjacent.clear();
    }
    adjList
}

fn read_position_file(fname: &str) -> io::Result<Vec<String>> {
    let f = File::open(fname)?;
    let reader = BufReader::new(f);
    let mut lines: Vec<String> = vec![];

    for line in reader.lines() {
        lines.push(line?);
    }
    Ok(lines)
}

#[cfg(test)]
mod util_tests {
    #[test]
    fn test_read_position_file() {
        use crate::util::read_position_file;
        let str = read_position_file("default.txt").unwrap();
        assert_eq!(
            str,
            vec![
                "A1 A4 D1",
                "A4 A1 A7 B4",
                "A7 A4 D7",
                "B2 B4 D2",
                "B4 A4 C4 B6 B2",
                "B6 B4 D6",
                "C3 C4 D3",
                "C4 C3 B4 C5",
                "C5 C4 D5",
                "D1 A1 G1 D2",
                "D2 B2 F2 D3 D1",
                "D3 C3 E3 D2",
                "D5 C4 E5 D6",
                "D6 B6 F6 D7 D5",
                "D7 A7 G7 D6",
                "E3 D3 E4",
                "E4 F4 E5 E3",
                "E5 D5 E4",
                "F2 D2 F3",
                "F4 E4 G4 F6 F2",
                "F6 D6 F4",
                "G1 D1 G4",
                "G4 F4 G7 G1",
                "G7 D7 G4",
            ]
        )
    }

    #[test]
    fn test_read_position() {
        use crate::util::read_position_file;
        use crate::util::read_positions;
        use crate::Coord;
        use crate::XCoord::{A, B, C, D, E, F, G};
        use crate::YCoord::{Five, Four, One, Seven, Six, Three, Two};
        let str = read_position_file("default.txt").unwrap();
        let vec = read_positions(str);

        assert_eq!(
            vec,
            vec![
                vec![Coord(A, One), Coord(A, Four), Coord(D, One)],
                vec![
                    Coord(A, Four),
                    Coord(A, One),
                    Coord(A, Seven),
                    Coord(B, Four)
                ],
                vec![Coord(A, Seven), Coord(A, Four), Coord(D, Seven)],
                vec![Coord(B, Two), Coord(B, Four), Coord(D, Two)],
                vec![
                    Coord(B, Four),
                    Coord(A, Four),
                    Coord(C, Four),
                    Coord(B, Six),
                    Coord(B, Two)
                ],
                vec![Coord(B, Six), Coord(B, Four), Coord(D, Six)],
                vec![Coord(C, Three), Coord(C, Four), Coord(D, Three)],
                vec![
                    Coord(C, Four),
                    Coord(C, Three),
                    Coord(B, Four),
                    Coord(C, Five)
                ],
                vec![Coord(C, Five), Coord(C, Four), Coord(D, Five)],
                vec![Coord(D, One), Coord(A, One), Coord(G, One), Coord(D, Two)],
                vec![
                    Coord(D, Two),
                    Coord(B, Two),
                    Coord(F, Two),
                    Coord(D, Three),
                    Coord(D, One)
                ],
                vec![
                    Coord(D, Three),
                    Coord(C, Three),
                    Coord(E, Three),
                    Coord(D, Two)
                ],
                vec![
                    Coord(D, Five),
                    Coord(C, Four),
                    Coord(E, Five),
                    Coord(D, Six)
                ],
                vec![
                    Coord(D, Six),
                    Coord(B, Six),
                    Coord(F, Six),
                    Coord(D, Seven),
                    Coord(D, Five)
                ],
                vec![
                    Coord(D, Seven),
                    Coord(A, Seven),
                    Coord(G, Seven),
                    Coord(D, Six)
                ],
                vec![Coord(E, Three), Coord(D, Three), Coord(E, Four)],
                vec![
                    Coord(E, Four),
                    Coord(F, Four),
                    Coord(E, Five),
                    Coord(E, Three)
                ],
                vec![Coord(E, Five), Coord(D, Five), Coord(E, Four)],
                vec![Coord(F, Two), Coord(D, Two), Coord(F, Three)],
                vec![
                    Coord(F, Four),
                    Coord(E, Four),
                    Coord(G, Four),
                    Coord(F, Six),
                    Coord(F, Two)
                ],
                vec![Coord(F, Six), Coord(D, Six), Coord(F, Four)],
                vec![Coord(G, One), Coord(D, One), Coord(G, Four)],
                vec![
                    Coord(G, Four),
                    Coord(F, Four),
                    Coord(G, Seven),
                    Coord(G, One)
                ],
                vec![Coord(G, Seven), Coord(D, Seven), Coord(G, Four)]
            ]
        )
    }
}
