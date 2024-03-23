use proconio::input;

fn main() {
    let mut a = vec![];
    loop {
        input! {
            x: usize,
        }
        a.push(x);
        if x == 0 {
            break;
        }
    }
    for x in a.iter().rev() {
        println!("{x}");
    }
}
