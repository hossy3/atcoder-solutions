use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        d: [isize; n],
        mut s: [isize; m],
    }

    s.sort_unstable();

    let mut result = 0isize;
    for &d in &d {
        result += match s.binary_search(&d) {
            Ok(_) => 0,
            Err(i) => {
                if i == 0 {
                    s[0] - d
                } else if i == m {
                    d - s[m - 1]
                } else {
                    (d - s[i - 1]).min(s[i] - d)
                }
            }
        };
    }
    println!("{result}");
}
