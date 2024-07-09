use std::collections::HashSet;
use std::error::Error;
use std::fs::read_to_string;

use itertools::Itertools;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
}

fn next_dir(dir: (i32, i32, Dir)) -> (i32, i32, Dir) {
    match dir.2 {
        Dir::Down => (dir.0 + 1, dir.1, dir.2),
        Dir::Up => (dir.0 - 1, dir.1, dir.2),
        Dir::Right => (dir.0, dir.1 + 1, dir.2),
        Dir::Left => (dir.0, dir.1 - 1, dir.2),
    }
}

fn dfs_energize(
    dir: (i32, i32, Dir),
    map: &Vec<Vec<char>>,
    energized: &mut Vec<Vec<bool>>,
    cache: &mut HashSet<(i32, i32, Dir)>,
) {
    if !(0..map.len()).contains(&(dir.0 as usize)) || !(0..map[0].len()).contains(&(dir.1 as usize))
    {
        return;
    }

    if !cache.insert(dir) {
        return;
    }

    energized[dir.0 as usize][dir.1 as usize] = true;
    let pos = map[dir.0 as usize][dir.1 as usize];

    if pos == '.' {
        return dfs_energize(next_dir(dir), map, energized, cache);
    }

    if pos == '-' {
        return match dir.2 {
            Dir::Left | Dir::Right => dfs_energize(next_dir(dir), map, energized, cache),
            Dir::Up | Dir::Down => {
                dfs_energize((dir.0, dir.1 + 1, Dir::Right), map, energized, cache);
                dfs_energize((dir.0, dir.1 - 1, Dir::Left), map, energized, cache);
            }
        };
    }

    if pos == '|' {
        return match dir.2 {
            Dir::Up | Dir::Down => dfs_energize(next_dir(dir), map, energized, cache),
            Dir::Right | Dir::Left => {
                dfs_energize((dir.0 + 1, dir.1, Dir::Down), map, energized, cache);
                dfs_energize((dir.0 - 1, dir.1, Dir::Up), map, energized, cache);
            }
        };
    }

    if pos == '/' {
        return match dir.2 {
            Dir::Up => dfs_energize((dir.0, dir.1 + 1, Dir::Right), map, energized, cache),
            Dir::Down => dfs_energize((dir.0, dir.1 - 1, Dir::Left), map, energized, cache),
            Dir::Left => dfs_energize((dir.0 + 1, dir.1, Dir::Down), map, energized, cache),
            Dir::Right => dfs_energize((dir.0 - 1, dir.1, Dir::Up), map, energized, cache),
        };
    }

    if pos == '\\' {
        return match dir.2 {
            Dir::Up => dfs_energize((dir.0, dir.1 - 1, Dir::Left), map, energized, cache),
            Dir::Down => dfs_energize((dir.0, dir.1 + 1, Dir::Right), map, energized, cache),
            Dir::Left => dfs_energize((dir.0 - 1, dir.1, Dir::Up), map, energized, cache),
            Dir::Right => dfs_energize((dir.0 + 1, dir.1, Dir::Down), map, energized, cache),
        };
    }

    panic!();
}

// fn dfs_energize(
//     dir: (i32, i32, Dir),
//     map: &Vec<Vec<char>>,
//     cache: &mut HashMap<(i32, i32, Dir), HashSet<(i32, i32)>>,
// ) -> HashSet<(i32, i32)> {
//     if !(0..map.len()).contains(&(dir.0 as usize)) || !(0..map[0].len()).contains(&(dir.1 as usize))
//     {
//         return HashSet::new();
//     }

//     if let Some(prev_pos) = cache.get(&dir) {
//         return prev_pos.clone();
//     }

//     cache.insert(dir, HashSet::new());
//     let pos = map[dir.0 as usize][dir.1 as usize];

//     let mut ener = match pos {
//         '.' => dfs_energize(next_dir(dir), map, cache),
//         '-' => match dir.2 {
//             Dir::Left | Dir::Right => dfs_energize(next_dir(dir), map, cache),
//             Dir::Up | Dir::Down => dfs_energize((dir.0, dir.1 + 1, Dir::Right), map, cache)
//                 .union(&dfs_energize((dir.0, dir.1 - 1, Dir::Left), map, cache))
//                 .cloned()
//                 .collect(),
//         },

//         '|' => match dir.2 {
//             Dir::Up | Dir::Down => dfs_energize(next_dir(dir), map, cache),
//             Dir::Right | Dir::Left => dfs_energize((dir.0 + 1, dir.1, Dir::Down), map, cache)
//                 .union(&dfs_energize((dir.0 - 1, dir.1, Dir::Up), map, cache))
//                 .cloned()
//                 .collect(),
//         },

//         '/' => match dir.2 {
//             Dir::Up => dfs_energize((dir.0, dir.1 + 1, Dir::Right), map, cache),
//             Dir::Down => dfs_energize((dir.0, dir.1 - 1, Dir::Left), map, cache),
//             Dir::Left => dfs_energize((dir.0 + 1, dir.1, Dir::Down), map, cache),
//             Dir::Right => dfs_energize((dir.0 - 1, dir.1, Dir::Up), map, cache),
//         },

