fn main() {}

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
    println!("part1: {a} {b}");
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
    println!("part2: {a} {b}");
}
