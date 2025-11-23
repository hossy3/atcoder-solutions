use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        mut b: [usize; n - 1],
    }
    a.sort();
    b.sort();
    let mut result = 0;
    for &x in a.iter().rev() {
        if let Some(&y) = b.last() {
            if y >= x {
                b.pop();
            } else {
                result += x;
            }
        } else {
            result += x;
        }
    }

    if b.is_empty() {
        println!("{result}");
    } else {
        println!("-1");
    }
}
