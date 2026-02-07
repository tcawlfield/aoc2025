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
        let largest_area = largest_rect_inside(&tiles);
        println!("Day 9 part 2: largest inner rect is {}", largest_area);
    }
}

#[derive(Debug, PartialEq, Clone)]
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
    input.lines().map(Tile::new).collect()
}

fn rect_area(t1: &Tile, t2: &Tile) -> i64 {
    ((t2.row - t1.row).abs() + 1) * ((t2.col - t1.col).abs() + 1)
}

fn largest_rect(tiles: &[Tile]) -> i64 {
    let mut max_area = 0;
    for (idx1, t1) in tiles.iter().take(tiles.len() - 1).enumerate() {
        for t2 in tiles.iter().skip(idx1 + 2) {
            max_area = max_area.max(rect_area(t1, t2));
        }
    }
    max_area
}

fn largest_rect_inside(tiles: &[Tile]) -> i64 {
    let poly = RectyPoly::new(tiles);
    let mut max_area = 0;
    for (idx1, t1) in tiles.iter().take(tiles.len() - 1).enumerate() {
        for t2 in tiles.iter().skip(idx1 + 2) {
            if t1.row == t2.row || t1.col == t2.col {
                break;
            }
            if poly.rect_is_in(t1, t2) {
                log::debug!("Is within polygon: {:?}:{:?}", t1, t2);
                max_area = max_area.max(rect_area(t1, t2));
            }
        }
    }
    max_area
}

#[derive(Debug, Clone, Copy)]
#[repr(usize)]
enum Dir {
    N,
    E,
    S,
    W,
}

const NUM_DIRS: usize = 4; // std::mem::variant_count::<Dir>() is unstable
static CW_TURNS: [[i32; NUM_DIRS]; NUM_DIRS] =
    [[0, 1, 2, -1], [-1, 0, 1, 2], [2, -1, 0, 1], [1, 2, -1, 0]];

fn right_turns(e1: &Edge, e2: &Edge) -> i32 {
    CW_TURNS[e1.dir as usize][e2.dir as usize]
}

#[derive(Debug, Clone)]
struct Edge {
    p1: Tile,
    #[allow(dead_code)]
    p2: Tile,
    dir: Dir,
    /// common coordinate
    c_coord: i64,
    min_coord: i64,
    max_coord: i64,
}

struct RectyPoly {
    #[allow(dead_code)]
    edges: Vec<Edge>,
    horizontal_edges: Vec<Edge>,
    vertical_edges: Vec<Edge>,
}

impl RectyPoly {
    fn new(tiles: &[Tile]) -> Self {
        let edges: Vec<Edge> = Edge::from_tiles(tiles);
        let mut horizontal_edges: Vec<Edge> = edges
            .iter()
            .filter(|e| !e.dir.is_vertical())
            .cloned()
            .collect();
        let mut vertical_edges: Vec<Edge> = edges
            .iter()
            .filter(|e| e.dir.is_vertical())
            .cloned()
            .collect();
        horizontal_edges.sort_by_key(|e| e.p1.col);
        vertical_edges.sort_by_key(|e| e.p1.row);
        Self {
            edges,
            horizontal_edges,
            vertical_edges,
        }
    }

    #[allow(dead_code)]
    fn intersects_any(&self, edge: &Edge) -> bool {
        let perp_edges = if edge.dir.is_vertical() {
            &self.horizontal_edges
        } else {
            &self.vertical_edges
        };
        for pe in perp_edges {
            if pe.min_coord > edge.c_coord {
                break;
            }
            if edge.does_cross(pe) {
                return true;
            }
        }
        false
    }

    fn is_inside(&self, tile: &Tile) -> bool {
        let mut wn = 0;
        for ve in &self.vertical_edges {
            if tile.row <= ve.min_coord {
                break;
            }
            if tile.col <= ve.c_coord {
                // A ray extending from tile is at or to the left of edge ve
                if tile.row <= ve.max_coord {
                    wn += match ve.dir {
                        Dir::N => 1,
                        Dir::S => -1,
                        _ => 0,
                    };
                }
            }
        }
        log::debug!(" point {:?} has winding number {}", tile, wn);
        wn != 0
    }

    fn is_on(&self, tile: &Tile) -> bool {
        for ve in &self.vertical_edges {
            if tile.row < ve.min_coord {
                break;
            }
            if (tile.col == ve.c_coord) && (tile.row <= ve.max_coord) {
                return true; // on edge
            }
        }
        for he in &self.horizontal_edges {
            if tile.col < he.min_coord {
                break;
            }
            if (tile.row == he.c_coord) && (tile.col <= he.max_coord) {
                return true; // on edge
            }
        }
        false
    }

