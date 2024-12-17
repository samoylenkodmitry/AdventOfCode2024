use itertools::Itertools;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

pub fn f1(s: &str) -> (String, i32, i32, i32) {
    let (abc, prg) = s.split_once("\n\n").expect("must have empty line");
    let abc = abc
        .lines()
        .map(|l| {
            let (_, n) = l.split_once(": ").expect("must have `:`");
            n.parse::<i32>().expect("must be a number")
        })
        .collect_vec();
    let (mut a, mut b, mut c) = (abc[0], abc[1], abc[2]);
    let prg = prg
        .trim_end()
        .strip_prefix("Program: ")
        .expect("must have `Program: `")
        .split(",")
        .map(|n| n.parse::<i32>().expect("must be a number"))
        .collect_vec();
    let mut out = vec![];
    let mut i = 0;
    let combo = |i: usize, a: i32, b: i32, c: i32| {
        let operand = prg[i];
        match operand {
            0..=3 => operand,
            4 => a,
            5 => b,
            6 => c,
            7 => panic!("used reserved operand 7, invalid program"),
            _ => panic!("unknown operand"),
        }
    };
    let literal = |i: usize| prg[i];
    loop {
        if i >= prg.len() {
            break;
        }
        let p = prg[i];
        i += 1;
        match p {
            //adv | bdv | cdv
            0 | 6 | 7 => {
                let numerator = a;
                if i == prg.len() {
                    break;
                }
                let denominator = 1 << combo(i, a, b, c);
                i += 1;
                match p {
                    0 => a = numerator / denominator,
                    6 => b = numerator / denominator,
                    7 => c = numerator / denominator,
                    _ => unreachable!(),
                }
            }
            //bxl
            1 => {
                if i == prg.len() {
                    break;
                }
                b ^= literal(i);
                i += 1;
            }
            // bst
            2 => {
                if i == prg.len() {
                    break;
                }
                b = combo(i, a, b, c) % 8;
                i += 1;
            }
            // jnz
            3 => {
                if a == 0 {
                    // does nothing
                } else {
                    if i == prg.len() {
                        break;
                    }
                    i = literal(i) as usize;
                }
            }
            // bxc
            4 => {
                b ^= c;
                i += 1;
            }
            // out
            5 => {
                if i == prg.len() {
                    break;
                }
                let o = combo(i, a, b, c) % 8;
                i += 1;
                out.push(o);
            }
            _ => panic!("unknown instruction"),
        }
    }
    (out.iter().map(|n| n.to_string()).join(","), a, b, c)
}

pub fn f2(s: &str) -> (String, i64, i64, i64) {
    let (abc, prg) = s.split_once("\n\n").expect("must have empty line");
    let abc = abc
        .lines()
        .map(|l| {
            let (_, n) = l.split_once(": ").expect("must have `:`");
            n.parse::<i64>().expect("must be a number")
        })
        .collect_vec();
    let (mut a, mut b, mut c) = (abc[0], abc[1], abc[2]);
    let sprg = prg
        .trim_end()
        .strip_prefix("Program: ")
        .expect("must have `Program: `");

    let x_len = sprg.len();
    let prg = sprg
        .split(",")
        .map(|n| n.parse::<i64>().expect("must be a number"))
        .collect_vec();
    let (sb, sc) = (b, c);
    let (mut res, mut res_a, mut res_b, mut res_c) = (vec![], 0, 0, 0);
    let mut q = vec![];
    q.push((0, 0));
    while let Some((match_count, guess)) = q.pop() {
        for i in 0..8 {
            let (out, a, b, c) = run(&prg, guess + i, 0, 0);
            if out.len() > 0 && prg[prg.len() - (match_count + 1)] == out[0] {
                if match_count + 1 == prg.len() {
                    res = out.clone();
                    res_a = guess + i;
                    res_b = b;
                    res_c = c;
                    break;
                }
                q.push((match_count + 1, (guess + i) * 8))
            }
        }
    }
    (
        res.iter().map(|n| n.to_string()).join(","),
        res_a,
        res_b,
        res_c,
    )
}

fn run(prg: &[i64], a: i64, b: i64, c: i64) -> (Vec<i64>, i64, i64, i64) {
    let (mut a, mut b, mut c) = (a, b, c);
    let mut out = vec![];
    let mut i = 0;
    let combo = |i: usize, a: i64, b: i64, c: i64| {
        let operand = prg[i] as i64;
        match operand {
            0..=3 => operand,
            4 => a,
            5 => b,
            6 => c,
            7 => panic!("used reserved operand 7, invalid program"),
            _ => panic!("unknown operand"),
        }
    };
    let literal = |i: usize| prg[i] as i64;
    loop {
        if i >= prg.len() {
            break;
        }
        let p = prg[i];
        i += 1;
        match p {
            //adv | bdv | cdv
            0 | 6 | 7 => {
                let numerator = a;
                if i == prg.len() {
                    break;
                }
                let denominator = 1 << combo(i, a, b, c);
                i += 1;
                match p {
                    0 => a = numerator / denominator,
                    6 => b = numerator / denominator,
                    7 => c = numerator / denominator,
                    _ => unreachable!(),
                }
            }
            //bxl
            1 => {
                if i == prg.len() {
                    break;
                }
                b ^= literal(i);
                i += 1;
            }
            // bst
            2 => {
                if i == prg.len() {
                    break;
                }
                b = combo(i, a, b, c) % 8;
                i += 1;
            }
            // jnz
            3 => {
                if a == 0 {
                    // does nothing
                } else {
                    if i == prg.len() {
                        break;
                    }
                    i = literal(i) as usize;
                }
            }
            // bxc
            4 => {
                b ^= c;
                i += 1;
            }
            // out
            5 => {
                if i == prg.len() {
                    break;
                }
                let o = combo(i, a, b, c) % 8;
                i += 1;
                out.push(o);
            }
            _ => panic!("unknown instruction"),
        }
    }
    (out, a, b, c)
}

#[cfg(test)]
mod tests {
    use super::*;

    //#[test]
    fn part1() {
        let b = "Register A: 10
Register B: 0
Register C: 0

Program: 5,0,5,1,5,4";
        let fb = f1(b);
        assert_eq!(fb.0, "0,1,2");

        let c = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
        let fc = f1(c);
        assert_eq!(fc.0, "4,2,5,6,7,7,7,7,3,1,0");
        assert_eq!(fc.1, 0);

        let d = "Register A: 0
Register B: 29
Register C: 0

Program: 1,7";
        let fd = f1(d);
        assert_eq!(fd.2, 26);

        let e = "Register A: 0
Register B: 2024
Register C: 43690

Program: 4,0";
        let fe = f1(e);
        assert_eq!(fe.2, 44354);

        let a = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
        let fa = f1(a);
        assert_eq!(fa.0, "4,6,3,5,6,3,5,2,1,0");
        let c = f1(include_str!(".././input.txt"));
        println!("part1: {:?} {:?}", fa, c.0);
    }

    #[test]
    fn part2() {
        let a = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";
        let fa = f2(a);
        assert_eq!(fa.0, "0,3,5,4,3,0");
        assert_eq!(fa.1, 117440);
        let c = f2(include_str!(".././input.txt"));
        println!("part2: {:?} {:?}", fa, c.1);
    }
}
