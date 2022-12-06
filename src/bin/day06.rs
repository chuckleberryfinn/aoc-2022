use itertools::Itertools;

fn get_inputs(window: usize) -> Vec<Vec<char>> {
    include_str!("../../input/day06.txt")
        .chars()
        .collect::<Vec<char>>()
        .windows(window)
        .map(|w|w.to_vec())
        .collect()
}

fn result(window: usize) -> usize {
    get_inputs(window)
        .iter()
        .enumerate()
        .find(|(_, g)| g.iter().unique().count() == window)
        .unwrap().0 + window
}

fn main() {
    println!("{}", result(4));
    println!("{}", result(14));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert!(result(4) == 1_760);
    }

    #[test]
    fn test_part2() {
        assert!(result(14) == 2_974);
    }
}
