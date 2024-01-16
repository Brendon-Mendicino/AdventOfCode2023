use core::panic;
use std::{error::Error, fs::read_to_string};

pub fn day6() -> Result<(), Box<dyn Error>> {
    // -----------------------------------------------
    // Part one 
    // -----------------------------------------------

    let races = read_to_string("input")?
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .skip(1)
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let races = races[0]
        .clone()
        .into_iter()
        .zip(races[1].clone())
        .collect::<Vec<_>>();

    let result = races
        .into_iter()
        .map(|(duration, distance)| {
            // Create the second degree equation
            // (duration - hold) * hold >= distance
            // - hold ** 2 + duration * hold - distance >= 0
            // (-duration +- sqrt(duration ** 2 + 4 * 2 * distance)) / (2 * 2)
            // let p = |root| (duration - root) * root - distance;

            // let delta = f64::from(duration.pow(2) - 4 * distance).sqrt();
            // let mut root1 = ((-duration as f64 + delta) / (2 * -1) as f64) as i32;
            // let mut root2 = ((-duration as f64 - delta) / (2 * -1) as f64) as i32;

            // root2 - root1 + 1

            let mut sum = 0;

            for hold in 0..duration {
                if (duration - hold) * hold > distance {
                    sum += 1;
                }
            }

            sum
        })
        .fold(1, |a, b| a * b);

    println!("{}", result);

    // -----------------------------------------------
    // Part two
    // -----------------------------------------------

    let race = read_to_string("input")?
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .skip(1)
                .fold("".to_string(), |prev, next| format!("{}{}", prev, next))
                .parse::<i64>()
                .unwrap()
        })
        .collect::<Vec<_>>();

    let [duration, distance] = race[..] else { panic!("vec.len() != 2") };
    let p = |root| (duration - root) * root - distance;

    // Create the second degree equation
    // (duration - hold) * hold >= distance
    // - hold ** 2 + duration * hold - distance >= 0
    // (-duration +- sqrt(duration ** 2 + 4 * 2 * distance)) / (2 * 2)
    // let p = |root| (duration - root) * root - distance;

    let mean = duration / 2;
    let mut max = mean;
    let mut min = 0;
    let mut last = 0;

    while max >= min {
        let avg = (max + min) / 2;
        if p(avg) > 0 {
            last = avg;
            max = avg - 1;
        } 
        else {
            min = avg + 1;
        }
    }

    let root1 = last;

    let mean = duration / 2;
    let mut max = duration;
    let mut min = mean;
    let mut last = 0;

    while max >= min {
        let avg = (max + min) / 2;
        if p(avg) > 0 {
            last = avg;
            min = avg + 1;
        } 
        else {
            max = avg - 1;
        }
    }

    let root2 = last;

    let result = root2 - root1 + 1;

    println!("{}", result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::day6;

    #[test]
    fn testtt() {
        day6();
    }
}