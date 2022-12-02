use itertools::Itertools;
use std::collections::HashMap;

fn get_inputs() -> Vec<(&'static str, &'static str)> {
    include_str!("../../input/day02.txt")
        .lines()
        .map(|s| s.split(' ').collect_tuple().unwrap())
        .collect()
}

fn part1() -> usize {
    let results = HashMap::from([
        ("A", HashMap::from([("X", 4), ("Y", 8), ("Z", 3)])),
        ("B", HashMap::from([("X", 1), ("Y", 5), ("Z", 9)])),
        ("C", HashMap::from([("X", 7), ("Y", 2), ("Z", 6)])),
    ]);

    result(&results)
}

fn part2() -> usize {
    let results = HashMap::from([
        ("A", HashMap::from([("X", 3), ("Y", 4), ("Z", 8)])),
        ("B", HashMap::from([("X", 1), ("Y", 5), ("Z", 9)])),
        ("C", HashMap::from([("X", 2), ("Y", 6), ("Z", 7)])),
    ]);

    result(&results)
}

fn result(results: &HashMap<&str, HashMap<&str, usize>>) -> usize {
    get_inputs().iter().fold(0, |t, round| {
        t + results.get(round.0).unwrap().get(round.1).unwrap()
    })
}

fn main() {
    println!("Part 1 {}", part1());
    println!("Part 2 {}", part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert!(part1() == 11_063);
    }

    #[test]
    fn test_part2() {
        assert!(part2() == 10_349);
    }
}
