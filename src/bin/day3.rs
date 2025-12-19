use aoc2025::get_input_string;

fn get_batteries(desc: &str) -> Vec<Vec<u8>> {
    let mut batteries = Vec::new();
    for line in desc.trim().split_whitespace() {
        let battery: Vec<u8> = line
            .chars()
            .filter_map(|ch| ch.to_digit(10).map(|n| n as u8))
            .collect();
        batteries.push(battery);
    }
    batteries
}

fn largest_cell(batt: &[u8]) -> Option<u8> {
    batt.iter().max().copied()
}

fn first_position(batt: &[u8], value: u8) -> Option<usize> {
    batt.iter().position(|v| *v == value)
}

fn best_joltage<const NUM_CELLS: usize>(batt: &[u8]) -> u64 {
    assert!(batt.len() > NUM_CELLS);
    // println!("Batt: {:?}", batt);
    let mut cells: [u8; NUM_CELLS] = [0; NUM_CELLS];
    let mut start = 0;
    for icell in 0..NUM_CELLS {
        let end = batt.len() - NUM_CELLS + icell + 1;
        let cell_val = largest_cell(&batt[start..end]).unwrap();
        // print!("  cell {} chose between {}..{}", icell, start, end);
        start = first_position(&batt[start..end], cell_val).unwrap() + start + 1;
        cells[icell] = cell_val;
        // println!(" and found {} at {}", cell_val, start - 1);
    }
    let mut jolts = 0;
    for &c in cells.iter() {
        jolts = jolts * 10 + c as u64;
    }
    jolts
}

fn main() {
    let input = get_input_string("input_d3.txt").unwrap();
    let bats = get_batteries(&input);
    {
        let jolts_sum: u64 = bats.iter().map(|b| best_joltage::<2>(b)).sum();
        println!("Day 3 pt1: ttl jolts = {}", jolts_sum);
    }
    {
        let jolts_sum: u64 = bats.iter().map(|b| best_joltage::<12>(b)).sum();
        println!("Day 3 pt2: ttl jolts = {}", jolts_sum);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"
987654321111111
811111111111119
234234234234278
818181911112111
        "#;

    #[test]
    fn test_get_batteries() {
        static SOME_BATTS: &str = r#"
1234
5678a
90
        "#;
        let bats = get_batteries(SOME_BATTS);
        assert_eq!(bats, vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 0]]);
    }

    #[test]
    fn test_pt1() {
        let bats = get_batteries(INPUT);
        let besties: Vec<u64> = bats.iter().map(|b| best_joltage::<2>(b)).collect();
        assert_eq!(besties, vec![98, 89, 78, 92]);
        let jolts_sum: u64 = besties.into_iter().sum();
        assert_eq!(jolts_sum, 357);
    }

    #[test]
    fn test_input_length() {
        let input = get_input_string("input_d3.txt").unwrap();
        let bats = get_batteries(&input);
        assert_eq!(bats.len(), 200);
        for bat in &bats {
            assert_eq!(bat.len(), 100);
        }
    }

    #[test]
    fn test_best_joltage() {
        assert_eq!(best_joltage::<2>(&vec![3, 3, 3, 9, 1]), 91);
        assert_eq!(best_joltage::<2>(&vec![3, 3, 3, 3, 9]), 39);
        assert_eq!(best_joltage::<2>(&vec![4, 3, 3, 3, 9]), 49);
        assert_eq!(best_joltage::<2>(&vec![8, 7, 6, 5, 9]), 89);
    }

    #[test]
    fn test_pt2() {
        let bats = get_batteries(INPUT);
        let besties: Vec<u64> = bats.iter().map(|b| best_joltage::<12>(b)).collect();
        assert_eq!(
            besties,
            vec![987654321111, 811111111119, 434234234278, 888911112111]
        );
        let jolts_sum: u64 = besties.into_iter().sum();
        assert_eq!(jolts_sum, 3121910778619);
    }
}
