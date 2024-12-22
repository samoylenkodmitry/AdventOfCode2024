use itertools::Itertools;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::mem;

pub fn f1(s: &str) -> u64 {
    s.lines()
        .map(|l| l.parse::<u64>().unwrap())
        .map(|s| {
            let mut x = s;
            for _ in 0..2000 {
                x = step(x)
            }
            x
        })
        .sum()
}

fn step(x: u64) -> u64 {
    let mut x = x;
    x ^= x * 64;
    x %= 16777216;
    x ^= x / 32; //(x as f64 / 32f64).round() as u64;
    x %= 16777216;
    x ^= x * 2048;
    x %= 16777216;
    x
}

pub fn f2(s: &str) -> u64 {
    let mut changes_global = HashMap::new();
    s.lines().map(|l| l.parse::<u64>().unwrap()).for_each(|s| {
        let mut x = s;
        let mut changes = HashMap::new();
        let mut prev = (s % 10) as i32;
        let mut key = VecDeque::new();

        for i in 1..=2000 {
            x = step(x);
            let change = ((x as i32 % 10) - prev) as i32;
            prev = (x % 10) as i32;
            key.push_back(change);
            if key.len() > 4 {
                key.pop_front();
            }
            if key.len() == 4 {
                if !changes.contains_key(&key) {
                    changes.insert(key.clone(), x % 10);
                    *changes_global.entry(key.clone()).or_insert(0) += x % 10;
                }
            }
        }
    });
    changes_global.into_values().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    //#[test]
    fn f() {
        assert_eq!(step(123), 15887950);
        assert_eq!(step(step(123)), 16495136);

        let r = vec![
            15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
            5908254,
        ];
        let mut x = 123;
        for r in r {
            x = step(x);
            assert_eq!(x, r)
        }
    }

    //#[test]
    fn part1() {
        let a = "1
10
100
2024";
        let fa = f1(a);
        assert_eq!(fa, 37327623);
        let c = f1(include_str!(".././input.txt"));
        println!("part1: {fa} {c}");
    }

    #[test]
    fn part2() {
        let a = "1
2
3
2024";
        let fa = f2(a);
        assert_eq!(fa, 23);
        let c = f2(include_str!(".././input.txt"));
        println!("part2: {fa} {c}");
    }
}
