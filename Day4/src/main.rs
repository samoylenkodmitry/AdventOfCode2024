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
    let dxy = [
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0),
        (-1, -1),
        (1, 1),
        (-1, 1),
        (1, -1),
    ];
    for y in 0..g.len() {
        for x in 0..g[0].len() {
            if g[y][x] == b'X' {
                for (dx, dy) in dxy {
                    let mut j = 0;
                    let (mut xx, mut yy) = (x, y);
                    while j < 4 && g[yy][xx] == p[j] {
                        j += 1;
                        let x = xx as i32 + dx;
                        let y = yy as i32 + dy;
                        if x.min(y) < 0 || x >= g[0].len() as i32 || y >= g.len() as i32 {
                            break;
                        }
                        xx = x as usize;
                        yy = y as usize;
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
                let a = g[y + 1][x + 1];
                let b = g[y - 1][x - 1];
                let c = g[y - 1][x + 1];
                let d = g[y + 1][x - 1];
                match [a, b, c, d] {
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
    let b = f2(include_str!(".././4.txt"));
    println!("day4 part2: {a} {b}");
}
