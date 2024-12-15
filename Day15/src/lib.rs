use itertools::Itertools;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

pub fn f1(s: &str) -> i64 {
    let (map, cmds) = s.split_once("\n\n").expect("must have empty line");
    let mut g: Vec<Vec<_>> = map.lines().map(|l| l.bytes().collect()).collect();

    let (mut cy, mut cx) = (0..g.len() as i64)
        .flat_map(|y| (0..g[0].len() as i64).map(move |x| (y, x)))
        .find(|&(y, x)| g[y as usize][x as usize] == b'@')
        .expect("must have @ symbol");
    let (ry, rx) = (0..g.len() as i64, 0..g[0].len() as i64);
    for c in cmds.bytes() {
        //println!("cmd:{c} cy,cx: {cy},{cx}");
        let (dx, dy) = match c {
            b'^' => (0, -1),
            b'>' => (1, 0),
            b'v' => (0, 1),
            b'<' => (-1, 0),
            _ => continue,
        };
        let (mut y, mut x) = (cy, cx);
        while ry.contains(&(y + dy)) && rx.contains(&(x + dx)) {
            let v = g[(y + dy) as usize][(x + dx) as usize];
            if v == b'#' {
                break;
            }
            (y, x) = (y + dy, x + dx);
            if v == b'.' {
                //println!("found spot y={y} x={x}, breaking");
                break;
            }
        }
        if g[y as usize][x as usize] == b'.' {
            while y != cy || x != cx {
                //println!("moving...");
                g[y as usize][x as usize] = g[(y - dy) as usize][(x - dx) as usize];
                (y, x) = (y - dy, x - dx);
            }
            g[y as usize][x as usize] = b'.';
            (cy, cx) = (cy + dy, cx + dx);
            //println!("new pos: cy, cx: {cy} {cx}");
        }
        //println!("cmd={c}, dx,dy=`{dx}`, `{dy}`");
        /*
                for y in 0..g.len() as i64 {
                    for x in 0..g[0].len() as i64 {
                        print!("{}", g[y as usize][x as usize] as char);
                    }
                    println!();
                }
        */
    }
    let mut res = 0;
    for y in 0..g.len() as i64 {
        for x in 0..g[0].len() as i64 {
            if g[y as usize][x as usize] == b'O' {
                res += y * 100 + x
            }
        }
    }
    res
}

