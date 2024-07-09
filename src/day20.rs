use std::error::Error;
use std::fs::read_to_string;

pub fn part1(path: &str) -> Result<usize, Box<dyn Error>> {
    Ok(0)
}

pub fn part2(path: &str) -> Result<usize, Box<dyn Error>> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(part1("inputs/20input_test").unwrap(), 0);
        assert_eq!(part1("inputs/20input").unwrap(), 0);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2("inputs/20input_test").unwrap(), 0);
        assert_eq!(part2("inputs/20input").unwrap(), 0);
    }
}
