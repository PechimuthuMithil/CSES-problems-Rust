use std::io::stdin;
use std::cmp::max;

fn take_vector() -> Vec<isize> {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn take_int() -> usize {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn solve() {
    let n = take_int();
    let mut adj_list = vec![vec![]; n];

    for _ in 0..n-1 {
        let input = take_vector();
        adj_list[input[0] as usize - 1].push(input[1]);
        adj_list[input[1] as usize - 1].push(input[0]);
    }

    // solution borrowed from: https://usaco.guide/gold/dp-trees?lang=cpp

    let mut dp1 = vec![0;n];
    let mut dp2 = vec![0;n];

    max_matching(&adj_list, 1, 0, &mut dp1, &mut dp2);

    println!("{}", max(dp1[0], dp2[0]));

}

fn max_matching(adj_list: &Vec<Vec<isize>>, i: usize, parent: usize, dp1: &mut Vec<isize>, dp2: &mut Vec<isize>) {
    for &j in &adj_list[i - 1] {
        if j as usize != parent{
            max_matching(adj_list, j as usize, i, dp1, dp2);
            dp2[i - 1] += max(dp1[j as usize - 1], dp2[j as usize - 1]);
        }
    }

    for &j in &adj_list[i - 1] {
        if j as usize != parent {
            dp1[i - 1] = max(dp1[i - 1], dp2[j as usize - 1] + 1 + dp2[i - 1] - max(dp2[j as usize - 1], dp1[j as usize - 1]));
        }
    }
}



fn main() {
    solve();
}
