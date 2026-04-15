use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        d: [usize; n],
    }

    let mut result = -1;
    for (i, &d0) in d.iter().enumerate() {
        if l >= d0 && (result == -1 || d0 > d[(result - 1) as usize]) {
            result = (i + 1) as isize;
        }
    }
    println!("{result}");
}
