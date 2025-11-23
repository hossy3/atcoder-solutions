use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut stack = vec![];
    for &x in &a {
        let mut x = x;
        while let Some(&y) = stack.last() {
            if x != y {
                break;
            }
            stack.pop();
            x += 1;
        }
        stack.push(x);
    }

    let result = stack.len();
    println!("{result}");
}
