use std::{
    collections::{HashMap, HashSet},
    io,
};

use tracing::{instrument, trace, Level};

use crate::error::{AppError, Result};

#[derive(Debug)]
struct Scanner {
    rules: [HashSet<usize>; 100],
}

impl Default for Scanner {
    fn default() -> Self {
        Self {
            rules: (0..100)
                .map(|_| HashSet::new())
                .collect::<Vec<_>>()
                .try_into()
                .expect("cannot convert vec to array"),
        }
    }
}

impl Scanner {
    #[instrument(skip(self), ret(level = Level::TRACE))]
    fn add_rule(&mut self, left: usize, right: usize) {
        self.rules[left - 1].insert(right);
    }

    #[instrument(skip(self), ret(level = Level::TRACE))]
    fn verify_line(&self, nums: &Vec<usize>) -> Option<usize> {
        let mut set = HashSet::new();
        for n in nums.iter() {
            if self.rules[n - 1].intersection(&set).count() > 0 {
                trace!(n = n, "Failed at");
                return None;
            }
            set.insert(*n);
        }
        Some(nums[nums.len() / 2])
    }

    #[instrument(skip(self), ret(level = Level::TRACE))]
    fn fix_line(&self, mut nums: Vec<usize>) -> Vec<usize> {
        let mut set = HashSet::new();
        let mut map = HashMap::new();
        for i in 0..nums.len() {
            let n = nums[i];
            if let Some(el) = self.rules[n - 1].intersection(&set).next() {
                let &ei = map.get(el).expect("item should be in the map");
                map.insert(n, ei);
                map.insert(*el, i);
                nums.swap(ei, i);
            }
            set.insert(n);
            map.insert(n, i);
        }
        nums
    }
}

pub fn solve(second: bool, line_reader: impl Iterator<Item = io::Result<String>>) -> Result<()> {
    if second {
        scan_update_hard(line_reader)?;
    } else {
        scan_update(line_reader)?;
    }
    Ok(())
}

#[instrument(skip_all, ret)]
fn scan_update(mut line_reader: impl Iterator<Item = io::Result<String>>) -> Result<usize> {
    let mut scanner = Scanner::default();
    for line in line_reader
        .by_ref()
        .filter_map(Result::ok)
        .take_while(|s| !s.is_empty())
    {
        let Some((left, right)) = line
            .split_once('|')
            .and_then(|(left, right)| left.parse::<usize>().ok().zip(right.parse::<usize>().ok()))
        else {
            return Err(AppError::DataParse(format!("on line: {line}")).into());
        };
        scanner.add_rule(left, right);
    }
    trace!(scanner = ?scanner, "Rules parsed");
    let mut res = 0;
    for line in line_reader.filter_map(Result::ok) {
        let update: Vec<usize> = line.split(',').filter_map(|s| s.parse().ok()).collect();
        res += scanner.verify_line(&update).unwrap_or(0);
    }
    Ok(res)
}

#[instrument(skip_all, ret)]
fn scan_update_hard(mut line_reader: impl Iterator<Item = io::Result<String>>) -> Result<usize> {
    let mut scanner = Scanner::default();
    for line in line_reader
        .by_ref()
        .filter_map(Result::ok)
        .take_while(|s| !s.is_empty())
    {
        let Some((left, right)) = line
            .split_once('|')
            .and_then(|(left, right)| left.parse::<usize>().ok().zip(right.parse::<usize>().ok()))
        else {
            return Err(AppError::DataParse(format!("on line: {line}")).into());
        };
        scanner.add_rule(left, right);
    }
    trace!(scanner = ?scanner, "Rules parsed");
    let mut res = 0;
    for line in line_reader.filter_map(Result::ok) {
        let mut update: Vec<usize> = line.split(',').filter_map(|s| s.parse().ok()).collect();
        let mut failed_first = false;
        loop {
            match scanner.verify_line(&update) {
                Some(_) if !failed_first => {
                    break;
                }
                Some(n) => {
                    res += n;
                    break;
                }
                None => {
                    failed_first = true;
                    update = scanner.fix_line(update);
                }
            }
        }
    }
    Ok(res)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn validate_one_star_example() {
        let data = r#"47|53
            97|13
            97|61
            97|47
            75|29
            61|13
            75|53
            29|13
            97|29
            53|29
            61|53
            97|53
            61|29
            47|13
            75|47
            97|75
            47|61
            75|61
            47|29
            75|13
            53|13

            75,47,61,53,29
            97,61,53,29,13
            75,29,13
            75,97,47,61,53
            61,13,29
            97,13,75,29,47"#;
        let res = scan_update(data.lines().map(|s| s.trim().to_string()).map(Ok));
        assert_eq!(res.unwrap(), 143);
    }

    #[test]
    fn validate_second_star_example() {
        let data = r#"47|53
            97|13
            97|61
            97|47
            75|29
            61|13
            75|53
            29|13
            97|29
            53|29
            61|53
            97|53
            61|29
            47|13
            75|47
            97|75
            47|61
            75|61
            47|29
            75|13
            53|13

            75,47,61,53,29
            97,61,53,29,13
            75,29,13
            75,97,47,61,53
            61,13,29
            97,13,75,29,47"#;
        let res = scan_update_hard(data.lines().map(|s| s.trim().to_string()).map(Ok));
        assert_eq!(res.unwrap(), 123);
    }
}
