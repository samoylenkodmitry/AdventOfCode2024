#![feature(test)]

extern crate test;
use std::{fs::File, io::Read};
use test::Bencher;

fn main() {
    let mut f = File::open("./4.txt").expect("can't open file");
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

fn f1(s: &str) -> usize {
    let g: Vec<Vec<_>> = s.lines().map(|l| l.bytes().collect()).collect();
    let mut cnt = 0;
    let p = [b'X', b'M', b'A', b'S', b'.'];
    for y in 1..=g.len() {
        for x in 1..=g[0].len() {
            if g[y - 1][x - 1] == b'X' {
                for (dx, dy) in [
                    (0, 1),
                    (1, 0),
                    (0, -1),
                    (-1, 0),
                    (-1, -1),
                    (1, 1),
                    (-1, 1),
                    (1, -1),
                ] {
                    let mut j = 0;
                    let (mut xx, mut yy) = (x, y);
                    while j < 4
                        && yy > 0
                        && yy <= g.len()
                        && xx > 0
                        && xx <= g[0].len()
                        && g[yy - 1][xx - 1] == p[j]
                    {
                        xx = (xx as i32 + dx) as usize;
                        yy = (yy as i32 + dy) as usize;
                        j += 1
                    }
                    if j > 3 {
                        cnt += 1
                    }
                }
            }
        }
    }
    cnt
}

fn f2(s: &str) -> usize {
    let g: Vec<Vec<_>> = s.lines().map(|l| l.bytes().collect()).collect();
    let mut cnt = 0;
    for y in 1..g.len() - 1 {
        for x in 1..g[0].len() - 1 {
            if g[y][x] == b'A' {
                match [
                    g[y + 1][x + 1],
                    g[y - 1][x - 1],
                    g[y - 1][x + 1],
                    g[y + 1][x - 1],
                ] {
                    [b'M', b'S', b'M', b'S'] => cnt += 1,
                    [b'M', b'S', b'S', b'M'] => cnt += 1,
                    [b'S', b'M', b'M', b'S'] => cnt += 1,
                    [b'S', b'M', b'S', b'M'] => cnt += 1,
                    _ => {}
                }
            }
        }
    }
    cnt
}

#[test]
fn day2_part1() {
    let s = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
    let a = f1(s);
    assert_eq!(a, 18);
    let b = f1(include_str!(".././4.txt"));
    println!("day4 part1: {a} {b}");
}

#[test]
fn day2_part2() {
    let s = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
    let a = f2(s);
    assert_eq!(a, 9);
    let b = f2(include_str!(".././4.txt"));
    println!("day4 part2: {a} {b}");
}
