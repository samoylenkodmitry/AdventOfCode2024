fn dfs(nums: &[u64], prev: u64, res: u64) -> bool {
    if nums.is_empty() || prev > res {
        return prev == res;
    }
    dfs(&nums[1..], nums[0] * prev, res) || dfs(&nums[1..], nums[0] + prev, res)
}

fn dfs2(nums: &[u64], prev: u64, res: u64) -> bool {
    if nums.is_empty() || prev > res {
        return prev == res;
    }
    let mut a = nums[0];
    let mut b = prev;
    let mut good = true;
    while a > 0 {
        b *= 10;
        if b > res {
            good = false;
            break;
        }
        a /= 10;
    }
    if good {
        b += nums[0]
    }

    good && dfs2(&nums[1..], b, res)
        || dfs2(&nums[1..], nums[0] * prev, res)
        || dfs2(&nums[1..], nums[0] + prev, res)
}
pub fn f1(s: &str) -> usize {
    s.lines()
        .map(|l| l.split_once(": ").expect("must have `: `"))
        .map(|(a, b)| {
            let (result, nums) = (
                a.parse::<u64>().expect("result must be a number"),
                b.split(" ")
                    .map(|v| v.parse::<u64>().expect("must be a number"))
                    .collect::<Vec<_>>(),
            );
            if dfs(&nums[1..], nums[0], result) {
                result
            } else {
                0
            }
        })
        .sum::<u64>() as usize
}

pub fn f2(s: &str) -> usize {
    s.lines()
        .map(|l| l.split_once(": ").expect("must have `: `"))
        .map(|(a, b)| {
            let (result, nums) = (
                a.parse::<u64>().expect("result must be a number"),
                b.split(" ")
                    .map(|v| v.parse::<u64>().expect("must be a number"))
                    .collect::<Vec<_>>(),
            );
            if dfs2(&nums[1..], nums[0], result) {
                result
            } else {
                0
            }
        })
        .sum::<u64>() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let s = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        let a = f1(s);
        assert_eq!(a, 3749);
        let b = f1(include_str!(".././input.txt"));
        println!("part1: {a} {b}");
    }

    #[test]
    fn part2() {
        let s = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        let a = f2(s);
        assert_eq!(a, 11387);
        let b = f2(include_str!(".././input.txt"));
        println!("part2: {a} {b}");
    }
}

