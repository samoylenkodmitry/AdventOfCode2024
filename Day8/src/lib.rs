use std::collections::{HashMap, HashSet};
pub fn f1(s: &str) -> usize {
    let g: Vec<Vec<u8>> = s.lines().map(|l| l.bytes().collect()).collect();
    let mut m = HashMap::new();
    (0..g.len())
        .flat_map(|y| (0..g[0].len()).map(move |x| (y, x)))
        .for_each(|(y, x)| {
            if g[y][x] != b'.' {
                m.entry(g[y][x]).or_insert(vec![]).push((y, x))
            }
        });
    let mut uniq = HashSet::new();
    let rx = 0..g[0].len() as i32;
    let ry = 0..g.len() as i32;
    m.iter().for_each(|(c, v)| {
        for i in 0..v.len() {
            let (x1, y1) = v[i];
            let (x1, y1) = (x1 as i32, y1 as i32);
            for j in 0..v.len() {
                if i == j {
                    continue;
                }
                let (x2, y2) = v[j];
                let (x2, y2) = (x2 as i32, y2 as i32);
                let dx = x2 - x1;
                let x3 = x2 + dx;

                let dy = y2 - y1;
                let y3 = y2 + dy;
                if rx.contains(&x3) && ry.contains(&y3) {
                    uniq.insert((y3, x3));
                }
            }
        }
    });
    uniq.len()
}

pub fn f2(s: &str) -> usize {
    let g: Vec<Vec<u8>> = s.lines().map(|l| l.bytes().collect()).collect();
    let mut m = HashMap::new();
    (0..g.len())
        .flat_map(|y| (0..g[0].len()).map(move |x| (y, x)))
        .for_each(|(y, x)| {
            if g[y][x] != b'.' {
                m.entry(g[y][x]).or_insert(vec![]).push((y, x))
            }
        });
    let mut uniq = HashSet::new();
    let rx = 0..g[0].len() as i32;
    let ry = 0..g.len() as i32;
    m.iter().for_each(|(c, v)| {
        for i in 0..v.len() {
            let (x1, y1) = v[i];
            let (x1, y1) = (x1 as i32, y1 as i32);
            uniq.insert((y1, x1));
            for j in 0..v.len() {
                if i == j {
                    continue;
                }
                let (x2, y2) = v[j];
                let (x2, y2) = (x2 as i32, y2 as i32);
                let dx = x2 - x1;
                let dy = y2 - y1;
                let mut x3 = x2;
                let mut y3 = y2;

                loop {
                    x3 += dx;
                    y3 += dy;
                    if !rx.contains(&x3) || !ry.contains(&y3) {
                        break;
                    }
                    uniq.insert((y3, x3));
                }
            }
        }
    });
    uniq.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let s = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        let a = f1(s);
        assert_eq!(a, 14);
        let b = f1(include_str!(".././input.txt"));
        println!("part1: {a} {b}");
    }

    #[test]
    fn part2() {
        let s = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        let a = f2(s);
        assert_eq!(a, 34);
        let b = f2(include_str!(".././input.txt"));
        println!("part2: {a} {b}");
    }
}
