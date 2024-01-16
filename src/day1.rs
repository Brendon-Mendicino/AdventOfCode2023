use std::error::Error;
use std::fs::read_to_string;

const FILE: &str = "inputs/1input";

pub fn part1() -> Result<i32, Box<dyn Error>> {
    // First part
    let sum = read_to_string(FILE)?
        .lines()
        .map(|line| {
            format!(
                "{}{}",
                line.chars()
                    .filter(|c| c.is_numeric())
                    .take(1)
                    .collect::<String>(),
                line.chars()
                    .filter(|c| c.is_numeric())
                    .rev()
                    .take(1)
                    .collect::<String>()
            )
        })
        .map(|line| line.parse::<u32>().unwrap())
        .sum::<u32>();

    Ok(sum as i32)
}

pub fn part2() -> Result<i32, Box<dyn Error>> {
    // Second part
    let sum = read_to_string(FILE)?
        .lines()
        .map(|line| {
            line.replace("one", "one1one")
                .replace("two", "two2two")
                .replace("three", "three3three")
                .replace("four", "four4four")
                .replace("five", "five5five")
                .replace("six", "six6six")
                .replace("seven", "seven7seven")
                .replace("eight", "eight8")
                .replace("nine", "nine9nine")
        })
        .map(|line| {
            format!(
                "{}{}",
                line.chars()
                    .filter(|c| c.is_numeric())
                    .take(1)
                    .collect::<String>(),
                line.chars()
                    .filter(|c| c.is_numeric())
                    .rev()
                    .take(1)
                    .collect::<String>()
            )
        })
        .map(|line| line.parse::<u32>().unwrap())
        .sum::<u32>();

    Ok(sum as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        dbg!(part1());
        dbg!(part2());
    }
}