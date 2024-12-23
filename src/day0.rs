use std::str::FromStr;

use aoc_runner_derive::aoc;

pub type Line = ();
pub type Answer = ();

pub struct Input {

}

impl FromStr for Input {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed_lines = unsafe {
            s.split_terminator('\n').map(|line| {
                let mut segments = line
                    .split_ascii_whitespace()
                    .map(|seg| seg.parse().unwrap_unchecked());
                
            })
        };

        Ok(Self {

        })
    }
}

#[aoc(day1, part1)]
pub fn part1(text: &str) -> Answer {
    let mut input: Input = unsafe { text.parse().unwrap_unchecked() };
    todo!()
}

#[aoc(day1, part2)]
pub fn part2(text: &str) -> Answer {
    let input: Input = unsafe { text.parse().unwrap_unchecked() };
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    const TEST_DATA: &str = r#""#;

    #[test]
    fn test_part1(){
        let result = part1(TEST_DATA);
        assert_eq!(result, 42);
    }

    #[test]
    fn test_part2(){
        let result = part2(TEST_DATA);
        assert_eq!(result, 42);
    }
}