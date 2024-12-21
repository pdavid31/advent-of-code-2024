advent_of_code::solution!(3);

const MIN_WINDOW_SIZE: usize = 8;
const MAX_WINDOW_SIZE: usize = 12;

#[derive(Debug)]
enum Instruction {
    Multiply(u64, u64),
}

impl Instruction {
    fn execute(&self) -> u64 {
        match self {
            Self::Multiply(a, b) => a * b,
        }
    }
}

impl TryFrom<&[char]> for Instruction {
    type Error = ();

    fn try_from(value: &[char]) -> Result<Self, Self::Error> {
        match value[..4] {
            ['m', 'u', 'l', '('] => {
                let position_l_bracket = 3;
                let position_r_bracket = value.iter().position(|c| *c == ')').ok_or(())?;
                let position_separator = value.iter().position(|c| *c == ',').ok_or(())?;

                let left_number = value[position_l_bracket + 1..position_separator]
                    .iter()
                    .collect::<String>()
                    .parse()
                    .map_err(|_| ())?;

                let right_number = value[position_separator + 1..position_r_bracket]
                    .iter()
                    .collect::<String>()
                    .parse()
                    .map_err(|_| ())?;

                Ok(Self::Multiply(left_number, right_number))
            }

            _ => Err(()),
        }
    }
}

fn parse_input(input: &str) -> Vec<Instruction> {
    let chars_vec: Vec<_> = input.chars().collect();
    let chars_length = input.len();

    let mut left_pointer = 0;
    let mut right_pointer = MAX_WINDOW_SIZE;

    let mut instructions = Vec::new();

    while left_pointer <= chars_length - MAX_WINDOW_SIZE {
        let window = &chars_vec[left_pointer..right_pointer];

        match Instruction::try_from(window) {
            Ok(i) => instructions.push(i),
            Err(_) => {
                continue;
            }
        }
    }

    // create windows of max size
    chars_vec
        .windows(MAX_WINDOW_SIZE)
        .inspect(|window| println!("{:?}", window))
        .filter_map(|window| Instruction::try_from(window).ok())
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let instructions = parse_input(input);

    let sum = instructions
        .into_iter()
        .inspect(|i| println!("{:?}", i))
        .map(|i| i.execute())
        .inspect(|res| println!("{}", res))
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
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
