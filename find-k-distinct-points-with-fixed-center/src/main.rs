use std::io::stdin;

fn take_int() -> usize {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn take_vector() -> Vec<isize> {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn solve() {
    // Take input of the form x y z
    let input = take_vector();
    let x = input[0];
    let y = input[1];
    let k = input[2] as usize;

    let mut points: Vec<Vec<isize>> = vec![vec![0; 2]; k];

    for i in 0..k-1 {
        points[i] = vec![x - (i as isize + 1), y - (i as isize + 1)];
    }

    let term = (k * (k - 1)) as isize / 2;
    points[k-1] = vec![x + term, y + term];

    // let mut xc: isize = 0;
    // let mut yc: isize = 0;

    // for i in 0..k {
    //     xc += points[i][0];
    //     yc += points[i][1];
    // }

    // println!("{} {}", xc / k as isize, yc / k as isize);
    for i in 0..k {
        println!("{} {}", points[i][0], points[i][1]);
    }
}

pub fn main() {
    let t = take_int();
    for _ in 0..t {
        solve();
    }
}
