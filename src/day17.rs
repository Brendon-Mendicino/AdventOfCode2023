use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::error::Error;
use std::fs::read_to_string;

use itertools::{concat, Itertools};

#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn inv(self) -> Self {
        match self {
            Self::Down => Self::Up,
            Self::Up => Self::Down,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }

    fn next_moves(self) -> Vec<Self> {
        let moves = vec![Self::Up, Self::Down, Self::Left, Self::Right];

        moves
            .into_iter()
            .filter(|&m| m != self && m != self.inv())
            .collect_vec()
    }

    fn next_pos(self) -> (i32, i32) {
        match self {
            Self::Down => (1, 0),
            Self::Up => (-1, 0),
            Self::Right => (0, 1),
            Self::Left => (0, -1),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct State {
    cost: usize,
    position: (usize, usize),
    repetition: usize,
    dir: Dir,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
            .then_with(|| self.dir.cmp(&other.dir))
            .then_with(|| self.repetition.cmp(&other.repetition))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Dijkstra's shortest path algorithm.

// Start at `start` and use `dist` to track the current shortest distance
// to each node. This implementation isn't memory-efficient as it may leave duplicate
// nodes in the queue. It also uses `usize::MAX` as a sentinel value,
// for a simpler implementation.
fn shortest_path(map: &Vec<Vec<usize>>, start: (usize, usize), goal: (usize, usize)) -> usize {
    let mut heap = BinaryHeap::new();
    let mut seen = HashSet::new();

    // We're at `start`, with a zero cost
    heap.push(State {
        cost: 0,
        position: start,
        repetition: 0,
        dir: Dir::Right,
    });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State {
        cost,
        position,
        repetition,
        dir,
    }) = heap.pop()
    {
        // Alternatively we could have continued to find all shortest paths
        if position == goal {
            return cost;
        }

        // Important as we may have already found a better way
        if seen.contains(&(position, repetition, dir)) {
            continue;
        }

        seen.insert((position, repetition, dir));

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for next_dir in
            dir.next_moves()
                .iter()
                .chain(if repetition != 3 { vec![&dir] } else { vec![] })
        {
            let next_pos = (
                match position.0 as i32 + next_dir.next_pos().0 {
                    x if (0..map.len() as i32).contains(&x) => x as usize,
                    _ => continue,
                },
                match position.1 as i32 + next_dir.next_pos().1 {
                    x if (0..map[0].len() as i32).contains(&x) => x as usize,
                    _ => continue,
                },
            );

            let next = State {
                cost: cost + map[next_pos.0][next_pos.1],
                position: next_pos,
                repetition: if dir == *next_dir { repetition + 1 } else { 1 },
                dir: *next_dir,
            };

            heap.push(next);
        }
    }

    // Goal not reachable
    panic!();
}

fn shortest_path_ultra_crucible(
    map: &Vec<Vec<usize>>,
    start: (usize, usize),
    goal: (usize, usize),
) -> usize {
    let mut heap = BinaryHeap::new();
    let mut seen = HashSet::new();

    // We're at `start`, with a zero cost
    heap.push(State {
        cost: 0,
        position: start,
        repetition: 0,
        dir: Dir::Right,
    });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State {
        cost,
        position,
        repetition,
        dir,
    }) = heap.pop()
    {
        // Alternatively we could have continued to find all shortest paths
        if position == goal && repetition >= 4 {
            return cost;
        }

        // Important as we may have already found a better way
        if seen.contains(&(position, repetition, dir)) {
            continue;
        }

        seen.insert((position, repetition, dir));

        let iter = match repetition {
            0..=3 => vec![dir],
            4..=9 => concat(vec![vec![dir], dir.next_moves()]),
            _ => dir.next_moves(),
        };

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for next_dir in iter {
            let next_pos = (
                match position.0 as i32 + next_dir.next_pos().0 {
                    x if (0..map.len() as i32).contains(&x) => x as usize,
                    _ => continue,
                },
                match position.1 as i32 + next_dir.next_pos().1 {
                    x if (0..map[0].len() as i32).contains(&x) => x as usize,
                    _ => continue,
                },
            );

            let next = State {
                cost: cost + map[next_pos.0][next_pos.1],
                position: next_pos,
                repetition: if dir == next_dir { repetition + 1 } else { 1 },
                dir: next_dir,
            };

            heap.push(next);
        }
    }

    // Goal not reachable
    panic!();
}

pub fn part1(path: &str) -> Result<usize, Box<dyn Error>> {
    let map = read_to_string(path)?
        .lines()
        .map(|l| {
            l.chars()
                .map(|a| a.to_digit(10).unwrap() as usize)
                .collect_vec()
        })
        .collect_vec();

    Ok(shortest_path(
        &map,
        (0, 0),
        (map.len() - 1, map[0].len() - 1),
    ))
}

pub fn part2(path: &str) -> Result<usize, Box<dyn Error>> {
    let map = read_to_string(path)?
        .lines()
        .map(|l| {
            l.chars()
                .map(|a| a.to_digit(10).unwrap() as usize)
                .collect_vec()
        })
        .collect_vec();

    Ok(shortest_path_ultra_crucible(
        &map,
        (0, 0),
        (map.len() - 1, map[0].len() - 1),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(part1("inputs/17input_test").unwrap(), 102);

        assert_eq!(part1("inputs/17input").unwrap(), 861);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2("inputs/17input_test2").unwrap(), 71);
        assert_eq!(part2("inputs/17input_test").unwrap(), 94);

        assert_eq!(part2("inputs/17input").unwrap(), 1037);
    }
}
