use aoc::*;

fn get_result(input: usize) -> usize {
    input
}

fn main() -> Result<()> {
    println!("Part 1 {}", get_result(get_result(1)));
    println!("Part 2 {}", get_result(get_result(2)));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert!(get_result(1) == 1);
    }

    #[test]
    fn test_part_2() {
        assert!(get_result(2) == 2);
    }
}
