use aoc_runner_derive::{aoc, aoc_generator};

pub type Line = ();

pub struct Input {
}

impl FromIterator<Line> for Input {
    fn from_iter<T: IntoIterator<Item = Line>>(iter: T) -> Self {

        Self {
        }
    }
}

#[aoc_generator(day1)]
pub fn generator(text: &str) -> Input {
    Input::from_iter(text.lines().map(|line| {
        let mut segments = line
            .split_whitespace()
            .map(|segment| segment.parse().unwrap());
        ()
    }))
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &Input) -> u32 {
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &Input) -> u32 {
}
