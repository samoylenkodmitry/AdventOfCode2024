#![feature(test)]

extern crate test;
use std::{fs::File, io::Read};
use test::Bencher;

fn main() {
    let mut f = File::open("./7.txt").expect("can't open file");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("can't read");
    let a = f1(s.as_str());
    let b = f2(s.as_str());
    println!("{a} {b}")
}

#[bench]
fn bench(b: &mut Bencher) {
    b.iter(|| {
        //main();
    })
}

fn f1(s: &str) -> usize {
    s.lines()
        .map(|l| l.split_once(": ").expect("must have `: `"))
        .map(|(a, b)| {
            let (result, nums) = (
                a.parse::<u64>().expect("result must be a number"),
                b.split(" ")
                    .map(|v| v.parse::<u64>().expect("must be a number"))
                    .collect::<Vec<_>>(),
            );
            fn dfs(nums: &[u64], prev: u64, res: u64) -> bool {
                if nums.len() < 1 || prev > res {
                    return prev == res;
                }
                dfs(&nums[1..], nums[0] as u64 * prev, res)
                    || dfs(&nums[1..], nums[0] as u64 + prev, res)
            }
            if dfs(&nums[1..], nums[0] as u64, result as u64) {
                result
            } else {
                0
            }
        })
        .sum::<u64>() as usize
}

fn f2(s: &str) -> usize {
    s.lines()
        .map(|l| l.split_once(": ").expect("must have `: `"))
        .map(|(a, b)| {
            let (result, nums) = (
                a.parse::<u128>().expect("result must be a number"),
                b.split(" ")
                    .map(|v| v.parse::<u128>().expect("must be a number"))
                    .collect::<Vec<_>>(),
            );
            fn dfs(nums: &[u128], prev: u128, res: u128) -> bool {
                if nums.len() < 1 || prev > res {
                    return prev == res;
                }
                let mut a = nums[0];
                let mut b = prev;
                let mut good = true;
                while a > 0 {
                    b = b * 10;
                    if b > res {
                        good = false;
                        break;
                    }
                    a = a / 10;
                }
                if good {
                    b += nums[0]
                }

                good && dfs(&nums[1..], b, res)
                    || dfs(&nums[1..], nums[0] * prev, res)
                    || dfs(&nums[1..], nums[0] + prev, res)
            }
            if dfs(&nums[1..], nums[0], result) {
                result
            } else {
                0
            }
        })
        .sum::<u128>() as usize
}

#[test]
fn day7_part1() {
    let s = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
    let a = f1(s);
    assert_eq!(a, 3749);
    let b = f1(include_str!(".././7.txt"));
    println!("day7 part1: {a} {b}");
}

#[test]
fn day7_part2() {
    let s = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
    let a = f2(s);
    assert_eq!(a, 11387);
    let b = f2(include_str!(".././7.txt"));
    println!("day7 part2: {a} {b}");
}
