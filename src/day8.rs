use std::{collections::HashMap, error::Error, fs::read_to_string};

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

pub fn day8() -> Result<(), Box<dyn Error>> {
    // ------------------------------------------
    // Part one
    // ------------------------------------------

    let moves = read_to_string("input")?
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => 0,
            'R' => 1,
            _ => panic!(),
        })
        .collect::<Vec<_>>();

    let mut lookup_table = HashMap::new();

    let nodes = read_to_string("input")?
        .lines()
        .skip(2)
        .enumerate()
        .map(|(index, line)| {
            let (id, lr) = line.split_once(" = ").unwrap();
            let (l, r) = lr.split_once(", ").unwrap();

            lookup_table.insert(id.to_string(), index);

            (
                id.to_string(),
                vec![l[1..].to_string(), r[..r.len() - 1].to_string()],
            )
        })
        .collect::<Vec<_>>();

    let start = lookup_table["AAA"];
    let mut curr_pos = start;
    let mut turn = 0;
    let mut steps = 0;

    while nodes[curr_pos].0 != "ZZZ" {
        let new_node = &nodes[curr_pos].1[moves[turn]];
        curr_pos = lookup_table[new_node];

        turn = (turn + 1) % moves.len();
        steps += 1;
    }

    println!("{}", steps);

    // ------------------------------------------
    // Part two
    // ------------------------------------------

    let moves = read_to_string("input")?
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => 0,
            'R' => 1,
            _ => panic!(),
        })
        .collect::<Vec<_>>();

    let mut lookup_table = HashMap::new();

    let nodes = read_to_string("input")?
        .lines()
        .skip(2)
        .enumerate()
        .map(|(index, line)| {
            let (id, lr) = line.split_once(" = ").unwrap();
            let (l, r) = lr.split_once(", ").unwrap();

            lookup_table.insert(id.to_string(), index);

            (
                id.to_string(),
                vec![l[1..].to_string(), r[..r.len() - 1].to_string()],
            )
        })
        .collect::<Vec<_>>();

    let a_ending_nodes = lookup_table
        .keys()
        .filter(|key| key.as_bytes()[2] == b'A')
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    let curr_nodes = a_ending_nodes;
    let mut periods = (0..curr_nodes.len()).map(|_| 0).collect::<Vec<_>>();

    for index in 0..curr_nodes.len() {
        let mut curr_dir = 0;
        let mut period = 0_usize;

        let mut node = curr_nodes[index].clone();
        while node.as_bytes()[2] != b'Z' {
            let node_index = lookup_table[&node];
            let next_node = &nodes[node_index].1[moves[curr_dir]];

            node = next_node.clone();

            curr_dir = (curr_dir + 1) % moves.len();
            period += 1;
        }

        periods[index] = period;
    }

    let steps = lcm(&periods);

    println!("{}", steps);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::day8;

    #[test]
    fn testt() {
        day8();
    }
}
