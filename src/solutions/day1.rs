use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    io,
};

use tracing::{debug, info, instrument, trace, Level};

use crate::error::{AppError, Result};

pub fn solve(second: bool, line_reader: impl Iterator<Item = io::Result<String>>) -> Result<()> {
    if second {
        let similarity_score = calc_similarity_score(line_reader);
        info!(similarity_score = ?similarity_score, "Similarity score calculated");
    } else {
        let distance = calc_distance(line_reader);
        info!(distance = ?distance, "Distance calculated");
    }

    Ok(())
}

#[instrument(skip_all, ret(level = Level::DEBUG))]
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

#[instrument(skip_all, ret(level = Level::DEBUG))]
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
        let data = vec![
            Ok("3   4".to_string()),
            Ok("4   3".to_string()),
            Ok("2   5".to_string()),
            Ok("1   3".to_string()),
            Ok("3   9".to_string()),
            Ok("3   3".to_string()),
        ];
        let res = calc_distance(data.into_iter());
        assert_eq!(res.unwrap(), 11);
    }

    #[test]
    fn validate_second_star_example() {
        let data = vec![
            Ok("3   4".to_string()),
            Ok("4   3".to_string()),
            Ok("2   5".to_string()),
            Ok("1   3".to_string()),
            Ok("3   9".to_string()),
            Ok("3   3".to_string()),
        ];
        let res = calc_similarity_score(data.into_iter());
        assert_eq!(res.unwrap(), 31);
    }
}
