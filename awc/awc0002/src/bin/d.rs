use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut c: [usize; n],
        mut r: [usize; m],
    }

    c.sort_unstable();
    r.sort_unstable();

    let mut count = 0;
    while let Some(r) = r.pop() {
        while let Some(c) = c.pop() {
            if c <= r {
                count += 1;
                break;
            }
        }
        if c.len() == 0 {
            break;
        }
    }
    println!("{count}");
}
