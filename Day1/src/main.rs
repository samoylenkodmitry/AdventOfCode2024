#![feature(test)]

extern crate test;
use std::{fs::File, io::Read};
use test::Bencher;

fn main() {
    let mut f = File::open("./1.txt").expect("can't open file");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("can't read");
    let a = f1(s.as_str());
    let b = f2(s.as_str());
    println!("{a} {b}")
}

#[bench]
fn bench(b: &mut Bencher) {
    b.iter(|| {
        main();
    })
}

fn f1(s: &str) -> i32 {
    let (mut l, mut r) = parse(s);
    l.sort_unstable();
    r.sort_unstable();
    l.iter().zip(r.iter()).map(|(l, r)| (l - r).abs()).sum()
}

fn f2(s: &str) -> i32 {
    let (l, r) = parse(s);
    use std::collections::HashMap;
    let mut freq = HashMap::new();
    for x in r {
        *freq.entry(x).or_default() += 1;
    }
    l.iter()
        .map(|&x| x * freq.get(&x).unwrap_or(&0))
        .sum::<i32>()
}

fn parse(s: &str) -> (Vec<i32>, Vec<i32>) {
    s.lines()
        .map(|line| {
            line.split("   ")
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|v| (v[0], v[1]))
        .unzip()
}

#[test]
fn part1() {
    let s = "3   4
4   3
2   5
1   3
3   9
3   3";
    let a = f1(s);
    let b = f1(include_str!(".././1.txt"));
    println!("day1 part1: {a} {b}");
}

#[test]
fn part2() {
    let s = "3   4
4   3
2   5
1   3
3   9
3   3";
    let a = f2(s);
    let b = f2(include_str!(".././1.txt"));
    println!("day1 part2: {a} {b}");
}
