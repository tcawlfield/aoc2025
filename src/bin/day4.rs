use std::{
    fs::File,
    io::{BufRead as _, BufReader, Read},
};

use aoc2025::get_input;
use ndarray::{Array, Array1, Array2, s};

fn read_map<R: Read>(rdr: &mut R) -> Array2<u8> {
    let mut rows: Vec<Vec<u8>> = Vec::new();
    let lrdr = BufReader::new(rdr);
    for line in lrdr.lines() {
        let line = line.unwrap();
        let sline = line.trim();
        let row: Vec<u8> = sline
            .chars()
            .filter_map(|ch| match ch {
                '.' => Some(0),
                '@' => Some(1),
                _ => None,
            })
            .collect();
        if rows.len() > 0 && row.len() != rows[0].len() {
            println!(
                "Error: Line {} has a different length than the first.",
                sline
            );
        } else {
            rows.push(row);
        }
    }
    // Embed this input into an array with an extra border of 1
    let width = rows[0].len() + 2;
    let height = rows.len() + 2;
    let mut map = Array::zeros((height, width));
    for (i, row) in rows.into_iter().enumerate() {
        map.slice_mut(s![i + 1, 1..width - 1])
            .assign(&Array1::from(row));
    }
    map
}

fn count_neighbors(map: &Array2<u8>, row: usize, col: usize) -> u8 {
    let region = map.slice(s![row - 1..row + 2, col - 1..col + 2]);
    if region.shape()[0] != 3 || region.shape()[1] != 3 {
        println!("Error: region shape is {:?}", region.shape());
        panic!("Bad index into map");
    }
    let ttl_count: u8 = region.iter().sum();
    ttl_count - region[[1, 1]]
}

fn num_accessable_bales(map: &Array2<u8>) -> usize {
    let mut count = 0;
    let rows = map.shape()[0];
    let cols = map.shape()[1];
    for irow in 1..rows - 1 {
        for jcol in 1..cols - 1 {
            if map[[irow, jcol]] == 1 && count_neighbors(&map, irow, jcol) < 4 {
                count += 1;
            }
        }
    }
    count
}

fn remove_and_count(map: &mut Array2<u8>) -> usize {
    let mut count = 0;
    let rows = map.shape()[0];
    let cols = map.shape()[1];
    for irow in 1..rows - 1 {
        for jcol in 1..cols - 1 {
            if map[[irow, jcol]] == 1 && count_neighbors(&map, irow, jcol) < 4 {
                count += 1;
                map[[irow, jcol]] = 0;
            }
        }
    }
    count
}

fn remove_all_possible(map: &mut Array2<u8>) -> usize {
    let mut count = 0;
    loop {
        let removed = remove_and_count(map);
        count += removed;
        if removed == 0 {
            break;
        }
    }
    count
}

fn main() {
    let in_path = get_input("input_d4.txt");
    let mut in_file = File::open(&in_path).unwrap();
    let mut map = read_map(&mut in_file);
    {
        let num = num_accessable_bales(&map);
        println!("Day 4 pt 1: There are {} accessable bales.", num);
    }

    {
        let removed = remove_all_possible(&mut map);
        println!("Day 4 pt 2: {} can be removed.", removed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
..........
        "#;

    #[test]
    fn test_read_example_input() {
        let mut input_file = std::io::Cursor::new(INPUT.trim().to_owned());
        let map = read_map(&mut input_file);
        assert_eq!(map.shape(), vec![13, 12]);
        assert_eq!(count_neighbors(&map, 1, 3), 3);
        assert_eq!(num_accessable_bales(&map), 13);
    }

    #[test]
    fn test_pt_2() {
        let mut input_file = std::io::Cursor::new(INPUT.trim().to_owned());
        let mut map = read_map(&mut input_file);
        let possible = remove_all_possible(&mut map);
        assert_eq!(possible, 43);
    }
}
