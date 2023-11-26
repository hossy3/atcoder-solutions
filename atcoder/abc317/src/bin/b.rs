use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    a.sort();
    for x in a.windows(2) {
        if x[1] - x[0] == 2 {
            println!("{}", x[0] + 1);
            return;
        }
    }
}
