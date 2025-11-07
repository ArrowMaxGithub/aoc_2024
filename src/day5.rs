use std::{collections::HashSet, str::FromStr};

use ahash::{HashMap, HashMapExt};
use aoc_runner_derive::aoc;

pub type Line = ();
pub type Answer = usize;

pub struct Input {
    rules: HashMap<u8, Vec<u8>>,
    updates: Vec<Vec<u8>>
}

impl FromStr for Input {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed_rules = s.split_terminator("\n").take_while(|line| !line.is_empty()).map(|line|{
            let (key, value) = line.split_once("|").unwrap();
            (key.parse().unwrap(), value.parse().unwrap())
        });

        let mut count = 0;
        let mut rules: HashMap<u8, Vec<u8>> = HashMap::new();
        parsed_rules.for_each(|(key, value)|{
            rules.entry(key).or_default().push(value);
            count += 1;
        });

        println!("skipping: {count}");

        let updates = s.split_terminator("\n").skip(count + 2).map(|line|{
            unsafe { line
                .split(",")
                .map(|seg| seg.parse::<u8>().unwrap_unchecked()) }
                .collect()
        }).collect();

        Ok(Self {
            rules,
            updates,
        })
    }
}

#[aoc(day5, part1)]
pub fn part1(text: &str) -> Answer {
    let input: Input = unsafe { text.parse().unwrap_unchecked() };
    let mut result = 0;

    'outer: for update in input.updates{
        let mut allowed: HashSet<u8> = HashSet::new();
        for page in &update{
            if allowed.get(&page).is_some() || allowed.is_empty(){
                let Some(new_allowed) = input.rules.get(&page) else { continue;};
                allowed.extend(new_allowed);
            } else {
                continue 'outer;
            }
        }
        println!("correct: {update:?}");
        result += update[update.len() / 2] as usize;
    }

    result
}

#[aoc(day5, part2)]
pub fn part2(text: &str) -> Answer {
    let input: Input = unsafe { text.parse().unwrap_unchecked() };
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    const TEST_DATA: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"#;

    #[test]
    fn test_part1(){
        let result = part1(TEST_DATA);
        assert_eq!(result, 143);
    }

    #[test]
    fn test_part2(){
        let result = part2(TEST_DATA);
        assert_eq!(result, 42);
    }
}