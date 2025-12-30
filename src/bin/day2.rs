use anyhow::{Context as _, Result, anyhow};
use aoc2025::get_input_string;
use ilog::IntLog as _;
use regex::Regex;
use std::sync::LazyLock;

static WHITESPACE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\s+").unwrap());

#[derive(Debug)]
struct Range {
    first: u64,
    last: u64,
}

impl Range {
    fn from_expr(rexpr: &str) -> Result<Range> {
        let (sfirst, slast) = rexpr
            .split_once('-')
            .context(anyhow!("Bad range {}", rexpr))?;
        let first: u64 = sfirst.parse()?;
        let last: u64 = slast.parse()?;
        Ok(Range { first, last })
    }
}

fn input_to_ranges(inputstr: &str) -> Result<Vec<Range>> {
    let mut ranges = Vec::new();
    let compacted = WHITESPACE.replace_all(inputstr, "");
    let range_exprs = compacted.split(',');
    for rexpr in range_exprs {
        ranges.push(Range::from_expr(rexpr)?)
    }
    Ok(ranges)
}

fn num_digits_of(val: u64) -> u32 {
    val.log10() as u32 + 1
}

fn make_number(repeated: u64, num_rpt_digits: u32, num_repeats: u32) -> u64 {
    let mut new_num = repeated;
    for _ in 0..num_repeats - 1 {
        new_num = new_num * 10u64.pow(num_rpt_digits) + repeated;
    }
    new_num
}

fn next_invalid_id(val: u64) -> u64 {
    let digits = num_digits_of(val);
    if digits % 2 == 1 {
        // Odd number
        let next_bigger_left_dig = digits.div_ceil(2);
        let pattern = 10u64.pow(next_bigger_left_dig - 1);
        return 10u64.pow(next_bigger_left_dig * 2 - 1) + pattern;
    }
    let mut pat_digs = digits / 2;
    let pat_scale = 10u64.pow(pat_digs);
    let mut pat = val / pat_scale;
    let mut next_val = make_number(pat, pat_digs, 2);
    if next_val <= val {
        pat += 1;
        pat_digs = num_digits_of(pat); // Just in case pat went from, say, 99 to 100.
        next_val = make_number(pat, pat_digs, 2);
    }
    next_val
}

fn sum_invalid(ranges: &[Range], pt2: bool) -> u64 {
    let mut sum = 0;
    for r in ranges {
        let mut iid = r.first - 1;
        loop {
            iid = if pt2 {
                next_invalid_id_pt2(iid)
            } else {
                next_invalid_id(iid)
            };
            if iid <= r.last {
                sum += iid;
            } else {
                break;
            }
        }
    }
    sum
}

// ----- Part 2
// Strategy: Any number < 10 has a next invalid number of 11.
// Find all two-number factorizations of the number of digits (num_rpt_digits, num_reps), plus (1, N).
// For each of these factorizations, find the smallest of the next-invalid numbers.
//   Call next-invalid up to twice per factorization: Try with original patn, keep that if it's > the original. Else add 1.
//   Edge case: if all 9's, there well be no next-invalid because in each factorization of digits, the pattern will be all 9's.

fn next_potential_invalid(val: u64, num_digits: u32, num_rpt_digits: u32) -> Option<u64> {
    let num_repeats = num_digits / num_rpt_digits;
    let baseval = val / 10u64.pow(num_rpt_digits * (num_repeats - 1));
    assert_eq!(num_digits_of(baseval), num_rpt_digits); // Cheap enough, may as well
    let first_try = make_number(baseval, num_rpt_digits, num_repeats);
    if first_try > val {
        return Some(first_try);
    }
    let next_base = baseval + 1;
    if num_digits_of(next_base) == num_rpt_digits {
        Some(make_number(next_base, num_rpt_digits, num_repeats))
    } else {
        None
    }
}

/// Returns a list of num_rpt_digits
fn get_factorizations(num_digits: u32) -> Vec<u32> {
    let factors = prime_factorization::Factorization::run(num_digits).factors;
    // BTreeSet is nice here because reproducible ordering helps unit testing.
    let mut num_rpt_digits_set = std::collections::BTreeSet::new();
    for some_factors in 1..factors.len() {
        for factor_list in gen_combinations::CombinationIterator::new(&factors, some_factors) {
            num_rpt_digits_set.insert(factor_list.iter().map(|i| **i).product::<u32>());
        }
    }
    let mut factorizations = Vec::new();
    factorizations.reserve(num_rpt_digits_set.len() + 1);
    factorizations.push(1);
    for rpt_digits in num_rpt_digits_set {
        factorizations.push(rpt_digits);
    }
    factorizations
}

fn next_invalid_id_pt2(val: u64) -> u64 {
    if val < 10 {
        return 11;
    }
    let num_digits = num_digits_of(val);
    let potentials: Vec<u64> = get_factorizations(num_digits)
        .into_iter()
        .filter_map(|num_rpt_digits| next_potential_invalid(val, num_digits, num_rpt_digits))
        .collect();
    potentials.into_iter().min().unwrap_or_else(|| {
        // val must have been 9999...9. Get smallest value with the next-highest number of digits
        let factorizations = get_factorizations(num_digits + 1);
        // These are ordered from lowest (always 1) to highest. Pick the highest.
        let num_rpt_digits = factorizations[factorizations.len() - 1];
        let pat = 10u64.pow(num_rpt_digits - 1);
        make_number(pat, num_rpt_digits, (num_digits + 1) / num_rpt_digits)
    })
}

fn main() -> Result<()> {
    let ranges_str = get_input_string("input_d2.txt")?;
    let ranges = input_to_ranges(&ranges_str)?;
    let sum = sum_invalid(&ranges, false);
    println!("Pt 1: IID sum = {}", sum);

    let sum2 = sum_invalid(&ranges, true);
    println!("Pt 2: IID sum = {}", sum2);
    Ok(())
}

// ----- TESTS -----

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124
        "#;

    #[test]
    fn test_num_digits_of() {
        assert_eq!(num_digits_of(99), 2);
        assert_eq!(num_digits_of(100), 3);
    }

    #[test]
    fn test_next_inval_id() {
        assert_eq!(next_invalid_id(333), 1010);
        assert_eq!(next_invalid_id(3300), 3333);
        assert_eq!(next_invalid_id(3333), 3434);
        assert_eq!(next_invalid_id(9999), 100100);
    }

    #[test]
    fn test_pt1() {
        let ranges = input_to_ranges(INPUT).unwrap();
        assert_eq!(ranges.len(), 11);
        let sum = sum_invalid(&ranges, false);
        assert_eq!(sum, 1227775554);
    }

    #[test]
    fn test_get_factorizations() {
        assert_eq!(get_factorizations(12), vec![1, 2, 3, 4, 6]);
        assert_eq!(get_factorizations(2), vec![1]);
    }

    #[test]
    fn test_all_nines_pt2() {
        assert_eq!(next_invalid_id_pt2(9999999), 10001000);
    }

    #[test]
    fn test_pt2() {
        let ranges = input_to_ranges(INPUT).unwrap();
        assert_eq!(ranges.len(), 11);
        let sum = sum_invalid(&ranges, true);
        assert_eq!(sum, 4174379265);
    }
}
