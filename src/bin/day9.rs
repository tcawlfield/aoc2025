use aoc2025::get_input_string;

fn main() {
    env_logger::init();
    let input = get_input_string("input_d9.txt").unwrap();
    let tiles = input_as_tiles(&input);
    {
        let largest_area = largest_rect(&tiles);
        println!("Day 9 part 1: largest rect is {}", largest_area);
    }
    {
        // println!("Day 8 part 2: {}", product_final_x);
    }
}

struct Tile {
    col: i64,
    row: i64,
}

impl Tile {
    fn new(line: &str) -> Self {
        let cr: Vec<&str> = line.split(",").collect();
        if cr.len() != 2 {
            panic!("Bad line: {}", line);
        }
        let col = cr[0].parse().unwrap();
        let row = cr[1].parse().unwrap();
        Self { col, row }
    }
}

fn input_as_tiles(input: &str) -> Vec<Tile> {
    input.lines().map(|l| Tile::new(l)).collect()
}

fn rect_area(t1: &Tile, t2: &Tile) -> i64 {
    ((t2.row - t1.row).abs() + 1) * ((t2.col - t1.col).abs() + 1)
}

fn largest_rect(tiles: &[Tile]) -> i64 {
    let mut max_area = 0;
    for (idx1, t1) in tiles.iter().take(tiles.len() - 1).enumerate() {
        for t2 in tiles.iter().skip(idx1 + 1) {
            max_area = max_area.max(rect_area(t1, t2));
        }
    }
    max_area
}

#[cfg(test)]
mod tests {
    use super::*;

    fn log_init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    static INPUT: &str = r#"
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"#;

    #[test]
    fn test_pt1() {
        log_init();
        let tiles = input_as_tiles(INPUT.trim());
        let largest_area = largest_rect(&tiles);
        assert_eq!(largest_area, 50);
    }

    #[test]
    fn test_pt2() {
        log_init();
        // let mut wiring = Wiring::new(INPUT);
        // let product: i64 = wiring.connect_until_one();
        // assert_eq!(product, 25272);
    }
}
