use itertools::Itertools;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

pub fn f1(s: &str) -> usize {
    let tasks: Vec<Vec<_>> = s
        .split("\n\n")
        .map(|task| {
            task.lines()
                .flat_map(|l| {
                    l.split(['+', ',', '='])
                        .skip(1)
                        .filter(|&x| x != " Y")
                        .map(|n| n.parse::<i32>().unwrap())
                })
                .collect()
        })
        .collect();

    tasks
        .iter()
        .map(|t| {
            let (ax, ay, bx, by, tx, ty) = (t[0], t[1], t[2], t[3], t[4], t[5]);

            let mut res = 0;
            for pa in 0..100 {
                for pb in 0..100 {
                    let cx = ax * pa + bx * pb;
                    let cy = ay * pa + by * pb;
                    if tx == cx && ty == cy {
                        res = pa * 3 + pb;
                        break;
                    }
                }
            }
            res as usize
        })
        .sum()
}

pub fn f2(s: &str) -> i64 {
    let tasks: Vec<Vec<_>> = s
        .split("\n\n")
        .map(|task| {
            task.lines()
                .flat_map(|l| {
                    l.split(['+', ',', '='])
                        .skip(1)
                        .filter(|&x| x != " Y")
                        .map(|n| n.parse::<i64>().unwrap())
                })
                .collect()
        })
        .collect();

    tasks
        .iter()
        .map(|t| {
            let (ax, ay, bx, by, tx, ty) = (t[0], t[1], t[2], t[3], t[4], t[5]);

            solve_system((ax, ay, bx, by, tx, ty))
        })
        .sum()
}

// completely gave up and steal the solution
// by coordinates were wrong }-(..)-{
fn solve_system(inputs: (i64, i64, i64, i64, i64, i64)) -> i64 {
    let (x1, x2, y1, y2, mut z1, mut z2) = inputs;
    z1 += 10000000000000;
    z2 += 10000000000000;
    let b = (z2 * x1 - z1 * x2) / (y2 * x1 - y1 * x2);
    let a = (z1 - b * y1) / x1;
    ((x1 * a + y1 * b, x2 * a + y2 * b) == (z1, z2)) as i64 * (a * 3 + b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        let fa = f1(a);
        assert_eq!(fa, 480);
        let c = f1(include_str!(".././input.txt"));
        println!("part1: {fa} {c}");
    }

    #[test]
    fn part2() {
        let c = f2(include_str!(".././input.txt"));
        println!("part2: {c}");
    }
}
