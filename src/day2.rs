use std::error::Error;
use std::fs::read_to_string;

#[derive(Debug)]
struct Colours {
    red: u32,
    green: u32,
    blue: u32,
}

impl Colours {
    fn from_iter<'a, I>(colours: I) -> Self
    where
        I: Iterator<Item = &'a str>,
    {
        let (red, green, blue) = colours
            .map(|c| {
                if let Some((qty, _)) = c.split_once(" red") {
                    return (qty.parse::<u32>().unwrap(), 0, 0);
                } else if let Some((qty, _)) = c.split_once(" green") {
                    return (0, qty.parse::<u32>().unwrap(), 0);
                } else if let Some((qty, _)) = c.split_once(" blue") {
                    return (0, 0, qty.parse::<u32>().unwrap());
                } else {
                    panic!("Undefined colour!");
                }
            })
            .fold((0, 0, 0), |a, b| (a.0 + b.0, a.1 + b.1, a.2 + b.2));

        Self { red, green, blue }
    }
}

pub fn day2() -> Result<(), Box<dyn Error>> {
    let tot_red = 12;
    let tot_green = 13;
    let tot_blue = 14;

    // Part one
    let sum = read_to_string("input")?
        .lines()
        .map(|line| {
            let (_, line) = line.split_at("Game ".len());
            let (id, line) = line.split_once(": ").unwrap();

            (id.parse::<u32>().unwrap(), line)
        })
        .map(|(id, line)| {
            let sets = line
                .split("; ")
                .map(|set| Colours::from_iter(set.split(", ")))
                .collect::<Vec<_>>();

            (id, sets)
        })
        .filter_map(|(id, colours)| {
            if colours
                .iter()
                .all(|c| c.red <= tot_red && c.blue <= tot_blue && c.green <= tot_green)
            {
                Some(id)
            } else {
                None
            }
        })
        .sum::<u32>();

    println!("{}", sum);

    // Part two
    let power = read_to_string("input")?
        .lines()
        .map(|line: &str| {
            let (_, line) = line.split_at("Game ".len());
            let (id, line) = line.split_once(": ").unwrap();

            (id.parse::<u32>().unwrap(), line)
        })
        .map(|(id, line)| {
            let sets = line
                .split("; ")
                .map(|set| Colours::from_iter(set.split(", ")))
                .collect::<Vec<_>>();

            (id, sets)
        })
        .map(|(_, colours)| Colours {
            red: colours.iter().map(|c| c.red).max().unwrap(),
            green: colours.iter().map(|c| c.green).max().unwrap(),
            blue: colours.iter().map(|c| c.blue).max().unwrap(),
        })
        .map(|c| c.red * c.blue * c.green)
        .sum::<u32>();

    println!("{}", power);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::day2;

    #[test]
    fn daay2() {
        day2();
    }
}
