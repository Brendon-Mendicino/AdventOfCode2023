use std::vec;
use std::{error::Error, fs::read_to_string};

const FILE: &str = "input";

fn add(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    (a.0 + b.0, a.1 + b.1)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Go {
    N,
    S,
    W,
    E,
}

impl Go {
    fn to_vec(self) -> (i32, i32) {
        match self {
            Self::E => (1, 0),
            Self::W => (-1, 0),
            Self::N => (0, -1),
            Self::S => (0, 1),
        }
    }

    fn inv(self) -> Self {
        match self {
            Self::E => Self::W,
            Self::W => Self::E,
            Self::N => Self::S,
            Self::S => Self::N,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum State {
    Mark(Dir),
    Norm(Dir),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Dir {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    G,
    S,
}

impl Dir {
    fn dir(self) -> Vec<Go> {
        match self {
            Dir::NS => vec![Go::N, Go::S],
            Dir::NE => vec![Go::N, Go::E],
            Dir::EW => vec![Go::W, Go::E],
            Dir::NW => vec![Go::N, Go::W],
            Dir::SE => vec![Go::E, Go::S],
            Dir::SW => vec![Go::W, Go::S],
            _ => vec![],
        }
    }

    fn next(self, from: Go) -> Option<Go> {
        let dir = self.dir();
        if !dir.contains(&from.inv()) {
            return None;
        }

        if dir[0] == from.inv() {
            Some(dir[1])
        } else {
            Some(dir[0])
        }
    }
}

impl From<char> for Dir {
    fn from(value: char) -> Self {
        match value {
            '|' => Self::NS,
            '-' => Self::EW,
            'L' => Self::NE,
            'J' => Self::NW,
            '7' => Self::SW,
            'F' => Self::SE,
            '.' => Self::G,
            'S' => Self::S,
            _ => panic!(),
        }
    }
}

fn part_one() -> Result<(), Box<dyn Error>> {
    let mut start = (0, 0);
    let map = read_to_string(FILE)?
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| {
                    let d = Dir::from(c);
                    if let Dir::S = d {
                        start = (col as i32, row as i32);
                    }
                    d
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let start = start;

    let y_range = 0..map.len() as i32;
    let x_range = 0..map[0].len() as i32;
    let possible = vec![Go::E, Go::W, Go::S, Go::N];
    let mut len = 0;

    'new_start: for mut go in possible {
        let mut next = add(start, go.to_vec());
        len = 0;

        while x_range.contains(&next.0) && y_range.contains(&next.1) {
            let to = map[next.1 as usize][next.0 as usize];

            if let Dir::S = to {
                break 'new_start;
            }

            let Some(step) = to.next(go) else {
                continue 'new_start;
            };

            go = step;
            next = add(next, go.to_vec());

            len += 1;
        }
    }

    let half_way = (len + 1) / 2;

    println!("{}", half_way);

    Ok(())
}

fn part_two() -> Result<(), Box<dyn Error>> {
    let mut start = (0, 0);
    let map = read_to_string(FILE)?
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| {
                    let d = Dir::from(c);
                    if let Dir::S = d {
                        start = (col as i32, row as i32);
                    }
                    d
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let start = start;

    let y_range = 0..map.len() as i32;
    let x_range = 0..map[0].len() as i32;
    let possible = vec![Go::E, Go::W, Go::S, Go::N];
    let mut starting_go = possible[0];

    // Find the loop
    'new_start: for mut go in possible {
        let mut next = add(start, go.to_vec());
        starting_go = go;

        while x_range.contains(&next.0) && y_range.contains(&next.1) {
            let to = map[next.1 as usize][next.0 as usize];

            if let Dir::S = to {
                break 'new_start;
            }

            let Some(step) = to.next(go) else {
                continue 'new_start;
            };

            go = step;
            next = add(next, go.to_vec());
        }
    }

    // Mark the loop
    let y_range = 0..map.len() as i32;
    let x_range = 0..map[0].len() as i32;

    // Find the loop
    let mut go = starting_go;
    let mut next = add(start, go.to_vec());
    let mut new_map = map
        .iter()
        .map(|d| d.iter().map(|dir| State::Norm(*dir)).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    while x_range.contains(&next.0) && y_range.contains(&next.1) {
        let to = map[next.1 as usize][next.0 as usize];

        new_map[next.1 as usize][next.0 as usize] = State::Mark(to);

        if let Dir::S = to {
            // Substitute S
            new_map[next.1 as usize][next.0 as usize] = match (go.inv(), starting_go) {
                (Go::N, Go::S) | (Go::S, Go::N) => State::Mark(Dir::NS),
                (Go::E, Go::W) | (Go::W, Go::E) => State::Mark(Dir::EW),
                (Go::N, Go::E) | (Go::E, Go::N) => State::Mark(Dir::NE),
                (Go::N, Go::W) | (Go::W, Go::N) => State::Mark(Dir::NW),
                (Go::S, Go::W) | (Go::W, Go::S) => State::Mark(Dir::SW),
                (Go::S, Go::E) | (Go::E, Go::S) => State::Mark(Dir::SE),
                _ => panic!("{:?}", (go.inv(), starting_go)),
            };
            break;
        }

        let Some(step) = to.next(go) else {
            panic!();
        };

        go = step;
        next = add(next, go.to_vec());
    }

    let enclosed = find_enclosed(new_map);

    println!("{}", enclosed);

    Ok(())
}

fn find_enclosed(map: Vec<Vec<State>>) -> i32 {
    let mut enclosed = 0;

    for row in 0..map.len() {
        let mut inside = false;
        let mut prev_curve = None;

        for col in 0..map[0].len() {
            match map[row][col] {
                State::Mark(pipe) => {
                    match pipe {
                        Dir::EW => {},
                        Dir::NS => inside = !inside,
                        Dir::NE => prev_curve = Some(Dir::NE),
                        Dir::SE => prev_curve = Some(Dir::SE),
                        Dir::NW if matches!(prev_curve, Some(Dir::SE)) => {
                            prev_curve = None;
                            inside = !inside;
                        },
                        Dir::NW if matches!(prev_curve, Some(Dir::NE)) => prev_curve = None,
                        Dir::SW if matches!(prev_curve, Some(Dir::NE)) => {
                            prev_curve = None;
                            inside = !inside;
                        },
                        Dir::SW if matches!(prev_curve, Some(Dir::SE)) => prev_curve = None,
                        _ => panic!("{:?}", (pipe, prev_curve, row, col)),
                    }

                    continue;
                },
                State::Norm(_) => {},
            }
            
            if inside {
                enclosed += 1;
            }
        }
    }

    enclosed
}

fn main() -> Result<(), Box<dyn Error>> {
    part_one()?;
    part_two()?;

    Ok(())
}
