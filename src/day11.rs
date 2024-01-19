use std::error::Error;
use std::fs::read_to_string;

fn get_map(path: &str) -> Result<Vec<Vec<u8>>, Box<dyn Error>> {
    Ok(read_to_string(path)?
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<_>>())
}

fn get_pairs(map: Vec<Vec<u8>>, scale: i64) -> Vec<(i64, i64)> {
    // Parse the pairs
    let mut pairs = vec![];

    let mut in_rows = vec![0; map.len()];
    let mut in_cols = vec![0; map[0].len()];

    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if map[row][col] != b'#' {
                continue;
            }

            pairs.push((row as i64, col as i64));
            in_rows[row] += 1;
            in_cols[col] += 1;
        }
    }

    dbg!(&pairs);

    let expand = |vec: Vec<i32>| {
        let first = if vec[0] == 0 { scale } else { 0 };
        vec
            .into_iter()
            .enumerate()
            .skip(1)
            .fold(vec![first], |mut v, (index, num)| {
                if num == 0 {
                    v.push(v[index - 1] + scale);
                } else {
                    v.push(v[index - 1]);
                }
                v
            })
    };

    // Calculate expansion by tile
    let row_expansion = expand(in_rows);

    dbg!(&row_expansion);

    let col_expansion = expand(in_cols);

    dbg!(&col_expansion);

    // Increase the size given by the universe expansion
    pairs
        .into_iter()
        .map(|(row, col)| (row + row_expansion[row as usize], col + col_expansion[col as usize]))
        .collect::<Vec<_>>()
}

fn tot_distance(pairs: Vec<(i64, i64)>) -> i64 {
    let mut tot = 0;

    for i in 0..pairs.len() - 1 {
        for j in (i+1)..pairs.len() {
            tot += dst(pairs[i], pairs[j]);
        }
    }

    tot
}

fn dst(a: (i64, i64), b: (i64, i64)) -> i64 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

pub fn part1(path: &str) -> Result<i64, Box<dyn Error>> {
    let map = get_map(path)?;

    let pairs = get_pairs(map, 1);
    dbg!(&pairs);

    Ok(tot_distance(pairs))
}

pub fn part2_test(path: &str) -> Result<i64, Box<dyn Error>> {
    let map = get_map(path)?;

    let pairs = get_pairs(map, 10 - 1);
    dbg!(&pairs);

    Ok(tot_distance(pairs))
}

pub fn part2(path: &str) -> Result<i64, Box<dyn Error>> {
    let map = get_map(path)?;

    let pairs = get_pairs(map, 1000000 - 1);

    Ok(tot_distance(pairs))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("inputs/11input_test").unwrap(), 374);
        
        assert_eq!(part1("inputs/11input").unwrap(), 9608724);
    }

    #[test]
    fn test_part2() { 
        assert_eq!(part2_test("inputs/11input_test").unwrap(), 1030);

        assert_eq!(part2("inputs/11input").unwrap(), 904633799472);
    }
}
