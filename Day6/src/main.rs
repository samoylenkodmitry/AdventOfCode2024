#![feature(test)]

extern crate test;
use std::{fs::File, io::Read};
use test::Bencher;

fn main() {
    let mut f = File::open("./6.txt").expect("can't open file");
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

use std::collections::HashSet;
fn f1(s: &str) -> usize {
    let g: Vec<Vec<u8>> = s.lines().map(|l| l.bytes().collect()).collect();
    let (y, x) = (0..g.len())
        .flat_map(|y| (0..g[0].len()).map(move |x| (y, x)))
        .find(|&(y, x)| g[y][x] != b'.' && g[y][x] != b'#')
        .expect("can't find guard");
    let mut guard = g[y][x];
    let (mut y, mut x) = (y, x);
    let ry = 0..g.len();
    let rx = 0..g[0].len();
    let mut visited = HashSet::new();
    let mut cnt = HashSet::new();
    while ry.contains(&y) && rx.contains(&x) {
        cnt.insert((y, x));
        let (ny, nx) = match guard {
            b'^' => (y - 1, x),
            b'>' => (y, x + 1),
            b'v' => (y + 1, x),
            _ => (y, x - 1),
        };
        if !ry.contains(&ny) || !rx.contains(&nx) {
            break;
        }
        let b = g[ny][nx];
        if b != b'#' {
            if !visited.insert((ny, nx, guard)) {
                break;
            }
            (y, x) = (ny, nx)
        } else {
            guard = match guard {
                b'^' => b'>',
                b'>' => b'v',
                b'v' => b'<',
                _ => b'^',
            };
        }
    }

    cnt.len()
}

fn f2(s: &str) -> usize {
    let g: Vec<Vec<u8>> = s.lines().map(|l| l.bytes().collect()).collect();
    let (y, x) = (0..g.len())
        .flat_map(|y| (0..g[0].len()).map(move |x| (y, x)))
        .find(|&(y, x)| g[y][x] != b'.' && g[y][x] != b'#')
        .expect("2: can't find guard");
    let sguard = g[y][x];
    let (sy, sx) = (y, x);
    (0..g[0].len())
        .flat_map(|y: usize| (0..g[0].len()).map(move |x| (y, x)))
        .filter(|&(y, x)| !(y == sy && x == sx) && g[y][x] == b'.')
        .filter(|&(y, x)| {
            println!(
                "try {} from 0..{}, {} from 0..{}",
                y,
                g.len(),
                x,
                g[0].len()
            );
            let (oy, ox) = (y, x);
            let mut guard = sguard;
            let (mut y, mut x) = (sy, sx);
            let ry = 0..g.len();
            let rx = 0..g[0].len();
            let mut visited = HashSet::new();
            let mut lp = false;
            while ry.contains(&y) && rx.contains(&x) {
                if !visited.insert((y, x, guard)) {
                    lp = true;
                    break;
                }
                if guard == b'^' && y == 0 || guard == b'<' && x == 0 {
                    break;
                }
                let (ny, nx) = match guard {
                    b'^' => (y.wrapping_sub(1), x),
                    b'>' => (y, x + 1),
                    b'v' => (y + 1, x),
                    _ => (y, x.wrapping_sub(1)),
                };
                if !ry.contains(&ny) || !rx.contains(&nx) {
                    break;
                }
                let b = g[ny][nx];
                if b != b'#' && !(ny == oy && nx == ox) {
                    (y, x) = (ny, nx)
                } else {
                    guard = match guard {
                        b'^' => b'>',
                        b'>' => b'v',
                        b'v' => b'<',
                        _ => b'^',
                    };
                }
            }
            lp
        })
        .count()
}

#[test]
fn day6_part1() {
    let s = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
    let a = f1(s);
    assert_eq!(a, 41);
    let b = f1(include_str!(".././6.txt"));
    println!("day6 part1: {a} {b}");
}

#[test]
fn day6_part2() {
    let s = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
    let a = f2(s);
    assert_eq!(a, 6);
    let b = f2(include_str!(".././6.txt"));
    println!("day6 part2: {a} {b}");
}
