use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut result = 1001;
    let mut count = 0;
    for i in 2..=1000 {
        let c = a.iter().filter(|&&x| x % i == 0).count();
        if count <= c {
            count = c;
            result = i;
        }
    }
    println!("{result}");
}
