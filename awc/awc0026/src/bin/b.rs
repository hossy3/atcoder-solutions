use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut takahashi = 0;
    let mut aoki = 0;
    for a in a {
        if takahashi + a <= k {
            takahashi += a;
        } else {
            aoki += a;
        }
    }
    let result = if takahashi > aoki {
        "Takahashi"
    } else if takahashi < aoki {
        "Aoki"
    } else {
        "Draw"
    };
    println!("{result}");
}
