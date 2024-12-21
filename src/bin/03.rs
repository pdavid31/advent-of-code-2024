advent_of_code::solution!(3);

#[derive(Debug)]
enum ParseError {
    Incomplete,
    Invalid,
}

#[derive(Debug)]
enum Instruction {
    Multiply(u64, u64),

    Do,
    Dont,
}

enum InstructionType {
    Mul,
    Do,
    Dont,
}

impl Instruction {
    fn execute(&self) -> Option<u64> {
        match self {
            Self::Multiply(a, b) => Some(a * b),

            _ => None,
        }
    }

    fn min_window_size(t: InstructionType) -> usize {
        match t {
            InstructionType::Mul => "mul(0,0)".len(),
            InstructionType::Do => "do()".len(),
            InstructionType::Dont => "don't()".len(),
        }
    }

    fn max_window_size(t: InstructionType) -> usize {
        match t {
            InstructionType::Mul => "mul(000,000)".len(),
            InstructionType::Do => "do()".len(),
            InstructionType::Dont => "don't()".len(),
        }
    }
}

impl TryFrom<&[char]> for Instruction {
    type Error = ParseError;

    fn try_from(value: &[char]) -> Result<Self, Self::Error> {
        let len = value.len();

        if len < 5 {
            return Err(ParseError::Incomplete);
        }

        if value.starts_with(&['d', 'o', 'n', '\'', 't']) {
            match value.get(5).zip(value.get(6)) {
                Some(('(', ')')) => Ok(Instruction::Dont),
                Some(_) => Err(ParseError::Invalid),
                None => Err(ParseError::Incomplete),
            }
        } else if value.starts_with(&['d', 'o']) {
            match value.get(2).zip(value.get(3)) {
                Some(('(', ')')) => Ok(Instruction::Do),
                Some(_) => Err(ParseError::Invalid),
                None => Err(ParseError::Incomplete),
            }
        } else if value.starts_with(&['m', 'u', 'l', '(']) {
            if len < Self::min_window_size(InstructionType::Mul) {
                return Err(ParseError::Incomplete);
            }

            let max_window_size = Self::max_window_size(InstructionType::Mul);
            let position_l_bracket = 3;

            // try to find the closing bracket
            //
            // if we don't find one and the frame is not yet
            // at the `MAX_WINDOW_SIZE`, we assume that it is not
            // yet complete
            //
            // if it is already at the max and there is no closing
            // bracket, it's definetly invalid
            let position_r_bracket = value.iter().position(|c| *c == ')').ok_or({
                if len < max_window_size {
                    ParseError::Incomplete
                } else {
                    ParseError::Invalid
                }
            })?;

            // if we found a closing bracket earlier, but not a separator
            // the frame is invalid
            let position_separator = value
                .iter()
                .position(|c| *c == ',')
                .ok_or(ParseError::Invalid)?;

            // if the separator is not between the brackets, the frame is invalid
            //
            // if there is no room between the bracekts and the separator for a number,
            // the frame is invalid was well
            if position_separator < position_l_bracket
                || position_separator > position_r_bracket
                || position_separator == position_l_bracket + 1
                || position_separator == position_r_bracket - 1
            {
                return Err(ParseError::Incomplete);
            }

            let left_number = value[position_l_bracket + 1..position_separator]
                .iter()
                .collect::<String>()
                .parse()
                .map_err(|_| ParseError::Invalid)?;

            let right_number = value[position_separator + 1..position_r_bracket]
                .iter()
                .collect::<String>()
                .parse()
                .map_err(|_| ParseError::Invalid)?;

            Ok(Self::Multiply(left_number, right_number))
        } else {
            Err(ParseError::Invalid)
        }
    }
}

fn parse_input(input: &str) -> Vec<Instruction> {
    let chars_vec: Vec<_> = input.chars().collect();

    chars_vec
        .into_iter()
        .scan(Vec::new(), |state, c| {
            state.push(c);

            loop {
                match Instruction::try_from(&state[..]) {
                    // if we find a valid instruction, we return that
                    Ok(instruction) => {
                        state.remove(0);

                        break Some(Some(instruction));
                    }

                    // if we find an incomplete frame, we have to buffer more data
                    Err(ParseError::Incomplete) => break Some(None),

                    // if we find an invalid frame, we remove data from the
                    // front and try to parse again
                    Err(ParseError::Invalid) => {
                        state.remove(0);

                        continue;
                    }
                }
            }
        })
        .flatten()
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let instructions = parse_input(input);

    let sum = instructions.into_iter().flat_map(|i| i.execute()).sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let instructions = parse_input(input);

    let sum = instructions
        .into_iter()
        .scan(true, |is_enabled, instruction| match instruction {
            Instruction::Do => {
                *is_enabled = true;
                Some(None)
            }

            Instruction::Dont => {
                *is_enabled = false;
                Some(None)
            }

            other if *is_enabled => Some(Some(other)),

            _ => Some(None),
        })
        .flatten()
        .flat_map(|i| i.execute())
        .sum();

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
