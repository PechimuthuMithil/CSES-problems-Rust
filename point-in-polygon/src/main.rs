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

fn side(input: Vec<isize>) -> isize {
    let (x1, y1, x2, y2, x3, y3) = (input[0], input[1], input[2], input[3], input[4], input[5]);

    let cross = (x2 - x1) * (y3 - y1) - (y2 - y1) * (x3 - x1);

    if cross > 0 {
        1
    } else if cross < 0 {
        -1
    } else {
        0
    }
}

fn solve() {
    let inp = take_vector();
    let n = inp[0] as usize;
    let m = inp[1] as usize;

    let mut poly_points = vec![(0, 0); n+1];
    let mut test_points = vec![(0, 0); m];

    for i in 0..n {
        let input = take_vector();
        poly_points[i as usize] = (input[0], input[1]);
    }
    poly_points[n] = poly_points[0];
    for i in 0..m {
        let input = take_vector();
        test_points[i as usize] = (input[0], input[1]);
    }

    for i in 0..m {
        let mut ans = side(vec![poly_points[0 as usize].0, poly_points[0 as usize].1, poly_points[(1) as usize].0, poly_points[(1) as usize].1, test_points[i as usize].0, test_points[i as usize].1]);
        for j in 1..n {
            if ans == 0{
                println!("BOUNDARY");
                break;
            }
            ans = ans*side(vec![poly_points[j as usize].0, poly_points[j as usize].1, poly_points[(j+1) as usize].0, poly_points[(j+1) as usize].1, test_points[i as usize].0, test_points[i as usize].1]);
            if ans == -1{
                println!("OUTSIDE");
                break;
            }
        }
        if ans == 1 {
            println!("INSIDE");
        } 
    }

}

fn main() {
    solve();
}
