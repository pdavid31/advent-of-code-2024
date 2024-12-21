use std::collections::HashMap;

advent_of_code::solution!(1);

fn parse_input(input: &str) -> (Vec<u64>, Vec<u64>) {
    let (left, right): (Vec<u64>, Vec<u64>) = input
        .lines()
        .map(|line| {
            let mut split = line.split_whitespace();

            match split.next().zip(split.next()) {
                Some(row) => row,

                _ => panic!("one side was missing a value"),
            }
        })
        .map(|(left, right)| (left.parse::<u64>(), right.parse::<u64>()))
        .map(|(left, right)| match left.ok().zip(right.ok()) {
            Some(row) => row,

            _ => panic!("was not able to parse both values to u64"),
        })
        .unzip();

    (left, right)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (mut left, mut right) = parse_input(input);

    left.sort();
    right.sort();

    let sum = left
        .into_iter()
        .zip(right)
        .map(|(l, r)| l.abs_diff(r))
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (left, right) = parse_input(input);

    let lookup_table = right.iter().fold(HashMap::new(), |mut acc, key| {
        // try to grab the count (value) at key (number),
        // if it's not set yet, set it to 1 since it's the first appearance
        acc.entry(key).and_modify(|v| *v += 1).or_insert(1);

        acc
    });

    let sum = left
        .iter()
        .map(|x| {
            let appearances_in_right = lookup_table.get(x).unwrap_or(&0);

            x * appearances_in_right
        })
        .sum();

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
