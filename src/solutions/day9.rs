use itertools::Itertools;
// use rayon::prelude::*;
use std::{
    cmp::Ordering,
    fmt::{self, Display},
    io,
};

use tracing::{debug, instrument, trace, Level};

use crate::error::{AppError, Result};

#[derive(Debug, Clone, Copy)]
enum Block {
    /// (ID, Len)
    Data(usize, usize),
    /// (Len)
    Free(usize),
}

impl Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Data(id, len) => id.to_string().repeat(*len),
            Self::Free(len) => '.'.to_string().repeat(*len),
        };
        f.write_str(&s)
    }
}

#[derive(Debug)]
struct Layout {
    blocks: Vec<Block>,
}

impl Display for Layout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for b in self.blocks.iter() {
            b.fmt(f)?;
        }
        Ok(())
    }
}

impl From<Vec<usize>> for Layout {
    fn from(mut value: Vec<usize>) -> Self {
        if value.len() % 2 == 1 {
            value.push(0);
        }
        let blocks = value
            .into_iter()
            .tuples()
            .enumerate()
            .flat_map(|(i, (block, free))| [Block::Data(i, block), Block::Free(free)])
            .collect_vec();
        Self { blocks }
    }
}

impl Layout {
    #[instrument(ret(level = Level::TRACE))]
    fn optimize_frag(&self) -> Self {
        let mut blocks = self.blocks.clone();
        let mut i = 0;
        while i < blocks.len() {
            match blocks[i] {
                Block::Data(_, _) => {
                    i += 1;
                }
                Block::Free(cap) => {
                    if let Some(Block::Data(id, len)) = blocks.pop() {
                        match cap.cmp(&len) {
                            Ordering::Greater => {
                                blocks[i] = Block::Free(cap - len);
                                blocks.insert(i, Block::Data(id, len));
                            }
                            Ordering::Equal => {
                                blocks[i] = Block::Data(id, len);
                            }
                            Ordering::Less => {
                                blocks[i] = Block::Data(id, cap);
                                blocks.push(Block::Data(id, len - cap));
                            }
                        }
                    }
                }
            }
        }
        Self { blocks }
    }

    #[instrument(skip_all, ret(level = Level::TRACE))]
    fn optimize_defrag(&self) -> Self {
        let mut blocks = self.blocks.clone();
        let mut i = blocks.len() - 1;
        while i > 0 {
            match blocks[i] {
                Block::Free(_) => {
                    i -= 1;
                }
                Block::Data(id, len) => {
                    let Some(free_pos) = blocks
                        .iter()
                        .take(i)
                        .position(|b| matches!(b, Block::Free(cap) if cap >= &len))
                    else {
                        i -= 1;
                        continue;
                    };
                    let Block::Free(cap) = blocks[free_pos] else {
                        unreachable!()
                    };
                    trace!(id = id, free_pos = free_pos, free_cap = cap, len = len);
                    blocks[i] = Block::Free(len);
                    if cap == len {
                        blocks[free_pos] = Block::Data(id, len);
                    } else {
                        blocks[free_pos] = Block::Free(cap - len);
                        blocks.insert(free_pos, Block::Data(id, len));
                    }
                }
            }
        }
        Self { blocks }
    }

    #[instrument(skip_all, ret(level = Level::TRACE))]
    fn checksum(&self) -> usize {
        self.blocks
            .iter()
            .fold((0, 0), |(cur_sum, cur_offset), block| {
                let (block_sum, block_offset) = match block {
                    Block::Free(len) => (0, len),
                    Block::Data(id, len) => {
                        let len_sum = (len * (cur_offset * 2 + len - 1)) / 2;
                        let block_sum = len_sum * id;
                        trace!(
                            id = id,
                            cur_offset = cur_offset,
                            len = len,
                            len_sum = len_sum,
                            block_sum = block_sum
                        );
                        (block_sum, len)
                    }
                };
                (cur_sum + block_sum, cur_offset + block_offset)
            })
            .0
    }
}

pub fn solve(second: bool, line_reader: impl Iterator<Item = io::Result<String>>) -> Result<()> {
    if second {
        soft_compress_blocks(line_reader)?;
    } else {
        compress_blocks(line_reader)?;
    }
    Ok(())
}

#[instrument(skip_all, ret)]
fn compress_blocks(
    line_reader: impl Iterator<Item = io::Result<String>>,
) -> Result<usize, AppError> {
    let raw_blocks = line_reader
        .map_ok(|s| {
            s.chars()
                .filter_map(|c| c.to_digit(10).map(|n| n as usize))
                .collect::<Vec<usize>>()
        })
        .collect::<io::Result<Vec<Vec<usize>>>>()
        .map_err(|e| AppError::DataParse(e.to_string()))?
        .into_iter()
        .flatten()
        .collect_vec();
    trace!(raw_blocks = ?raw_blocks);
    let l: Layout = raw_blocks.into();
    debug!("layout -> {l}");
    let l = l.optimize_frag();
    debug!("optimized_layout -> {l}");
    Ok(l.checksum())
}

#[instrument(skip_all, ret)]
fn soft_compress_blocks(
    line_reader: impl Iterator<Item = io::Result<String>>,
) -> Result<usize, AppError> {
    {
        let raw_blocks = line_reader
            .map_ok(|s| {
                s.chars()
                    .filter_map(|c| c.to_digit(10).map(|n| n as usize))
                    .collect::<Vec<usize>>()
            })
            .collect::<io::Result<Vec<Vec<usize>>>>()
            .map_err(|e| AppError::DataParse(e.to_string()))?
            .into_iter()
            .flatten()
            .collect_vec();
        trace!(raw_blocks = ?raw_blocks);
        let l: Layout = raw_blocks.into();
        debug!("layout -> {l}");
        let l = l.optimize_defrag();
        debug!("optimized_layout -> {l}");
        Ok(l.checksum())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn validate_one_star_example() {
        let data = "2333133121414131402";
        let res = compress_blocks(data.lines().map(|s| s.trim().to_string()).map(Ok));
        assert_eq!(res.unwrap(), 1928);
    }

    #[test]
    fn validate_second_star_example() {
        let data = "2333133121414131402";
        let res = soft_compress_blocks(data.lines().map(|s| s.trim().to_string()).map(Ok));
        assert_eq!(res.unwrap(), 2858);
    }
}
