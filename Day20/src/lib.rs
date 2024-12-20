use itertools::Itertools;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

pub fn f1(s: &str, save: i32) -> usize {
    let g = s.lines().map(|l| l.bytes().collect_vec()).collect_vec();
    let syx = s.bytes().position(|b| b == b'S').expect("must have `S`");
    let (sy, sx) = (0..g.len())
        .flat_map(|y| (0..g[0].len()).map(move |x| (y, x)))
        .find(|&(y, x)| g[y][x] == b'S')
        .expect("must have `S`");
    let mut q = VecDeque::from([(sy as i32, sx as i32, 0)]);
    let (rw, rh) = (0..g.len() as i32, 0..g[0].len() as i32);
    let mut time = HashMap::new();
    while let Some((y, x, step)) = q.pop_front() {
        let prev_time = *time.get(&(y, x)).unwrap_or(&i32::MAX);
        if prev_time <= step {
            continue;
        }
        time.insert((y, x), step);
        if g[y as usize][x as usize] == b'E' {
            continue;
        }
        let (y, x) = (y as i32, x as i32);
        for (y1, x1) in [(y - 1, x), (y + 1, x), (y, x - 1), (y, x + 1)] {
            if rw.contains(&y1)
                && rh.contains(&x1)
                && (g[y1 as usize][x1 as usize] == b'.' || (g[y1 as usize][x1 as usize] == b'E'))
            {
                q.push_back((y1, x1, step + 1));
            }
        }
    }
    if false {
        for y in 0..g.len() {
            for x in 0..g[0].len() {
                if g[y][x] == b'.' || g[y][x] == b'E' {
                    print!("{:0>3}", time.get(&(y as i32, x as i32)).unwrap_or(&0));
                } else {
                    print!("{}{}{}", g[y][x] as char, g[y][x] as char, g[y][x] as char);
                }
            }
            println!("");
        }
    }
    let mut r = HashMap::new();
    for y in 0..g.len() as i32 {
        for x in 0..g[0].len() as i32 {
            if g[y as usize][x as usize] == b'.' || g[y as usize][x as usize] == b'E' {
                let curr = *time.get(&(y, x)).expect("should have visited time already");
                if rw.contains(&(y - 1)) && g[(y - 1) as usize][x as usize] == b'#' {
                    let t = *time.get(&(y - 2, x)).unwrap_or(&i32::MAX);
                    if t < curr {
                        let diff = curr - t - 2;
                        *r.entry(diff).or_insert(0) += 1
                    }
                }
                if rw.contains(&(y + 1)) && g[(y + 1) as usize][x as usize] == b'#' {
                    let t = *time.get(&(y + 2, x)).unwrap_or(&i32::MAX);
                    if t < curr {
                        let diff = curr - t - 2;
                        *r.entry(diff).or_insert(0) += 1
                    }
                }
                if rw.contains(&(x - 1)) && g[y as usize][(x - 1) as usize] == b'#' {
                    let t = *time.get(&(y, x - 2)).unwrap_or(&i32::MAX);
                    if t < curr {
                        let diff = curr - t - 2;
                        *r.entry(diff).or_insert(0) += 1
                    }
                }
                if rw.contains(&(x + 1)) && g[y as usize][(x + 1) as usize] == b'#' {
                    let t = *time.get(&(y, x + 2)).unwrap_or(&i32::MAX);
                    if t < curr {
                        let diff = curr - t - 2;
                        *r.entry(diff).or_insert(0) += 1
                    }
                }
            }
        }
    }
    let mut res = 0;
    for (diff, cnt) in r.iter() {
        if *diff >= save {
            res += cnt;
        }
    }

    res
}

