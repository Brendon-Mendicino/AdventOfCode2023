use itertools::{self, concat, Itertools};
use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;

fn read(input: &str) -> Result<Vec<(Vec<char>, Vec<usize>)>, Box<dyn Error>> {
    Ok(read_to_string(input)?
        .lines()
        .map(|l| {
            let (springs, count) = l.split_once(" ").unwrap();

            (
                springs.chars().collect::<Vec<_>>(),
                count
                    .split(",")
                    .map(|c| c.parse::<_>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>())
}

fn count_combinations(
    lru_cache: &mut HashMap<(Vec<char>, Vec<usize>), i64>,
    springs: &[char],
    nums: &[usize],
) -> i64 {
    let mut tot_combinations = 0;

    if let Some(tot) = lru_cache.get(&(springs.to_vec(), nums.to_vec())) {
        return *tot;
    }

    match (springs.len(), nums.len()) {
        (_, 0) => return if springs.contains(&'#') { 0 } else { 1 },
        (0, _) => return 0,
        _ => {}
    }

    if ['.', '?'].contains(&springs[0]) {
        tot_combinations += count_combinations(lru_cache, &springs[1..], nums);
    }

    if springs.len() >= nums[0]
        && !springs[..nums[0]].contains(&'.')
        && (springs.len() == nums[0] || springs[nums[0]] != '#')
    {
        tot_combinations += count_combinations(
            lru_cache,
            &springs[(nums[0] + 1).min(springs.len())..],
            &nums[1..],
        );
    }

    lru_cache.insert((springs.to_vec(), nums.to_vec()), tot_combinations);
    tot_combinations
}

pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let parts = read(input)?;
    let mut lru_cache = HashMap::new();

    Ok(parts
        .into_iter()
        .map(|(p, n)| count_combinations(&mut lru_cache, &p, &n))
        .sum())
}

pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    let parts = read(input)?;
    let mut lru_cache = HashMap::new();

    Ok(parts
        .into_iter()
        .map(|(s, n)| {
            (
                (1..5).fold(s.clone(), |v, _| concat([v, vec!['?'], s.clone()])),
                n.iter().cycle().take(n.len() * 5).cloned().collect_vec(),
            )
        })
        .map(|(s, n)| count_combinations(&mut lru_cache, &s, &n))
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(part1("inputs/12input_test").unwrap(), 21);

        assert_eq!(part1("inputs/12input").unwrap(), 7260);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2("inputs/12input_test").unwrap(), 525152);

        assert_eq!(part2("inputs/12input").unwrap(), 1909291258644);
    }
}
