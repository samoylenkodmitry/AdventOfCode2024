#![feature(test)]

extern crate test;
use std::{fs::File, io::Read};
use test::Bencher;

fn main() {
    let mut f = File::open("./5.txt").expect("can't open file");
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

use std::collections::{HashMap, HashSet};

fn f1(s: &str) -> usize {
    let (rules, updates) = s.split_once("\n\n").expect("must be double line break");
    let mut g: HashMap<i32, HashSet<i32>> = HashMap::new();
    rules.lines().for_each(|l| {
        let (from, to) = l.split_once("|").expect("line must have |");
        let (f, t) = (
            from.parse::<i32>().expect("not number"),
            to.parse::<i32>().expect("not number"),
        );
        g.entry(f).or_default().insert(t);
    });
    updates
        .lines()
        .map(|l| {
            l.split(",")
                .map(|x| x.parse::<i32>().expect("not number"))
                .collect::<Vec<_>>()
        })
        .map(|u| {
            let mut visited = HashSet::new();
            let mut good = true;
            for i in (0..u.len()).rev() {
                let curr = u[i];
                if visited.contains(&curr) {
                    good = false;
                    break;
                }

                if let Some(sibl) = g.get(&curr) {
                    for s in sibl {
                        if visited.insert(s) {}
                    }
                }
            }
            if good {
                u[u.len() / 2]
            } else {
                0
            }
        })
        .sum::<i32>() as usize
}

fn f2(s: &str) -> usize {
    let (rules, updates) = s.split_once("\n\n").expect("must be double line break");
    let mut g: HashMap<i32, HashSet<i32>> = HashMap::new();
    let rules: Vec<_> = rules
        .lines()
        .map(|l| {
            let (from, to) = l.split_once("|").expect("line must have |");
            let (f, t) = (
                from.parse::<i32>().expect("not number"),
                to.parse::<i32>().expect("not number"),
            );
            (f, t)
        })
        .collect();
    rules.iter().for_each(|&(f, t)| {
        g.entry(f).or_default().insert(t);
    });
    updates
        .lines()
        .map(|l| {
            l.split(",")
                .map(|x| x.parse::<i32>().expect("not number"))
                .collect::<Vec<_>>()
        })
        .map(|u| {
            let mut visited = HashSet::new();
            let mut good = true;
            for i in (0..u.len()).rev() {
                let curr = u[i];
                if visited.contains(&curr) {
                    good = false;
                    break;
                }

                if let Some(sibl) = g.get(&curr) {
                    for s in sibl {
                        if visited.insert(s) {}
                    }
                }
            }
            if good {
                0
            } else {
                // from some clever guy on reddit
                let rs: Vec<_> = rules
                    .iter()
                    .filter(|(f, t)| u.contains(f) && u.contains(t))
                    .collect();
                u.into_iter()
                    .find(|&x| {
                        rs.iter().filter(|&(f, t)| *f == x).count()
                            == rs.iter().filter(|&(f, t)| *t == x).count()
                    })
                    .unwrap()
            }
        })
        .sum::<i32>() as usize
}

#[test]
fn day2_part1() {
    let s = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
    let a = f1(s);
    assert_eq!(a, 143);
    let b = f1(include_str!(".././5.txt"));
    println!("day5 part1: {a} {b}");
}

#[test]
fn day2_part2() {
    let s = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
    let a = f2(s);
    assert_eq!(a, 123);
    let b = f2(include_str!(".././5.txt"));
    println!("day5 part2: {a} {b}");
}
