use std::collections::VecDeque;
use std::io::stdin;

fn take_vector() -> Vec<isize> {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect()
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
    let mut visited = vec![vec![String::new(); m]; n];
    let mut start = (0, 0);
    let mut end = (0, 0);

    for i in 0..n {
        let row = take_string();
        for j in 0..m {
            match row.as_bytes()[j] {
                b'#' => graph[i][j] = 0,
                b'.' => graph[i][j] = 1,
                b'A' => {
                    graph[i][j] = 1;
                    start = (i, j);
                }
                b'B' => {
                    graph[i][j] = 1;
                    end = (i, j);
                }
                _ => {}
            }
        }
    }

    let mut reachable = false;
    bfs(&graph, &mut visited, start.0 as isize, start.1 as isize, n as isize, m as isize);

    if !visited[end.0][end.1].is_empty() {
        reachable = true;
    }

    if reachable {
        println!("YES");
        println!("{}", visited[end.0][end.1].len() - 1);
        println!("{}", &visited[end.0][end.1][1..]);
    } else {
        println!("NO");
    }
}

fn isvalid(i: isize, j: isize, n: isize, m: isize) -> bool {
    i >= 0 && j >= 0 && i < n && j < m
}

fn bfs(
    graph: &Vec<Vec<isize>>,
    visited: &mut Vec<Vec<String>>,
    i: isize,
    j: isize,
    n: isize,
    m: isize,
) {
    let directions = vec![('R', 0, 1), ('L', 0, -1), ('D', 1, 0), ('U', -1, 0)];
    let mut queue = VecDeque::new();
    queue.push_back((i, j));
    visited[i as usize][j as usize].push_str("N");

    while let Some((x, y)) = queue.pop_front() {
        for direction in &directions {
            let new_x = x + direction.1;
            let new_y = y + direction.2;
            if isvalid(new_x, new_y, n, m)
                && graph[new_x as usize][new_y as usize] == 1
                && visited[new_x as usize][new_y as usize].is_empty()
            {
                queue.push_back((new_x, new_y));
                let path = visited[x as usize][y as usize].clone() + &direction.0.to_string();
                visited[new_x as usize][new_y as usize] = path;
            }
        }
    }
}

fn main() {
    solve();
}

