use std::collections::HashMap;
use std::fmt;

use aoc2025::get_input_string;

fn main() {
    env_logger::init();
    let input = get_input_string("input_d8.txt").unwrap();
    {
        let mut wiring = Wiring::new(&input);
        wiring.connect_n_pairs(1000);
        let product = wiring.top_three_product();
        println!("Day 8 part 1: {}", product);
    }
    {
        let mut wiring = Wiring::new(&input);
        let product_final_x = wiring.connect_until_one();
        println!("Day 8 part 2: {}", product_final_x);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct JB {
    x: i64,
    y: i64,
    z: i64,
}

impl JB {
    fn from_str(line: &str) -> Self {
        let xyz: Vec<i64> = line.split(",").map(|s| s.parse::<i64>().unwrap()).collect();
        if xyz.len() != 3 {
            panic!("Cannot read {} as a coordinate", line);
        }
        Self {
            x: xyz[0],
            y: xyz[1],
            z: xyz[2],
        }
    }

    fn dist_to(&self, other: &Self) -> f64 {
        let dist_sq = ((self.x - other.x).pow(2)
            + (self.y - other.y).pow(2)
            + (self.z - other.z).pow(2)) as f64;
        dist_sq.sqrt()
    }
}

impl fmt::Display for JB {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

#[derive(Debug)]
struct JBPair {
    a: JB,
    b: JB,
    dist: f64,
}

fn all_pairs(jbs: &[JB]) -> Vec<JBPair> {
    let mut pairs = Vec::new();
    for (ia, a) in jbs.iter().take(jbs.len() - 1).enumerate() {
        for b in jbs.iter().skip(ia + 1) {
            pairs.push(JBPair {
                a: a.clone(),
                b: b.clone(),
                dist: a.dist_to(b),
            });
        }
    }
    pairs.sort_unstable_by(|p1, p2| p1.dist.partial_cmp(&p2.dist).unwrap());
    pairs
}

struct Wiring {
    jbs: Vec<JB>,
    cluster: HashMap<JB, usize>,
    next_cluster_id: usize,
}

impl Wiring {
    fn new(in_str: &str) -> Self {
        Self {
            jbs: in_str.trim().lines().map(|l| JB::from_str(l)).collect(),
            cluster: HashMap::new(),
            next_cluster_id: 0,
        }
    }

    fn connect_pair(&mut self, pair: &JBPair) {
        let clust_a = self.cluster.get(&pair.a).cloned();
        let clust_b = self.cluster.get(&pair.b).cloned();
        match (clust_a, clust_b) {
            (None, None) => {
                let id = self.next_cluster_id;
                log::debug!("{} and {} are in new {}", pair.a, pair.b, id);
                self.cluster.insert(pair.a.clone(), id);
                self.cluster.insert(pair.b.clone(), id);
                self.next_cluster_id += 1;
            }
            (Some(ca), None) => {
                log::debug!("{} joins {} in {}", pair.b, pair.a, ca);
                self.cluster.insert(pair.b.clone(), ca);
            }
            (None, Some(cb)) => {
                log::debug!("{} joins {} in {}", pair.a, pair.b, cb);
                self.cluster.insert(pair.a.clone(), cb);
            }
            (Some(ca), Some(cb)) => {
                // Make cb -> ca
                if ca == cb {
                    log::debug!("{} and {} are already in {}", pair.a, pair.b, ca);
                } else {
                    log::debug!("Cluster {} merges into {}", cb, ca);
                    for clust_id in self.cluster.values_mut() {
                        if *clust_id == cb {
                            *clust_id = ca;
                        }
                    }
                }
            }
        }
    }

    fn connect_n_pairs(&mut self, closest_n: usize) {
        let pairs = all_pairs(&self.jbs);
        for p in pairs.iter().take(closest_n) {
            self.connect_pair(p);
        }
    }

    fn all_cluster_memberships(&self) -> Vec<i64> {
        let mut membership: HashMap<usize, i64> = HashMap::new();
        log::info!("clusters:");
        for (k, v) in self.cluster.iter() {
            log::info!("  {} is in {}", k, v);
        }
        for &id in self.cluster.values() {
            membership
                .entry(id)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        log::debug!("membership: {:?}", membership);
        membership.values().copied().collect()
    }

    fn top_three_product(&self) -> i64 {
        let mut cluster_pops = self.all_cluster_memberships();
        cluster_pops.sort();
        cluster_pops.into_iter().rev().take(3).product()
    }

    fn connect_until_one(&mut self) -> i64 {
        // TODO: This method is really too slow!
        let pairs = all_pairs(&self.jbs);
        let mut final_pair_product_x = 0;
        for p in pairs.iter() {
            self.connect_pair(p);
            let membership_pops = self.all_cluster_memberships();
            log::info!("After pair {:?}, cluster_pops = {:?}", p, membership_pops);
            if membership_pops.len() == 1 && membership_pops[0] == self.jbs.len() as i64 {
                final_pair_product_x = p.a.x * p.b.x;
                break; // Success!
            }
        }
        final_pair_product_x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn log_init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    static INPUT: &str = r#"
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"#;

    #[test]
    fn test_pt1() {
        log_init();
        let mut wiring = Wiring::new(INPUT);
        wiring.connect_n_pairs(10);
        let product: i64 = wiring.top_three_product();
        assert_eq!(product, 40);
    }

    #[test]
    fn test_pt2() {
        log_init();
        let mut wiring = Wiring::new(INPUT);
        let product: i64 = wiring.connect_until_one();
        assert_eq!(product, 25272);
    }
}
