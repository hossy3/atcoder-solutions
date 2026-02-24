use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [usize; n],
    }

    let mut result = 0;
    let mut max = 0;
    for h in h {
        if h > max {
            result += 1;
            max = h;
        }
    }
    println!("{result}");
}
