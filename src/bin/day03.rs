use std::collections::HashSet;

fn get_inputs() -> Vec<Vec<usize>> {
    include_str!("../../input/day03.txt")
        .lines()
        .map(|l| {
            l.as_bytes()
                .iter()
                .map(|&b| ((b - 38) % 58) as usize)
                .collect()
        })
        .collect()
}

fn intersect(s1: &[usize], s2: &[usize]) -> Vec<usize> {
    s1.iter()
        .copied()
        .collect::<HashSet<usize>>()
        .intersection(&s2.iter().copied().collect::<HashSet<usize>>())
        .copied()
        .collect::<Vec<usize>>()
}

fn part1() -> usize {
    get_inputs()
        .iter()
        .map(|s| s.split_at(s.len() / 2))
        .map(|l| intersect(l.0, l.1)[0])
        .sum()
}

fn part2() -> usize {
    get_inputs()
        .chunks(3)
        .map(|l| intersect(&intersect(&l[0], &l[1]), &l[2])[0])
        .sum()
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
        assert!(part1() == 7_581);
    }

    #[test]
    fn test_part2() {
        assert!(part2() == 2_525);
    }
}
