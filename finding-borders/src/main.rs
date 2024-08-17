// use std::collections::HashMap;
use std::io::stdin;

const MOD: usize = 1_000_000_007;

fn take_int() -> usize {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn take_string() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input
}


fn solve() {
    let inp_string = take_string();
    let n_string = inp_string.trim().to_string();
    let n = n_string.len();

    let n_bytes = n_string.as_bytes();


    // Create the LPS (Longest Prefix Suffix) array
    let mut lps = vec![0; n];
    let mut j = 0;

    // Preprocess the pattern to fill LPS array
    for i in 1..n {
        while j > 0 && n_bytes[i] != n_bytes[j] {
            j = lps[j - 1];
        }
        if n_bytes[i] == n_bytes[j] {
            j += 1;
        }
        lps[i] = j;
    }

    let mut border_len = lps[n - 1];

    let mut stack = Vec::new();
    while border_len > 0 {
        stack.push(border_len);
        border_len = lps[border_len - 1];
    }

    if stack.is_empty() {
        // println!("Impossible");
        return;
    }

    for i in (1..stack.len()).rev() {
        print!("{} ", stack[i]);
    }
    println!("{}", stack[0]);
}

fn main() {
    solve();
}