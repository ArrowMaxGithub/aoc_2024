use std::str::FromStr;

use aoc_runner_derive::aoc;

pub type Line = Vec<char>;
pub type Answer = usize;

#[cfg(test)]
const WIDTH: usize = 10;
#[cfg(not(test))]
const WIDTH: usize = 140;

pub struct Input {
    chars: Vec<char>,
}

impl FromStr for Input {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed_lines = s.split_terminator('\n').map(|line| line.chars()).flatten();

        Ok(Self {
            chars: parsed_lines.collect(),
        })
    }
}

#[derive(Clone, Copy)]
enum Search {
    Initial,
    // F == Forward
    // R == Reverse
    MF,
    AF,
    SF,
    XR,
    MR,
    AR,
}

impl Search {
    pub const fn nom_part_one(&mut self, char: char) -> usize {
        match (&self, char) {
            (Search::Initial, 'X') => {
                *self = Search::MF;
                0
            }
            (Search::MF, 'M') => {
                *self = Search::AF;
                0
            }
            (Search::AF, 'A') => {
                *self = Search::SF;
                0
            }
            (Search::SF, 'S') => {
                *self = Search::AR; // we could match xma-s-amx in one go
                1
            }

            (Search::Initial, 'S') => {
                *self = Search::AR;
                0
            }
            (Search::AR, 'A') => {
                *self = Search::MR;
                0
            }
            (Search::MR, 'M') => {
                *self = Search::XR;
                0
            }
            (Search::XR, 'X') => {
                *self = Search::MF; // we could match sam-x-mas in one go
                1
            }

            (_, 'X') => {
                *self = Search::MF;
                0
            }

            (_, 'S') => {
                *self = Search::AR;
                0
            }

            _ => {
                *self = Search::Initial;
                0
            }
        }
    }
}

#[aoc(day4, part1)]
pub fn part1(text: &str) -> Answer {
    let input: Input = unsafe { text.parse().unwrap_unchecked() };
    let mut mode_horizontal = Search::Initial;
    let mut modes_vertical = [Search::Initial; WIDTH];
    let mut diagonal_l = [Search::Initial; WIDTH];
    let mut diagonal_r = [Search::Initial; WIDTH];

    let mut result = 0;
    input.chars.array_chunks::<WIDTH>().for_each(|chunk| {
        mode_horizontal = Search::Initial; // avoid matching across line breaks

        for (i, c) in chunk.into_iter().enumerate() {
            result += mode_horizontal.nom_part_one(*c);
            result += modes_vertical[i].nom_part_one(*c);
            result += diagonal_l[i].nom_part_one(*c);
            result += diagonal_r[i].nom_part_one(*c);
        }

        diagonal_l[WIDTH - 1] = Search::Initial; // avoid matching across line breaks
        diagonal_r[0] = Search::Initial; // avoid matching across line breaks

        diagonal_l.rotate_right(1); // next match will be one to the right
        diagonal_r.rotate_left(1); // next match will be one to the left
    });

    result
}

#[aoc(day4, part2)]
pub fn part2(text: &str) -> Answer {
    let input: Input = unsafe { text.parse().unwrap_unchecked() };

    let mut result = 0;
    for y in 1..WIDTH - 1 {
        for x in 1..WIDTH - 1 {
            let center = input.chars[y * WIDTH + x];
            if center != 'A' {
                continue;
            }

            let c_top_left = input.chars[(y - 1) * WIDTH + (x - 1)];
            let c_top_right = input.chars[(y - 1) * WIDTH + (x + 1)];
            let c_bot_left = input.chars[(y + 1) * WIDTH + (x - 1)];
            let c_bot_right = input.chars[(y + 1) * WIDTH + (x + 1)];

            if c_top_left == 'M' && c_bot_right == 'S' || c_top_left == 'S' && c_bot_right == 'M' {
                if c_top_right == 'M' && c_bot_left == 'S'
                    || c_top_right == 'S' && c_bot_left == 'M'
                {
                    result += 1;
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"#;

    #[test]
    fn test_part1() {
        let result = part1(TEST_DATA);
        assert_eq!(result, 18);
    }

    #[test]
    fn test_part2() {
        let result = part2(TEST_DATA);
        assert_eq!(result, 9);
    }
}
