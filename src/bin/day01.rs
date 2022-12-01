use aoc::*;

fn get_inputs() -> Vec<u32> {
    include_str!("../../input/day01.txt")
        .split("\n\n")
        .map(|s| s.lines().map(|c| c.trim().parse::<u32>().unwrap()).sum())
        .collect::<std::collections::BinaryHeap<u32>>()
        .into_sorted_vec()
}

fn result(n: usize) -> u32 {
    get_inputs()
        .iter()
        .rev()
        .take(n)
        .sum()
}

fn main() -> Result<()> {
    println!("Part 1 {}", result(1));
    println!("Part 2 {}", result(3));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert!(result(1) == 69_310);
    }

    #[test]
    fn test_part_2() {
        assert!(result(3) == 206_104);
    }
}
