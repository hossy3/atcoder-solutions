use proconio::input;

fn main() {
    input! {
        n: usize,
        mut w: [usize; n],
        mut c: [usize; n],
    }

    w.sort_unstable();
    c.sort_unstable();
    c.reverse();

    let mut result = 0usize;
    for w in w {
        while let Some(c) = c.pop() {
            if c >= w {
                result += 1;
                break;
            }
        }
    }
    println!("{result}");
}
