use std::{fs::File, io::Read};
use day::{f1, f2};

fn main() {
    let mut f = File::open("./input.txt").expect("can't open file");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("can't read");
    let a = f1(s.as_str());
    let b = f2(s.as_str());
    println!("{a} {b}")
}
