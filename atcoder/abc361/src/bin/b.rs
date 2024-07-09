use proconio::input;

fn is_intersect(
    (x0, y0, z0, x1, y1, z1): (usize, usize, usize, usize, usize, usize),
    (x2, y2, z2, x3, y3, z3): (usize, usize, usize, usize, usize, usize),
) -> bool {
    x0.max(x2) < x1.min(x3) && y0.max(y2) < y1.min(y3) && z0.max(z2) < z1.min(z3)
}

fn main() {
    input! {
        abcdef: (usize, usize, usize, usize, usize, usize),
        ghijkl: (usize, usize, usize, usize, usize, usize),
    }
    let yes = is_intersect(abcdef, ghijkl);
    println!("{}", if yes { "Yes" } else { "No" });
}
