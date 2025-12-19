use aoc2025::get_input;
use bstr::io::BufReadExt as _;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

struct Dial {
    n: i64,
    zeros: u32,
}

const DIAL_SIZE: i64 = 100;

trait HasDial {
    fn rotate(&mut self, twist: i64);
    fn get_zeros(&self) -> u32;
    fn do_all(&mut self, turns: &[i64]) -> u32 {
        for &turn in turns {
            self.rotate(turn);
        }
        self.get_zeros()
    }
}

impl Dial {
    fn new() -> Self {
        Self { n: 50, zeros: 0 }
    }
}
impl HasDial for Dial {
    fn rotate(&mut self, twist: i64) {
        self.n += twist;
        if self.n % DIAL_SIZE == 0 {
            self.zeros += 1;
        }
    }

    fn get_zeros(&self) -> u32 {
        self.zeros
    }
}

struct DialPt2 {
    n: i64,
    zeros: u32,
}

impl DialPt2 {
    fn new() -> Self {
        Self { n: 50, zeros: 0 }
    }
}

impl HasDial for DialPt2 {
    fn rotate(&mut self, twist: i64) {
        if self.n * (self.n + twist) < 0 {
            // crossed zero
            self.zeros += 1;
        }
        self.n += twist;
        // Add full laps
        self.zeros += (self.n.abs() / DIAL_SIZE) as u32;
        if twist != 0 && self.n == 0 {
            // Landed at zero
            self.zeros += 1;
        }
        self.n = self.n % DIAL_SIZE;
    }

    fn get_zeros(&self) -> u32 {
        self.zeros
    }
}

fn get_instructions<R: Read>(rdr: &mut R) -> Vec<i64> {
    let lrdr = BufReader::new(rdr);
    let mut turns = Vec::new();
    for line in lrdr.byte_lines() {
        let line = line.unwrap();
        let turn = match &line[0..1] {
            b"R" => str::parse(str::from_utf8(&line[1..]).unwrap()).unwrap(),
            b"L" => -1 * str::parse::<i64>(str::from_utf8(&line[1..]).unwrap()).unwrap(),
            _ => panic!("Bad input line"),
        };
        turns.push(turn);
    }
    turns
}

fn main() {
    let input1 = get_input("input_d1.txt");
    let mut d = Dial::new();
    let mut instructions = File::open(&input1).unwrap();
    let turns = get_instructions(&mut instructions);
    let zeros = d.do_all(&turns);
    println!("Day 1 part 1: {}", zeros);

    let mut d2 = DialPt2::new();
    let zeros2 = d2.do_all(&turns);
    println!("Day 1 part 2: {}", zeros2);
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"
L68
L30
R48
L5
R60
L55
L1
L99
R14
L282
        "#;

    #[test]
    fn test_day_1_pt1() {
        let mut input_file = std::io::Cursor::new(INPUT.trim().to_owned());
        let turns = get_instructions(&mut input_file);
        let mut d = Dial::new();
        let zeros = d.do_all(&turns);
        assert_eq!(zeros, 3);
    }

    #[test]
    fn test_day_1_pt2() {
        let mut input_file = std::io::Cursor::new(INPUT.trim().to_owned());
        let turns = get_instructions(&mut input_file);
        let mut d = DialPt2::new();
        let zeros = d.do_all(&turns);
        assert_eq!(zeros, 8);
    }
}
