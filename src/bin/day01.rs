use itertools::Itertools;

fn get_inputs() -> Vec<u32> {
    include_str!("../../input/day01.txt")
        .split("\n\n")
        .map(|s| s.lines().map(|c| c.trim().parse::<u32>().unwrap()).sum())
        .sorted()
        .collect()
}

fn result(n: usize) -> u32 {
    get_inputs().iter().rev().take(n).sum()
}

fn main() {
    println!("Part 1 {}", result(1));
    println!("Part 2 {}", result(3));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert!(result(1) == 69_310);
    }

    #[test]
    fn test_part2() {
        assert!(result(3) == 206_104);
    }
}
