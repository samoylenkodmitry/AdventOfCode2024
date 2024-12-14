use itertools::Itertools;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

pub fn f1(s: &str, w: i64, h: i64) -> i64 {
    let robots: Vec<(i64, i64, i64, i64)> = s
        .lines()
        .map(|l| {
            let l: Vec<_> = l
                .split(['=', ',', ' ', 'v'])
                .skip(1)
                .filter(|&x| x != "")
                .map(|x| x.parse::<i64>().expect("must be a number"))
                .collect();
            (l[0], l[1], l[2], l[3])
        })
        .collect();

    let mut q = [0, 0, 0, 0];
    let rx = [0..w / 2, w / 2 + 1..w];
    let ry = [0..h / 2, h / 2 + 1..h];
    let steps = 100;
    for &(x, y, vx, vy) in robots.iter() {
        let x1 = (w + (x + vx * steps) % w) % w;
        let y1 = (h + (y + vy * steps) % h) % h;
        let (mut qx, mut qy) = (3, 3);
        for i in 0..=1 {
            if rx[i].contains(&x1) {
                qx = i
            }
            if ry[i].contains(&y1) {
                qy = i
            }
        }
        if qx < 3 && qy < 3 {
            q[qy * 2 + qx] += 1;
        }

        //println!(
        //    "{x} {y} {vx} {vy} : {x1} {y1}, {qx} {qy}, q={}",
        //    qy * 2 + qx
        //);
    }
    q[0] * q[1] * q[2] * q[3]
}

pub fn f2(s: &str, w: i64, h: i64) -> i64 {
    let robots: Vec<(i64, i64, i64, i64)> = s
        .lines()
        .map(|l| {
            let l: Vec<_> = l
                .split(['=', ',', ' ', 'v'])
                .skip(1)
                .filter(|&x| x != "")
                .map(|x| x.parse::<i64>().expect("must be a number"))
                .collect();
            (l[0], l[1], l[2], l[3])
        })
        .collect();

    let mut set = HashSet::new();
    let mut cycle = 0;
    let mut res = 0;
    for steps in 1..10_000 {
        let mut g = [[' '; 101]; 103];
        for &(x, y, vx, vy) in robots.iter() {
            let x1 = (w + (x + vx * steps) % w) % w;
            let y1 = (h + (y + vy * steps) % h) % h;
            g[y1 as usize][x1 as usize] = '#';
        }
        /*
                let good = g
                    .iter()
                    .any(|row| row[..].windows(6).any(|w| w.iter().all(|&c| c == '.')));
        */
        let good =
            (0..w as usize).any(|x| (0..h as usize).filter(|&y| g[y][x] == '#').count() > 34);
        if !good {
            continue;
        }
        for i in 0..10 {
            println!("----{steps}----");
        }
        for y in 0..h as usize {
            for x in 0..w as usize {
                print!("{}", g[y][x]);
            }
            println!();
        }
        println!("---------");

        if good {
            res = steps;
            break;
        }
        if !set.insert(g) {
            cycle += 1;
            println!("cycle in {steps} step");
            if cycle > 0 {
                break;
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    //#[test]
    fn part1() {
        let a = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        let fa = f1(a, 11, 7);
        assert_eq!(fa, 12);
        let c = f1(include_str!(".././input.txt"), 101, 103);
        println!("part1: {fa} {c}");
    }

    #[test]
    fn part2() {
        let c = f2(include_str!(".././input.txt"), 101, 103);
        println!("part1: {c}");
    }
}
