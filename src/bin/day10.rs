use itertools::Itertools;

fn get_inputs() -> Vec<i32> {
    vec![1]
        .into_iter()
        .chain(
            include_str!("../../input/day10.txt")
                .lines()
                .scan(1, |state, l| match l {
                    "noop" => Some(vec![*state]),
                    _ => {
                        let c = l.split_once(' ').unwrap().1.parse::<i32>().unwrap();
                        *state += c;
                        Some(vec![*state - c, *state])
                    }
                })
                .flatten(),
        )
        .collect()
}

fn part1() -> i32 {
    get_inputs().iter().zip(0..).fold(0, |acc, (x, i)| match i {
        19 | 59 | 99 | 139 | 179 | 219 => acc + ((i + 1) * x),
        _ => acc,
    })
}

#[allow(unstable_name_collisions)]
fn part2() -> String {
    get_inputs()
        .iter()
        .zip(0..)
        .map(|(r, i)| match ((r - 1)..(r + 2)).contains(&(i % 40)) {
            true => match i % 40 {
                0 => "\n#",
                _ => "#",
            },
            false => match i % 40 {
                0 => "\n.",
                _ => ".",
            },
        })
        .intersperse(" ")
        .collect::<String>()
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
        assert!(part1() == 17_180);
    }

    #[test]
    fn test_part2() {
        assert!(
            part2()
                ==  "\n# # # . . # # # # . # . . # . # # # . . # # # . . # . . . . # . . # . # # # . . \n\
                       # . . # . # . . . . # . . # . # . . # . # . . # . # . . . . # . . # . # . . # . \n\
                       # . . # . # # # . . # # # # . # . . # . # . . # . # . . . . # . . # . # # # . . \n\
                       # # # . . # . . . . # . . # . # # # . . # # # . . # . . . . # . . # . # . . # . \n\
                       # . # . . # . . . . # . . # . # . . . . # . # . . # . . . . # . . # . # . . # . \n\
                       # . . # . # # # # . # . . # . # . . . . # . . # . # # # # . . # # . . # # # . . \n."
        );
    }
}
