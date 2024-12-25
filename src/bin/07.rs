use itertools::Itertools;

advent_of_code::solution!(7);

#[derive(Debug, Clone)]
enum Operator {
    Add,
    Mul,
}

fn parse_input(input: &str) -> Vec<(u64, Vec<u64>)> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split(':');

            let result = split.next().unwrap().parse().unwrap();

            let numbers: Vec<_> = split
                .next()
                .unwrap()
                .split_whitespace()
                .map(|value| value.parse().unwrap())
                .collect();

            (result, numbers)
        })
        .collect()
}

fn generate_combinations(length: usize) -> Vec<Vec<Operator>> {
    let number_of_combinations = 2 * length;

    // create an iterator over numbers between 0 and the number of combinations we need
    (0..number_of_combinations)
        .map(|x| {
            // create an iterator over the numbers between 0 and the length
            (0..length)
                // for every n, access `x` n-th bit
                .map(|n| (x >> n) & 1)
                // convert the bit to Operator
                .map(|x| match x {
                    0 => Operator::Add,
                    1 => Operator::Mul,

                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

fn evaluate(operators: &[Operator], values: &[u64]) -> u64 {
    let mut values_iter = values.iter();

    let initial_value = *values_iter.next().unwrap();

    values_iter
        .zip(operators)
        .fold(initial_value, |acc, (next, operation)| match operation {
            Operator::Add => acc + next,
            Operator::Mul => acc * next,
        })
}

pub fn part_one(input: &str) -> Option<u64> {
    let rows = parse_input(input);

    let sum = rows
        .into_iter()
        .filter(|(result, values)| {
            // generate all combinations of operators
            // length should be values.len() - 1
            let operators_combinations = generate_combinations(values.len() - 1);

            // evaluate all combinations of operators
            // if one of them returns the actual result, the equation is solvable
            for operators in operators_combinations {
                let computed_res = evaluate(&operators, values);

                if &computed_res == result {
                    return true;
                }
            }

            false
        })
        .map(|(result, _)| result)
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
