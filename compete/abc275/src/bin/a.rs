use proconio::input;

fn main() {
    input! {
        a: [usize],
    }
    let mut index = 0;
    let mut max = a[0];
    for i in 1..a.len() {
        if max < a[i] {
            max = a[i];
            index = i;
        }
    }
    println!("{}", index + 1);
}
