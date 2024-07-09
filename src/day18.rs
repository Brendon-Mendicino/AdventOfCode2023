use std::error::Error;
use std::fs::read_to_string;

use itertools::Itertools;

fn next_dir(c: char) -> (isize, isize) {
    match c {
        'R' => (0, 1),
        'L' => (0, -1),
        'U' => (-1, 0),
        'D' => (1, 0),
        _ => panic!("{}", c),
    }
}

pub fn part1(path: &str) -> Result<usize, Box<dyn Error>> {
    let dirs = read_to_string(path)?
        .lines()
        .map(|l| {
            l.split(' ')
                .take(2)
                .collect_tuple::<(&str, &str)>()
                .unwrap()
        })
        .map(|(a, b)| (a.chars().next().unwrap(), b.parse::<usize>().unwrap()))
        .collect_vec();

    let mut pos = (0, 0);
    let mut sum = 0_isize;

    for (&(dir, step), &(next_d, _)) in dirs.iter().zip(dirs[1..].iter()) {
        let next = next_dir(dir);
        match dir {
            'R' => sum -= pos.0 as isize * step as isize,
            'L' => sum += (pos.0 + 1) as isize * step as isize,
            'D' => sum += step as isize,
            _ => (),
        }

        match (dir, next_d) {
            ('D', 'L') => sum += 1,
            ('L', 'D') => sum -= 1,
            _ => (),
        }

        pos = (
            (pos.0 as isize + next.0 * step as isize) as usize,
            (pos.1 as isize + next.1 * step as isize) as usize,
        );
    }

    Ok(sum as usize)
}

fn convert_digit(d: u32) -> char {
    match d {
        0 => 'R',
        1 => 'D',
        2 => 'L',
        3 => 'U',
        _ => panic!(),
    }
}

pub fn part2(path: &str) -> Result<usize, Box<dyn Error>> {
    let dirs = read_to_string(path)?
        .lines()
        .map(|l| {
            l.split(' ')
                .skip(2)
                .next()
                .map(|hex| &hex[2..hex.len() - 1])
                .map(|hex| {
                    (
                        convert_digit(hex[5..].chars().next().unwrap().to_digit(10).unwrap()),
                        usize::from_str_radix(&hex[..5], 16).unwrap(),
                    )
                })
                .unwrap()
        })
        .collect_vec();

    let mut pos = (0, 0);
    let mut sum = 0_isize;

    for (&(dir, step), &(next_d, _)) in dirs.iter().zip(dirs[1..].iter()) {
        let next = next_dir(dir);
        match dir {
            'R' => sum -= pos.0 as isize * step as isize,
            'L' => sum += (pos.0 + 1) as isize * step as isize,
            'D' => sum += step as isize,
            _ => (),
        }

        match (dir, next_d) {
            ('D', 'L') => sum += 1,
            ('L', 'D') => sum -= 1,
            _ => (),
        }

        pos = (
            (pos.0 as isize + next.0 * step as isize) as usize,
            (pos.1 as isize + next.1 * step as isize) as usize,
        );
    }

    Ok(sum as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(part1("inputs/18input_test").unwrap(), 62);

        assert_eq!(part1("inputs/18input").unwrap(), 35244);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2("inputs/18input_test").unwrap(), 952408144115);

        assert_eq!(part2("inputs/18input").unwrap(), 85070763635666);
    }
}
