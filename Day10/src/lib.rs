use std::collections::HashMap;
pub fn f1(s: &str) -> usize {
    let g: Vec<Vec<_>> = s
        .lines()
        .map(|l| l.bytes().map(|b| b - b'0').collect())
        .collect();
    (0..g.len())
        .flat_map(|y| (0..g[0].len()).map(move |x| (y, x)))
        .filter(|(y, x)| g[*y][*x] == 0)
        .map(|(y, x)| {
            let mut dp = HashMap::new();
            fn dfs(
                cy: i32,
                cx: i32,
                g: &Vec<Vec<u8>>,
                dp: &mut HashMap<(i32, i32), usize>,
            ) -> usize {
                let curr = g[cy as usize][cx as usize];
                if curr == 9 {
                    return if dp.insert((cy, cx), 1).is_none() {
                        1
                    } else {
                        0
                    };
                }
                let mut sum = 0;
                for (y, x) in [(cy - 1, cx), (cy + 1, cx), (cy, cx - 1), (cy, cx + 1)] {
                    if (0..g.len() as i32).contains(&y) && (0..g[0].len() as i32).contains(&x) {
                        let next = g[y as usize][x as usize];
                        if next == curr + 1 {
                            sum += dfs(y, x, g, dp);
                        }
                    }
                }
                sum
            }

            dfs(y as i32, x as i32, &g, &mut dp)
        })
        .sum()
}

pub fn f2(s: &str) -> usize {
    let g: Vec<Vec<_>> = s
        .lines()
        .map(|l| l.bytes().map(|b| b - b'0').collect())
        .collect();
    let mut dp = HashMap::new();
    (0..g.len())
        .flat_map(|y| (0..g[0].len()).map(move |x| (y, x)))
        .filter(|(y, x)| g[*y][*x] == 0)
        .map(|(y, x)| {
            fn dfs(
                cy: i32,
                cx: i32,
                g: &Vec<Vec<u8>>,
                dp: &mut HashMap<(i32, i32), usize>,
            ) -> usize {
                let curr = g[cy as usize][cx as usize];
                if curr == 9 {
                    return 1;
                }
                if let Some(&cache) = dp.get(&(cy, cx)) {
                    return cache;
                }
                let mut sum = 0;
                for (y, x) in [(cy - 1, cx), (cy + 1, cx), (cy, cx - 1), (cy, cx + 1)] {
                    if (0..g.len() as i32).contains(&y) && (0..g[0].len() as i32).contains(&x) {
                        let next = g[y as usize][x as usize];
                        if next == curr + 1 {
                            sum += dfs(y, x, g, dp);
                        }
                    }
                }
                dp.insert((cy, cx), sum);
                sum
            }

            dfs(y as i32, x as i32, &g, &mut dp)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        let fa = f1(a);
        assert_eq!(fa, 36);
        let c = f1(include_str!(".././input.txt"));
        println!("part1: {fa} {c}");
    }

    #[test]
    fn part2() {
        let a = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        let fa = f2(a);
        assert_eq!(fa, 81);
        let b = f2(include_str!(".././input.txt"));
        println!("part2: {fa} {b}");
    }
}
