fn get_inputs() -> (Vec<Vec<char>>, Vec<(usize, usize, usize)>) {
    include_str!("../../input/day05.txt")
        .split_once("\n\n")
        .map(|(stacks, commands)| (parse_stacks(stacks), parse_commands(commands)))
        .unwrap()
}

fn parse_stacks(stacks: &'static str) -> Vec<Vec<char>> {
    stacks
        .lines()
        .map(|l| {
            l.chars()
                .collect::<Vec<char>>()
                .chunks(4)
                .map(|c| *c.get(1).unwrap())
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>()
}

fn parse_commands(commands: &'static str) -> Vec<(usize, usize, usize)> {
    commands
        .lines()
        .map(|c| c.split(' ').collect())
        .map(|x: Vec<&str>| {
            (
                x[1].parse().unwrap(),
                x[3].parse().unwrap(),
                x[5].parse().unwrap(),
            )
        })
        .collect()
}

fn parse_all_stacks(stacks: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut all_stacks: Vec<Vec<char>> = (0..(stacks[0].len())).map(|_| vec![]).collect();

    for stack in stacks.iter().rev() {
        for (i, &v) in stack.iter().enumerate() {
            if v != ' ' {
                all_stacks[i].push(v);
            }
        }
    }
    all_stacks
}

fn format_string(all_stacks: &[Vec<char>]) -> String {
    all_stacks
        .iter()
        .map(|v| v.last().unwrap())
        .collect::<String>()
}

fn part1() -> String {
    let (stacks, commands) = get_inputs();
    let mut all_stacks = parse_all_stacks(&stacks);

    for m in &commands {
        let mut temp: Vec<char> = (0..m.0)
            .map(|_| all_stacks[m.1 - 1].pop().unwrap())
            .collect();
        all_stacks[m.2 - 1].append(&mut temp);
    }
    format_string(&all_stacks)
}

fn part2() -> String {
    let (stacks, commands) = get_inputs();
    let mut all_stacks = parse_all_stacks(&stacks);

    for m in &commands {
        let temp: Vec<char> = (0..m.0)
            .map(|_| all_stacks[m.1 - 1].pop().unwrap())
            .collect();
        all_stacks[m.2 - 1].append(&mut temp.into_iter().rev().collect());
    }
    format_string(&all_stacks)
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
        assert!(part1() == "CNSZFDVLJ");
    }

    #[test]
    fn test_part2() {
        assert!(part2() == "QNDWLMGNS");
    }
}