pub fn f2(s: &str) -> i64 {
    let cnt = s.bytes().filter(|&b| b == b'O').count();
    let (map, cmds) = s.split_once("\n\n").expect("must have empty line");
    let mut g: Vec<Vec<_>> = map
        .lines()
        .map(|l| {
            l.bytes()
                .flat_map(|b| match b {
                    b'#' => [b'#', b'#'],
                    b'O' => [b'[', b']'],
                    b'.' => [b'.', b'.'],
                    b'@' => [b'@', b'.'],
                    _ => panic!("unknown char"),
                })
                .collect()
        })
        .collect();

    let (mut cy, mut cx) = (0..g.len() as i64)
        .flat_map(|y| (0..g[0].len() as i64).map(move |x| (y, x)))
        .find(|&(y, x)| g[y as usize][x as usize] == b'@')
        .expect("must have @ symbol");
    let (ry, rx) = (0..g.len() as i64, 0..g[0].len() as i64);
    for c in cmds.bytes() {
        let before = g.clone();
        //println!("cmd:{c} cy,cx: {cy},{cx}");
        let (dx, dy) = match c {
            b'^' => (0, -1),
            b'>' => (1, 0),
            b'v' => (0, 1),
            b'<' => (-1, 0),
            _ => continue,
        };
        if c == b'^' || c == b'v' {
            let dy = if c == b'^' { -1 } else { 1 };
            let mut layers = vec![vec![(cy, cx)]];
            let mut bad_end = false;
            loop {
                let last = layers.last().unwrap();
                let next: Vec<_> = last
                    .iter()
                    .filter(|&&(y, x)| g[y as usize][x as usize] != b'.')
                    .map(|&(y, x)| (y + dy, x))
                    .collect();
                if next.iter().any(|(y, x)| {
                    !ry.contains(y) || !rx.contains(x) || g[(*y) as usize][(*x) as usize] == b'#'
                }) {
                    bad_end = true;
                    break;
                }
                let next: Vec<_> = next
                    .iter()
                    .flat_map(|&(y, x)| match g[y as usize][x as usize] {
                        b'[' => [(y, x), (y, x + 1)],
                        b']' => [(y, x - 1), (y, x)],
                        _ => [(y, x), (y, x)],
                    })
                    .dedup()
                    .collect();
                let all_good = next.iter().all(|&(y, x)| g[y as usize][x as usize] == b'.');
                layers.push(next);
                if all_good {
                    break;
                }
            }
            if bad_end {
                // ignoring
            } else {
                let all_good = layers[layers.len() - 1]
                    .iter()
                    .all(|&(y, x)| g[y as usize][x as usize] == b'.');
                if !all_good {
                    panic!("at the end layer all chars are '.'")
                }
                /*
                                if layers.len() > 2 {
                                    println!("before");
                                    for y in 0..g.len() as i64 {
                                        for x in 0..g[0].len() as i64 {
                                            print!("{}", g[y as usize][x as usize] as char);
                                        }
                                        println!();
                                    }
                                }
                */
                for i in (0..layers.len() - 1).rev() {
                    for &(y, x) in &layers[i] {
                        if g[(y + dy) as usize][x as usize] == b'.' {
                            g[(y + dy) as usize][x as usize] = g[y as usize][x as usize];
                            g[y as usize][x as usize] = b'.';
                        }
                    }
                }
                if layers.len() > 2 {
                    let a = layers[1][0];
                    let b = layers[1][1];
                    // move box part1
                    g[(cy + dy) as usize][a.1 as usize] = b'.';
                    // move box part2
                    g[(cy + dy) as usize][b.1 as usize] = b'.';
                    // place self
                    g[(cy + dy) as usize][cx as usize] = b'@';
                }
                g[cy as usize][cx as usize] = b'.';
                cy += dy;

                if layers.len() > 2 {
                    /*
                                        for y in 0..g.len() as i64 {
                                            for x in 0..g[0].len() as i64 {
                                                print!("{}", g[y as usize][x as usize] as char);
                                            }
                                            println!();
                                        }
                                        for x in layers {
                                            println!("layer: {:?}", x);
                                        }
                    */
                    //panic!("look :)");
                }
            }
        } else if c == b'v' {
        } else {
            let (mut y, mut x) = (cy, cx);
            while ry.contains(&(y + dy)) && rx.contains(&(x + dx)) {
                let v = g[(y + dy) as usize][(x + dx) as usize];
                if v == b'#' {
                    break;
                }
                (y, x) = (y + dy, x + dx);
                if v == b'.' {
                    break;
                }
            }
            if g[y as usize][x as usize] == b'.' {
                while y != cy || x != cx {
                    g[y as usize][x as usize] = g[(y - dy) as usize][(x - dx) as usize];
                    (y, x) = (y - dy, x - dx);
                }
                g[y as usize][x as usize] = b'.';
                (cy, cx) = (cy + dy, cx + dx);
            }
        }
        //       println!("cmd={c}, dx,dy=`{dx}`, `{dy}`");
        let mut c1 = 0;
        let mut c2 = 0;
        for y in 0..g.len() as i64 {
            for x in 0..g[0].len() as i64 {
                if g[y as usize][x as usize] == b'[' {
                    c1 += 1
                }
                if g[y as usize][x as usize] == b']' {
                    c2 += 1
                }
                if x > 0 {
                    let a = g[y as usize][x as usize];
                    let b = g[y as usize][(x - 1) as usize];
                    if (a == b'[' || a == b']') && a == b {
                        panic!("invalid");
                    }
                }
                //              print!("{}", g[y as usize][x as usize] as char);
            }
            //          println!();
        }
        if c1 != cnt || c2 != cnt {
            println!("cmd:{c} cy,cx: {cy},{cx}");
            println!("cmd={c}, dx,dy=`{dx}`, `{dy}`");
            for y in 0..g.len() as i64 {
                for x in 0..g[0].len() as i64 {
                    print!("{}", g[y as usize][x as usize] as char);
                }
                println!();
            }
            println!("before:");
            for y in 0..g.len() as i64 {
                for x in 0..g[0].len() as i64 {
                    print!("{}", before[y as usize][x as usize] as char);
                }
                println!();
            }
            panic!("wrong count");
        }
    }
    let mut res = 0;
    for y in 0..g.len() as i64 {
        for x in 0..g[0].len() as i64 {
            if g[y as usize][x as usize] == b'[' {
                res += y * 100 + x
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
        let b = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
        let fb = f1(b);
        assert_eq!(fb, 2028);
        let a = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
        let fa = f1(a);
        assert_eq!(fa, 10092);
        let c = f1(include_str!(".././input.txt"));
        println!("part1: {fa} {c}");
    }

    #[test]
    fn part2() {
        let s = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";
        let fs = f2(s);
        assert_eq!(fs, 618);
        let a = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
        let fa = f2(a);
        assert_eq!(fa, 9021);
        let c = f2(include_str!(".././input.txt"));
        println!("part2: {fa} {c}");
    }
}
