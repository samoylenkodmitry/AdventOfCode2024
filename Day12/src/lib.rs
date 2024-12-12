use std::collections::{HashMap, HashSet, VecDeque};
pub fn f2(s: &str) -> usize {
    let g: Vec<Vec<_>> = s.lines().map(|l| l.bytes().collect()).collect();
    let mut visited = HashSet::new();
    let mut res = 0;
    for y in 0..g.len() {
        for x in 0..g[0].len() {
            if !visited.contains(&(y, x)) {
                let mut q = VecDeque::new();
                q.push_back((y, x));
                let (mut p, mut s) = (0, 0);
                while let Some((y, x)) = q.pop_front() {
                    if visited.insert((y, x)) {
                        let a = g[y][x];
                        s += 1;
                        // top-left corner
                        if y == 0 && x == 0 {
                            p += 1
                        }
                        // top-right corner
                        if y == 0 && x == g[0].len() - 1 {
                            p += 1
                        }
                        // bottom-left corner
                        if y == g.len() - 1 && x == 0 {
                            p += 1
                        }
                        // bottom-right corner
                        if y == g.len() - 1 && x == g[0].len() - 1 {
                            p += 1
                        }
                        // top border
                        if y == 0 {
                            // left
                            if x > 0 {
                                if a != g[y][x - 1] {
                                    p += 1;
                                }
                            }
                            // right
                            if x < g[0].len() - 1 {
                                if a != g[y][x + 1] {
                                    p += 1;
                                }
                            }
                        }
                        // bottom border
                        if y == g[0].len() - 1 {
                            // left
                            if x > 0 {
                                if a != g[y][x - 1] {
                                    p += 1;
                                }
                            }
                            // right
                            if x < g[0].len() - 1 {
                                if a != g[y][x + 1] {
                                    p += 1;
                                }
                            }
                        }
                        // left border
                        if x == 0 {
                            // top
                            if y > 0 {
                                if a != g[y - 1][x] {
                                    p += 1;
                                }
                            }
                            // bottom
                            if y < g.len() - 1 {
                                if a != g[y + 1][x] {
                                    p += 1;
                                }
                            }
                        }
                        // right border
                        if x == g[0].len() - 1 {
                            // top
                            if y > 0 {
                                if a != g[y - 1][x] {
                                    p += 1;
                                }
                            }
                            // bottom
                            if y < g.len() - 1 {
                                if a != g[y + 1][x] {
                                    p += 1;
                                }
                            }
                        }
                        //left-top
                        if x > 0 && y > 0 {
                            // dB
                            // C*
                            let b = g[y - 1][x];
                            let c = g[y][x - 1];
                            let d = g[y - 1][x - 1];
                            if b != a && c != a {
                                p += 1;
                            }
                            // da
                            // a*
                            if d != a && b == a && c == a {
                                p += 1;
                            }
                        }
                        //left-bottom
                        if x > 0 && y < g.len() - 1 {
                            // C*
                            // dB
                            let b = g[y + 1][x];
                            let c = g[y][x - 1];
                            let d = g[y + 1][x - 1];
                            if b != a && c != a {
                                p += 1;
                            }
                            // a*
                            // da
                            if d != a && b == a && c == a {
                                p += 1;
                            }
                        }
                        // right-top
                        if x < g[0].len() - 1 && y > 0 {
                            // bd
                            // *c
                            let b = g[y - 1][x];
                            let d = g[y - 1][x + 1];
                            let c = g[y][x + 1];
                            if a != b && a != c {
                                p += 1;
                            }
                            // ad
                            // *a
                            if a != d && a == b && a == c {
                                p += 1;
                            }
                        }
                        // right-bottom
                        if x < g[0].len() - 1 && y < g.len() - 1 {
                            // *c
                            // bd
                            let b = g[y + 1][x];
                            let d = g[y + 1][x + 1];
                            let c = g[y][x + 1];
                            if a != b && a != c {
                                p += 1;
                            }
                            // ad
                            // *a
                            if a != d && a == b && a == c {
                                p += 1;
                            }
                        }
                        if y > 0 {
                            if g[y - 1][x] == g[y][x] {
                                q.push_back((y - 1, x));
                            }
                        }
                        if x > 0 {
                            if g[y][x - 1] == g[y][x] {
                                q.push_back((y, x - 1));
                            }
                        }
                        if y < g.len() - 1 {
                            if g[y + 1][x] == g[y][x] {
                                q.push_back((y + 1, x));
                            }
                        }
                        if x < g[0].len() - 1 {
                            if g[y][x + 1] == g[y][x] {
                                q.push_back((y, x + 1));
                            }
                        }
                    }
                }
                res += p * s;
            }
        }
    }
    res
}

pub fn f1(s: &str) -> usize {
    let g: Vec<Vec<_>> = s.lines().map(|l| l.bytes().collect()).collect();
    let mut visited = HashSet::new();
    let mut res = 0;
    for y in 0..g.len() {
        for x in 0..g[0].len() {
            if !visited.contains(&(y, x)) {
                let mut q = VecDeque::new();
                q.push_back((y, x));
                let (mut p, mut s) = (0, 0);
                while let Some((y, x)) = q.pop_front() {
                    if visited.insert((y, x)) {
                        s += 1;
                        p += 4;
                        if y > 0 {
                            if g[y - 1][x] == g[y][x] {
                                p -= 1;
                                q.push_back((y - 1, x));
                            }
                        }
                        if x > 0 {
                            if g[y][x - 1] == g[y][x] {
                                p -= 1;
                                q.push_back((y, x - 1));
                            }
                        }
                        if y < g.len() - 1 {
                            if g[y + 1][x] == g[y][x] {
                                p -= 1;
                                q.push_back((y + 1, x));
                            }
                        }
                        if x < g[0].len() - 1 {
                            if g[y][x + 1] == g[y][x] {
                                p -= 1;
                                q.push_back((y, x + 1));
                            }
                        }
                    }
                }
                res += p * s;
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "AAAA
BBCD
BBCC
EEEC";
        let fa = f1(a);
        assert_eq!(fa, 140);
        let b = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
        let fb = f1(b);
        assert_eq!(fb, 772);
        let d = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        let fd = f1(d);
        assert_eq!(fd, 1930);
        let c = f1(include_str!(".././input.txt"));
        println!("part1: {fa} {c}");
    }

    #[test]
    fn part2() {
        let a = "AAAA
BBCD
BBCC
EEEC";
        let fa = f2(a);
        assert_eq!(fa, 80);
        let b = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
        let fb = f2(b);
        assert_eq!(fb, 436);
        let d = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";
        let fd = f2(d);
        assert_eq!(fd, 236);
        let e = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";
        let fe = f2(e);
        assert_eq!(fe, 368);
        let g = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        let fg = f2(g);
        assert_eq!(fg, 1206);
        let c = f2(include_str!(".././input.txt"));
        println!("part2: {fa} {c}");
    }
}
