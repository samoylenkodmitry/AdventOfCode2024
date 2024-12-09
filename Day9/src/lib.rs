use std::collections::{HashMap, HashSet};
pub fn f1(s: &str) -> usize {
    let mut v = vec![];
    let bs: Vec<_> = s
        .trim_end()
        .bytes()
        .enumerate()
        .map(|(i, b)| (b - b'0') as usize)
        .collect();

    let mut id = 0;
    for (i, &cnt) in bs.iter().enumerate() {
        if i % 2 == 0 {
            for j in 0..cnt {
                v.push(id);
            }
        } else {
            id += 1;
            for j in 0..cnt {
                v.push(0);
            }
        }
    }
    let mut j = v.len() - 1;
    let mut i = bs[0];
    while i < j {
        while i < j && v[i] != 0 {
            i += 1
        }
        while i < j && v[j] == 0 {
            j -= 1
        }
        if i < j {
            v[i] = v[j];
            v[j] = 0;
        }
    }
    let mut res = 0;
    for (i, id) in v.iter().enumerate() {
        res += i * id;
    }

    res
}

// 8509897721111 too high

pub fn f2(s: &str) -> usize {
    let mut v = vec![];
    let bs: Vec<_> = s
        .trim_end()
        .bytes()
        .enumerate()
        .map(|(i, b)| (b - b'0') as usize)
        .collect();

    let mut id = 0;
    let mut spaces = vec![];
    let mut files = vec![];
    let mut pos = 0;
    for (i, &cnt) in bs.iter().enumerate() {
        if i % 2 == 0 {
            for j in 0..cnt {
                v.push(id);
            }
            files.push((pos, id, cnt));
        } else {
            for j in 0..cnt {
                v.push(0);
            }
            id += 1;
            spaces.push((pos, cnt))
        }
        pos += cnt;
    }
    for f in (0..files.len()).rev() {
        let (file_pos, file_id, file_len) = files[f];
        let fit_ind = spaces
            .iter()
            .position(|&(pos, cnt)| pos < file_pos && cnt >= file_len);
        if let Some(ind) = fit_ind {
            let (i, old_cnt) = &mut spaces[ind];
            *old_cnt -= file_len;
            for x in 0..file_len {
                v[*i + x] = file_id;
                v[file_pos + x] = 0;
            }
            *i += file_len;
        }
    }
    let mut res = 0;
    for (i, id) in v.iter().enumerate() {
        res += i * id;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    //#[test]
    fn part1() {
        let s = "2333133121414131402";
        let a = f1(s);
        assert_eq!(a, 1928);
        let b = f1(include_str!(".././input.txt"));
        println!("part1: {a} {b}");
    }

    #[test]
    fn part2() {
        let s = "2333133121414131402";
        let a = f2(s);
        assert_eq!(a, 2858);
        let s1 = "12345";
        let a1 = f2(s1);
        assert_eq!(a1, 132);
        let b = f2(include_str!(".././input.txt"));
        println!("part2: {a} {b}");
    }
}