//         '\\' => match dir.2 {
//             Dir::Up => dfs_energize((dir.0, dir.1 - 1, Dir::Left), map, cache),
//             Dir::Down => dfs_energize((dir.0, dir.1 + 1, Dir::Right), map, cache),
//             Dir::Left => dfs_energize((dir.0 - 1, dir.1, Dir::Up), map, cache),
//             Dir::Right => dfs_energize((dir.0 + 1, dir.1, Dir::Down), map, cache),
//         },
//         _ => panic!("{}", pos),
//     };

//     ener.insert((dir.0, dir.1));
//     cache.insert(dir, ener.clone());
//     ener
// }

// fn dfs_energize(
//     dir: (i32, i32, Dir),
//     map: &Vec<Vec<char>>,
//     cache: &mut HashMap<(i32, i32, Dir), HashSet<(i32, i32)>>,
// )  {
//     let mut queue = VecDeque::from([dir]);
//     let mut later_tiles = HashSet::new();

//     while let Some(dir) = queue.pop_back() {
//         if !(0..map.len()).contains(&(dir.0 as usize)) || !(0..map[0].len()).contains(&(dir.1 as usize))
//         {
//             later_tiles = HashSet::new();
//             continue;
//         }

//         if let Some(prev_pos) = cache.get(&dir) {
//             later_tiles = prev_pos.clone();
//             continue;
//         }

//         cache.insert(dir, HashSet::new());
//         let pos = map[dir.0 as usize][dir.1 as usize];

//         let mut ener = match pos {
//             '.' => next_dir(dir), map, cache),
//             '-' => match dir.2 {
//                 Dir::Left | Dir::Right => dfs_energize(next_dir(dir), map, cache),
//                 Dir::Up | Dir::Down => dfs_energize((dir.0, dir.1 + 1, Dir::Right), map, cache)
//                     .union(&dfs_energize((dir.0, dir.1 - 1, Dir::Left), map, cache))
//                     .cloned()
//                     .collect(),
//             },

//             '|' => match dir.2 {
//                 Dir::Up | Dir::Down => dfs_energize(next_dir(dir), map, cache),
//                 Dir::Right | Dir::Left => dfs_energize((dir.0 + 1, dir.1, Dir::Down), map, cache)
//                     .union(&dfs_energize((dir.0 - 1, dir.1, Dir::Up), map, cache))
//                     .cloned()
//                     .collect(),
//             },

//             '/' => match dir.2 {
//                 Dir::Up => dfs_energize((dir.0, dir.1 + 1, Dir::Right), map, cache),
//                 Dir::Down => dfs_energize((dir.0, dir.1 - 1, Dir::Left), map, cache),
//                 Dir::Left => dfs_energize((dir.0 + 1, dir.1, Dir::Down), map, cache),
//                 Dir::Right => dfs_energize((dir.0 - 1, dir.1, Dir::Up), map, cache),
//             },

//             '\\' => match dir.2 {
//                 Dir::Up => dfs_energize((dir.0, dir.1 - 1, Dir::Left), map, cache),
//                 Dir::Down => dfs_energize((dir.0, dir.1 + 1, Dir::Right), map, cache),
//                 Dir::Left => dfs_energize((dir.0 - 1, dir.1, Dir::Up), map, cache),
//                 Dir::Right => dfs_energize((dir.0 + 1, dir.1, Dir::Down), map, cache),
//             },
//             _ => panic!("{}", pos),
//         };

//         ener.insert((dir.0, dir.1));
//         cache.insert(dir, ener.clone());
//         ener
//     }
// }

pub fn part1(path: &str) -> Result<usize, Box<dyn Error>> {
    let map = read_to_string(path)?
        .lines()
        .map(|l| l.chars().collect_vec())
        .collect_vec();

    let mut cache = HashSet::new();
    let mut e = (0..map.len())
        .map(|_| (0..map[0].len()).map(|_| false).collect_vec())
        .collect_vec();

    dfs_energize((0, 0, Dir::Right), &map, &mut e, &mut cache);

    // Ok(cache.values().map(HashSet::len).max().unwrap())
    Ok(e.iter().flatten().filter(|f| **f).count())
}

pub fn part2(path: &str) -> Result<usize, Box<dyn Error>> {
    let map = read_to_string(path)?
        .lines()
        .map(|l| l.chars().collect_vec())
        .collect_vec();

    Ok((0..map.len() as i32)
        .map(|i| vec![(i, 0, Dir::Right), (i, map[0].len() as i32 - 1, Dir::Left)])
        .flatten()
        .chain(
            (0..map[0].len() as i32)
                .map(|i| vec![(0, i, Dir::Down), (map.len() as i32 - 1, i, Dir::Up)])
                .flatten(),
        )
        .map(|dir| {
            let mut e = (0..map.len())
                .map(|_| (0..map[0].len()).map(|_| false).collect_vec())
                .collect_vec();
            let mut cache = HashSet::new();
            dfs_energize(dir, &map, &mut e, &mut cache);
            e.iter().flatten().filter(|f| **f).count()
        })
        .max()
        .unwrap())
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(part1("inputs/16input_test").unwrap(), 46);

        assert_eq!(part1("inputs/16input").unwrap(), 6816);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2("inputs/16input_test").unwrap(), 51);

        assert_eq!(part2("inputs/16input").unwrap(), 8163);
    }
}
