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

fn orientation(p1: (isize, isize), p2: (isize, isize), p3: (isize, isize)) -> isize {
    let val = (p2.1 - p1.1) * (p3.0 - p2.0) - (p2.0 - p1.0) * (p3.1 - p2.1);
    if val == 0 {
        return 0; // collinear
    }
    if val > 0 {
        return 1; // clockwise
    }
    return 2; // counterclockwise
}

fn on_segment(p1: (isize, isize), p2: (isize, isize), p3: (isize, isize)) -> bool {
    if p2.0 <= p1.0.max(p3.0) && p2.0 >= p1.0.min(p3.0) &&
       p2.1 <= p1.1.max(p3.1) && p2.1 >= p1.1.min(p3.1) {
        return true;
    }
    return false;
}

fn do_intersect(p1: (isize, isize), p2: (isize, isize), p3: (isize, isize), p4: (isize, isize)) -> bool {
    let o1 = orientation(p1, p2, p3);
    let o2 = orientation(p1, p2, p4);
    let o3 = orientation(p3, p4, p1);
    let o4 = orientation(p3, p4, p2);
    
    // General case
    if o1 != o2 && o3 != o4 {
        return true;
    }

    // Special cases
    if o1 == 0 && on_segment(p1, p3, p2) { return true; }
    if o2 == 0 && on_segment(p1, p4, p2) { return true; }
    if o3 == 0 && on_segment(p3, p1, p4) { return true; }
    if o4 == 0 && on_segment(p3, p2, p4) { return true; }

    return false;
}

fn solve() {
    let input = take_vector();
    let p1 = (input[0], input[1]);
    let p2 = (input[2], input[3]);
    let p3 = (input[4], input[5]);
    let p4 = (input[6], input[7]);

    if do_intersect(p1, p2, p3, p4) {
        println!("YES");
    } else {
        println!("NO");
    }
}

fn main() {
    let t = take_vector()[0];
    for _ in 0..t {
        solve();
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

// fn take_int() -> usize {
//     let mut input = String::new();
//     stdin().read_line(&mut input).unwrap();
//     input.trim().parse().unwrap()
// }

// fn solve() {
//     let input = take_vector();
//     let (p1, p2, p3, p4) = (
//         vec![input[0], input[1]],
//         vec![input[2], input[3]],
//         vec![input[4], input[5]],
//         vec![input[6], input[7]],
//     );
    
//     // Initialize m1 and m2 as large values for vertical lines
//     let m1: f64;
//     let m2: f64;
    
//     if p2[0] == p1[0] {
//         m1 = f64::INFINITY; // Slope is infinite for vertical line
//     } else {
//         m1 = (p2[1] - p1[1]) as f64 / (p2[0] - p1[0]) as f64;
//     }

//     if p4[0] == p3[0] {
//         m2 = f64::INFINITY; // Slope is infinite for vertical line
//     } else {
//         m2 = (p4[1] - p3[1]) as f64 / (p4[0] - p3[0]) as f64;
//     }
    
//     let c1 = p1[1] as f64 - m1 * p1[0] as f64;
//     let c2 = p3[1] as f64 - m2 * p3[0] as f64;

//     if (m1 - m2).abs() < 1e-9 { // Compare slopes with a tolerance
//         if (c1 - c2).abs() < 1e-9 { // Compare intercepts with a tolerance
//             println!("YES");
//         } else {
//             println!("NO");
//         }
//     } else {
//         println!("YES");
//     }
// }

// fn main() {
//     let t = take_int();
//     for _ in 0..t {
//         solve();
//     }
// }
// use std::io::stdin;

// // Function to read a line of input and return as a vector of integers
// fn take_vector() -> Vec<f64> {
//     let mut input = String::new();
//     stdin().read_line(&mut input).unwrap();
//     input
//         .trim()
//         .split_whitespace()
//         .map(|x| x.parse().unwrap())
//         .collect()
// }

// // Function to determine if two line segments intersect
// fn solve() {
//     let input = take_vector();
//     let (x1, y1, x2, y2, x3, y3, x4, y4) = (input[0], input[1], input[2], input[3], input[4], input[5], input[6], input[7]);

//     // Calculate the determinants
//     let a1 = y2 - y1;
//     let b1 = x1 - x2;
//     let c1 = a1 * x1 + b1 * y1;

//     let a2 = y4 - y3;
//     let b2 = x3 - x4;
//     let c2 = a2 * x3 + b2 * y3;

//     let determinant = a1 * b2 - a2 * b1;
//     use std::io::stdin;

//     fn take_vector() -> Vec<isize> {
//         let mut input = String::new();
//         stdin().read_line(&mut input).unwrap();
//         input
//             .trim()
//             .split_whitespace()
//             .map(|x| x.parse().unwrap())
//             .collect()
//     }
    
//     fn orientation(p1: (isize, isize), p2: (isize, isize), p3: (isize, isize)) -> isize {
//         let val = (p2.1 - p1.1) * (p3.0 - p2.0) - (p2.0 - p1.0) * (p3.1 - p2.1);
//         if val == 0 {
//             return 0; // collinear
//         }
//         if val > 0 {
//             return 1; // clockwise
//         }
//         return 2; // counterclockwise
//     }
    
//     fn on_segment(p1: (isize, isize), p2: (isize, isize), p3: (isize, isize)) -> bool {
//         if p2.0 <= p1.0.max(p3.0) && p2.0 >= p1.0.min(p3.0) &&
//            p2.1 <= p1.1.max(p3.1) && p2.1 >= p1.1.min(p3.1) {
//             return true;
//         }
//         return false;
//     }
    
//     fn do_intersect(p1: (isize, isize), p2: (isize, isize), p3: (isize, isize), p4: (isize, isize)) -> bool {
//         let o1 = orientation(p1, p2, p3);
//         let o2 = orientation(p1, p2, p4);
//         let o3 = orientation(p3, p4, p1);
//         let o4 = orientation(p3, p4, p2);
        
//         // General case
//         if o1 != o2 && o3 != o4 {
//             return true;
//         }
    
//         // Special cases
//         if o1 == 0 && on_segment(p1, p3, p2) { return true; }
//         if o2 == 0 && on_segment(p1, p4, p2) { return true; }
//         if o3 == 0 && on_segment(p3, p1, p4) { return true; }
//         if o4 == 0 && on_segment(p3, p2, p4) { return true; }
    
//         return false;
//     }
    
//     fn solve() {
//         let input = take_vector();
//         let p1 = (input[0], input[1]);
//         let p2 = (input[2], input[3]);
//         let p3 = (input[4], input[5]);
//         let p4 = (input[6], input[7]);
    
//         if do_intersect(p1, p2, p3, p4) {
//             println!("YES");
//         } else {
//             println!("NO");
//         }
//     }
    
//     fn main() {
//         let t = take_vector()[0];
//         for _ in 0..t {
//             solve();
//         }
//     }
//     Intersection point (x5, y5)
//         let x5 = (b2 * c1 - b1 * c2) / determinant;
//         let y5 = (a1 * c2 - a2 * c1) / determinant;

//         // Check if (x5, y5) lies on both segments
//         if x5 >= x1.min(x2) && x5 <= x1.max(x2) && y5 >= y1.min(y2) && y5 <= y1.max(y2) &&
//            x5 >= x3.min(x4) && x5 <= x3.max(x4) && y5 >= y3.min(y4) && y5 <= y3.max(y4) {
//             println!("YES");
//         } else {
//             println!("NO");
//         }
//     }
// }

// fn main() {
//     let t = take_vector()[0] as usize;
//     for _ in 0..t {
//         solve();
//     }
// }
