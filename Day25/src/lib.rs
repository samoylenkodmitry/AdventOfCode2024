use itertools::Itertools;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::io::Bytes;
use std::iter::once;
use std::mem;

pub fn f1(s: &str) -> u64 {
    let x = s
        .split("\n\n")
        .map(|e| {
            let yx = e.lines().map(|x| x.bytes().collect_vec()).collect_vec();
            let is_key = yx[0][0] == b'.';
            (
                is_key,
                (0..5usize)
                    .map(|x| (1..6usize).filter(|&y| yx[y][x] == b'#').count())
                    .collect_vec(),
            )
        })
        .collect_vec();
    let keys = x.iter().filter(|(k, x)| *k).map(|(k, x)| x).collect_vec();
    let locks = x.iter().filter(|(k, x)| !*k).map(|(k, x)| x).collect_vec();
    let mut cnt = 0;
    for k in keys {
        for l in &locks {
            if k.iter().zip(*l).all(|(&k, &l)| k + l <= 5) {
                cnt += 1;
            }
        }
    }
    cnt
}

pub fn f2(s: &str) -> String {
    "".into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";
        let fa = f1(a);
        assert_eq!(fa, 3);
        let c = f1(include_str!(".././input.txt"));
        println!("part1: `{fa}` `{c}`");
    }

    //#[test]
    fn part2() {}
}
