use proconio::input;

fn main() {
    input! {
        n: usize,
        mut w: usize,
        c: [usize; n],
    }

    let mut result = 0;
    for c in c {
        if w >= c {
            w -= c;
            result += 1;
        } else {
            break;
        }
    }
    println!("{result}");
}
