use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: usize,
        mut b: usize,
        mut c: usize,
        s: [String; n],
    }

    let mut results = vec![];
    for (i, s0) in s.iter().enumerate() {
        match s0.as_str() {
            "AB" => {
                if a == 0 && b == 0 {
                    println!("No");
                    return;
                }
                if a < b || (a == b && i < n - 1 && s[i + 1].as_str() == "AC") {
                    results.push('A');
                    a += 1;
                    b -= 1;
                } else {
                    results.push('B');
                    a -= 1;
                    b += 1;
                }
            }
            "AC" => {
                if a == 0 && c == 0 {
                    println!("No");
                    return;
                }
                if a < c || (a == c && i < n - 1 && s[i + 1].as_str() == "AB") {
                    results.push('A');
                    a += 1;
                    c -= 1;
                } else {
                    results.push('C');
                    a -= 1;
                    c += 1;
                }
            }
            "BC" => {
                if b == 0 && c == 0 {
                    println!("No");
                    return;
                }
                if b < c || (b == c && i < n - 1 && s[i + 1].as_str() == "AB") {
                    results.push('B');
                    b += 1;
                    c -= 1;
                } else {
                    results.push('C');
                    b -= 1;
                    c += 1;
                }
            }
            _ => unreachable!(),
        };
    }

    println!("Yes");
    for result in results {
        println!("{result}");
    }
}
