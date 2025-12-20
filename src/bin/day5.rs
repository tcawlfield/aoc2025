use std::{
    fs::File,
    io::{BufRead as _, BufReader, Read},
};

use aoc2025::get_input;

struct Stock {
    fresh_ranges: Vec<(u64, u64)>,
    available: Vec<u64>,
}

impl Stock {
    fn from_rdr<R: Read>(rdr: &mut R) -> Self {
        let mut fresh_ranges = Vec::new();
        let mut available = Vec::new();
        let lrdr = BufReader::new(rdr);
        let mut line_iter = lrdr.lines();

        for line in &mut line_iter {
            let line = line.unwrap();
            let range = line.trim();
            if range.len() == 0 {
                if fresh_ranges.len() > 0 {
                    break;
                } else {
                    continue;
                }
            }
            let first_last = range.split_once("-").unwrap();
            let (first, last) = (
                first_last.0.parse::<u64>().unwrap(),
                first_last.1.parse::<u64>().unwrap(),
            );
            fresh_ranges.push((first, last));
        }

        for line in &mut line_iter {
            let line = line.unwrap();
            let iid = line.trim();
            let ingredient_id = iid.parse::<u64>().unwrap();
            available.push(ingredient_id);
        }

        fresh_ranges.sort_by_key(|r| r.0);

        Self {
            fresh_ranges,
            available,
        }
    }

    fn count_fresh(&self) -> usize {
        let mut num_fresh = 0;
        for &id in self.available.iter() {
            let fresh = self.fresh_ranges.iter().any(|r| id >= r.0 && id <= r.1);
            if fresh {
                num_fresh += 1;
            }
        }
        num_fresh
    }

    fn combine_fresh_ranges(&mut self) {
        let mut combined = Vec::with_capacity(self.fresh_ranges.len());
        // This requires fresh_ranges to be sorted by starting ID, which it now is.
        combined.push(self.fresh_ranges[0]);
        for range in &self.fresh_ranges[1..] {
            let latest = combined.last_mut().unwrap();
            if range.0 <= latest.1 + 1 {
                if range.1 > latest.1 {
                    latest.1 = range.1;
                }
            } else {
                combined.push(*range);
            }
        }
        self.fresh_ranges = std::mem::take(&mut combined);
    }

    fn count_all_fresh(&self) -> u64 {
        self.fresh_ranges.iter().map(|r| r.1 - r.0 + 1).sum()
    }

    fn _debug_fresh_ranges(&self) {
        for r in &self.fresh_ranges {
            println!("{} - {}", r.0, r.1);
        }
    }
}

fn main() {
    let in_path = get_input("input_d5.txt");
    let mut in_file = File::open(&in_path).unwrap();
    let mut stock = Stock::from_rdr(&mut in_file);
    {
        let nf = stock.count_fresh();
        println!("Day 5 pt 1: There are {} fresh ingredients.", nf);
    }

    {
        stock.combine_fresh_ranges();
        let all_fresh = stock.count_all_fresh();
        println!("Day 5 pt 2: {} total possible fresh IDs", all_fresh);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"
3-5
10-14
16-20
12-18

1
5
8
11
17
32
        "#;

    fn get_test_stock() -> Stock {
        let mut input_file = std::io::Cursor::new(INPUT.trim().to_owned());
        Stock::from_rdr(&mut input_file)
    }

    #[test]
    fn test_pt1() {
        let stock = get_test_stock();
        assert_eq!(stock.count_fresh(), 3);
    }

    #[test]
    fn test_pt2() {
        let mut stock = get_test_stock();
        stock.combine_fresh_ranges();
        assert_eq!(stock.count_fresh(), 3); // Same as pt1
        assert_eq!(stock.fresh_ranges, vec![(3, 5), (10, 20)]);
        assert_eq!(stock.count_all_fresh(), 14);
    }

    #[test]
    fn test_combined() {
        let in_path = get_input("input_d5.txt");
        let mut in_file = File::open(&in_path).unwrap();
        let mut stock = Stock::from_rdr(&mut in_file);
        assert_eq!(stock.count_fresh(), 652);
        stock.combine_fresh_ranges();
        assert_eq!(stock.count_fresh(), 652);
    }
}
