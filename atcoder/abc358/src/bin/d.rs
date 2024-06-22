use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n],
        mut b: [usize; m],
    }

    // 降順に並べる
    a.sort();
    a.reverse();
    b.sort();
    b.reverse();

    let mut result = 0;
    'outer: while let Some(b) = b.pop() {
        while let Some(a) = a.pop() {
            if a >= b {
                result += a;
                continue 'outer;
            }
        }
        println!("-1");
        return;
    }
    println!("{result}");
}
