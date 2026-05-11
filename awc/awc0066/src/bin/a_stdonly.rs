fn f(stdin: String) -> String {
    let mut stdin = stdin.split_whitespace();

    let n = stdin.next().unwrap().parse::<usize>().unwrap();
    let q = stdin.next().unwrap().parse::<usize>().unwrap();
    let mut a: Vec<usize> = stdin
        .by_ref()
        .take(n)
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    let b: Vec<usize> = stdin
        .by_ref()
        .take(q)
        .map(|s| s.parse::<usize>().unwrap() - 1)
        .collect();

    for i in b {
        if i < n - 1 {
            a[i + 1] += a[i];
        }
        a[i] = 0;
    }

    let result = a
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    result
}

fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let result = f(stdin);
    println!("{result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        assert_eq!(
            f("
            5 3
            3 1 4 1 5
            1
            3
            5"
            .to_string()),
            "0 4 0 5 0"
        )
    }
}
