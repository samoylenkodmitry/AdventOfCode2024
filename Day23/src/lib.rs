use itertools::Itertools;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::iter::once;
use std::mem;

fn g(s: &str) -> HashMap<&str, Vec<&str>> {
    let mut g = HashMap::new();
    s.lines()
        .map(|l| l.split_once('-').unwrap())
        .for_each(|(a, b)| {
            g.entry(a).or_insert(vec![]).push(b);
            g.entry(b).or_insert(vec![]).push(a);
        });
    g
}
pub fn f1(s: &str) -> u64 {
    let g = g(s);
    let mut res = HashSet::new();
    for &k in g.keys() {
        if k.starts_with('t') {
            let sibl = g.get(k).unwrap();
            for &s1 in sibl {
                let sibl2 = g.get(s1).unwrap();
                for &s2 in sibl2 {
                    let sibl3 = g.get(s2).unwrap();
                    if sibl3.contains(&k) {
                        let mut x = [k, s1, s2];
                        x.sort_unstable();
                        res.insert(x);
                    }
                }
            }
        }
    }
    res.len() as u64
}

fn bron_kerbosch<'a>(
    curr: HashSet<&'a str>,
    mut potential: HashSet<&'a str>,
    mut processed: HashSet<&'a str>,
    g: &'a HashMap<&'a str, HashSet<&'a str>>,
    res: &mut HashSet<&'a str>,
) {
    if potential.is_empty() && processed.is_empty() {
        if curr.len() > res.len() {
            *res = curr;
        }
        return;
    }
    for node in potential.clone() {
        let sibl = &g[node];
        bron_kerbosch(
            curr.iter().copied().chain(once(node)).collect(),
            potential.intersection(sibl).copied().collect(),
            processed.intersection(sibl).copied().collect(),
            g,
            res,
        );
        potential.remove(node);
        processed.insert(node);
    }
}
pub fn f2(s: &str) -> String {
    let g = g(s);
    let mut g2: HashMap<&str, HashSet<&str>> = HashMap::new();
    for (k, v) in g {
        g2.insert(k, HashSet::from_iter(v.into_iter()));
    }
    let potential = g2.keys().copied().collect();
    let processed = HashSet::new();
    let mut res = HashSet::new();
    let curr = HashSet::new();
    bron_kerbosch(curr, potential, processed, &g2, &mut res);
    res.into_iter().sorted_unstable().join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";
        let fa = f1(a);
        assert_eq!(fa, 7);
        let c = f1(include_str!(".././input.txt"));
        println!("part1: `{fa}` `{c}`");
    }

    #[test]
    fn part2() {
        let a = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";
        let fa = f2(a);
        assert_eq!(fa, "co,de,ka,ta");
        let c = f2(include_str!(".././input.txt"));
        println!("part2: `{fa}` `{c}`");
    }
}
