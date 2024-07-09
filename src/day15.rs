use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::fs::read_to_string;

use itertools::Itertools;

fn read_input(path: &str) -> Result<Vec<Vec<u8>>, Box<dyn Error>> {
    Ok(read_to_string(path)?
        .lines()
        .map(|l| {
            l.split(",")
                .map(|c| c.as_bytes().iter().cloned().collect_vec())
                .collect_vec()
        })
        .next()
        .unwrap())
}

pub fn part1(path: &str) -> Result<usize, Box<dyn Error>> {
    let strings = read_input(path)?;

    Ok(strings
        .into_iter()
        .map(|s| {
            let mut curr_value = 0;

            for ch in s {
                curr_value = ((ch as u16 + curr_value as u16) * 17 % 256) as u8;
            }

            curr_value as usize
        })
        .sum())
}

enum Op {
    Dash(Vec<u8>),
    Equal(Vec<u8>, u8),
}

fn hash(s: &[u8]) -> u8 {
    let mut curr_value = 0;

    for &ch in s {
        curr_value = ((ch as u16 + curr_value as u16) * 17 % 256) as u8;
    }

    curr_value
}

pub fn part2(path: &str) -> Result<usize, Box<dyn Error>> {
    let strings = read_to_string(path)?
        .lines()
        .map(|l| {
            l.split(",")
                .map(|s| {
                    if s.contains('=') {
                        let (lab, lens) = s.split_once('=').unwrap();
                        Op::Equal(
                            lab.as_bytes().iter().cloned().collect_vec(),
                            lens.parse::<u8>().unwrap(),
                        )
                    } else {
                        Op::Dash(s[..s.len() - 1].as_bytes().iter().cloned().collect_vec())
                    }
                })
                .collect_vec()
        })
        .next()
        .unwrap();

    let mut boxes = (0..256)
        .map(|_| (HashMap::new(), VecDeque::new()))
        .collect_vec();

    for op in strings {
        match op {
            Op::Equal(lab, lens) => {
                let lab_hash = hash(&lab);
                let boxx = &mut boxes[lab_hash as usize];

                // The label was not in the box
                if let None = boxx.0.insert(lab.clone(), lens) {
                    boxx.1.push_back(lab);
                }
            }
            Op::Dash(lab) => {
                let lab_hash = hash(&lab);
                let boxx = &mut boxes[lab_hash as usize];

                if boxx.0.contains_key(&lab) {
                    let index = boxx
                        .1
                        .iter()
                        .enumerate()
                        .filter(|(_, l)| **l == lab)
                        .map(|(i, _)| i)
                        .next()
                        .unwrap();

                    boxx.1.remove(index);
                    boxx.0.remove(&lab);
                }
            }
        }
    }

    Ok(boxes
        .into_iter()
        .enumerate()
        .map(|(index, (lenses, labels))| {
            labels
                .into_iter()
                .enumerate()
                // .map(|a| {
                //     dbg!((&lenses[&a.1], &a));
                //     a
                // })
                .map(|(i, lab)| lenses[&lab] as usize * (i as usize + 1) * (index as usize + 1))
                .sum::<usize>()
        })
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(part1("inputs/15input_test").unwrap(), 1320);

        assert_eq!(part1("inputs/15input").unwrap(), 516469);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2("inputs/15input_test").unwrap(), 145);

        assert_eq!(part2("inputs/15input").unwrap(), 221627);
    }
}
