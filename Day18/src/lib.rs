use itertools::Itertools;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

pub fn f1(s: &str, cnt: usize, w: usize) -> u64 {
    let mut g = vec![vec![b'.'; w]; w];
    let xy: Vec<(usize, usize)> = s
        .lines()
        .map(|l| {
            l.split(",")
                .map(|n| n.parse::<usize>().expect("must be a number"))
                .collect_tuple()
                .expect("must be two numbers")
        })
        .collect();
    xy.iter().take(cnt).for_each(|&(y, x)| {
        g[y][x] = b'#';
    });
    let mut q = VecDeque::from_iter([(0i32, 0i32)]);
    let wxy = 0..w as i32;
    let mut steps = 0;
    'out: while q.len() > 0 {
        let sz = q.len();
        for _ in 0..sz {
            let (y, x) = q.pop_front().unwrap();
            if y == w as i32 - 1 && x == w as i32 - 1 {
                break 'out;
            }
            if g[y as usize][x as usize] == b'.' {
                g[y as usize][x as usize] = b'O';
                for (y1, x1) in [(y - 1, x), (y + 1, x), (y, x - 1), (y, x + 1)] {
                    if wxy.contains(&y1) && wxy.contains(&x1) {
                        q.push_back((y1, x1));
                    }
                }
            }
        }
        steps += 1;
    }
    steps
}

pub fn f2(s: &str, cnt: usize, w: usize) -> String {
    let mut g = vec![vec![b'.'; w]; w];
    let xy: Vec<(usize, usize)> = s
        .lines()
        .map(|l| {
            l.split(",")
                .map(|n| n.parse::<usize>().expect("must be a number"))
                .collect_tuple()
                .expect("must be two numbers")
        })
        .collect();
    xy.iter()
        .find(|&&(y, x)| {
            g[y][x] = b'#';
            let mut g = g.clone();
            let mut q = VecDeque::from_iter([(0i32, 0i32)]);
            let wxy = 0..w as i32;
            let mut good = false;
            'out: while q.len() > 0 {
                let sz = q.len();
                for _ in 0..sz {
                    let (y, x) = q.pop_front().unwrap();
                    if y == w as i32 - 1 && x == w as i32 - 1 {
                        good = true;
                        break 'out;
                    }
                    if g[y as usize][x as usize] == b'.' {
                        g[y as usize][x as usize] = b'O';
                        for (y1, x1) in [(y - 1, x), (y + 1, x), (y, x - 1), (y, x + 1)] {
                            if wxy.contains(&y1) && wxy.contains(&x1) {
                                q.push_back((y1, x1));
                            }
                        }
                    }
                }
            }
            !good
        })
        .map_or("all passed".into(), |&(y, x)| format!("{y},{x}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    //#[test]
    fn part1() {
        let a = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
        let fa = f1(a, 12, 7);
        assert_eq!(fa, 22);
        let c = f1(include_str!(".././input.txt"), 1024, 71);
        println!("part1: {:?} {:?}", fa, c);
    }

    #[test]
    fn part2() {
        let a = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
        let fa = f2(a, 12, 7);
        assert_eq!(fa, "6,1");
        let c = f2(include_str!(".././input.txt"), 1024, 71);
        println!("part2: {:?} {:?}", fa, c);
    }
}
