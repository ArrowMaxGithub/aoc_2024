use std::str::FromStr;

use aoc_runner_derive::aoc;

#[cfg(test)]
const N: usize = 6;
#[cfg(not(test))]
const N: usize = 1000;

// pub type Line = [i8; 8];
pub type Answer = usize;

pub struct Input {
    flat_reports: [i8; N * 8],
}

impl FromStr for Input {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut i = 0;
        let mut flat_reports = [0; N * 8];

        unsafe {
            s.lines().for_each(|line| {
                let segments = line
                    .split_ascii_whitespace()
                    .map(|seg| seg.parse::<i8>().unwrap_unchecked());

                flat_reports[i..i + 8]
                    .iter_mut()
                    .zip(segments)
                    .for_each(|(level, seg)| {
                        *level = seg;
                    });

                i += 8;
            })
        };

        Ok(Self { flat_reports })
    }
}

#[aoc(day2, part1)]
pub fn part1(text: &str) -> Answer {
    let input: Input = unsafe { text.parse().unwrap_unchecked() };

    let unsafe_reports = input
        .flat_reports
        .into_iter()
        .array_chunks()
        .filter(|report| is_report_unsafe(*report));

    N - unsafe_reports.count()
}

#[aoc(day2, part2)]
pub fn part2(text: &str) -> Answer {
    let input: Input = unsafe { text.parse().unwrap_unchecked() };

    let inital_failed = input
        .flat_reports
        .into_iter()
        .array_chunks()
        .filter(|report| is_report_unsafe(*report));

    let unsafe_reports =
        inital_failed.filter(|report| bruteforce_retest_is_report_unsafe(*report));

    N - unsafe_reports.count()
}

fn bruteforce_retest_is_report_unsafe(report: [i8; 8]) -> bool {
    for i in 0..8 {
        let mut cloned_report = report;
        cloned_report[i] = 0;

        if !is_report_unsafe(cloned_report) {
            return false;
        }
    }

    true
}

fn is_report_unsafe(report: [i8; 8]) -> bool {
    let mut initial_dir = None;
    let mut previous = None;

    for level in report {
        // skip any 0s in the parsed report
        if level == 0 {
            continue;
        }

        // if this is the first non-zero level, set it as starting point
        let Some(compare) = previous else {
            previous = Some(level);
            continue;
        };

        previous = Some(level);

        let diff = level - compare;
        if diff.abs() > 3 {
            return true;
        }

        let dir = diff.signum();

        // if this is the second non-zero level, set the inital direction
        let Some(initial_dir) = initial_dir else {
            initial_dir = Some(dir);
            continue;
        };

        // if the new dir diverges from the previous dir, this report is unsafe, exit early
        if initial_dir * dir <= 0 {
            return true;
        }
    }

    false // no early returns => this report is safe
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9

";

    #[test]
    fn test_part1() {
        let result = part1(TEST_DATA);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part2() {
        let result = part2(TEST_DATA);
        assert_eq!(result, 4);
    }
}
