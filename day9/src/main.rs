use std::{error::Error, fs::read_to_string};

const FILE: &str = "input";

fn next_value(history: Vec<i32>) -> i32 {
    let mut subtractions = vec![history];
    let mut curr = 0;

    while !subtractions[curr].iter().all(|&n| n == 0) {
        let new_sub = std::iter::zip(&subtractions[curr][..], &subtractions[curr][1..])
            .map(|(a, b)| b - a)
            .collect::<Vec<_>>();

        subtractions.push(new_sub);

        curr += 1;
    }

    subtractions
        .into_iter()
        .rev()
        .skip(1)
        .fold(0, |last, sub| sub.last().unwrap() + last)
}

fn prev_value(history: Vec<i32>) -> i32 {
    let mut subtractions = vec![history];
    let mut curr = 0;

    while !subtractions[curr].iter().all(|&n| n == 0) {
        let new_sub = std::iter::zip(&subtractions[curr][..], &subtractions[curr][1..])
            .map(|(a, b)| b - a)
            .collect::<Vec<_>>();

        subtractions.push(new_sub);

        curr += 1;
    }

    subtractions
        .into_iter()
        .rev()
        .skip(1)
        .fold(0, |last, sub| sub.first().unwrap() - last)
}

fn part_one() -> Result<(), Box<dyn Error>> {
    let histories = read_to_string(FILE)?
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let sum = histories
        .into_iter()
        .map(|history| next_value(history))
        .sum::<i32>();

    println!("{}", sum);

    Ok(())
}

fn part_two() -> Result<(), Box<dyn Error>> {
    let histories = read_to_string(FILE)?
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let sum = histories
        .into_iter()
        .map(|history| prev_value(history))
        .sum::<i32>();

    println!("{}", sum);

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    part_one()?;
    part_two()?;

    Ok(())
}
