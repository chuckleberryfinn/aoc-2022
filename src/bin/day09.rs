use std::collections::HashSet;
fn get_inputs() -> Vec<(char, i32)> {
    include_str!("../../input/day09.txt")
        .lines()
        .map(|l| {
            l.split_once(' ')
                .map(|(d, c)| (d.chars().next().unwrap(), c.parse::<i32>().unwrap()))
                .unwrap()
        })
        .collect()
}

fn is_adjacent(head: &(i32, i32), tail: &(i32, i32)) -> bool {
    (head.0 == tail.0 && (head.1 - tail.1).abs() <= 1)
        || (head.1 == tail.1 && (head.0 - tail.0).abs() <= 1)
        || ((head.0 - tail.0).abs() <= 1 && (head.1 - tail.1).abs() <= 1)
}

fn result(length: usize) -> usize {
    let mut knots = vec![(0, 0); length];
    let n = length - 1;

    get_inputs()
        .iter()
        .flat_map(|i| {
            (0..i.1)
                .map(|_| {
                    match i.0 {
                        'U' => knots[0].0 += 1,
                        'D' => knots[0].0 -= 1,
                        'L' => knots[0].1 -= 1,
                        _ => knots[0].1 += 1,
                    };
                    *(0..n)
                        .map(|x| {
                            match is_adjacent(&knots[x], &knots[x + 1]) {
                                true => (),
                                false => {
                                    match knots[x + 1].0.cmp(&knots[x].0) {
                                        std::cmp::Ordering::Less => knots[x + 1].0 += 1,
                                        std::cmp::Ordering::Greater => knots[x + 1].0 -= 1,
                                        _ => (),
                                    };
                                    match knots[x + 1].1.cmp(&knots[x].1) {
                                        std::cmp::Ordering::Less => knots[x + 1].1 += 1,
                                        std::cmp::Ordering::Greater => knots[x + 1].1 -= 1,
                                        _ => (),
                                    };
                                }
                            }
                            knots[x + 1]
                        })
                        .collect::<Vec<(i32, i32)>>()
                        .last()
                        .unwrap()
                })
                .collect::<Vec<(i32, i32)>>()
        })
        .collect::<HashSet<(i32, i32)>>()
        .len()
}

fn part1() -> usize {
    result(2)
}

fn part2() -> usize {
    result(10)
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
        assert!(part1() == 6_209);
    }

    #[test]
    fn test_part2() {
        assert!(part2() == 2_460);
    }
}
