use std::{io, mem};

use tracing::{debug, instrument, Level};

use crate::error::Result;

#[derive(Debug, Default, Clone, Copy)]
enum Tolerance {
    #[default]
    NotAvailable,
    Available,
    InProgress,
}

#[derive(Debug, Default, Clone)]
struct SafetyReportAcc {
    tolerance: Tolerance,
    history: Vec<usize>,
    real_history: Vec<usize>,
}

impl SafetyReportAcc {
    fn tolerated() -> Self {
        Self {
            tolerance: Tolerance::Available,
            ..Default::default()
        }
    }
}

impl SafetyReportAcc {
    #[instrument(ret(level = Level::TRACE))]
    fn try_correct(mut self) -> Option<SafetyReportAcc> {
        match self.tolerance {
            Tolerance::NotAvailable | Tolerance::InProgress => None,
            Tolerance::Available => {
                self.tolerance = Tolerance::InProgress;
                let start = self.real_history.len().checked_sub(4).unwrap_or_default();
                for i in (start..self.real_history.len()).rev() {
                    let mut report = self.clone();
                    report.history = report.real_history.clone();
                    report.history.remove(i);
                    if let Some(mut res) = report.try_replay() {
                        res.tolerance = Tolerance::Available;
                        res.real_history = self.real_history;
                        return Some(res);
                    }
                }
                None
            }
        }
    }

    #[instrument(ret(level = Level::TRACE))]
    fn try_replay(mut self) -> Option<SafetyReportAcc> {
        let mut replay = vec![];
        mem::swap(&mut replay, &mut self.history);
        self.real_history = vec![];
        replay
            .into_iter()
            .try_fold(self, |rep, v| rep.try_advance(v))
    }

    #[instrument(ret(level = Level::TRACE))]
    fn try_advance(mut self, next_value: usize) -> Option<SafetyReportAcc> {
        self.real_history.push(next_value);
        self.history.push(next_value);
        let Some([first, second]) = &self.history.get(0..2) else {
            return Some(self);
        };
        let prev_value = self
            .history
            .get(self.history.len() - 2)
            .expect("prev_value should exists");
        let seq_direction = first.cmp(second);
        let current_direction = prev_value.cmp(&next_value);
        let diff = prev_value.abs_diff(next_value);
        if !(1..=3).contains(&diff) || seq_direction != current_direction {
            self.history.pop();
            return self.try_correct();
        }
        Some(self)
    }
}

pub fn solve(second: bool, line_reader: impl Iterator<Item = io::Result<String>>) -> Result<()> {
    if second {
        count_safe_reports_tolerated(line_reader)?;
    } else {
        count_safe_reports(line_reader)?;
    }

    Ok(())
}

#[instrument(skip_all, ret)]
fn count_safe_reports(line_reader: impl Iterator<Item = io::Result<String>>) -> Result<usize> {
    let mut safe_count = 0;

    for line in line_reader {
        let line = line?;
        let is_safe = line
            .split(char::is_whitespace)
            .filter_map(|s| s.parse::<usize>().ok())
            .try_fold(SafetyReportAcc::default(), SafetyReportAcc::try_advance);
        debug!(line = line, is_safe = ?is_safe, "Parsed line");
        safe_count += is_safe.is_some() as usize;
    }

    Ok(safe_count)
}

#[instrument(skip_all, ret)]
fn count_safe_reports_tolerated(
    line_reader: impl Iterator<Item = io::Result<String>>,
) -> Result<usize> {
    let mut safe_count = 0;

    for line in line_reader {
        let line = line?;
        let is_safe = line
            .split(char::is_whitespace)
            .filter_map(|s| s.parse::<usize>().ok())
            .try_fold(SafetyReportAcc::tolerated(), SafetyReportAcc::try_advance);
        debug!(line = line, is_safe = ?is_safe, "Parsed line");
        safe_count += is_safe.is_some() as usize;
    }

    Ok(safe_count)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn validate_one_star_example() {
        let data = r#"7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9"#;
        let res = count_safe_reports(data.lines().map(|s| s.trim().to_string()).map(Ok));
        assert_eq!(res.unwrap(), 2);
    }

    #[test]
    fn validate_second_star_example() {
        let data = r#"7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9"#;
        let res = count_safe_reports_tolerated(data.lines().map(|s| s.trim().to_string()).map(Ok));
        assert_eq!(res.unwrap(), 4);
    }

    #[test]
    fn validate_second_star_custom_example_1() {
        let data = r#"9 5 4 3 1"#;
        let res = count_safe_reports_tolerated(data.lines().map(|s| s.trim().to_string()).map(Ok));
        assert_eq!(res.unwrap(), 1);
    }

    #[test]
    fn validate_second_star_custom_example_2() {
        let data = r#"4 6 4 3 1"#;
        let res = count_safe_reports_tolerated(data.lines().map(|s| s.trim().to_string()).map(Ok));
        assert_eq!(res.unwrap(), 1);
    }

    #[test]
    fn validate_second_star_custom_example_3() {
        let data = r#"51 52 55 58 60 61 62 61"#;
        let res = count_safe_reports_tolerated(data.lines().map(|s| s.trim().to_string()).map(Ok));
        assert_eq!(res.unwrap(), 1);
    }

    #[test]
    fn validate_second_star_real_example_1() {
        let data = r#"82 86 83 84 87"#;
        let res = count_safe_reports_tolerated(data.lines().map(|s| s.trim().to_string()).map(Ok));
        assert_eq!(res.unwrap(), 1);
    }

    #[test]
    fn validate_second_star_real_example_2() {
        let data = r#"20 23 22 19 17 15"#;
        let res = count_safe_reports_tolerated(data.lines().map(|s| s.trim().to_string()).map(Ok));
        assert_eq!(res.unwrap(), 1);
    }

    #[test]
    fn validate_second_star_real_example_3() {
        let data = r#"21 22 25 28 31 29 34"#;
        let res = count_safe_reports_tolerated(data.lines().map(|s| s.trim().to_string()).map(Ok));
        assert_eq!(res.unwrap(), 1);
    }

    #[test]
    fn validate_second_star_real_example_4() {
        let data = r#"52 50 45 42 39"#;
        let res = count_safe_reports_tolerated(data.lines().map(|s| s.trim().to_string()).map(Ok));
        assert_eq!(res.unwrap(), 0);
    }
}
