use std::collections::HashSet;
use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    // ---------------------------------------------
    // Part one
    // ---------------------------------------------
    
    let tot_points = read_to_string("input")?
        .lines()
        // Parse the scratchcards in two lists
        .map(|line| {
            let (winning, my) = line.split_once(": ")
                .unwrap()
                .1
                .split_once(" | ")
                .unwrap();

            (
                winning.split_ascii_whitespace()
                    .into_iter()
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect::<HashSet<_>>(),
                my.split_ascii_whitespace()
                    .into_iter()
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        // Convert each scratchcard in points
        .map(|(winning, my)| {
            let count = my.iter()
                .filter(|num| winning.contains(num))
                .count();

            if count == 0 {
                0
            } else {
                2u32.pow(count as u32 - 1)
            }
        })
        .sum::<u32>();

    println!("{}", tot_points);

    // ---------------------------------------------
    // Part two
    // ---------------------------------------------
    
    let mut cards = read_to_string("input")?
        .lines()
        // Parse the scratchcards in two lists
        .map(|line| {
            let (winning, my) = line.split_once(": ")
                .unwrap()
                .1
                .split_once(" | ")
                .unwrap();

            
            (
                1usize,
                winning.split_ascii_whitespace()
                    .into_iter()
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect::<HashSet<_>>(),
                my.split_ascii_whitespace()
                    .into_iter()
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    // Update the cards number while itereting over it
    for id in 0..cards.len() {
        let (nums, winners, mine) = &cards[id];

        let nums = *nums;
        let winning = mine.iter()
            .filter(|num| winners.contains(num))
            .count();
        
        for to_add in id + 1..id + 1 + winning {
            cards[to_add].0 += nums;
        }
    }

    // Sum the cards
    let tot_cards = cards.iter()
        .map(|c| c.0)
        .sum::<usize>();

    println!("{}", tot_cards);

    Ok(())
}
