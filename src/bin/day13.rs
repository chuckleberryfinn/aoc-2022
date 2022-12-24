use std::cmp::Ordering;
use itertools::Itertools;

fn get_inputs() -> Vec<(&'static str, &'static str)> {
    include_str!("../../input/day13.txt")
        .split("\n\n")
        .filter(|l| !l.is_empty())
        .map(|l| l.split_once('\n').unwrap())
        .collect()
}

fn cmp(left: &'static str, right: &'static str) -> Ordering {
    if left.is_empty() {
        return Ordering::Greater;
    }

    if right.is_empty() {
        return Ordering::Less;
    }

    let lhs = left.chars().next().unwrap();
    let rhs = right.chars().next().unwrap();

    if lhs.is_ascii_digit() && rhs.is_ascii_digit() {
        let mut i = 0;
        for c in left.chars() {
            if !c.is_ascii_digit() {
                break;
            }
            i += 1;
        };
        let l = left[..i].parse::<u32>().unwrap();

        let mut i = 0;
        for c in right.chars() {
            if !c.is_ascii_digit() {
                break;
            }
            i += 1;
        };
        let r = right[..i].parse::<u32>().unwrap();

        if l < r {
            return Ordering::Less;
        }

        if r < l {
            return Ordering::Greater;
        }

        return cmp(&left[1..], &right[1..]);
    }

    if (lhs == '[' && rhs == '[') || (lhs == ']' && rhs == ']') {
        return cmp(&left[1..], &right[1..]);
    }

    if lhs == ']' && rhs != ']' {
        return Ordering::Less;
    }

    if rhs == ']' {
        return Ordering::Greater;
    }

    if lhs.is_ascii_digit() {
        let mut i = 0;
        for c in left.chars() {
            if !c.is_ascii_digit() {
                break;
            }
            i += 1;
        };
        let l = left[..i].parse::<u32>().unwrap();

        i = 1;
        loop {
            if right[i..i+1].parse::<u32>().is_ok() {
                break;
            } else if right.chars().nth(i).unwrap() == ']' {
                return Ordering::Greater;
            }
            i += 1;
        };
        let s = i;
        for c in right[s..].chars() {
            if !c.is_ascii_digit() {
                break;
            }
            i += 1;
        };
        let r = right[s..i].parse::<u32>().unwrap();

        if l <  r {
            return Ordering::Less;
        }

        if l > r {
            return Ordering::Greater;
        }

        if right.chars().nth(i+1).unwrap() != ']' {
            return Ordering::Less;
        }

        return cmp(&left[1..], &right[i+2..]);
    }

    if rhs.is_ascii_digit() {
        let mut i = 0;
        for c in right.chars() {
            if !c.is_ascii_digit() {
                break;
            }
            i += 1;
        };
        let r = right[..i].parse::<u32>().unwrap();

        i = 1;
        loop {
            if left[i..i+1].parse::<u32>().is_ok() {
                break;
            } else if left.chars().nth(i).unwrap() == ']' {
                return Ordering::Less;
            }
            i += 1;
        };
        let s = i;
        for c in left[s..].chars() {
            if !c.is_ascii_digit() {
                break;
            }
            i += 1;
        };
        let l = left[s..i].parse::<u32>().unwrap();

        if l < r {
            return Ordering::Less;
        }

        if l > r {
            return Ordering::Greater;
        }

        if left.chars().nth(i+1).unwrap() != ']' {
            return Ordering::Greater;
        }

        return cmp(&left[i+2..], &right[1..]);
    }

    cmp(&left[1..], &right[1..])
}

fn part1() -> usize {
    (1..)
        .zip(get_inputs())
        .filter_map(|(i, x)| match cmp(x.0, x.1) { Ordering::Less => Some(i), _ => None })
        .sum()
}

fn part2() -> usize {
    std::iter::once(&("[[2]]", "[[6]]"))
        .chain(get_inputs().iter())
        .flat_map(|s| vec![s.0, s.1])
        .sorted_by(|&a, &b| cmp(a, b))
        .zip(1..)
        .filter_map(|(x, i)| match x { "[[2]]" | "[[6]]" => Some(i), _ => None})
        .product()
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
        assert!(part1() == 4_734);
    }

    #[test]
    fn test_part2() {
        assert!(part2() == 21_836);
    }
}
