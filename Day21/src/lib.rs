use itertools::Itertools;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::mem;

pub fn f1(s: &str) -> u64 {
    solve(s, 2)
}

pub fn f2(s: &str) -> u64 {
    solve(s, 25)
}
fn solve(s: &str, l: u8) -> u64 {
    fn solve(map: &mut HashMap<u32, u64>, d: u8, s: &str, l: u8, p: fn(u8) -> u8) -> u64 {
        let mut f = p(b'A');
        s.bytes()
            .map(p)
            .map(|t| keypad(map, d, mem::replace(&mut f, t), t, l))
            .sum()
    }
    fn keypad(map: &mut HashMap<u32, u64>, d: u8, from: u8, to: u8, layer: u8) -> u64 {
        let key = u32::from_ne_bytes([d, from, to, layer]);
        if let Some(&p) = map.get(&key) {
            return p;
        }
        let (mut st, mut min) = (vec![(from, String::new())], u64::MAX);
        while let Some((from, press)) = st.pop() {
            if from == to {
                let p = if layer == 0 {
                    press.len() as u64 + 1
                } else {
                    let pos = |b| [2, 1, 3, 5, 4][usize::from(b) * 71 + 6 >> 4 & 7];
                    solve(map, 0, &(press + "A"), layer - 1, pos)
                };
                min = min.min(p);
            } else if from != d {
                if from / 3 > to / 3 {
                    st.push((from - 3, press.clone() + "^"));
                }
                if from / 3 < to / 3 {
                    st.push((from + 3, press.clone() + "v"));
                }
                if from % 3 > to % 3 {
                    st.push((from - 1, press.clone() + "<"));
                }
                if from % 3 < to % 3 {
                    st.push((from + 1, press.clone() + ">"));
                }
            }
        }
        *map.entry(key).insert_entry(min).get()
    }
    let d = 9u8;
    let mut m = HashMap::new();
    let p = |b| [10, 6, 7, 11, 8, 3, 4, 5, 0, 1, 2][usize::from(b) * 9 + 85 >> 3 & 15];
    s.split_terminator('\n').fold(0, |a, s| {
        let n = s.strip_suffix('A').unwrap().parse::<u64>().unwrap();
        a + n * solve(&mut m, 9, s, l, p)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "029A
980A
179A
456A
379A";
        let fa = f1(a);
        assert_eq!(fa, 126384);
        let c = f1(include_str!(".././input.txt"));
        println!("part1: {fa} {c}");
    }

    #[test]
    fn part2() {
        let a = "029A
980A
179A
456A
379A";
        let fa = f1(a);
        assert_eq!(fa, 126384);
        let c = f2(include_str!(".././input.txt"));
        println!("part2: {fa} {c}");
    }
}
