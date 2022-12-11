use itertools::Itertools;

#[derive(Debug)]
struct Monkey {
    starting: Vec<usize>,
    operation: (&'static str, &'static str),
    test: usize,
    true_throw: usize,
    false_throw: usize,
}

impl Monkey {
    fn pop(&mut self) -> usize {
        self.starting.pop().unwrap()
    }

    fn push(&mut self, x: usize) {
        self.starting.push(x)
    }
}

fn get_inputs() -> Vec<Monkey> {
    include_str!("../../input/day11.txt")
        .split("\n\n")
        .map(|s| {
            let attributes = s.lines().collect::<Vec<&'static str>>();
            let starting = attributes[1]
                .split_once(": ")
                .unwrap()
                .1
                .split(", ")
                .map(|d| d.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
                .into_iter()
                .rev()
                .collect();
            let operation = attributes[2]
                .split_once("old ")
                .unwrap()
                .1
                .split_once(' ')
                .unwrap();
            let test = attributes[3]
                .split_once("by ")
                .unwrap()
                .1
                .parse::<usize>()
                .unwrap();
            let true_throw = attributes[4]
                .split_once("monkey ")
                .unwrap()
                .1
                .parse::<usize>()
                .unwrap();
            let false_throw = attributes[5]
                .split_once("monkey ")
                .unwrap()
                .1
                .parse::<usize>()
                .unwrap();
            Monkey {
                starting,
                operation,
                test,
                true_throw,
                false_throw,
            }
        })
        .collect()
}

fn result(rounds: usize) -> usize {
    let mut monkeys = get_inputs();
    let divisor = monkeys.iter().map(|m| m.test).product::<usize>();
    (0..rounds)
        .map(|_| {
            (0..monkeys.len())
                .map(|x| {
                    let starting_len = monkeys[x].starting.len();
                    for _ in 0..starting_len {
                        let s = monkeys[x].pop();
                        let worry = match monkeys[x].operation.0 {
                            "+" => s + monkeys[x].operation.1.parse::<usize>().unwrap(),
                            _ => match monkeys[x].operation.1 {
                                "old" => s * s,
                                _ => s * monkeys[x].operation.1.parse::<usize>().unwrap(),
                            },
                        };
                        let l = match rounds {
                            20 => worry / 3,
                            _ => worry % divisor,
                        };
                        match l % monkeys[x].test {
                            0 => {
                                let t = monkeys[x].true_throw;
                                monkeys[t].push(l);
                            }
                            _ => {
                                let f = monkeys[x].false_throw;
                                monkeys[f].push(l);
                            }
                        };
                    }
                    starting_len
                })
                .collect::<Vec<usize>>()
        })
        .reduce(|accum, item| {
            accum
                .iter()
                .zip(&item)
                .map(|(a, b)| a + b)
                .collect::<Vec<usize>>()
        })
        .unwrap()
        .iter()
        .sorted_by(|a, b| b.cmp(a))
        .take(2)
        .product()
}

fn main() {
    println!("{}", result(20));
    println!("{}", result(10_000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert!(result(20) == 316_888);
    }

    #[test]
    fn test_part2() {
        assert!(result(10_000) == 35_270_398_814);
    }
}
