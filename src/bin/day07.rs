use regex::Regex;
fn get_inputs() -> Vec<&'static str> {
    include_str!("../../input/day07.txt")
    .lines()
    .collect()
}

fn get_levels() -> Vec<usize> {
    let inputs = get_inputs();
    let re = Regex::new(r"\d+").unwrap();

    let mut current = 0;
    let mut levels = vec![];
    let mut stack = vec![];

    for i in inputs {
        if i == "$ cd .." {
            levels.push(current);
            current += stack.pop().unwrap();
        } else if i.starts_with("$ cd") {
            stack.push(current);
            current = 0;
        } else if let Some(number) = re.find(i) {
            current += number.as_str().parse::<usize>().unwrap();
        }
    }
    levels.push(current + stack.iter().sum::<usize>());
    levels
}

fn part1() -> usize {
    get_levels()
        .iter()
        .filter(|&&l| l <= 100_000)
        .sum()
}

fn part2() -> usize {
    let levels = get_levels();
    let required = *levels.iter().last().unwrap();
    *levels
        .iter()
        .filter(|&&l| l >= 30_000_000 - (70_000_000 - required))
        .min()
        .unwrap()
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
        assert!(part1() == 1_334_506);
    }

    #[test]
    fn test_part2() {
        assert!(part2() == 7_421_137);
    }
}
