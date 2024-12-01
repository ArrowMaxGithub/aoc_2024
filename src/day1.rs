use aoc_runner_derive::{aoc, aoc_generator};

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

#[aoc_generator(day1)]
pub fn generator(text: &str) -> Input {
    Input::from_iter(text.lines().map(|line| {
        let mut segments = line
            .split_whitespace()
            .map(|segment| segment.parse().unwrap());
        (segments.next().unwrap(), segments.next().unwrap())
    }))
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &Input) -> u32 {
    input
        .left_column
        .iter()
        .zip(input.right_column.iter())
        .fold(0, |acc, (left, right)| acc + left.abs_diff(*right))
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &Input) -> u32 {
    input.left_column.iter().fold(0, |acc, left| {
        acc + input
            .right_column
            .iter()
            .filter(|right| *right == left)
            .count() as u32
            * left
    })
}
