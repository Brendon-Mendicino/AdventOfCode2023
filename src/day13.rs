use std::error::Error;
use std::fs::read_to_string;
use std::vec;

use itertools::Itertools;

fn read(path: &str) -> Result<Vec<Vec<Vec<char>>>, Box<dyn Error>> {
    Ok(read_to_string(path)?
        .lines()
        .fold(vec![Vec::new()], |mut list, line| {
            if line.is_empty() {
                list.push(vec![]);
            } else {
                let last = list.len() - 1;
                list[last].push(line.chars().collect_vec());
            }

            list
        }))
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Pos {
    Oriz(usize),
    Vert(usize),
}

fn check_vert(map: &[Vec<char>], similar: &mut Vec<usize>) -> Option<usize> {
    if similar.len() == 0 {
        return None;
    }

    if map.len() == 0 {
        return if similar.len() == 1 {
            Some(similar[0])
        } else {
            None
        };
    }

    let mut to_remove = vec![];

    for index in 0..similar.len() {
        let s = similar[index];

        let (mut first, mut second) = map[0].split_at(s);

        let min_len = first.len().min(second.len());

        first = &first[first.len() - min_len..];
        second = &second[..min_len];

        assert_eq!(first.len(), second.len());

        let second_rev = second.iter().rev().cloned().collect_vec();
        if first != second_rev.as_slice() {
            to_remove.push(s);
        }
    }

    similar.retain(|s| !to_remove.contains(s));

    return check_vert(&map[1..], similar);
}

fn check_vert_part2(map: &[Vec<char>]) -> Option<usize> {
    for reflection in 1..map[0].len() {
        let tot = map
            .iter()
            .map(|row| {
                let (mut first, mut second) = row.split_at(reflection);

                let min_len = first.len().min(second.len());

                first = &first[first.len() - min_len..];
                second = &second[..min_len];

                assert_eq!(first.len(), second.len());

                let second_rev = second.iter().rev().cloned().collect_vec();
                first
                    .iter()
                    .zip(&second_rev)
                    .map(|(f, s)| if f == s { 0 } else { 1 })
                    .sum::<usize>()
            })
            .sum::<usize>();

        if tot == 1 {
            return Some(reflection);
        }
    }

    None
}

fn get_refl(map: Vec<Vec<char>>) -> Option<Pos> {
    let mut similar_vert = (1..map[0].len()).collect_vec();

    if let Some(v) = check_vert(&map, &mut similar_vert) {
        return Some(Pos::Vert(v));
    }

    // Transpose map
    let map_t = {
        let len = map[0].len();
        let mut iters = map.into_iter().map(|n| n.into_iter()).collect_vec();

        (0..len)
            .map(|_| iters.iter_mut().map(|n| n.next().unwrap()).collect_vec())
            .collect_vec()
    };

    let mut similar_oriz = (1..map_t[0].len()).collect_vec();
    if let Some(o) = check_vert(&map_t, &mut similar_oriz) {
        return Some(Pos::Oriz(o));
    }

    None
}

fn get_refl_part2(map: Vec<Vec<char>>) -> Option<Pos> {
    if let Some(v) = check_vert_part2(&map) {
        return Some(Pos::Vert(v));
    }

    // Transpose map
    let map_t = {
        let len = map[0].len();
        let mut iters = map.into_iter().map(|n| n.into_iter()).collect_vec();

        (0..len)
            .map(|_| iters.iter_mut().map(|n| n.next().unwrap()).collect_vec())
            .collect_vec()
    };

    if let Some(o) = check_vert_part2(&map_t) {
        return Some(Pos::Oriz(o));
    }

    None
}

pub fn part1(path: &str) -> Result<usize, Box<dyn Error>> {
    let maps_list = read(path)?;

    let (v, o) = maps_list
        .into_iter()
        .map(|map| get_refl(map).unwrap())
        .fold((vec![], vec![]), |mut vecs, pos| {
            match pos {
                Pos::Vert(v) => vecs.0.push(v),
                Pos::Oriz(o) => vecs.1.push(o),
            }
            vecs
        });

    Ok(o.iter().sum::<usize>() * 100 + v.iter().sum::<usize>())
}

pub fn part2(path: &str) -> Result<usize, Box<dyn Error>> {
    let maps_list = read(path)?;

    let (v, o) = maps_list
        .into_iter()
        .map(|map| get_refl_part2(map).unwrap())
        .fold((vec![], vec![]), |mut vecs, pos| {
            match pos {
                Pos::Vert(v) => vecs.0.push(v),
                Pos::Oriz(o) => vecs.1.push(o),
            }
            vecs
        });

    Ok(o.iter().sum::<usize>() * 100 + v.iter().sum::<usize>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(part1("inputs/13input_test").unwrap(), 405);

        assert_eq!(part1("inputs/13input").unwrap(), 34100);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2("inputs/13input_test").unwrap(), 400);

        assert_eq!(part2("inputs/13input").unwrap(), 33106);
    }
}
