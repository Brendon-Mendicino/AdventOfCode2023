use std::{error::Error, fs::read_to_string, cmp::Ordering};

fn char_to_id_part1(ch: char) -> u32 {
    match ch {
        '2' => 1,
        '3' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '7' => 6,
        '8' => 7,
        '9' => 8,
        'T' => 9,
        'J' => 10,
        'Q' => 11,
        'K' => 12,
        'A' => 13,
        _ => panic!("Blabla: {}", ch),
    }
}

fn hand_score_part1(hand: &[u32]) -> u32 {
    let mut times = (0..=13).map(|id| (id, 0)).collect::<Vec<_>>();

    for &card in hand { 
        times[card as usize].1 += 1;
    }

    times.sort_by(|(_, times1), (_, times2)| times2.cmp(times1));

    match times[..] {
        [(_, 5), ..] => 7,
        [(_, 4), ..] => 6,
        [(_, 3), (_, 2), ..] => 5,
        [(_, 3), (_, 1), ..] => 4,
        [(_, 2), (_, 2), ..] => 3,
        [(_, 2), (_, 1), ..] => 2,
        [(_, 1), ..] => 1,
        _ => panic!("Could not be here!"),
    }
}

fn char_to_id_part2(ch: char) -> u32 {
    match ch {
        'J' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'Q' => 11,
        'K' => 12,
        'A' => 13,
        _ => panic!("Blabla: {}", ch),
    }
}

fn hand_score_part2(hand: &[u32]) -> u32 {
    let mut times = (0..=13).map(|id| (id, 0)).collect::<Vec<_>>();

    for &card in hand { 
        times[card as usize].1 += 1;
    }

    let jokers = times[1].1;
    times[1].1 = 0;

    times.sort_by(|(_, times1), (_, times2)| times2.cmp(times1));
    times[0].1 += jokers;

    match times[..] {
        [(_, 5), ..] => 7,
        [(_, 4), ..] => 6,
        [(_, 3), (_, 2), ..] => 5,
        [(_, 3), ..] => 4,
        [(_, 2), (_, 2), ..] => 3,
        [(_, 2), ..] => 2,
        [(_, 1), ..] => 1,
        [(_, 0), ..] => 0,
        _ => panic!("Could not be here!"),
    }
}

fn cmp_hand(hand1: (u32, &[u32]), hand2: (u32, &[u32])) -> Ordering {
    let (score1, hand1) = hand1;
    let (score2, hand2) = hand2;

    if !matches!(score1.cmp(&score2), Ordering::Equal) {
        return score1.cmp(&score2);
    }

    for (&c1, &c2) in std::iter::zip(hand1, hand2) {
        if let Ordering::Equal = c1.cmp(&c2) {
            continue;
        }

        return c1.cmp(&c2);
    }

    panic!("Hand zero length");
}

fn main() -> Result<(), Box<dyn Error>> {
    // -----------------------------------------------------
    // Part one
    // -----------------------------------------------------

    let hands = read_to_string("input")?
        .lines()
        .map(|line| line.split_once(" ").unwrap())
        .map(|(hand, bid)| {
            (
                hand.chars().map(|c| char_to_id_part1(c)).collect::<Vec<_>>(),
                bid.parse::<u32>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let mut hands = hands
        .into_iter()
        .map(|(hand, bid)| {
            (
                hand_score_part1(&hand),
                hand,
                bid
            )
        })
        .collect::<Vec<_>>();

    hands
        .sort_by(|(score1, hand1, _), (score2, hand2, _)| {
            cmp_hand((*score1, hand1), (*score2, hand2))
        });

    let winnings = hands
        .into_iter()
        .enumerate()
        .fold(0, |prev, (rank, (_, _, bid))| {
            (rank as u32 + 1) * bid + prev
        });

    println!("{}", winnings);


    // -----------------------------------------------------
    // Part one
    // -----------------------------------------------------
    let hands = read_to_string("input")?
        .lines()
        .map(|line| line.split_once(" ").unwrap())
        .map(|(hand, bid)| {
            (
                hand.chars().map(|c| char_to_id_part2(c)).collect::<Vec<_>>(),
                bid.parse::<u32>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let mut hands = hands
        .into_iter()
        .map(|(hand, bid)| {
            (
                hand_score_part2(&hand),
                hand,
                bid
            )
        })
        .collect::<Vec<_>>();

    hands
        .sort_by(|(score1, hand1, _), (score2, hand2, _)| {
            cmp_hand((*score1, hand1), (*score2, hand2))
        });

    let winnings = hands
        .into_iter()
        .enumerate()
        .fold(0, |prev, (rank, (_, _, bid))| {
            (rank as u32 + 1) * bid + prev
        });

    println!("{}", winnings);

    Ok(())
}
