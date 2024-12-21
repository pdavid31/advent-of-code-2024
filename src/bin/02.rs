advent_of_code::solution!(2);

fn parse_input(input: &str) -> Vec<Vec<u64>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect()
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let data = parse_input(input);

    let count = data
        .into_iter()
        .map(|row| {
            // build an iterator over adjacent pairs
            // and compute the differences from those
            let differences: Vec<_> = row
                .iter()
                .zip(row.iter().skip(1))
                .map(|(a, b)| (*a as i64, *b as i64))
                .map(|(a, b)| b - a)
                .collect();

            // check if the report is only increasing (e.g. all differences positive)
            let is_increasing = differences.iter().all(|x| *x >= 0);
            // check if the report is only decreasing (e.g. all differences negative)
            let is_decreasing = differences.iter().all(|x| *x <= 0);

            if is_increasing || is_decreasing {
                // check if all differences are in the valid range
                differences.iter().all(|x| x.abs() >= 1 && x.abs() <= 3)
            } else {
                false
            }
        })
        .filter(|is_safe| *is_safe)
        .count();

    Some(count as u64)
}

fn is_row_safe(row: &[u64], allowed_outliers: usize) -> bool {
    // build an iterator over adjacent pairs
    // and compute the differences from those
    let differences: Vec<_> = row
        .iter()
        .zip(row.iter().skip(1))
        .map(|(a, b)| (*a as i64, *b as i64))
        .map(|(a, b)| b - a)
        .collect();

    let length = differences.len();
    let min_no_of_matching_nodes = length - allowed_outliers;

    // check if the report is only increasing (e.g. all differences positive)
    let increasing_nodes = differences.iter().filter(|diff| *diff > &0).count();
    let is_increasing = increasing_nodes >= min_no_of_matching_nodes;
    // check if the report is only decreasing (e.g. all differences negative)
    let decreasing_nodes = differences.iter().filter(|diff| *diff < &0).count();
    let is_decreasing = decreasing_nodes >= min_no_of_matching_nodes;

    if is_increasing {
        if increasing_nodes < length {
            // NOTE: we can safely unwrap here, since we already know,
            // that this will find a node in the iterator
            let idx = differences.iter().position(|diff| diff <= &0).unwrap();

            let mut cloned_row = row.to_vec();
            cloned_row.remove(idx);

            is_row_safe(&cloned_row, allowed_outliers - 1)
        } else {
            differences.iter().all(|diff| (1..=3).contains(diff))
        }
    } else if is_decreasing {
        if decreasing_nodes < length {
            // NOTE: we can safely unwrap here, since we already know,
            // that this will find a node in the iterator
            let idx = differences.iter().position(|diff| diff >= &0).unwrap();

            let mut cloned_row = row.to_vec();
            cloned_row.remove(idx);

            is_row_safe(&cloned_row, allowed_outliers - 1)
        } else {
            differences.iter().all(|diff| (-3..=-1).contains(diff))
        }
    } else {
        false
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let data = parse_input(input);

    let allowed_outliers = 1;

    let count = data
        .iter()
        .map(|row| is_row_safe(row, allowed_outliers))
        .filter(|is_safe| *is_safe)
        .count();

    Some(count as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
