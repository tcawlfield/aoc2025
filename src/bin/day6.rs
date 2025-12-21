use aoc2025::get_input_string;

fn main() {
    let input_str = get_input_string("input_d6.txt").unwrap();
    {
        let homework = Homework::new_from(&input_str);
        let answers = homework.compute_all();
        let grand_total: i64 = answers.iter().sum();
        println!("Day 6 Part 1: grand total: {}", grand_total);
    }

    {
        let homework = Homework::from_cephalopod(&input_str);
        let answers = homework.compute_all();
        let grand_total: i64 = answers.iter().sum();
        println!("Day 6 Part 2: grand total: {}", grand_total);
    }
}

struct Homework {
    cols: Vec<Vec<i64>>,
    opers: Vec<Oper>,
}

#[derive(Debug)]
enum Oper {
    Plus,
    Mult,
}

impl Oper {
    fn from_str(word: &str) -> Option<Self> {
        match word {
            "+" => Some(Self::Plus),
            "*" => Some(Self::Mult),
            _ => None,
        }
    }

    fn operate_on(&self, numbers: &[i64]) -> i64 {
        match self {
            Self::Plus => numbers.iter().sum::<i64>(),
            Self::Mult => numbers.iter().product::<i64>(),
        }
    }
}

impl Homework {
    fn new_from(input_str: &str) -> Self {
        let mut rows = Vec::new();
        let mut opers = Vec::new();
        let tinput = input_str.trim();
        let num_lines = tinput.lines().count();
        for (row, line) in tinput.lines().enumerate() {
            let words = line.split_whitespace();
            if row < num_lines - 1 {
                let nums: Vec<i64> = words.map(|w| w.parse::<i64>().unwrap()).collect();
                rows.push(nums);
            } else {
                opers.extend(words.map(|w| Oper::from_str(w).unwrap()));
            }
        }

        // Transpose rows into cols
        let mut cols = Vec::with_capacity(opers.len());
        let mut col: Vec<i64> = Vec::new();
        let mut row_iters: Vec<_> = rows.iter().map(|r| r.iter()).collect();
        loop {
            col.reserve(num_lines - 1);
            for riter in row_iters.iter_mut() {
                if let Some(val) = riter.next() {
                    col.push(*val);
                }
            }
            if col.len() > 0 {
                cols.push(std::mem::take(&mut col));
            } else {
                break;
            }
        }
        Self { cols, opers }
    }

    fn from_cephalopod(input_str: &str) -> Self {
        let mut cols = Vec::new();
        let tinput = input_str.trim_matches(&['\r', '\n']); // Should handle Unix & DOS
        let num_lines = tinput.lines().count();
        println!("tinput: {} ({} lines)", tinput, num_lines);
        let mut row_iter = tinput.lines();
        let row_strs: Vec<Vec<char>> = (&mut row_iter)
            .take(num_lines - 1)
            .map(|l| l.chars().collect())
            .collect();
        for rs in row_strs.iter() {
            println!("Row: {:?}", rs);
        }
        let mut opers: Vec<Oper> = row_iter
            .next()
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|w| Oper::from_str(w).unwrap())
            .collect();
        opers.reverse();
        println!("Opers: {:?}", opers);
        let mut row_nums = Vec::new();
        let mut row_chars: Vec<_> = row_strs.iter().map(|r| r.iter().rev()).collect();
        row_nums.clear();
        loop {
            let col: String = row_chars
                .iter_mut()
                .map(|it| it.next())
                .filter_map(|c| c.cloned())
                .collect();
            if col.len() == 0 {
                if row_nums.len() > 0 {
                    cols.push(std::mem::take(&mut row_nums));
                }
                break;
            }
            let tcol = col.trim();
            if tcol.len() == 0 {
                cols.push(std::mem::take(&mut row_nums));
            } else {
                row_nums.push(tcol.parse::<i64>().unwrap());
            }
        }
        Self { cols, opers }
    }

    fn compute_all(&self) -> Vec<i64> {
        self.opers
            .iter()
            .zip(self.cols.iter())
            .map(|(o, col)| o.operate_on(col))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "#;

    #[test]
    fn test_pt1() {
        let homework = Homework::new_from(INPUT);
        let answers = homework.compute_all();
        assert_eq!(answers, vec![33210, 490, 4243455, 401]);
        let grand_total: i64 = answers.iter().sum();
        assert_eq!(grand_total, 4277556);
    }

    #[test]
    fn test_pt2() {
        let homework = Homework::from_cephalopod(INPUT);
        for row in homework.cols.iter() {
            println!("Col: {:?}", row);
        }
        let answers = homework.compute_all();
        assert_eq!(answers, vec![1058, 3253600, 625, 8544]);
        let grand_total: i64 = answers.iter().sum();
        assert_eq!(grand_total, 3263827);
    }
}
