use num_traits::{PrimInt, Unsigned};
use rust_lapper::*;
use std::error::Error;
use std::fs::read_to_string;

#[derive(Debug, Clone)]
struct MyInterval<I, T>(Interval<I, T>)
where
    I: PrimInt + Unsigned + Ord + Clone + Send + Sync,
    T: Eq + Clone + Send + Sync;

impl<I, T> MyInterval<I, T>
where
    I: PrimInt + Unsigned + Ord + Clone + Send + Sync,
    T: Eq + Clone + Send + Sync,
{
    fn intersetion(self, other: &Self) -> Self {
        Self(Interval {
            start: std::cmp::max(self.0.start, other.0.start),
            stop: std::cmp::min(self.0.stop, other.0.stop),
            val: self.0.val,
        })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // ------------------------------------------------
    // Part one
    // ------------------------------------------------

    // Parse all the seeds
    let seeds = read_to_string("input")?
        .lines()
        .next()
        .unwrap()
        .replace("seeds: ", "")
        .split_ascii_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let categories = read_to_string("input")?
        .split("map:\n")
        .skip(1)
        .map(|c| c.to_string())
        .collect::<Vec<_>>();

    let set_of_categories = categories
        .into_iter()
        .map(|lines| {
            lines
                .lines()
                .filter_map(|line| match line.split(" ").collect::<Vec<_>>()[..] {
                    [dst, src, len] => Some((
                        dst.parse::<usize>().unwrap(),
                        src.parse::<usize>().unwrap(),
                        len.parse::<usize>().unwrap(),
                    )),
                    _ => None,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut mappings = seeds;

    // Find the last mapping
    for category in set_of_categories {
        // Get the new mappings
        mappings = mappings
            .iter()
            .map(|&mapping| {
                category
                    .iter()
                    .filter(|&&(_, src, len)| mapping >= src && mapping < src + len)
                    .next()
                    .map_or(mapping, |&(dst, src, _)| dst + mapping - src)
            })
            .collect();
    }

    let Some(&min_location) = mappings.iter().min() else {
        panic!("There is no min")
    };

    println!("{}", min_location);

    // ------------------------------------------------
    // Part two
    // ------------------------------------------------

    // Parse the seed ranges
    let seed_ranges = read_to_string("input")?
        .lines()
        .next()
        .unwrap()
        .replace("seeds: ", "")
        .split_ascii_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect::<Vec<_>>()
        .chunks_exact(2)
        .map(|s| {
            MyInterval(Interval {
                start: s[0],
                stop: s[0] + s[1],
                val: 0,
            })
        })
        .collect::<Vec<_>>();

    let categories = read_to_string("input")?
        .split("map:\n")
        .skip(1)
        .map(|c| c.to_string())
        .collect::<Vec<_>>();

    let set_of_categories = categories
        .into_iter()
        .map(|lines| {
            let v = lines
                .lines()
                .filter_map(|line| match line.split(" ").collect::<Vec<_>>()[..] {
                    [dst, src, len] => Some(Interval {
                        start: src.parse::<usize>().unwrap(),
                        stop: src.parse::<usize>().unwrap() + len.parse::<usize>().unwrap(),
                        val: dst.parse::<usize>().unwrap(),
                    }),
                    _ => None,
                })
                .collect::<Vec<_>>();

            Lapper::new(v)
        })
        .collect::<Vec<_>>();

    let mut mappings = seed_ranges;

    for category in &set_of_categories {
        let mut new_mappings = vec![];

        // Get the new mappings
        while !mappings.is_empty() {
            let mut leftovers = vec![];

            for mapping in &mappings {
                // Find the new range of mappings on the leftovers from the intersection
                let next_mappings = category
                    .find(mapping.0.start, mapping.0.stop)
                    .next()
                    .map_or((mapping.clone(), vec![]), |range| {
                        let inter = MyInterval(range.clone()).intersetion(mapping);

                        let starting = MyInterval(Interval {
                            start: mapping.0.start,
                            stop: inter.0.start,
                            val: 0usize,
                        });
                        let offset = inter.0.start - range.start;
                        let middle = MyInterval(Interval {
                            start: offset + range.val,
                            stop: offset + range.val + inter.0.stop - inter.0.start,
                            val: 0,
                        });
                        let ending = MyInterval(Interval {
                            start: inter.0.stop,
                            stop: mapping.0.stop,
                            val: 0,
                        });

                        let mut ret = vec![];

                        if starting.0.stop - starting.0.start != 0 {
                            ret.push(starting);
                        }

                        if ending.0.stop - ending.0.start != 0 {
                            ret.push(ending);
                        }


                        (middle, ret)
                    });

                new_mappings.push(next_mappings.0);
                leftovers.extend(next_mappings.1);
            }

            // Keep iterating over the old mappings until we run out of them
            mappings = leftovers;
        }

        // Iterate over the new mappings obtained from the intersections
        mappings = new_mappings;
    }

    let Some(min_location) = mappings.iter().map(|r| r.0.start).min() else {
        panic!("There is no min!")
    };

    println!("{}", min_location);

    Ok(())
}
