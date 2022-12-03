use std::collections::HashSet;

fn get_inputs() -> Vec<Vec<u8>> {
    include_str!("../../input/day03.txt")
        .lines()
        .map(|l| {
            l.as_bytes()
                .iter()
                .map(|&b| if b >= 96 { b % 96 } else { b - 38 })
                .collect()
        })
        .collect()
}

fn intersect(s1: &[u8], s2: &[u8]) -> Vec<u8> {
    s1.iter()
        .cloned()
        .collect::<HashSet<u8>>()
        .intersection(&s2.iter().cloned().collect::<HashSet<u8>>())
        .cloned()
        .collect::<Vec<u8>>()
}

fn part1() -> usize {
    get_inputs()
        .iter()
        .map(|s| s.split_at(s.len() / 2))
        .map(|l| intersect(&l.0, &l.1)[0])
        .map(|i| i as usize)
        .sum()
}

fn part2() -> usize {
    get_inputs()
        .chunks(3)
        .map(|l| intersect(&intersect(&l[0], &l[1]), &l[2])[0])
        .map(|i| i as usize)
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
