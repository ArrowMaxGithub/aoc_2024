use aoc_runner_derive::aoc;

pub type Line = Vec<char>;
pub type Answer = usize;

#[cfg(test)]
const WIDTH: usize = 10;
#[cfg(not(test))]
const WIDTH: usize = 140;

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
    #[inline]
    pub const fn nom_part_one(&mut self, byte: u8) -> usize {
        match (&self, byte) {
            (Search::Initial, b'X') => {
                *self = Search::MF;
                0
            }
            (Search::MF, b'M') => {
                *self = Search::AF;
                0
            }
            (Search::AF, b'A') => {
                *self = Search::SF;
                0
            }
            (Search::SF, b'S') => {
                *self = Search::AR; // we could match xma-s-amx in one go
                1
            }

            (Search::Initial, b'S') => {
                *self = Search::AR;
                0
            }
            (Search::AR, b'A') => {
                *self = Search::MR;
                0
            }
            (Search::MR, b'M') => {
                *self = Search::XR;
                0
            }
            (Search::XR, b'X') => {
                *self = Search::MF; // we could match sam-x-mas in one go
                1
            }

            (_, b'X') => {
                *self = Search::MF;
                0
            }

            (_, b'S') => {
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
pub fn part1(ínput: &[u8]) -> Answer {
    let mut mode_horizontal = Search::Initial;
    let mut modes_vertical = [Search::Initial; WIDTH];
    let mut diagonal_l = [Search::Initial; WIDTH];
    let mut diagonal_r = [Search::Initial; WIDTH];

    let mut result = 0;
    ínput
        .iter()
        .filter(|b| **b != 0x0A)
        .array_chunks::<WIDTH>()
        .for_each(|chunk| {
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
pub fn part2(ínput: &[u8]) -> Answer {
    let input: Vec<u8> = ínput.iter().copied().filter(|b| *b != 0x0A).collect();

    let mut result = 0;
    for y in 1..WIDTH - 1 {
        for x in 1..WIDTH - 1 {
            let center = input[y * WIDTH + x];
            if center != b'A' {
                continue;
            }

            let c_top_left = input[(y - 1) * WIDTH + (x - 1)];
            let c_top_right = input[(y - 1) * WIDTH + (x + 1)];
            let c_bot_left = input[(y + 1) * WIDTH + (x - 1)];
            let c_bot_right = input[(y + 1) * WIDTH + (x + 1)];

            if (c_top_left == b'M' && c_bot_right == b'S'
                || c_top_left == b'S' && c_bot_right == b'M')
                && (c_top_right == b'M' && c_bot_left == b'S'
                    || c_top_right == b'S' && c_bot_left == b'M')
            {
                result += 1;
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
        let result = part1(TEST_DATA.as_bytes());
        assert_eq!(result, 18);
    }

    #[test]
    fn test_part2() {
        let result = part2(TEST_DATA.as_bytes());
        assert_eq!(result, 9);
    }
}
