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

    let mut diameter = vec![0;n];
    let mut depth = vec![0;n];

    get_diameter(&adj_list, 1, 0, &mut diameter, &mut depth);
    println!("{}", diameter[0]);
}

fn get_diameter(adj_list: &Vec<Vec<isize>>, i: usize, parent: usize, diameter: &mut Vec<isize>, depth: &mut Vec<isize>) {

    let mut max1 = 0;
    let mut max2 = 0;

    for &j in adj_list[i-1].iter() {
        if j as usize != parent {
            get_diameter(adj_list, j as usize, i, diameter, depth);
            depth[i-1] = max(depth[j as usize - 1] + 1, depth[i-1]);
            diameter[i-1] = max(diameter[j as usize - 1], diameter[i-1]);

            if depth[j as usize - 1] + 1 >= max1 {
                max2 = max1;
                max1 = depth[j as usize - 1] + 1;
            } else if depth[j as usize - 1] + 1 > max2 {
                max2 = depth[j as usize - 1] + 1;
            }
        }
    }

    diameter[i-1] = max(diameter[i-1], max1 + max2);
}

fn main() {
    solve();
}
