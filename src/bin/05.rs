use std::{cmp::Ordering, collections::HashMap};

advent_of_code::solution!(5);

type PageOrderingRules = HashMap<u64, Vec<PageUpdate>>;
type PageUpdate = u64;

fn parse_input(input: &str) -> (PageOrderingRules, Vec<Vec<PageUpdate>>) {
    let rules = input
        .lines()
        .filter(|line| line.contains("|"))
        .map(|line| {
            let mut split = line.split("|");

            let key = split.next().unwrap().parse().unwrap();
            let value = split.next().unwrap().parse().unwrap();

            (key, value)
        })
        .fold(HashMap::new(), |mut acc, (key, value)| {
            acc.entry(key)
                .and_modify(|entry: &mut Vec<PageUpdate>| entry.push(value))
                .or_insert_with(|| vec![value]);

            acc
        });

    let updates = input
        .lines()
        .filter(|line| line.contains(","))
        .map(|line| {
            line.split(",")
                .map(|value| value.parse().unwrap())
                .collect()
        })
        .collect();

    (rules, updates)
}

fn is_sorted(rules: &PageOrderingRules, updates: &[PageUpdate]) -> bool {
    for (idx_of_current, update) in updates.iter().enumerate() {
        let update_rules = match rules.get(update) {
            Some(r) => r,
            _ => continue,
        };

        // check if there is a rule violation
        for rule in update_rules {
            match updates.iter().position(|x| x == rule) {
                // check if the page required by our current rule
                // appears after it in the order
                Some(idx_of_other) if idx_of_current > idx_of_other => {
                    return false;
                }

                _ => continue,
            }
        }
    }

    true
}

fn get_middle_number(updates: &[PageUpdate]) -> Option<u64> {
    let length = updates.len();

    if length % 2 == 0 {
        // compute the mean of the two elements surrounding the middle
        let idx_right = length / 2;
        let idx_left = idx_right - 1;

        updates
            .get(idx_left)
            .zip(updates.get(idx_right))
            .map(|(left, right)| (left + right) / 2)
    } else {
        // division on usizes automatically floors the value
        let idx = length / 2;
        updates.get(idx).cloned()
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let (rules, updates) = parse_input(input);

    let sum = updates
        .iter()
        .filter(|update_row| is_sorted(&rules, update_row))
        .filter_map(|update_row| get_middle_number(update_row))
        .sum();

    Some(sum)
}

fn sort_updates(rules: &PageOrderingRules, a: &u64, b: &u64) -> Ordering {
    let update_rules = match rules.get(a) {
        Some(r) => r,
        _ => return Ordering::Equal,
    };

    for rule in update_rules {
        if rule == b {
            return Ordering::Less;
        }
    }

    Ordering::Equal
}

pub fn part_two(input: &str) -> Option<u64> {
    let (rules, updates) = parse_input(input);

    let sum = updates
        .into_iter()
        .filter(|update_row| !is_sorted(&rules, update_row))
        .map(|mut update_row| {
            // sort our update rows
            update_row.sort_by(|a, b| sort_updates(&rules, a, b));

            update_row
        })
        .filter_map(|update_row| get_middle_number(&update_row))
        .sum();

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
