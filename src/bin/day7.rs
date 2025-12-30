use aoc2025::get_input;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

const BLANK: char = '.';
const SPLITTER: char = '^';
const START: char = 'S';

struct Manifold {
    beams: BTreeMap<usize, usize>,
    splits: usize,
    width: usize,
}

impl Manifold {
    fn new() -> Self {
        Self {
            beams: BTreeMap::new(),
            splits: 0,
            width: 0,
        }
    }

    fn first_line(&mut self, line: &str) {
        let tline = line.trim();
        if tline == "" {
            return;
        }
        self.width = 0;
        for ch in tline.chars() {
            if ch == START {
                self.beams.insert(self.width, 1);
            }
            self.width += 1;
        }
    }

    fn propagate(&mut self, line: &str) {
        for (col, ch) in line.trim().chars().enumerate() {
            match ch {
                SPLITTER => {
                    if let Some(cur) = self.beams.remove(&col) {
                        if col > 0 {
                            self.add_beam(col - 1, cur);
                        }
                        if col + 1 < self.width {
                            self.add_beam(col + 1, cur);
                        }
                        self.splits += 1;
                    }
                }
                BLANK => (),
                _ => {
                    println!("Unrecognized line element {}", ch);
                }
            }
        }
    }

    fn add_beam(&mut self, col: usize, timelines: usize) {
        self.beams
            .entry(col)
            .and_modify(|curr| *curr += timelines)
            .or_insert(timelines);
    }

    fn count_timelines(&self) -> usize {
        self.beams.values().sum()
    }
}

fn process_manifold<R: Read>(m: &mut Manifold, rdr: &mut R) {
    let lrdr = BufReader::new(rdr);
    let mut lines = lrdr.lines();
    let first_line = lines.next().unwrap().unwrap();
    m.first_line(&first_line);
    for line in lines {
        let line = line.unwrap();
        m.propagate(&line);
    }
}

fn main() {
    let input_fn = get_input("input_d7.txt");
    let mut input = File::open(&input_fn).unwrap();
    {
        let mut m = Manifold::new();
        process_manifold(&mut m, &mut input);
        println!("Day 7 pt 1: Found {} splits", m.splits);
    }

    {
        input.seek(std::io::SeekFrom::Start(0)).unwrap();
        let mut m = Manifold::new();
        process_manifold(&mut m, &mut input);
        let timelines = m.count_timelines();
        println!("Day 7 pt2: {} timelines created", timelines);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = r#"
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."#;

    #[test]
    fn test_pt1() {
        let mut input = std::io::Cursor::new(INPUT.trim().to_owned());
        let mut m = Manifold::new();
        process_manifold(&mut m, &mut input);
        assert_eq!(m.splits, 21);
    }

    #[test]
    fn test_pt2() {
        let mut input = std::io::Cursor::new(INPUT.trim().to_owned());
        let mut m = Manifold::new();
        process_manifold(&mut m, &mut input);
        assert_eq!(m.count_timelines(), 40);
    }
}
