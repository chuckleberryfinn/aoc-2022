fn get_inputs() -> Vec<Vec<u32>> {
    include_str!("../../input/day08.txt")
    .lines()
    .map(|l| l.chars().map(|t| t.to_digit(10).unwrap()).collect())
    .collect()
}

fn is_visible(trees: &[Vec<u32>], i: usize, j: usize, l: usize) -> bool {
    let rows = trees[i]
        .iter()
        .enumerate()
        .map(|(x, &t)| i == 0 || i == (l-1) || j == (l-1) || j == 0 || x == j || t < trees[i][j])
        .collect::<Vec<bool>>();

    let cols = (0..l)
        .map(|x| i == 0 || i == (l-1) || j == (l-1) || j == 0 || x == i || trees[x][j] < trees[i][j])
        .collect::<Vec<bool>>();
    rows[0..j].iter().all(|&b| b) || rows[j..].iter().all(|&b| b) || cols[0..i].iter().all(|&b| b) || cols[i..].iter().all(|&b| b)
}

fn part1() -> usize {
    let trees = get_inputs();
    let l = trees.len();
    (0..l)
        .fold(0, |acc, i| {
            acc + (0..trees.len()).fold(0, |acc, j| {
                if is_visible(&trees, i, j, l) {
                    acc + 1
                } else {
                    acc
                }
            })
        })
}

fn can_see(trees: &[Vec<u32>], i: usize, j: usize, x: usize) -> usize {
    let v = trees[i][j];
    vec![
        (i - (0..i)
            .rev()
            .find(|&y| trees[y][j] >= v)
            .unwrap_or(0)),
        (i+1..x)
            .find(|&y| trees[y][j] >= v)
            .unwrap_or(x) - i,
        (j - (0..j)
            .rev()
            .find(|&y| trees[i][y] >= v)
            .unwrap_or(0)),
        ((j+1..x)
            .find(|&y| trees[i][y] >= v)
            .unwrap_or(x) - (j+1))
    ]
    .iter()
    .product()
}

fn part2() -> usize {
    let trees = get_inputs();
    let x = trees.len();

    (0..x)
        .flat_map(|i| (0..x)
            .map(|j| can_see(&trees, i, j, x))
            .collect::<Vec<usize>>()
        )
        .max()
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
        assert!(part1() == 1_829);
    }

    #[test]
    fn test_part2() {
        assert!(part2() == 291_840);
    }
}
