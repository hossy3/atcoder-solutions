use proconio::input;

fn main() {
    input! {
        n: usize,
        xa: isize,
        ya: isize,
        r: isize,
        xyp: [(isize, isize, usize); n],
    }

    let mut result = 0;
    for &(x, y, p) in &xyp {
        if (x - xa).pow(2) + (y - ya).pow(2) <= r.pow(2) {
            result += p;
        }
    }
    println!("{result}");
}
