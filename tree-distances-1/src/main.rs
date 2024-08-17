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
    // Borrowed solution from: https://usaco.guide/problems/cses-1132-tree-distances-i/solution
    
    let n = take_int();
    let mut adj_list = vec![vec![]; n];

    for _ in 0..n-1 {
        let input = take_vector();
        adj_list[input[0] as usize - 1].push(input[1]);
        adj_list[input[1] as usize - 1].push(input[0]);
    }

    let mut distance = vec![vec![0;n];2];
    let mx_node = dfs(&adj_list, &mut distance, 1, 0, 0, 0);

    let mx_node2 = dfs(&adj_list, &mut distance, mx_node as usize, mx_node as usize, 0, 0);

    let _ = dfs(&adj_list, &mut distance, mx_node2 as usize, mx_node2 as usize, 0, 1);

    for i in 0..n-1 {
        print!("{} ", max(distance[0][i], distance[1][i]));
    }
    println!("{}", max(distance[0][n-1], distance[1][n-1]));
    // let mut distance = vec![0;n];
    // let mut depth = vec![0;n];

    // get_depth(&adj_list, 1, 0, &mut depth);
    // distance[0] = depth[0];
    // get_distance(&adj_list, 1, 0, &mut distance, &depth);
    // println!("{:?}", depth);
    // println!("{:?}", distance);
}

// fn get_depth(adj_list: &Vec<Vec<isize>>, i: usize, parent: usize, depth: &mut Vec<isize>) {
//     for &j in adj_list[i-1].iter() {
//         if j as usize != parent {
//             get_depth(adj_list, j as usize, i, depth);
//             depth[i-1] = max(depth[j as usize - 1] + 1, depth[i-1]);
//         }https://usaco.guide/problems/cses-1132-tree-distances-i/solution
//     }
// }

// fn get_distance(adj_list: &Vec<Vec<isize>>, i: usize, parent: usize, distance: &mut Vec<isize>, depth: &Vec<isize>) {
//     for &j in adj_list[i-1].iter() {
//         if j as usize != parent {
//             distance[j as usize - 1] = max(1 + distance[i - 1], depth[j as usize - 1]);
//             get_distance(adj_list, j as usize, i, distance, depth);
//         }
//     }
// }

fn dfs(adj_list: &Vec<Vec<isize>>, distance: &mut Vec<Vec<isize>>, i: usize, parent: usize, d: usize, index: usize) -> isize{

    distance[index][i-1] = d as isize;
    let mut opt = -1;
    for &j in adj_list[i-1].iter() {
        if j as usize != parent {
            let x = dfs(adj_list, distance, j as usize, i, d+1, index);
            if opt == -1 || distance[index][x as usize - 1] > distance[index][opt as usize - 1] {
                opt = x;
            }
        }
    }
    if opt == -1{
        return i as isize;
    } else {
        return opt as isize;
    }
}

fn main() {
    solve();
}
