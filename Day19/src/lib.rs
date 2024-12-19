use itertools::Itertools;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

pub fn f1(s: &str) -> usize {
    let (pat, test) = s
        .split("\n\n")
        .collect_tuple()
        .expect("should have empty line");
    let pat: HashSet<String> = HashSet::from_iter(pat.split(", ").map(|s| s.to_string()));

    let mut dp = HashMap::new();
    test.lines()
        .filter(|&l| {
            fn dfs(s: &str, pat: &HashSet<String>, dp: &mut HashMap<String, bool>) -> bool {
                if let Some(cache) = dp.get(&s.to_string()) {
                    *cache
                } else {
                    let res = s.len() == 0
                        || (0..s.len())
                            .any(|i| pat.contains(&s[..=i]) && dfs(&s[i + 1..], pat, dp));
                    dp.insert(s.to_string(), res);
                    res
                }
            }
            dfs(l, &pat, &mut dp)
        })
        .count()
}

pub fn f2(s: &str) -> i128 {
    let (pat, test) = s
        .split("\n\n")
        .collect_tuple()
        .expect("should have empty line");
    let pat: HashSet<String> = HashSet::from_iter(pat.split(", ").map(|s| s.to_string()));

    let mut dp = HashMap::new();
    test.lines()
        .map(|l| {
            fn dfs(s: &str, pat: &HashSet<String>, dp: &mut HashMap<String, i128>) -> i128 {
                if let Some(cache) = dp.get(&s.to_string()) {
                    *cache
                } else {
                    let res = if s.len() == 0 {
                        1
                    } else {
                        (0..s.len())
                            .map(|i| {
                                let cnts = pat.contains(&s[..=i]);
                                if cnts {
                                    dfs(&s[i + 1..], pat, dp)
                                } else {
                                    0
                                }
                            })
                            .sum()
                    };
                    dp.insert(s.to_string(), res);
                    res
                }
            }
            dfs(l, &pat, &mut dp)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
        let fa = f1(a);
        assert_eq!(fa, 6);
        let c = f1(include_str!(".././input.txt"));
        println!("part1: {fa} {c}");
    }

    #[test]
    fn part2() {
        let a = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
        let fa = f2(a);
        assert_eq!(fa, 16);
        let c = f2(include_str!(".././input.txt"));
        println!("part2: {fa} {c}");
    }
}
