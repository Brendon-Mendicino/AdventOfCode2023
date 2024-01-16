use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;
use std::vec;

#[derive(Debug)]
struct MaybeGear {
    num: usize, 
    gear_pos: (usize, usize),
}

/// An index that can wrap around text in a line.
#[derive(Debug)]
struct WrapIndex {
    row: usize,
    col: usize,
    len: usize,
    dir: Dir,
    pos: (isize, isize),
}

#[derive(Debug)]
enum Dir {
    N,
    S,
    E,
    W,
}

impl WrapIndex {
    fn new(row: usize, col: usize, len: usize) -> Self {
        Self {
            row,
            col,
            len,
            dir: Dir::E,
            pos: (col as isize - 1, row as isize - 1),
        }
    }
}

impl Iterator for WrapIndex {
    type Item = (isize, isize);

    fn next(&mut self) -> Option<Self::Item> {
        if matches!(self.dir, Dir::N) && self.pos == (self.col as isize - 1, self.row as isize - 1)
        {
            return None;
        }

        let ret = self.pos;

        // State machine
        let top_end: (isize, isize) = (
            self.col as isize - 1 + self.len as isize + 1,
            self.row as isize - 1,
        );
        if matches!(self.dir, Dir::E) && self.pos == top_end {
            self.dir = Dir::S;
        }

        let bot_end = (
            self.col as isize - 1 + self.len as isize + 1,
            self.row as isize - 1 + 2,
        );
        if matches!(self.dir, Dir::S) && self.pos == bot_end {
            self.dir = Dir::W;
        }

        let bot_start = (self.col as isize - 1, self.row as isize - 1 + 2);
        if matches!(self.dir, Dir::W) && self.pos == bot_start {
            self.dir = Dir::N;
        }

        // Move pos
        match self.dir {
            Dir::N => self.pos.1 -= 1,
            Dir::S => self.pos.1 += 1,
            Dir::W => self.pos.0 -= 1,
            Dir::E => self.pos.0 += 1,
        }

        Some(ret)
    }
}

fn read_matrix() -> Result<Vec<Vec<u8>>, std::io::Error> {
    let ret = read_to_string("input")?
        .lines()
        .map(|l| {
            l.to_string()
                .as_bytes()
                .into_iter()
                .map(|a| *a)
                .collect::<Vec<u8>>()
        })
        .collect();

    Ok(ret)
}

fn scan_around_number(row: usize, col: usize, len: usize, matrix: &[Vec<u8>]) -> Option<usize> {
    for (c, r) in WrapIndex::new(row, col, len) {
        if !(0..matrix.len() as isize).contains(&r) || !(0..matrix[0].len() as isize).contains(&c) {
            continue;
        }

        let point = matrix[r as usize][c as usize];
        if point.is_ascii_punctuation() && point != b'.' {
            return match std::str::from_utf8(&matrix[row][col..col + len]) {
                Ok(n) => Some(n.parse().unwrap()),
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            };
        }
    }

    None
}

fn scan_for_gears(row: usize, col: usize, len: usize, matrix: &[Vec<u8>]) -> Option<MaybeGear> {
    for (c, r) in WrapIndex::new(row, col, len) {
        if !(0..matrix.len() as isize).contains(&r) || !(0..matrix[0].len() as isize).contains(&c) {
            continue;
        }

        let point = matrix[r as usize][c as usize];
        if point == b'*' {
            return match std::str::from_utf8(&matrix[row][col..col + len]) {
                Ok(n) => Some(MaybeGear {
                    num: n.parse().unwrap(),
                    gear_pos: (r as usize, c as usize),
                }),
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            };
        }
    }

    None
}

pub fn day3() -> Result<(), Box<dyn Error>> {
    let matrix = read_matrix()?;
    let mut parts = vec![];
    let width = matrix[0].len();
    let height = matrix.len();

    // Part one
    for row in 0..height {
        let mut col_range = 0..width;
        while let Some(col) = col_range.next() {
            // Check for any number and check it's length
            let mut len = 0;
            while col + len < width && matrix[row][col + len].is_ascii_digit() {
                len += 1;
            }

            if len != 0 {
                scan_around_number(row, col, len, &matrix)
                    .into_iter()
                    .for_each(|part| parts.push(part));

            }
            
            col_range = col + 1 + len..width;
        }
    }

    println!("{}", parts.iter().sum::<usize>());

    // --------------------------------------------------------
    // Part two
    // --------------------------------------------------------
    let mut gears = vec![];

    for row in 0..height {
        let mut col_range = 0..width;
        while let Some(col) = col_range.next() {
            // Check for any number and check it's length
            let mut len = 0;
            while col + len < width && matrix[row][col + len].is_ascii_digit() {
                len += 1;
            }

            if len != 0 {
                scan_for_gears(row, col, len, &matrix)
                    .into_iter()
                    .for_each(|gear| gears.push(gear));

            }
            
            col_range = col + 1 + len..width;
        }
    }

    // Group gears
    let gears_len = gears.len();
    let final_ratio = gears.into_iter()
        // Group by '*' position
        .fold(HashMap::<(usize, usize), Vec<usize>>::with_capacity(gears_len), |mut set, gear| {
            set.entry(gear.gear_pos)
                .or_default()
                .push(gear.num);

            set
        })
        .into_iter()
        // Filter non-coupled gears
        .filter_map(|(_, gears)| {
            match gears[..] {
                [first, second] => Some(first * second),
                _ => None,
            }
        })
        .sum::<usize>();

    println!("{}", final_ratio);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::day3;

    #[test]
    fn testt() {
        day3();
    }
}