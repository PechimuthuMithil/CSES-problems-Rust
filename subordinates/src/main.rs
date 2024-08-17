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

fn take_int() -> usize {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn solve() {
    let n = take_int();
    let input = take_vector();
    let mut adj_list = vec![vec![]; n];
    
    for i in 0..n - 1 {
        adj_list[input[i] as usize - 1].push(i + 2);
    }
    
    let mut subordinates = vec![0; n];
    count_subordinates(&adj_list, 1, &mut subordinates);
    
    for count in subordinates {
        print!("{} ", count);
    }
    println!();
}

fn count_subordinates(adj_list: &Vec<Vec<usize>>, i: usize, subordinates: &mut Vec<isize>) -> isize {
    let mut count = 0;

    for &j in &adj_list[i - 1] {
        count += count_subordinates(adj_list, j, subordinates) + 1;
    }

    subordinates[i - 1] = count;
    count
}

fn main() {
    solve();
}
