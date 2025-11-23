use proconio::input;

fn main() {
    input! {
        n: usize,
        l: [u8; n],
    }
    let result = if l.iter().all(|&x| x == 0) {
        0
    } else {
        let l0 = l.iter().position(|&x| x == 1).unwrap();
        let l1 = l.iter().rposition(|&x| x == 1).unwrap();
        l1 - l0
    };
    println!("{result}");
}
