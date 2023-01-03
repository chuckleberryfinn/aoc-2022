use itertools::Itertools;
use rayon::prelude::*;
use regex::Regex;

fn get_distance(x: (i64, i64), y: (i64, i64)) -> i64 {
    (x.0 - y.0).abs() + (x.1 - y.1).abs()
}

fn get_inputs() -> Vec<(i64, i64, i64, i64, i64)> {
    let re = Regex::new(r"\-?\d+").unwrap();
    include_str!("../../input/day15.txt")
        .lines()
        .map(|l| {
            re.find_iter(l)
                .map(|v| v.as_str().parse::<i64>().unwrap())
                .collect_tuple()
                .map(|t: (i64, i64, i64, i64)| (t.0, t.1, t.2, t.3, get_distance((t.0, t.1), (t.2, t.3))))
                .unwrap()
        })
        .collect()
}

fn search(y: i64, inputs: &[(i64, i64, i64, i64, i64)], lower: i64, upper: i64) -> Vec<(i64, i64)> {
    let ranges: Vec<(i64, i64)> =
        inputs
            .iter()
            .map(|i| {
                let t = (y-i.1).abs();
                if t >= i.4 {
                    return (0, 0)
                }
                let d = i.4 - t;
                let j = i.0 - d;
                let k = i.0 + d;
                let r = if j < k {
                    (j, k)
                } else {
                    (k, j)
                };
                (std::cmp::max(lower, r.0), std::cmp::min(upper, r.1))
            })
            .sorted()
            .collect();

    let mut result = vec![ranges[0]];
    for range in ranges.iter().skip(1) {
        let r = result.pop().unwrap();
        if range.0 > r.1 {
            return vec![r, *range];
        } else if range.0 >= r.0 && range.1 <= r.1 {
            result.push(r);
        } else if range.0 >= r.0 && range.0 <= r.1 && range.1 >= r.1 {
            result.push((r.0, range.1));
        }
    }
    result
}

fn part1() -> i64 {
    let inputs = get_inputs().into_iter().sorted().collect::<Vec<(i64, i64, i64, i64, i64)>>();
    let lower: i64 = inputs[0].2;
    let upper: i64 = inputs[inputs.len()-1].2;
    let res = search(2_000_000, &inputs, lower, upper);
    res[0].1-res[0].0
}

fn part2() -> i64 {
    let inputs = get_inputs();
    (0..=4_000_000)
        .into_par_iter()
        .find_map_any(|y| { let res = search(y, &inputs, 0, 4_000_000); match res.len() > 1 { true => Some(((res[0].1 + 1) * 4_000_000) + y), false => None, }}).unwrap()
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
        assert!(part1() == 5_040_643);
    }

    #[test]
    fn test_part2() {
        assert!(part2() == 11_016_575_214_126);
    }
}
