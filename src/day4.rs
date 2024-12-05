use std::simd::u8x4;

use aoc_runner_derive::aoc;

pub type Line = Vec<char>;
pub type Answer = usize;

#[cfg(test)]
const WIDTH: usize = 10;
#[cfg(not(test))]
const WIDTH: usize = 140;

const XMAS: u8x4 = u8x4::from_array([b'X', b'M', b'A', b'S']);
const SMAX: u8x4 = u8x4::from_array([b'S', b'A', b'M', b'X']);

#[inline(always)]
fn search1(bytes: u8x4) -> usize {
    (bytes == XMAS || bytes == SMAX) as usize
}

#[aoc(day4, part1)]
pub fn part1(ínput: &[u8]) -> Answer {
    let mut horizontal = u8x4::from_array([0; 4]);
    let mut vertical = [u8x4::from_array([0; 4]); WIDTH];
    let mut diagonal_l = [u8x4::from_array([0; 4]); WIDTH];
    let mut diagonal_r = [u8x4::from_array([0; 4]); WIDTH];

    let mut result = 0;
    ínput
        .iter()
        .filter(|b| **b != 0x0A)
        .array_chunks::<WIDTH>()
        .for_each(|chunk| {
            horizontal = u8x4::from_array([0; 4]); // avoid matching across line breaks

            for (i, c) in chunk.into_iter().copied().enumerate() {
                horizontal = horizontal.rotate_elements_right::<1>();
                vertical[i] = vertical[i].rotate_elements_right::<1>();
                diagonal_l[i] = diagonal_l[i].rotate_elements_right::<1>();
                diagonal_r[i] = diagonal_r[i].rotate_elements_right::<1>();

                horizontal[0] = c;
                vertical[i][0] = c;
                diagonal_l[i][0] = c;
                diagonal_r[i][0] = c;

                result += search1(horizontal);
                result += search1(vertical[i]);
                result += search1(diagonal_l[i]);
                result += search1(diagonal_r[i]);
            }

            diagonal_l[WIDTH - 1] = u8x4::from_array([0; 4]); // avoid matching across line breaks
            diagonal_r[0] = u8x4::from_array([0; 4]); // avoid matching across line breaks

            diagonal_l.rotate_right(1); // next match will be one to the right
            diagonal_r.rotate_left(1); // next match will be one to the left
        });

    result
}

#[aoc(day4, part2)]
pub fn part2(input: &[u8]) -> Answer {
    let mut input: Vec<u8> = input.iter().copied().filter(|b| *b != 0x0A).collect();

    let mut result = 0;
    for y in 1..WIDTH - 1 {
        for x in 1..WIDTH - 1 {
            if unsafe { *input.get_unchecked(y * WIDTH + x) } != b'A' {
                continue;
            }

            let chars = unsafe {
                input.get_many_unchecked_mut([
                    (y + 1) * WIDTH + (x - 1),
                    (y + 1) * WIDTH + (x + 1),
                    (y - 1) * WIDTH + (x - 1),
                    (y - 1) * WIDTH + (x + 1),
                ])
            };

            if (*chars[0] == b'M' && *chars[3] == b'S' || *chars[0] == b'S' && *chars[3] == b'M')
                && (*chars[1] == b'M' && *chars[2] == b'S'
                    || *chars[1] == b'S' && *chars[2] == b'M')
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
