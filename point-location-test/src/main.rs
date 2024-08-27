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

fn solve(input: Vec<isize>) {
    let (x1, y1, x2, y2, x3, y3) = (input[0], input[1], input[2], input[3], input[4], input[5]);

    let cross = (x2 - x1) * (y3 - y1) - (y2 - y1) * (x3 - x1);

    if cross > 0 {
        println!("LEFT");
    } else if cross < 0 {
        println!("RIGHT");
    } else {
        println!("TOUCH");
    }
}

fn main() {
    let t = take_int();
    for _ in 0..t {
        let input = take_vector();
        solve(input);
    }
}

// use std::io::stdin;

// fn take_vector() -> Vec<isize> {
//     let mut input = String::new();
//     stdin().read_line(&mut input).unwrap();
//     input
//         .trim()
//         .split_whitespace()
//         .map(|x| x.parse().unwrap())
//         .collect()
// }

// fn take_string() -> String {
//     let mut input = String::new();
//     stdin().read_line(&mut input).unwrap();
//     input
// }

// fn take_int() -> usize {
//     let mut input = String::new();
//     stdin().read_line(&mut input).unwrap();
//     input.trim().parse().unwrap()
// }

// fn solve() {
//     let input = take_vector();
//     let mut p1, p2, p3 = vec![input[0], input[1]], vec![input[2], input[3]], vec![input[4], input[5]];
    
//     if (p2[0] - p1[0]) != 0 {
//         let mut tan = (p2[1] - p1[1])/ (p2[0] - p1[0]);
//         if tan >= 0 {
//             let sin = tan / (1 + tan * tan).sqrt(); 
//             let cos = 1 / (1 + tan * tan).sqrt();
//             p3[0] = (p3[0]-p1[0])*sin + (p3[1]-p1[1])*cos;
//             p3[1] = -(p3[0]-p1[0])*cos + (p3[1]-p1[1])*sin;
//         } else if tan < 0 {
//             let sin = tan / (1 + tan * tan).sqrt();
//             let cos = -1 / (1 + tan * tan).sqrt();
//             p3[0] = (p3[0]-p1[0])*sin + (p3[1]-p1[1])*cos;
//             p3[1] = -(p3[0]-p1[0])*cos + (p3[1]-p1[1])*sin;
//         }
//     }
 
//     if p3[0] == 0 {
//         println!("TOUCH");
//     } else if p3[0] > 0 {
//         println!("RIGHT");
//     } else {
//         println!("LEFT");
//     }
// }

// fn main() {
//     let t = take_int();
//     for _ in 0..t {
//         solve();
//     }
// }