    fn rect_is_in(&self, p1: &Tile, p2: &Tile) -> bool {
        let p1a = Tile {
            row: p1.row,
            col: p2.col,
        };
        let p2a = Tile {
            row: p2.row,
            col: p1.col,
        };
        let corners: [Tile; 4] = [p1.clone(), p1a, p2.clone(), p2a];
        // TODO: Arrange corners to be CW
        if corners.iter().any(|p| !self.is_on(p) && !self.is_inside(p)) {
            return false;
        }
        let edges = Edge::from_tiles(&corners);
        !edges.iter().any(|e| self.intersects_any(e))
    }
}

impl Dir {
    fn a_to_b(a: &Tile, b: &Tile) -> Option<Dir> {
        if a == b {
            return None;
        }
        if a.col == b.col {
            if b.row > a.row {
                Some(Dir::S) // Like image coords, (0, 0) is upper-left
            } else {
                Some(Dir::N)
            }
        } else if a.row == b.row {
            if b.col > a.col {
                Some(Dir::E)
            } else {
                Some(Dir::W)
            }
        } else {
            None
        }
    }

    fn is_vertical(&self) -> bool {
        match self {
            Dir::N => true,
            Dir::E => false,
            Dir::S => true,
            Dir::W => false,
        }
    }
}

impl Edge {
    /// Gets the direction a->b, then orients such that
    /// p1 is always < p2
    fn new(a: &Tile, b: &Tile) -> Self {
        let dir = Dir::a_to_b(a, b).unwrap();
        let (c, o1, o2) = if dir.is_vertical() {
            (a.col, a.row, b.row)
        } else {
            (a.row, a.col, b.col)
        };
        Self {
            p1: a.clone(),
            p2: b.clone(),
            dir,
            c_coord: c,
            min_coord: o1.min(o2),
            max_coord: o1.max(o2),
        }
    }

    fn from_tiles(tiles: &[Tile]) -> Vec<Edge> {
        tiles
            .iter()
            .zip(tiles.iter().cycle().skip(1))
            .map(|(a, b)| Edge::new(a, b))
            .collect()
    }

    fn is_perp_to(&self, other: &Edge) -> bool {
        self.dir.is_vertical() ^ other.dir.is_vertical()
    }

    fn does_cross(&self, other: &Edge) -> bool {
        if !self.is_perp_to(other) {
            return false;
        }
        let (cr1, cr2) = (
            (self.c_coord, other.min_coord, other.max_coord),
            (other.c_coord, self.min_coord, self.max_coord),
        );
        let crosses_excl = |(coord, min, max)| coord > min && coord < max;
        // let crosses_incl = |(coord, min, max)| coord >= min && coord <= max;
        crosses_excl(cr1) && crosses_excl(cr2)
    }
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

    fn get_turns(poly: &RectyPoly) -> Vec<i32> {
        poly.edges
            .iter()
            .zip(poly.edges.iter().cycle().skip(1))
            .map(|(e1, e2)| right_turns(e1, e2))
            .collect()
    }

    #[test]
    fn test_poly() {
        let tiles = input_as_tiles(INPUT.trim());
        let poly = RectyPoly::new(&tiles);
        let turns = get_turns(&poly);
        for &turn in turns.iter() {
            assert!(turn.abs() == 1); // All turns are left or right
        }
        let turns_ttl: i32 = turns.iter().sum();
        assert_eq!(turns_ttl, 4); // turns == 988

        assert!(poly.is_inside(&Tile::new("8,2")));
    }

    #[test]
    fn test_pt2_assumptions() {
        let input = get_input_string("input_d9.txt").unwrap();
        let tiles = input_as_tiles(&input);
        let poly = RectyPoly::new(&tiles);
        let turns: Vec<i32> = get_turns(&poly);
        for &turn in turns.iter() {
            assert!(turn.abs() == 1); // All turns are left or right
        }
        let turns_ttl: i32 = turns.iter().sum();
        assert_eq!(turns_ttl, 4); // turns == 988

        for e in &poly.edges {
            assert!(!poly.intersects_any(e));
        }

        let min_edge_length = poly
            .edges
            .iter()
            .map(|e| e.max_coord - e.min_coord + 1)
            .min()
            .unwrap();
        assert_eq!(min_edge_length, 5);
    }

    fn edge_len(e: &Edge) -> i64 {
        e.max_coord - e.min_coord + 1
    }

    #[test]
    fn test_pt2() {
        log_init();
        let tiles = input_as_tiles(INPUT.trim());
        let largest_area = largest_rect_inside(&tiles);
        assert_eq!(largest_area, 24);
    }

    #[test]
    fn test_inner_detour() {
        // Adding extra loop
        static LOOPY_INPUT: &str = r#"
7,1
11,1
11,7
9,7
9,5
6,5
6,4
4,4
4,5
2,5
2,3
7,3"#;
        let tiles = input_as_tiles(LOOPY_INPUT.trim());
        // let largest_area = largest_rect_inside(&tiles);
        // assert_eq!(largest_area, 21);
        let poly = RectyPoly::new(&tiles);
        assert!(!poly.rect_is_in(&tiles[4], &tiles[10]));
    }
}
