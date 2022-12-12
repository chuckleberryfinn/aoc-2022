use std::collections::HashSet;
use std::collections::VecDeque;

fn get_inputs() -> (Vec<Vec<u8>>, (i32, i32), (i32, i32), Vec<(i32, i32)>) {
    let mut grid: Vec<Vec<u8>> = include_str!("../../input/day12.txt")
        .lines()
        .map(|l| l.bytes().collect())
        .collect();

    let mut s = (0, 0);
    let mut e = (0, 0);
    let mut starts: Vec<(i32, i32)> = vec![];

    for (x, row) in (0..).zip(grid.iter()) {
        for (y, col) in (0..).zip(row.iter()) {
            if *col == b'S' {
                s = (x, y);
            } else if *col == b'E' {
                e = (x, y);
            } else if *col == b'a' {
                starts.push((x, y));
            }
        }
    }
    grid[s.0 as usize][s.1 as usize] = b'a';
    grid[e.0 as usize][e.1 as usize] = b'z';

    (grid, s, e, starts)
}

fn bfs(grid: &Vec<Vec<u8>>, s: &(i32, i32), e: &(i32, i32)) -> Option<usize> {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;
    let mut paths: Vec<Vec<(i32, i32)>> = vec![];
    let mut q: VecDeque<Vec<(i32, i32)>> = VecDeque::from([vec![*s]]);
    let mut seen: HashSet<(i32, i32)> = HashSet::new();
    let deltas = [(-1, 0), (0, -1), (1, 0), (0, 1)];

    while !q.is_empty() {
        let l = q.len();
        for _ in 0..l {
            let current_path = q.pop_front().unwrap();
            let (r, c) = *current_path.iter().last().unwrap();
            let current_char = grid[r as usize][c as usize];

            for delta in deltas {
                let current_coord = (r + delta.0, c + delta.1);
                if current_coord.0 < 0
                    || current_coord.0 >= rows
                    || current_coord.1 < 0
                    || current_coord.1 >= cols
                    || seen.contains(&current_coord)
                    || current_char < grid[current_coord.0 as usize][current_coord.1 as usize] - 1
                {
                    continue;
                }

                seen.insert(current_coord);
                let mut x = current_path.clone();
                if current_coord == *e {
                    paths.push(x)
                } else {
                    x.push(current_coord);
                    q.push_back(x);
                }
            }
        }
    }

    paths.iter().map(|p| p.len()).min()
}

fn part1() -> usize {
    let (grid, s, e, _) = get_inputs();
    bfs(&grid, &s, &e).unwrap()
}

fn part2() -> usize {
    let (grid, _, e, starts) = get_inputs();
    starts
        .iter()
        .filter_map(|s| bfs(&grid, s, &e))
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
        assert!(part1() == 504);
    }

    #[test]
    fn test_part2() {
        assert!(part2() == 500);
    }
}
