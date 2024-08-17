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

// fn take_string() -> String {
//     let mut input = String::new();
//     stdin().read_line(&mut input).unwrap();
//     input
// }

fn solve() {
    let input = take_vector();
    let n = input[0] as usize;
    let m = input[1] as usize;

    let mut graph = vec![vec![0; n]; n];
    let mut visited = vec![false; n];

    for _ in 0..m {
        let row = take_vector();
        graph[row[0] as usize - 1][row[1] as usize - 1] = 1;
        graph[row[1] as usize - 1][row[0] as usize - 1] = 1;
    }

    let mut city_groups = 0;
    let mut cities = vec![];

    for i in 0..n {
        
        if !visited[i] {
            // cities[city_groups] = i as isize + 1;
            cities.push(i as isize + 1);
            dfs(&graph, &mut visited, i as isize, n as isize);
            city_groups += 1;
    
        }
    }

    println!("{}", city_groups - 1);
    for i in 0..cities.len() - 1 {
        println!("{} {}", cities[i], cities[i + 1]);
    }
    // println!("{:?}", cities);
}

fn dfs(graph: &Vec<Vec<isize>>, visited: &mut Vec<bool>, i: isize, n: isize) {
    visited[i as usize] = true;

    for j in 0..n {
        if graph[i as usize][j as usize] == 1 && !visited[j as usize] {
            dfs(graph, visited, j, n);
        }
    }
}

fn main() {
    solve();
}
