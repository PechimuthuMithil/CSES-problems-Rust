use std::collections::VecDeque;
use std::io::stdin;

fn take_vector() -> Vec<isize> {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect()
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
    let mut colouring = vec![0; n];

    for _ in 0..m {
        let row = take_vector();
        graph[row[0] as usize - 1][row[1] as usize - 1] = 1;
        graph[row[1] as usize - 1][row[0] as usize - 1] = 1;
    }

    for i in 0..n {
        if colouring[i] == 0 {
            let val = bfs(&graph, &mut colouring, i as isize, 1, n as isize);
            if val == -1 {
                println!("IMPOSSIBLE");
                return;
            }
        }
    }

    for i in 0..n-1 {
        if colouring[i] == 1 {
            print!("{} ", colouring[i]);
        } else {
            print!("{} ", 2);
        }
    }
    if colouring[n-1] == 1 {
        println!("{} ", colouring[n-1]);
    } else {
        println!("{} ", 2);
    }
}


fn bfs(
    graph: &Vec<Vec<isize>>,
    colouring: &mut Vec<isize>,
    i: isize,
    colour: isize,
    n: isize,
) -> isize {
    let mut queue = VecDeque::new();
    queue.push_back(i);
    colouring[i as usize] = colour;

    while let Some(x) = queue.pop_front() {
        for y in 0..n {
            if graph[x as usize][y as usize] == 1 && colouring[y as usize] != 0 {
                if colouring[y as usize] == colouring[x as usize] {
                    // println!("IMPOSSIBLE");
                    return -1;
                }
            }
            if graph[x as usize][y as usize] == 1 && colouring[y as usize] == 0 {
                queue.push_back(y);
                colouring[y as usize] = -colouring[x as usize];
            }
        }
    }
    return 0;
}

fn main() {
    solve();
}

