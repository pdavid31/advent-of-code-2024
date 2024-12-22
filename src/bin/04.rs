advent_of_code::solution!(4);

fn is_valid_coordinate(x: isize, y: isize, max_x: isize, max_y: isize) -> bool {
    (0..max_x).contains(&x) && (0..max_y).contains(&y)
}

fn find_word_in_direction(
    grid: &[Vec<char>],
    max_x: isize,
    max_y: isize,
    word: &[char],
    index: usize,
    x: isize,
    y: isize,
    dir_x: isize,
    dir_y: isize,
) -> bool {
    if index == word.len() {
        return true;
    }

    if is_valid_coordinate(x, y, max_x, max_y) && word[index] == grid[x as usize][y as usize] {
        return find_word_in_direction(
            grid,
            max_x,
            max_y,
            word,
            index + 1,
            x + dir_x,
            y + dir_y,
            dir_x,
            dir_y,
        );
    }

    false
}

fn search_word(grid: Vec<Vec<char>>, word: &str) -> Vec<(usize, usize)> {
    let mut indices = Vec::new();

    let max_x = grid.len();
    let max_y = grid.first().map(|column| column.len()).unwrap_or(0);

    let characters: Vec<char> = word.chars().collect();

    let directions: Vec<(isize, isize)> = vec![
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];

    for i in 0..max_x {
        for j in 0..max_y {
            if grid[i][j] == characters[0] {
                for (dir_x, dir_y) in &directions {
                    if find_word_in_direction(
                        &grid,
                        max_x as isize,
                        max_y as isize,
                        &characters,
                        0,
                        i as isize,
                        j as isize,
                        *dir_x,
                        *dir_y,
                    ) {
                        indices.push((i, j));
                    }
                }
            }
        }
    }

    indices
}

fn search_word_cross(grid: Vec<Vec<char>>, word: &str) -> Vec<(usize, usize)> {
    let mut indices = Vec::new();

    let max_x = grid.len();
    let max_y = grid.first().map(|column| column.len()).unwrap_or(0);

    let characters: Vec<char> = word.chars().collect();

    let directions: Vec<(isize, isize)> = vec![
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];

    for i in 0..max_x {
        for j in 0..max_y {
            if grid[i][j] == characters[1] {
                let matches: Vec<_> = directions
                    .iter()
                    .map(|(dir_x, dir_y)| {
                        find_word_in_direction(
                            &grid,
                            max_x as isize,
                            max_y as isize,
                            &characters,
                            0,
                            i as isize - dir_x,
                            j as isize - dir_y,
                            *dir_x,
                            *dir_y,
                        )
                    })
                    .collect();

                // look for horizontal and vertical matches
                let is_horizontal_vertical_match =
                    (matches[0] || matches[1]) && (matches[2] || matches[3]);
                let is_diagonal_match = (matches[4] || matches[7]) && (matches[5] || matches[6]);

                if is_horizontal_vertical_match || is_diagonal_match {
                    indices.push((i, j));
                }
            }
        }
    }

    indices
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let data = parse_input(input);
    let word = "XMAS";

    let count = search_word(data, word).len();

    Some(count as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let data = parse_input(input);
    let word = "MAS";

    let count = search_word_cross(data, word).len();

    Some(count as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
