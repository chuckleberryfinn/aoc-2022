fn get_inputs() -> Vec<(usize, usize, usize, usize)> {
    include_str!("../../input/day04.txt")
        .lines()
        .map(|s| s.split(|c| c == ',' || c == '-').collect())
        .map(|x: Vec<&str>| {
            (
                x[0].parse().unwrap(),
                x[1].parse().unwrap(),
                x[2].parse().unwrap(),
                x[3].parse().unwrap(),
            )
        })
        .collect()
}

fn part1() -> usize {
    get_inputs()
        .iter()
        .filter(|c| (c.0 >= c.2 && c.1 <= c.3) || (c.2 >= c.0 && c.3 <= c.1))
        .count()
}

fn part2() -> usize {
    get_inputs()
        .iter()
        .filter(|c| !((c.2 < c.0 || c.1 < c.2) && c.0 > c.3 || c.0 < c.2 && c.1 < c.2 && c.3 > c.1))
        .count()
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert!(part1() == 487);
    }

    #[test]
    fn test_part2() {
        assert!(part2() == 849);
    }
}
