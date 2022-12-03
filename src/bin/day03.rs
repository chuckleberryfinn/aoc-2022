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

fn part1() -> usize {
    get_inputs()
        .iter()
        .map(|s| s.split_at(s.len() / 2))
        .map(|l| {
            *(l.0
                .iter()
                .cloned()
                .collect::<HashSet<u8>>()
                .intersection(&l.1.iter().cloned().collect::<HashSet<u8>>()))
            .next()
            .unwrap()
        })
        .map(|i| i as usize)
        .sum()
}

fn part2() -> usize {
    get_inputs()
        .chunks(3)
        .map(|l| {
            *(l[0]
                .iter()
                .cloned()
                .collect::<HashSet<u8>>()
                .intersection(&l[1].iter().cloned().collect::<HashSet<u8>>()))
            .cloned()
            .collect::<HashSet<u8>>()
            .intersection(&l[2].iter().cloned().collect::<HashSet<u8>>())
            .next()
            .unwrap()
        })
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
