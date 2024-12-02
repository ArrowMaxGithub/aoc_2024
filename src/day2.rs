use std::collections::BinaryHeap;

use aoc_runner_derive::aoc;

use crate::util::parse_fast_whitespeace_separated;

pub type Line = Vec<i8>;
pub type Answer = usize;

#[aoc(day2, part1)]
pub fn part1(input: &[u8]) -> Answer {
    let reports = parse_fast_whitespeace_separated::<8, 1000>(input);

    let unsafe_reports = reports
        .iter()
        .filter(|report| is_report_unsafe(report));

    1000 - unsafe_reports.count()
}

#[aoc(day2, part2)]
pub fn part2(input: &[u8]) -> Answer {
    let reports = parse_fast_whitespeace_separated::<8, 1000>(input);

    let inital_failed = reports
        .into_iter()
        .filter(|report| is_report_unsafe(report));
    
    let unsafe_reports = inital_failed.filter(|report| bruteforce_retest_is_report_unsafe(report));

    1000 - unsafe_reports.count()
}

fn bruteforce_retest_is_report_unsafe(report: &[i32; 8]) -> bool {
    for i in 0..report.len() {
        let mut cloned_report = report.clone();
        cloned_report[i] = 0;

        if !is_report_unsafe(&cloned_report) {
            return false;
        }
    }

    true
}

fn is_report_unsafe(report: &[i32; 8]) -> bool {
    let mut initial_dir = None;
    let mut previous = None;
    let mut mad = BinaryHeap::with_capacity(8); // MaxAdjacentDifference, MAD

    for level in report{
        // skip any 0s in the parsed report
        if *level == 0 {
            continue;
        }

        // if this is the first non-zero level, set it as starting point
        let Some(compare) = previous else {
            previous = Some(level);
            continue;
        };

        previous = Some(level);

        let diff = level - *compare;
        let dir = diff.signum();

        mad.push(diff.abs());

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

    // the report is either steadily increasing or decreasing, but it may still contain excessive MAD
    let mad_max = mad.pop().unwrap();
    mad_max > 3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1(include_bytes!("../input/2024/day2.txt"));
        assert_eq!(result, 479);
    }

    #[test]
    fn test_part2() {
        let result = part2(include_bytes!("../input/2024/day2.txt"));
        assert_eq!(result, 531);
    }
}
