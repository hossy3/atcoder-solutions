use proconio::input;

fn f(cl: &[(char, usize)]) -> String {
    let mut result = vec![];
    for &(c, l) in cl {
        if result.len() + l > 100 {
            return String::from("Too Long");
        }
        let mut v = vec![c; l];
        result.append(&mut v);
    }

    result.iter().collect::<String>()
}

fn main() {
    input! {
        n: usize,
        cl: [(char, usize); n],
    }
    let result = f(&cl);
    println!("{}", result);
}
