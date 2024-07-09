use std::collections::btree_map::Values;
use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;

use itertools::Itertools;

#[derive(Clone, Copy, Debug)]
enum Cmp {
    Less,
    Greater,
}

fn translate_step(
    step: &str,
) -> Box<impl FnMut(&HashMap<String, usize>, &mut usize) -> Option<String>> {
    let (condition, dest) = step
        .split_once(":")
        .map_or((None, step.to_string()), |c| (Some(c.0), c.1.to_string()));

    let key_value_comp = condition.map(|condition| {
        condition
            .split_once("<")
            .map(|kv| (kv.0, kv.1, Cmp::Less))
            .or(condition
                .split_once(">")
                .map(|kv| (kv.0, kv.1, Cmp::Greater)))
            .map(|(k, v, c)| (k.to_string(), v.parse::<usize>().unwrap(), c))
            .unwrap()
    });

    Box::new(
        move |input: &HashMap<String, usize>, sum: &mut usize| -> Option<String> {
            let Some((key, value, comp)) = &key_value_comp else {
                if dest == "A".to_string() {
                    *sum = *sum + input.values().sum::<usize>();
                }
                return Some(dest.clone());
            };
            let &in_value = input.get(key).unwrap();

            match comp {
                Cmp::Less if in_value < *value => {
                    if dest == "A".to_string() {
                        *sum = *sum + input.values().sum::<usize>();
                    }
                    Some(dest.clone())
                }
                Cmp::Greater if in_value > *value => {
                    if dest == "A".to_string() {
                        *sum = *sum + input.values().sum::<usize>();
                    }
                    Some(dest.clone())
                }
                _ => None,
            }
        },
    )
}

fn trans(step: &str) -> (String, HashMap<String, (usize, usize)>) {
    let (condition, dest) = step
        .split_once(":")
        .map_or((None, step.to_string()), |c| (Some(c.0), c.1.to_string()));

    let mut map = condition
        .map(|condition| {
            condition
                .split_once("<")
                .map(|kv| (kv.0, kv.1, Cmp::Less))
                .or(condition
                    .split_once(">")
                    .map(|kv| (kv.0, kv.1, Cmp::Greater)))
                .map(|(k, v, c)| (k.to_string(), v.parse::<usize>().unwrap(), c))
                .unwrap()
        })
        .map_or(HashMap::new(), |(key, value, comp)| match comp {
            Cmp::Greater => HashMap::from_iter([(key, (value + 1, 4000))]),
            Cmp::Less => HashMap::from_iter([(key, (1, value))]),
        });

    ["x", "m", "a", "s"]
        .into_iter()
        .map(str::to_string)
        .for_each(|s| {
            map.entry(s).or_insert((1, 4000));
        });

    (dest, map)
}

pub fn part1(path: &str) -> Result<usize, Box<dyn Error>> {
    let temp = read_to_string(path)?;

    let mut workflows = temp
        .split_once("\n\n")
        .unwrap()
        .0
        .lines()
        .map(|line| {
            let (label, steps) = line[..line.len() - 1]
                .split_once("{")
                .map(|(label, steps)| (label.to_string(), steps))
                .unwrap();
            let trans_steps = steps.split(",").map(translate_step).collect_vec();

            (label, trans_steps)
        })
        .chain(vec![("A".to_string(), vec![]), ("R".to_string(), vec![])])
        .fold(HashMap::new(), |mut dict, (label, steps)| {
            dict.insert(label, steps);
            dict
        });

    let pieces = temp
        .split_once("\n\n")
        .unwrap()
        .1
        .lines()
        .map(|l| &l[1..l.len() - 1])
        .map(|l| {
            l.split(",")
                .map(|piece| {
                    piece
                        .split_once("=")
                        .map(|(key, value)| (key.to_string(), value.parse::<usize>().unwrap()))
                        .unwrap()
                })
                .fold(HashMap::new(), |mut dict, p| {
                    dict.insert(p.0, p.1);
                    dict
                })
        })
        .collect_vec();

    let mut sum = 0;

    for input in pieces {
        let mut next_label = vec!["in".to_string()];

        'next_workflow: while let Some(label) = next_label.pop() {
            let flow = workflows.get_mut(&label).unwrap();

            for f in flow {
                match f(&input, &mut sum) {
                    Some(next) => {
                        next_label.push(next);
                        continue 'next_workflow;
                    }
                    None => (),
                }
            }
        }
    }

    Ok(sum)
}

fn inter(a: (usize, usize), b: (usize, usize)) -> (usize, usize) {
    if a.0 < b.0 {
        if a.1 < b.0 {
            (1, 1)
        } else {
            (a.0.max(b.0), a.1.min(b.1))
        }
    } else {
        if b.1 < a.0 {
            (1, 1)
        } else {
            (a.0.max(b.0), a.1.min(b.1))
        }
    }
}

    fn union(a: (usize, usize), b: (usize, usize)) -> (usize, usize) {
        (a.0.min(b.0), a.1.max(b.1))
    }

fn explore(
    curr_label: &String,
    workflows: &HashMap<String, Vec<(String, HashMap<String, (usize, usize)>)>>,
    intervals: &mut HashMap<String, (usize, usize)>,
    cache: &mut HashMap<String, HashMap<String, (usize, usize)>>,
) -> HashMap<String, (usize, usize)> {
    if let Some(intervals) = cache.get(curr_label) {
        return intervals.clone();
    }

    let mut full = ["x", "m", "a", "s"]
        .into_iter()
        .map(str::to_string)
        .map(|s| (s, (1, 4000)))
        .collect::<HashMap<_, (usize, usize)>>();

    let steps = &workflows[curr_label];
    for s in steps {
    }

    HashMap::new()
}

pub fn part2(path: &str) -> Result<usize, Box<dyn Error>> {
    let mut workflows = read_to_string(path)?
        .split_once("\n\n")
        .unwrap()
        .0
        .lines()
        .map(|line| {
            let (label, steps) = line[..line.len() - 1]
                .split_once("{")
                .map(|(label, steps)| (label.to_string(), steps))
                .unwrap();
            let trans_steps = steps.split(",").map(trans).collect_vec();

            (label, trans_steps)
        })
        .chain(vec![("A".to_string(), vec![]), ("R".to_string(), vec![])])
        .fold(HashMap::new(), |mut dict, (label, steps)| {
            dict.insert(label, steps);
            dict
        });

    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(part1("inputs/19input_test").unwrap(), 19114);
        assert_eq!(part1("inputs/19input").unwrap(), 397061);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2("inputs/19input_test").unwrap(), 167409079868000);
        assert_eq!(part2("inputs/19input").unwrap(), 0);
    }
}
