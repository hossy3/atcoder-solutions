use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        c: usize,
        mut a: [usize; n],
        mut b: [usize; m],
    }

    a.sort();
    b.sort();

    let mut result = 0usize;
    for &a in a.iter().rev() {
        while let Some(b) = b.pop() {
            if a >= b {
                result += c;
                break;
            }
        }
    }
    println!("{result}");
}
