use std::str::FromStr;

use aoc_runner_derive::aoc;

pub type Line = (u32, u32);

pub struct Input {
    left_column: Vec<u32>,
    right_column: Vec<u32>,
}

impl FromIterator<Line> for Input {
    fn from_iter<T: IntoIterator<Item = Line>>(iter: T) -> Self {
        let (mut left_column, mut right_column): (Vec<u32>, Vec<u32>) = iter.into_iter().unzip();
        left_column.sort();
        right_column.sort();

        Self {
            left_column,
            right_column,
        }
    }
}

impl FromStr for Input {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed_lines = s.lines().map(|line| {
            let mut segments = line.split_whitespace().map(|seg| seg.parse().unwrap());
            (segments.next().unwrap(), segments.next().unwrap())
        });

        Ok(Self::from_iter(parsed_lines))
    }
}

#[aoc(day1, part1)]
pub fn part1(text: &str) -> u32 {
    let input: Input = text.parse().unwrap();

    input
        .left_column
        .iter()
        .zip(input.right_column.iter())
        .fold(0, |acc, (left, right)| acc + left.abs_diff(*right))
}

#[aoc(day1, part2)]
pub fn part2(text: &str) -> u32 {
    let input: Input = text.parse().unwrap();

    input.left_column.iter().fold(0, |acc, left| {
        acc + input
            .right_column
            .iter()
            .filter(|right| *right == left)
            .count() as u32
            * left
    })
}
