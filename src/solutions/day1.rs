use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    io,
};

use tracing::{debug, instrument, trace};

use crate::error::{AppError, Result};

pub fn solve(second: bool, line_reader: impl Iterator<Item = io::Result<String>>) -> Result<()> {
    if second {
        calc_similarity_score(line_reader)?;
    } else {
        calc_distance(line_reader)?;
    }
    Ok(())
}

#[instrument(skip_all, ret)]
fn calc_distance(line_reader: impl Iterator<Item = io::Result<String>>) -> Result<usize> {
    let mut left_heap = BinaryHeap::new();
    let mut right_heap = BinaryHeap::new();

    debug!("Parsing file");
    for (line, i) in line_reader.zip(1..) {
        let line = line?;
        let Some((left, right)) = line.split_once(char::is_whitespace) else {
            return Err(AppError::DataParse(format!("on line: {i:?}")).into());
        };
        let left: usize = left.trim().parse()?;
        let right: usize = right.trim().parse()?;

        trace!(left = left, right = right, "Parsed line");

        left_heap.push(Reverse(left));
        right_heap.push(Reverse(right));
    }

    debug!("Calculating distance");
    let mut distance = 0;
    while let Some((Reverse(left), Reverse(right))) = left_heap.pop().zip(right_heap.pop()) {
        let diff = right.abs_diff(left);
        distance += diff;
        trace!(left = ?left, right = ?right, distance = ?distance, diff = ?diff, "Diff");
    }

    Ok(distance)
}

#[instrument(skip_all, ret)]
fn calc_similarity_score(line_reader: impl Iterator<Item = io::Result<String>>) -> Result<usize> {
    let mut left_map = HashMap::new();
    let mut right_map = HashMap::new();

    debug!("Parsing file");
    for (line, i) in line_reader.zip(1..) {
        let line = line?;
        let Some((left, right)) = line.split_once(char::is_whitespace) else {
            return Err(AppError::DataParse(format!("on line: {i:?}")).into());
        };
        let left: usize = left.trim().parse()?;
        let right: usize = right.trim().parse()?;

        trace!(left = left, right = right, "Parsed line");

        left_map.entry(left).and_modify(|c| *c += 1).or_insert(1);
        right_map.entry(right).and_modify(|c| *c += 1).or_insert(1);
    }

    debug!("Calculating similarity score");
    let mut similarity_score = 0;
    for (key, value) in left_map.iter() {
        let local_score = *key * *value * *right_map.get(key).unwrap_or(&0);
        similarity_score += local_score;
        trace!(key = ?key, value = ?value, similarity_score = ?similarity_score, local_score = ?local_score, "Similarity score");
    }

    Ok(similarity_score)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn validate_one_star_example() {
        let data = r#"3   4
            4   3
            2   5
            1   3
            3   9
            3   3"#;
        let res = calc_distance(data.lines().map(|s| s.trim().to_string()).map(Ok));
        assert_eq!(res.unwrap(), 11);
    }

    #[test]
    fn validate_second_star_example() {
        let data = r#"3   4
            4   3
            2   5
            1   3
            3   9
            3   3"#;
        let res = calc_similarity_score(data.lines().map(|s| s.trim().to_string()).map(Ok));
        assert_eq!(res.unwrap(), 31);
    }
}
