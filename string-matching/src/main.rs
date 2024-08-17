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

fn kmp_pattern_search(s: &str, pattern: &str) -> usize {
    let n = s.len();
    let m = pattern.len();
    let s_bytes = s.as_bytes();
    let p_bytes = pattern.as_bytes();

    // Create the LPS (Longest Prefix Suffix) array
    let mut lps = vec![0; m];
    let mut j = 0;

    // Preprocess the pattern to fill LPS array
    for i in 1..m {
        while j > 0 && p_bytes[i] != p_bytes[j] {
            j = lps[j - 1];
        }
        if p_bytes[i] == p_bytes[j] {
            j += 1;
        }
        lps[i] = j;
    }

    let mut count = 0;
    let mut i = 0;
    j = 0;

    // Search for the pattern in the string
    while i < n {
        if p_bytes[j] == s_bytes[i] {
            j += 1;
            i += 1;
        }

        if j == m {
            count += 1;
            j = lps[j - 1];
        } else if i < n && p_bytes[j] != s_bytes[i] {
            if j != 0 {
                j = lps[j - 1];
            } else {
                i += 1;
            }
        }
    }

    count
}

fn z_algorithm(s: &str, pattern: &str) -> usize {
    let combined = format!("{}${}", pattern, s);
    let combined_bytes = combined.as_bytes();
    let z = compute_z_array(&combined);
    let mut count = 0;
    let pattern_len = pattern.len();

    for &z_val in z.iter().skip(pattern_len + 1) {
        if z_val == pattern_len {
            count += 1;
        }
    }

    count
}

fn compute_z_array(s: &str) -> Vec<usize> {
    let n = s.len();
    let s_bytes = s.as_bytes();
    let mut z = vec![0; n];
    let mut l = 0;
    let mut r = 0;

    for i in 1..n {
        if i <= r {
            z[i] = std::cmp::min(r - i + 1, z[i - l]);
        }
        while i + z[i] < n && s_bytes[z[i]] == s_bytes[i + z[i]] {
            z[i] += 1;
        }
        if i + z[i] - 1 > r {
            l = i;
            r = i + z[i] - 1;
        }
    }

    z
}

fn solve() {
    let inp_string1 = take_string();
    let n_string = inp_string1.trim().to_string();
    // let n = n_string.len();

    let inp_string2 = take_string();
    let m_string = inp_string2.trim().to_string();
    // let m = m_string.len();

    // println!("{}", z_algorithm(&n_string, &m_string));
    println!("{}", kmp_pattern_search(&n_string, &m_string));
}

fn main() {
    solve();
}
