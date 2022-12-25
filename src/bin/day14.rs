use std::collections::HashSet;

fn get_inputs() -> Vec<Vec<Vec<(usize, usize)>>> {
    include_str!("../../input/day14.txt")
        .lines()
        .map(|l| {
            l.split(" -> ")
                .collect::<Vec<&str>>()
                .windows(2)
                .map(|w| {
                    w.iter()
                        .map(|e| {
                            e.split_once(',')
                                .map(|(v, w)| {
                                    (v.parse::<usize>().unwrap(), w.parse::<usize>().unwrap())
                                })
                                .unwrap()
                        })
                        .collect::<Vec<(usize, usize)>>()
                })
                .collect()
        })
        .collect()
}

fn part1() -> usize {
    let mut lowest = 0;
    let mut points: HashSet::<(usize, usize)> = HashSet::new();
    for input in get_inputs() {
        for i in input {
            for x in (i[0].0..=i[1].0).chain(i[1].0..=i[0].0) {
                for y in (i[0].1..=i[1].1).chain(i[1].1..=i[0].1) {
                    points.insert((x, y));
                    lowest = std::cmp::max(y, lowest);
                }
            }
        }
    }

    let mut count = 0;
    let mut start = (500, 0);
    while start.1 <= lowest {
        start = (500, 0);
        while start.1 <= lowest {
            if points.contains(&(start.0, start.1+1)) {
                if points.contains(&(start.0-1, start.1+1)) && points.contains(&(start.0+1, start.1+1)) {
                    points.insert(start);
                    count += 1;
                    break;
                }

                if points.contains(&(start.0-1, start.1+1)) {
                    start.0 += 1;
                } else {
                    start.0 -= 1;
                }
            }
            start.1 += 1;
        }
    }
    count
}

fn part2() -> usize {
    let mut lowest = 0;
    let mut points: HashSet::<(usize, usize)> = HashSet::new();
    for input in get_inputs() {
        for i in input {
            for x in (i[0].0..=i[1].0).chain(i[1].0..=i[0].0) {
                for y in (i[0].1..=i[1].1).chain(i[1].1..=i[0].1) {
                    points.insert((x, y));
                    lowest = std::cmp::max(y, lowest);
                }
            }
        }
    }

    lowest += 1;
    let mut count = 0;
    while !points.contains(&(500, 0)) {
        let mut start = (500, 0);
        loop {
            if points.contains(&(start.0, start.1+1)) || start.1 == lowest {
                if (points.contains(&(start.0-1, start.1+1)) && points.contains(&(start.0+1, start.1+1))) || start.1 == lowest {
                    points.insert(start);
                    count += 1;
                    break;
                }

                if points.contains(&(start.0-1, start.1+1)) {
                    start.0 += 1;
                } else {
                    start.0 -= 1;
                }
            }
            start.1 += 1;
        }
    }
    count
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
        assert!(part1() == 843);
    }

    #[test]
    fn test_part2() {
        assert!(part2() == 27_625);
    }
}
