fn f1(s: &str) -> usize {
    s.lines()
        .filter(|l| {
            let nums = l
                .split_whitespace()
                .map(|w| w.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            is_ok(&nums[..])
        })
        .count()
}

fn is_ok(nums: &[i32]) -> bool {
    let sign = nums[0] < nums[1];
    nums[..]
        .windows(2)
        .all(|w| (w[0] - w[1]).abs() > 0 && (w[0] - w[1]).abs() < 4 && (w[0] < w[1]) == sign)
}

fn f2(s: &str) -> usize {
    s.lines()
        .filter(|l| {
            let nums = l
                .split_whitespace()
                .map(|w| w.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            is_ok(&nums[..])
                || (0..nums.len()).any(|i| is_ok(&[&nums[..i], &nums[i + 1..]].concat()))
        })
        .count()
}

#[test]
fn day2_part1() {
    let s = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
    let a = f1(s);
    let b = f1(include_str!(".././2.txt"));
    println!("day2 part1: {a} {b}");
}

#[test]
fn day2_part2() {
    let s = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
    let a = f2(s);
    let b = f2(include_str!(".././2.txt"));
    println!("day2 part2: {a} {b}");
}
