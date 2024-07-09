use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;

use itertools::Itertools;

fn read_map(path: &str) -> Result<Vec<Vec<char>>, Box<dyn Error>> {
    Ok(read_to_string(path)?
        .lines()
        .map(|l| l.chars().collect_vec())
        .collect_vec())
}

fn move_north(mut map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    for col in 0..map[0].len() {
        let mut last_pos = 0;

        for row in 0..map.len() {
            if map[row][col] == '#' {
                last_pos = row + 1;
                continue;
            }

            if map[row][col] == 'O' {
                map[row][col] = '.';
                map[last_pos][col] = 'O';
                last_pos += 1;
            }
        }
    }

    map
}

fn move_cycle(mut map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    // North
    for col in 0..map[0].len() {
        let mut last_pos = 0;

        for row in 0..map.len() {
            if map[row][col] == '#' {
                last_pos = row + 1;
                continue;
            }

            if map[row][col] == 'O' {
                map[row][col] = '.';
                map[last_pos][col] = 'O';
                last_pos += 1;
            }
        }
    }

    // West
    for row in 0..map.len() {
        let mut last_pos = 0;

        for col in 0..map[0].len() {
            if map[row][col] == '#' {
                last_pos = col + 1;
                continue;
            }

            if map[row][col] == 'O' {
                map[row][col] = '.';
                map[row][last_pos] = 'O';
                last_pos += 1;
            }
        }
    }

    // South
    for col in 0..map[0].len() {
        let mut last_pos = map.len();

        for row in (0..map.len()).rev() {
            if map[row][col] == '#' {
                last_pos = row;
                continue;
            }

            if map[row][col] == 'O' {
                map[row][col] = '.';
                map[last_pos - 1][col] = 'O';
                last_pos -= 1;
            }
        }
    }

    // East
    for row in 0..map.len() {
        let mut last_pos = map[0].len();

        for col in (0..map[0].len()).rev() {
            if map[row][col] == '#' {
                last_pos = col;
                continue;
            }

            if map[row][col] == 'O' {
                map[row][col] = '.';
                map[row][last_pos - 1] = 'O';
                last_pos -= 1;
            }
        }
    }

    map
}

pub fn part1(path: &str) -> Result<usize, Box<dyn Error>> {
    let map = read_map(path)?;

    let nmap = move_north(map);

    let rows = nmap.len();
    Ok(nmap
        .into_iter()
        .enumerate()
        .map(|(row, col)| col.iter().filter(|c| **c == 'O').count() * (rows - row))
        .sum())
}

pub fn part2(path: &str, cycles: usize) -> Result<usize, Box<dyn Error>> {
    let mut map = read_map(path)?;

    let mut last_pos = HashMap::new();
    last_pos.insert(map.clone(), 0);

    for c in 1..cycles + 1 {
        map = move_cycle(map);

        if let Some(last) = last_pos.insert(map.clone(), c) {
            let clock = c - last;
            let index = ((cycles - c) % clock) + last;

            map = last_pos
                .iter()
                .filter(|(_, value)| **value == index || **value == index + clock)
                .map(|m| m.0)
                .cloned()
                .next()
                .unwrap();

            break;
        }
    }

    let rows = map.len();
    Ok(map
        .into_iter()
        .enumerate()
        .map(|(row, col)| col.iter().filter(|c| **c == 'O').count() * (rows - row))
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(part1("inputs/14input_test").unwrap(), 136);

        assert_eq!(part1("inputs/14input").unwrap(), 111979);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2("inputs/14input_test", 1_000_000_000).unwrap(), 64);

        assert_eq!(part2("inputs/14input", 1_000_000_000).unwrap(), 102055);
    }
}
