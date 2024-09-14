use std::io::stdin;

fn take_int() -> usize {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn take_vector() -> Vec<isize> {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn solve() {
    // ======================= Code Here =========================
    let n = take_int();
    let p = take_vector();
    let mut q = p.clone();
    q.reverse();

    for i in 0..n {
        print!("{} ", q[i]);
    }
    println!();
}

pub fn main() {
    let t = take_int();
    for _ in 0..t {
        solve();
    }
}