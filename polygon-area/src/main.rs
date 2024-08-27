use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut vertices: Vec<(isize, isize)> = Vec::with_capacity(n);

    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut coords = line.split_whitespace();
        let x: isize = coords.next().unwrap().parse().unwrap();
        let y: isize = coords.next().unwrap().parse().unwrap();
        vertices.push((x, y));
    }

    let mut area: isize = 0;

    for i in 0..n {
        let (x1, y1) = vertices[i];
        let (x2, y2) = if i == n - 1 {
            vertices[0] // Wrap around to the first vertex
        } else {
            vertices[i + 1]
        };

        area += x1 * y2 - y1 * x2;
    }

    println!("{}", area.abs()); // Print twice the area
}
