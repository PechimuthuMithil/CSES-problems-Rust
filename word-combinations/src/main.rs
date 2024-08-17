// use std::io::stdin;
// // use std::cmp::max;
// use std::collections::HashMap;

// // fn take_vector() -> Vec<isize> {
// //     let mut input = String::new();
// //     stdin().read_line(&mut input).unwrap();
// //     input
// //         .trim()
// //         .split_whitespace()
// //         .map(|x| x.parse().unwrap())
// //         .collect()
// // }

// fn take_int() -> usize {
//     let mut input = String::new();
//     stdin().read_line(&mut input).unwrap();
//     input.trim().parse().unwrap()
// }

// fn take_string() -> String {
//     let mut input = String::new();
//     stdin().read_line(&mut input).unwrap();
//     input
// }

// fn solve() {

//     let binding = take_string();
//     let inp_string = binding.trim().to_string();
//     let k = take_int();
//     let mut dictionary = HashMap::new();
//     let n = inp_string.len();
//     let mut dp: Vec<i64> = vec![0;n+1];

//     for _ in 0..k {
//         let word = take_string();
//         let trimmed_word = word.trim().to_string(); // Convert trimmed &str back to String
//         dictionary.insert(trimmed_word, 1);
//     }

//     // println!("{:?}", dictionary);

//     dp[n] = 1;
//     for i in (0..n).rev() {
//         for j in i..n {
//             // println!("i {} j {}", i, j);
//             // println!("{:?}", &inp_string[i..j+1]);
//             if dictionary.contains_key(&inp_string[i..j+1]) {
//                 // println!("i {} j {}", i, j);
//                 // println!("{:?}", &inp_string[i..j+1]);
//                 // println!("before {:?}", dp);
//                 dp[i] = (dp[i] + dp[j+1]) % 10000000007;
//                 // println!("after {:?}", dp);
//             }
//         }
//     } 

//     println!("{}", dp[0]);
// }

// fn main() {
//     solve();
// }

use std::collections::HashMap;
use std::io::stdin;

const MOD: usize = 1_000_000_007;

struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end_of_word: bool,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            is_end_of_word: false,
        }
    }
}

struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }

    fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;
        for c in word.chars() {
            node = node.children.entry(c).or_insert_with(TrieNode::new);
        }
        node.is_end_of_word = true;
    }

    fn search(&self, s: &str, start: usize, end: usize) -> bool {
        let mut node = &self.root;
        for c in s[start..end].chars() {
            if let Some(next_node) = node.children.get(&c) {
                node = next_node;
            } else {
                return false;
            }
        }
        node.is_end_of_word
    }
}

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
    let binding = take_string();
    let inp_string = binding.trim().to_string();
    let k = take_int();
    let mut trie = Trie::new();
    let n = inp_string.len();
    let mut dp = vec![0; n + 1];

    for _ in 0..k {
        let word = take_string();
        let trimmed_word = word.trim().to_string();
        trie.insert(&trimmed_word);
    }

    dp[n] = 1; // Base case: 1 way to match the empty suffix

    for i in (0..n).rev() {
        for j in i + 1..=n {
            if trie.search(&inp_string, i, j) {
                dp[i] = (dp[i] + dp[j]) % MOD;
            }
        }
    }

    println!("{}", dp[0]);
}

fn main() {
    solve();
}
