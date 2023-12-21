use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    // First part
    let sum = read_to_string("input")?
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

    println!("{}", sum);

    // Second part
    let sum = read_to_string("input")?
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

    println!("{}", sum);

    Ok(())
}
