use std::collections::HashMap;
pub fn f1(s: &str) -> usize {
    let mut dp = HashMap::new();
    s.trim_end()
        .split(" ")
        .map(|w| {
            let x = w.to_string();
            blink(&x, 25, &mut dp)
        })
        .sum::<usize>()
}

pub fn f2(s: &str) -> usize {
    let mut dp = HashMap::new();
    s.trim_end()
        .split(" ")
        .map(|w| {
            let x = w.to_string();
            blink(&x, 75, &mut dp)
        })
        .sum::<usize>()
}

fn blink(n: &String, cnt: usize, dp: &mut HashMap<(String, usize), usize>) -> usize {
    if cnt == 0 {
        return 1;
    }
    let key = (n.clone(), cnt);
    if let Some(dp) = dp.get(&key) {
        return *dp;
    }
    if n == "0" {
        return blink(&"1".to_string(), cnt - 1, dp);
    } else if n.len() % 2 == 0 {
        let mut i = n.len() / 2;
        while i < n.len() - 1 && n.as_bytes()[i] == b'0' {
            i += 1
        }
        return blink(&n[..n.len() / 2].into(), cnt - 1, dp) + blink(&n[i..].into(), cnt - 1, dp);
    }
    let x = n.parse::<u128>();
    let x = if x.is_err() {
        println!("can't convert to number: `{n}`");
        panic!("failed to convert")
    } else {
        x.unwrap()
    };
    let sx = format!("{}", x * 2024);
    let ss = sx.to_string();
    let res = blink(&ss, cnt - 1, dp);
    dp.insert(key, res);
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    //#[test]
    fn part1() {
        let a = "125 17";
        let fa = f1(a);
        assert_eq!(fa, 55312);
        let c = f1(include_str!(".././input.txt"));
        println!("part1: {fa} {c}");
    }

    #[test]
    fn part2() {
        let a = "125 17";
        let fa = f1(a);
        assert_eq!(fa, 55312);
        let c = f2(include_str!(".././input.txt"));
        println!("part2: {fa} {c}");
    }
}
