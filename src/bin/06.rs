use std::{collections::HashSet, str::FromStr};

advent_of_code::solution!(6);

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = ();

    fn try_from(s: char) -> Result<Self, Self::Error> {
        match s {
            '^' => Ok(Self::Up),
            'v' => Ok(Self::Down),
            '>' => Ok(Self::Right),
            '<' => Ok(Self::Left),

            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
enum Field {
    Nothing,
    Obstacle,
    Guard(Direction),
}

impl TryFrom<char> for Field {
    type Error = ();

    fn try_from(s: char) -> Result<Self, Self::Error> {
        match s {
            '.' => Ok(Self::Nothing),
            '#' => Ok(Self::Obstacle),
            '^' | 'v' | '<' | '>' => {
                let direction = Direction::try_from(s)?;

                Ok(Self::Guard(direction))
            }

            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
struct Grid {
    grid: Vec<Vec<Field>>,

    initial_guard_position: (usize, usize),
    initial_guard_direction: Direction,

    current_guard_position: (usize, usize),
    current_guard_direction: Direction,
}

impl Grid {
    fn get_guard_info(grid: &[Vec<Field>]) -> Option<((usize, usize), Direction)> {
        for (x, row) in grid.iter().enumerate() {
            for (y, col) in row.iter().enumerate() {
                match col {
                    Field::Guard(direction) => return Some(((x, y), direction.clone())),

                    _ => continue,
                }
            }
        }

        None
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: Vec<_> = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|char| Field::try_from(char).unwrap())
                    .collect()
            })
            .collect();

        let (guard_position, guard_direction) = Self::get_guard_info(&grid).ok_or(())?;

        Ok(Grid {
            grid,

            initial_guard_position: guard_position.clone(),
            initial_guard_direction: guard_direction.clone(),

            current_guard_position: guard_position,
            current_guard_direction: guard_direction,
        })
    }
}

impl Iterator for Grid {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        // update guard position
        let min = 0;
        let max_x = (self.grid.len() - 1) as isize;
        let max_y = (self.grid[0].len() - 1) as isize;

        let current_x = self.current_guard_position.0 as isize;
        let current_y = self.current_guard_position.1 as isize;

        let (delta_x, delta_y): (isize, isize) = match self.current_guard_direction {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        };

        let updated_x = current_x + delta_x;
        let updated_y = current_y + delta_y;

        // check if the guard left the grid by seeing if one of the values left the valid range
        if !(min..=max_x).contains(&updated_x) || !(min..=max_y).contains(&updated_y) {
            return None;
        }

        // if it did not, we have a valid position, thus updating the guard's position
        self.current_guard_position.0 = updated_x as usize;
        self.current_guard_position.1 = updated_y as usize;

        // now we want to check if the next field in direction is an obstacle
        let next_x = updated_x + delta_x;
        let next_y = updated_y + delta_y;

        // check if the guard is at the edge and facing off the grid
        if !(min..=max_x).contains(&next_x) || !(min..=max_y).contains(&next_y) {
            // return early as we don't have to change the direction
            return Some(self.current_guard_position);
        }

        // check if the next field is an obstacle
        let next_field_opt = self
            .grid
            .get(next_x as usize)
            .and_then(|row| row.get(next_y as usize));

        if let Some(Field::Obstacle) = next_field_opt {
            // update the direction
            let next_direction = match self.current_guard_direction {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            };

            self.current_guard_direction = next_direction;
        };

        Some(self.current_guard_position)
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = Grid::from_str(input).unwrap();

    let positions: HashSet<(usize, usize)> = grid
        .into_iter()
        // .inspect(|x| println!("pos: {:?}", x))
        .collect();
    let positions_visited = positions.len();

    Some(positions_visited as u64)
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
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
