use std::{collections::BinaryHeap, str::FromStr};

use aoc_runner_derive::aoc;

pub type Line = Vec<i8>;
pub type Answer = usize;

pub struct Input {
    reports: Vec<Line>,
}

impl FromStr for Input {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed_lines = unsafe {
            s.split_terminator('\n').map(|line| {
                let segments = line
                    .split_ascii_whitespace()
                    .map(|seg| seg.parse::<i8>().unwrap_unchecked());
                segments.collect()
            })
        };

        let reports = parsed_lines.collect();

        Ok(Self { reports })
    }
}

#[aoc(day2, part1)]
pub fn part1(text: &str) -> Answer {
    let input: Input = unsafe { text.parse().unwrap_unchecked() };

    let total = input.reports.len();
    let unsafe_reports = input
        .reports
        .iter()
        .filter(|report| is_report_unsafe(&report));
    total - unsafe_reports.count()
}

#[aoc(day2, part2)]
pub fn part2(text: &str) -> Answer {
    let input: Input = unsafe { text.parse().unwrap_unchecked() };

    let total = input.reports.len();
    let inital_failed = input
        .reports
        .into_iter()
        .filter(|report| is_report_unsafe(report));
    let unsafe_reports = inital_failed.filter(|report| bruteforce_retest_is_report_unsafe(report));

    total - unsafe_reports.count()
}

fn bruteforce_retest_is_report_unsafe(report: &[i8]) -> bool {
    for i in 0..report.len() {
        let (start, rest) = report.split_at(i);
        let end = match rest {
            [_i, rest @ ..] => rest,
            [rest @ ..] => rest,
        };

        let test_slice = [start, end].concat();

        if !is_report_unsafe(&test_slice) {
            return false;
        }
    }

    true
}

fn is_report_unsafe(report: &[i8]) -> bool {
    let diff = report[1] - report[0]; // positive for increasing, negative for decreasing
    let mut steady = diff != 0; // edge-case where report[1] == report[0]

    let mut max_adj_diff = BinaryHeap::with_capacity(report.len());
    max_adj_diff.push(diff.abs());

    let mut previous = &report[1];

    for level in &report[2..] {
        let this_diff = *level - previous;
        steady &= (diff * this_diff) > 0;
        previous = level;
        max_adj_diff.push(this_diff.abs());
    }

    let mad = max_adj_diff.pop().unwrap();

    !steady || mad > 3
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"#;

    const EDGE_CASE_IGNORE_FIRST: &str = r#"45 44 46 47 50 52 54"#;
    const EDGE_CASE_IGNORE_LAST: &str = r#"44 46 47 50 52 54 54"#;

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

    #[test]
    fn test_part2_edge_case_ignore_first() {
        let result = part2(EDGE_CASE_IGNORE_FIRST);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_part2_edge_case_ignore_last() {
        let result = part2(EDGE_CASE_IGNORE_LAST);
        assert_eq!(result, 1);
    }
}
