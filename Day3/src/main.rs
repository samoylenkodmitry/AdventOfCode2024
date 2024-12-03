#![feature(test)]

extern crate test;
use std::{fs::File, io::Read};
use test::Bencher;

fn main() {
    let mut f = File::open("./3.txt").expect("can't open file");
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
    let inds: Vec<_> = s.match_indices("mul(").collect();
    let bytes = s.as_bytes();
    inds.iter()
        .map(|&(i, _)| {
            let mut j = i + 4;
            let mut a = 0;
            let mut b = 0;
            let mut first = true;
            let mut good = false;
            while j < bytes.len() && j < i + 4 + 3 + 1 + 3 + 1 {
                if bytes[j] == b')' {
                    if b > 0 {
                        good = true
                    }
                    break;
                }
                if bytes[j] == b',' {
                    first = false
                } else {
                    let dig = bytes[j] as i32 - b'0' as i32;
                    if (0..=9).contains(&dig) {
                        if first {
                            a = a * 10 + dig;
                        } else {
                            b = b * 10 + dig;
                        }
                    } else {
                        break;
                    }
                }
                j += 1;
            }
            if good {
                (a * b) as usize
            } else {
                0
            }
        })
        .sum()
}

//250296639 too high
//11      1         1
//   000   00
//                  i
//          j
fn f2(s: &str) -> usize {
    let s = ["do()", s].concat();
    let s = s.as_str();
    let dos: Vec<_> = s.match_indices("do()").collect();
    let donts: Vec<_> = s.match_indices("don't()").collect();
    let mut i = 0;
    let mut j = 0;
    let mut res = 0;
    while i < dos.len() {
        let from = dos[i].0;
        while j < donts.len() && donts[j].0 < from {
            j += 1
        }
        if j < donts.len() {
            let to = donts[j].0;
            res += f1(&s[from + 4..to]);
            while i < dos.len() && dos[i].0 < to {
                i += 1
            }
        } else {
            res += f1(&s[from + 4..]);
            break;
        }
    }
    res
}

#[test]
fn day2_part1() {
    let s = "
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
";
    let a = f1(s);
    let b = f1(include_str!(".././3.txt"));
    println!("day3 part1: {a} {b}");
}

#[test]
fn day2_part2() {
    let s = "
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
";
    let a = f2(s);
    let b = f2(include_str!(".././3.txt"));
    println!("day3 part2: {a} {b}");
}
