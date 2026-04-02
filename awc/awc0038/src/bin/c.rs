use proconio::input;

fn main() {
    input! {
        n: usize,
        mut l: usize,
        k: usize,
        mut a: [usize; n],
    }

    a.sort_unstable();
    for (i, &x) in a[..k].iter().enumerate() {
        if l < x {
            println!("{i}");
            return;
        }
        l -= x;
    }
    let result = if k < n && l >= a[k] { k + 1 } else { k };
    println!("{result}");
}
