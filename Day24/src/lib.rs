use itertools::Itertools;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::iter::once;
use std::mem;

pub fn f1(s: &str) -> u64 {
    let (init, conn) = s.split_once("\n\n").expect("should have an empty line");
    let mut w = HashMap::new();
    for line in init.lines() {
        let (id, v) = line.split_once(": ").expect("should have `: `");
        w.insert(id, v.parse::<u8>().expect("must be a number"));
    }
    let mut g = HashMap::new();
    let mut og = HashMap::new();
    let mut outs = vec![];
    for line in conn.lines() {
        let (gate, out) = line.split_once(" -> ").expect("must have ` -> `");
        let (a, op, b) = gate.split(' ').collect_tuple().expect("");
        g.insert((a, op, b), out);
        og.insert(out, (a, op, b));
        if out.starts_with('z') {
            outs.push(out);
        }
    }
    fn dfs<'a>(
        id: &'a str,
        w: &mut HashMap<&'a str, u8>,
        og: &HashMap<&str, (&'a str, &str, &'a str)>,
    ) -> u8 {
        if let Some(v) = w.get(id) {
            return *v;
        }
        let (a, op, b) = og[id];
        let va = dfs(a, w, og);
        let vb = dfs(b, w, og);

        let res = match op {
            "AND" => va & vb,
            "XOR" => va ^ vb,
            "OR" => va | vb,
            _ => panic!("unsupported op"),
        };
        w.insert(id, res);
        res
    }

    outs.sort_unstable();
    outs.reverse();
    let res = outs.iter().map(|&o| dfs(o, &mut w, &og)).collect_vec();
    res.iter().fold(0, |r, &t| r * 2 + t as u64)
}

pub fn f2(s: &str) -> String {
    let (init, conn) = s.split_once("\n\n").expect("should have an empty line");
    let mut wire_map: HashMap<&str, Vec<(&str, &str)>> = HashMap::default();
    let mut gate_connections = vec![];
    for line in conn.lines() {
        let (gate, out) = line.split_once(" -> ").expect("must have ` -> `");
        let (a, op, b) = gate.split(' ').collect_tuple().expect("");
        gate_connections.push([a, op, b, out]);
    }

    // we need a map to know what operations follow another operation
    for &[lhs, op, rhs, ret] in gate_connections.iter() {
        wire_map.entry(lhs).or_insert(vec![]).push((op, ret));
        wire_map.entry(rhs).or_insert(vec![]).push((op, ret));
    }

    let mut wrong_outputs = vec![];
    for &[lhs, op, rhs, ret] in gate_connections.iter() {
        // basically we ensure the adder looks like this:
        // https://en.wikipedia.org/wiki/Adder_(electronics)#/media/File:Fulladder.gif
        let chained_ops = wire_map.get(&ret);
        let chained_ops_contain =
            |op| chained_ops.is_some_and(|v| v.iter().find(|a| a.0 == op).is_some());

        let has_chained_xor = chained_ops_contain("XOR");
        let has_chained_and = chained_ops_contain("AND");
        let has_chained_or = chained_ops_contain("OR");
        let takes_first_input = lhs.ends_with("00") && rhs.ends_with("00");
        let takes_input_bit = (lhs.starts_with('x') && rhs.starts_with('y'))
            || (rhs.starts_with('x') && lhs.starts_with('y'));
        let outputs_bit = ret.starts_with('z');
        let outputs_last_bit = ret == "z45";

        let valid = match op {
            "XOR" => {
                // XOR only outputs a bit if it doesn't take an input bit
                if !takes_input_bit && outputs_bit {
                    true
                // XOR only takes an input bit if a XOR follows it
                } else if takes_input_bit && has_chained_xor {
                    true
                // unless the input bits are the first bits (no carryover bit exists)
                } else if takes_first_input && outputs_bit {
                    true
                } else {
                    false
                }
            }
            "OR" => {
                // OR either outputs into z45 or an AND and XOR (carryover bit)
                if outputs_last_bit || (has_chained_and && has_chained_xor) {
                    true
                } else {
                    false
                }
            }
            "AND" => {
                // ANDs only lead into ORs
                if has_chained_or {
                    true
                // unless the input bits are the first bits (no carryover bit exists)
                } else if takes_first_input {
                    true
                } else {
                    false
                }
            }
            _ => {
                unreachable!()
            }
        };
        if !valid {
            wrong_outputs.push(ret);
        }
    }

    wrong_outputs.sort_unstable();
    wrong_outputs.join(",").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    //#[test]
    fn part1() {
        let a = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";
        let fa = f1(a);
        assert_eq!(fa, 4);
        let b = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";
        let fb = f1(b);
        assert_eq!(fb, 2024);
        let c = f1(include_str!(".././input.txt"));
        println!("part1: `{fa}` `{c}`");
    }

    #[test]
    fn part2() {
        let a = "x00: 0
x01: 1
x02: 0
x03: 1
x04: 0
x05: 1
y00: 0
y01: 0
y02: 1
y03: 1
y04: 0
y05: 1

x00 AND y00 -> z05
x01 AND y01 -> z02
x02 AND y02 -> z01
x03 AND y03 -> z03
x04 AND y04 -> z04
x05 AND y05 -> z00";
        let fa = f2(a);
        //assert_eq!(fa, "z00,z01,z02,z05");
        let c = f2(include_str!(".././input.txt"));
        println!("part2: `{fa}` `{c}`");
    }
}
