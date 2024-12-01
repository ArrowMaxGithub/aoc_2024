use ahash::AHashMap;
use std::str::FromStr;

use aoc_runner_derive::aoc;

pub type Line = (u32, u32);
pub type Answer = u32;

pub struct Input {
    left_column: Vec<u32>,
    right_column: Vec<u32>,
}

impl FromStr for Input {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed_lines = unsafe {
            s.split_terminator('\n').map(|line| {
                let mut segments = line
                    .split_ascii_whitespace()
                    .map(|seg| seg.parse::<u32>().unwrap_unchecked());
                (
                    segments.next().unwrap_unchecked(),
                    segments.next().unwrap_unchecked(),
                )
            })
        };

        let (left_column, right_column) = parsed_lines.unzip();

        Ok(Self {
            left_column,
            right_column,
        })
    }
}

#[aoc(day1, part1)]
pub fn part1(text: &str) -> Answer {
    let mut input: Input = unsafe { text.parse().unwrap_unchecked() };

    radsort::sort(&mut input.left_column);
    radsort::sort(&mut input.right_column);

    input
        .left_column
        .iter()
        .zip(input.right_column.iter())
        .fold(0, |acc, (left, right)| acc + left.abs_diff(*right))
}

#[aoc(day1, part2)]
pub fn part2(text: &str) -> Answer {
    let input: Input = unsafe { text.parse().unwrap_unchecked() };

    let mut appearances = AHashMap::with_capacity(input.right_column.len());

    input.right_column.iter().for_each(|value| {
        *appearances.entry(value).or_insert(0) += 1;
    });

    input.left_column.iter().fold(0, |acc, left| {
        acc + left * appearances.get(left).unwrap_or(&0)
    })
}