pub fn f2(s: &str, save: i32) -> i128 {
    let g = s.lines().map(|l| l.bytes().collect_vec()).collect_vec();
    let syx = s.bytes().position(|b| b == b'S').expect("must have `S`");
    let (sy, sx) = (0..g.len())
        .flat_map(|y| (0..g[0].len()).map(move |x| (y, x)))
        .find(|&(y, x)| g[y][x] == b'S')
        .expect("must have `S`");
    let mut q = VecDeque::from([(sy as i32, sx as i32, 0)]);
    let (rw, rh) = (0..g.len() as i32, 0..g[0].len() as i32);
    let mut time = HashMap::new();
    while let Some((y, x, step)) = q.pop_front() {
        let prev_time = *time.get(&(y, x)).unwrap_or(&i32::MAX);
        if prev_time <= step {
            continue;
        }
        time.insert((y, x), step);
        if g[y as usize][x as usize] == b'E' {
            continue;
        }
        let (y, x) = (y as i32, x as i32);
        for (y1, x1) in [(y - 1, x), (y + 1, x), (y, x - 1), (y, x + 1)] {
            if rw.contains(&y1)
                && rh.contains(&x1)
                && (g[y1 as usize][x1 as usize] == b'.' || (g[y1 as usize][x1 as usize] == b'E'))
            {
                q.push_back((y1, x1, step + 1));
            }
        }
    }
    if false {
        for y in 0..g.len() {
            for x in 0..g[0].len() {
                if g[y][x] == b'.' || g[y][x] == b'E' {
                    print!("{:0>3}", time.get(&(y as i32, x as i32)).unwrap_or(&0));
                } else {
                    print!("{}{}{}", g[y][x] as char, g[y][x] as char, g[y][x] as char);
                }
            }
            println!("");
        }
    }
    let mut res = 0;
    let (my, mx) = (g.len() as i32 - 1, g[0].len() as i32 - 1);
    for y in 0..g.len() as i32 {
        for x in 0..g[0].len() as i32 {
            if g[y as usize][x as usize] == b'.' || g[y as usize][x as usize] == b'E' {
                let curr = *time.get(&(y, x)).expect("should have visited time already");
                let (x_s, y_s, x_e, y_e) =
                    (0.max(x - 20), 0.max(y - 20), mx.min(x + 20), my.min(y + 20));
                for y1 in y_s..=y_e {
                    for x1 in x_s..=x_e {
                        let len = (y1 - y).abs() + (x1 - x).abs();
                        if len <= 20 {
                            let t = *time.get(&(y1, x1)).unwrap_or(&(i32::MAX / 2));
                            if curr > t + len {
                                let diff = curr - t - len;
                                if diff >= save {
                                    res += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    //#[test]
    fn part1() {
        let a = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
        let fa = f1(a, 64);
        assert_eq!(fa, 1);
        let fb = f1(a, 40);
        assert_eq!(fb, 2);
        let fd = f1(a, 38);
        assert_eq!(fd, 3);
        let fe = f1(a, 36);
        assert_eq!(fe, 4);
        let f5 = f1(a, 20);
        assert_eq!(f5, 5);
        let f6 = f1(a, 12);
        assert_eq!(f6, 8);
        let f7 = f1(a, 10);
        assert_eq!(f7, 10);
        let f8 = f1(a, 8);
        assert_eq!(f8, 14);
        let f9 = f1(a, 6);
        assert_eq!(f9, 16);
        let f10 = f1(a, 4);
        assert_eq!(f10, 30);
        let f11 = f1(a, 2);
        assert_eq!(f11, 44);
        let c = f1(include_str!(".././input.txt"), 100);
        println!("part1: {fa} {c}");
    }

    #[test]
    fn part2() {
        let a = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
        let fa = f2(a, 76);
        assert_eq!(fa, 3);
        let fb = f2(a, 74);
        assert_eq!(fb, 7);
        let fd = f2(a, 72);
        assert_eq!(fd, 29);
        let fe = f2(a, 70);
        assert_eq!(fe, 41);
        let f5 = f2(a, 68);
        assert_eq!(f5, 55);
        let f6 = f2(a, 66);
        assert_eq!(f6, 67);
        let f7 = f2(a, 64);
        assert_eq!(f7, 86);
        let f8 = f2(a, 62);
        assert_eq!(f8, 106);
        let f9 = f2(a, 60);
        assert_eq!(f9, 129);
        let f10 = f2(a, 58);
        assert_eq!(f10, 154);
        let f11 = f2(a, 56);
        assert_eq!(f11, 193);
        let f12 = f2(a, 54);
        assert_eq!(f12, 222);
        let f13 = f2(a, 52);
        assert_eq!(f13, 253);
        let f14 = f2(a, 50);
        assert_eq!(f14, 285);
        let c = f2(include_str!(".././input.txt"), 100);
        println!("part2: {fa} {c}");
    }
}
