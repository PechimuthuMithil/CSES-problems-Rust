use std::io::stdin;

fn take_vector() -> Vec<isize> {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn take_string() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input
}

fn solve() {
    let input = take_vector();
    let n = input[0] as usize;
    let m = input[1] as usize;

    let mut graph = vec![vec![0; m]; n];
    let mut visited = vec![vec![false; m]; n];

    for i in 0..n {
        let row = take_string();
        for j in 0..m {
            if row.as_bytes()[j] == b'#' {
                graph[i][j] = 0;
            } else {
                graph[i][j] = 1;
            }
        }
    }

    let mut rooms = 0;
    for i in 0..n {
        for j in 0..m {
            if !visited[i][j] && graph[i][j] == 1 {
                dfs(&graph, &mut visited, i as isize, j as isize, n as isize, m as isize);
                rooms += 1;
            }
        }
    }

    println!("{}", rooms);
}

fn isvalid(i: isize, j: isize, n: isize, m: isize) -> bool {
    i >= 0 && j >= 0 && i < n && j < m
}

fn dfs(graph: &Vec<Vec<isize>>, visited: &mut Vec<Vec<bool>>, i: isize, j: isize, n: isize, m: isize) {
    visited[i as usize][j as usize] = true;
    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    for direction in &directions {
        let new_i = i + direction.0;
        let new_j = j + direction.1;
        if isvalid(new_i, new_j, n, m) && !visited[new_i as usize][new_j as usize] && graph[new_i as usize][new_j as usize] == 1 {
            dfs(graph, visited, new_i, new_j, n, m);
        }
    }
}

fn main() {
    solve();
}
