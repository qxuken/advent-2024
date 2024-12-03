use std::{io, str::Chars};

use tracing::{debug, instrument, Level};

use crate::error::Result;

#[derive(Debug)]
struct Scanner {
    cond_enabled: bool,
    mul_enabled: bool,
}

impl Default for Scanner {
    #[instrument(skip_all, ret(level = Level::TRACE))]
    fn default() -> Self {
        Self {
            cond_enabled: false,
            mul_enabled: true,
        }
    }
}

impl<'a> Scanner {
    #[instrument(skip_all, ret(level = Level::TRACE))]
    fn with_cond(mut self) -> Self {
        self.cond_enabled = true;
        self
    }

    #[instrument(skip_all, ret(level = Level::TRACE))]
    fn scan_commands(&mut self, chars: &mut Chars<'a>) -> Option<(isize, isize)> {
        while let Some(ch) = chars.next() {
            match ch {
                'm' if self.mul_enabled => {
                    if let Some(args) = Self::scan_mul(chars) {
                        return Some(args);
                    }
                }
                'd' if self.cond_enabled => {
                    if let Some(new_mul) = Self::scan_cond(chars) {
                        self.mul_enabled = new_mul;
                    }
                }
                _ => continue,
            };
        }
        None
    }

    #[instrument(skip_all, ret(level = Level::TRACE))]
    fn scan_cond(chars: &mut Chars<'a>) -> Option<bool> {
        let mut buf = String::with_capacity(7);
        buf.push('d');
        for ch in chars.by_ref() {
            buf.push(ch);
            if &buf == "do()" {
                return Some(true);
            }
            if &buf == "don't()" {
                return Some(false);
            }
            if "do()".starts_with(&buf) || "don't()".starts_with(&buf) {
                continue;
            }
            return None;
        }
        None
    }

    #[instrument(skip_all, ret(level = Level::TRACE))]
    fn scan_mul(chars: &mut Chars<'a>) -> Option<(isize, isize)> {
        let mut prev_char = 'm';
        while let Some(ch) = chars.next() {
            match (prev_char, ch) {
                ('m', 'u') => {
                    prev_char = 'u';
                    continue;
                }
                ('u', 'l') => return Self::parse_mul_args(chars),
                _ => return None,
            };
        }
        None
    }

    #[instrument(skip_all, ret(level = Level::TRACE))]
    fn parse_mul_args(chars: &mut Chars<'a>) -> Option<(isize, isize)> {
        if chars.next().is_none_or(|ch| ch != '(') {
            return None;
        }
        let left = chars
            .clone()
            .take(3)
            .take_while(|ch| ch.is_numeric())
            .collect::<String>();
        for _ in 0..left.len() {
            chars.next();
        }
        let left = left.parse().ok()?;
        if chars.next().is_none_or(|ch| ch != ',') {
            return None;
        }
        let right = chars
            .clone()
            .take(3)
            .take_while(|ch| ch.is_numeric())
            .collect::<String>();
        for _ in 0..right.len() {
            chars.next();
        }
        let right = right.parse().ok()?;
        if chars.next().is_none_or(|ch| ch != ')') {
            return None;
        }
        Some((left, right))
    }
}

pub fn solve(second: bool, line_reader: impl Iterator<Item = io::Result<String>>) -> Result<()> {
    if second {
        scan_multiply_with_cond(line_reader)?;
    } else {
        scan_multiply(line_reader)?;
    }

    Ok(())
}

#[instrument(skip_all, ret)]
fn scan_multiply(line_reader: impl Iterator<Item = io::Result<String>>) -> Result<isize> {
    let mut res = 0;

    let mut scanner = Scanner::default();
    for line in line_reader.filter_map(Result::ok) {
        let mut chars = line.chars();
        while let Some((left, right)) = scanner.scan_commands(chars.by_ref()) {
            res += left * right;
            debug!(left = left, right = right, res = res, "Found mul args");
        }
    }

    Ok(res)
}

#[instrument(skip_all, ret)]
fn scan_multiply_with_cond(line_reader: impl Iterator<Item = io::Result<String>>) -> Result<isize> {
    let mut res = 0;
    let mut scanner = Scanner::default().with_cond();
    for line in line_reader.filter_map(Result::ok) {
        let mut chars = line.chars();
        while let Some((left, right)) = scanner.scan_commands(chars.by_ref()) {
            res += left * right;
            debug!(left = left, right = right, res = res, "Found mul args");
        }
    }

    Ok(res)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn validate_one_star_example() {
        let data = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let res = scan_multiply(data.lines().map(|s| s.trim().to_string()).map(Ok));
        assert_eq!(res.unwrap(), 161);
    }

    #[test]
    fn validate_second_star_example() {
        let data = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let res = scan_multiply_with_cond(data.lines().map(|s| s.trim().to_string()).map(Ok));
        assert_eq!(res.unwrap(), 48);
    }
}
