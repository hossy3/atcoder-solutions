use proconio::input;

fn main() {
    input! {
        n: usize,
        a_s: [(usize, char); n]
    }
    let mut l = 0_usize;
    let mut r = 0_usize;
    let mut result = 0;
    for (a, s) in a_s {
        match s {
            'L' => {
                if l != 0 {
                    result += l.abs_diff(a);
                }
                l = a;
            }
            'R' => {
                if r != 0 {
                    result += r.abs_diff(a);
                }
                r = a;
            }
            _ => unreachable!(),
        }
    }
    println!("{result}");
}
