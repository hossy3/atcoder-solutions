use proconio::input;

fn swap_cols(
    i: usize,
    count: usize,
    a: &[Vec<usize>],
    b: &[Vec<usize>],
    rows: &Vec<usize>,
    cols: &Vec<usize>,
) -> usize {
    if i == 0 {
        for i in 0..rows.len() {
            for j in 0..cols.len() {
                if a[rows[i]][cols[j]] != b[i][j] {
                    return usize::MAX;
                }
            }
        }
        return count;
    }

    let mut result = usize::MAX;
    if i >= 5 {
        let mut cols0 = cols.clone();
        cols0.swap(0, 1);
        cols0.swap(1, 2);
        cols0.swap(2, 3);
        cols0.swap(3, 4);
        result = result.min(swap_cols(4, count + 4, a, b, rows, &cols0));

        let mut cols0 = cols.clone();
        cols0.swap(1, 2);
        cols0.swap(2, 3);
        cols0.swap(3, 4);
        result = result.min(swap_cols(4, count + 3, a, b, rows, &cols0));

        let mut cols0 = cols.clone();
        cols0.swap(0, 1);
        cols0.swap(2, 3);
        cols0.swap(3, 4);
        result = result.min(swap_cols(4, count + 3, a, b, rows, &cols0));

        let mut cols0 = cols.clone();
        cols0.swap(0, 1);
        cols0.swap(1, 2);
        cols0.swap(3, 4);
        result = result.min(swap_cols(4, count + 3, a, b, rows, &cols0));

        let mut cols0 = cols.clone();
        cols0.swap(0, 1);
        cols0.swap(3, 4);
        result = result.min(swap_cols(4, count + 2, a, b, rows, &cols0));

        let mut cols0 = cols.clone();
        cols0.swap(1, 2);
        cols0.swap(3, 4);
        result = result.min(swap_cols(4, count + 2, a, b, rows, &cols0));

        let mut cols0 = cols.clone();
        cols0.swap(2, 3);
        cols0.swap(3, 4);
        result = result.min(swap_cols(4, count + 2, a, b, rows, &cols0));

        let mut cols0 = cols.clone();
        cols0.swap(3, 4);
        result = result.min(swap_cols(4, count + 1, a, b, rows, &cols0));
    }

    if i >= 4 {
        let mut cols0 = cols.clone();
        cols0.swap(0, 1);
        cols0.swap(1, 2);
        cols0.swap(2, 3);
        result = result.min(swap_cols(3, count + 3, a, b, rows, &cols0));

        let mut cols0 = cols.clone();
        cols0.swap(1, 2);
        cols0.swap(2, 3);
        result = result.min(swap_cols(3, count + 2, a, b, rows, &cols0));

        let mut cols0 = cols.clone();
        cols0.swap(0, 1);
        cols0.swap(2, 3);
        result = result.min(swap_cols(3, count + 2, a, b, rows, &cols0));

        let mut cols0 = cols.clone();
        cols0.swap(2, 3);
        result = result.min(swap_cols(3, count + 1, a, b, rows, &cols0));
    }

    if i >= 3 {
        let mut cols0 = cols.clone();
        cols0.swap(0, 1);
        cols0.swap(1, 2);
        result = result.min(swap_cols(2, count + 2, a, b, rows, &cols0));

        let mut cols0 = cols.clone();
        cols0.swap(1, 2);
        result = result.min(swap_cols(2, count + 1, a, b, rows, &cols0));
    }

    if i >= 2 {
        let mut cols0 = cols.clone();
        cols0.swap(0, 1);
        result = result.min(swap_cols(1, count + 1, a, b, rows, &cols0));
    }

    result = result.min(swap_cols(0, count, a, b, rows, cols));
    result
}

fn swap_rows(
    i: usize,
    count: usize,
    a: &[Vec<usize>],
    b: &[Vec<usize>],
    rows: &Vec<usize>,
) -> usize {
    if i == 0 {
        let w = a[0].len();
        let mut cols = vec![0; w];
        for i in 0..w {
            cols[i] = i;
        }
        return swap_cols(a[0].len(), count, a, b, rows, &cols);
    }
    let mut result = usize::MAX;
    if i >= 5 {
        let mut rows0 = rows.clone();
        rows0.swap(0, 1);
        rows0.swap(1, 2);
        rows0.swap(2, 3);
        rows0.swap(3, 4);
        result = result.min(swap_rows(4, count + 4, a, b, &rows0));

        let mut rows0 = rows.clone();
        rows0.swap(1, 2);
        rows0.swap(2, 3);
        rows0.swap(3, 4);
        result = result.min(swap_rows(4, count + 3, a, b, &rows0));

        let mut rows0 = rows.clone();
        rows0.swap(0, 1);
        rows0.swap(2, 3);
        rows0.swap(3, 4);
        result = result.min(swap_rows(4, count + 3, a, b, &rows0));

        let mut rows0 = rows.clone();
        rows0.swap(0, 1);
        rows0.swap(1, 2);
        rows0.swap(3, 4);
        result = result.min(swap_rows(4, count + 3, a, b, &rows0));

        let mut rows0 = rows.clone();
        rows0.swap(0, 1);
        rows0.swap(3, 4);
        result = result.min(swap_rows(4, count + 2, a, b, &rows0));

        let mut rows0 = rows.clone();
        rows0.swap(1, 2);
        rows0.swap(3, 4);
        result = result.min(swap_rows(4, count + 2, a, b, &rows0));

        let mut rows0 = rows.clone();
        rows0.swap(2, 3);
        rows0.swap(3, 4);
        result = result.min(swap_rows(4, count + 2, a, b, &rows0));

        let mut rows0 = rows.clone();
        rows0.swap(3, 4);
        result = result.min(swap_rows(4, count + 1, a, b, &rows0));
    }

    if i >= 4 {
        let mut rows0 = rows.clone();
        rows0.swap(0, 1);
        rows0.swap(1, 2);
        rows0.swap(2, 3);
        result = result.min(swap_rows(3, count + 3, a, b, &rows0));

        let mut rows0 = rows.clone();
        rows0.swap(1, 2);
        rows0.swap(2, 3);
        result = result.min(swap_rows(3, count + 2, a, b, &rows0));

        let mut rows0 = rows.clone();
        rows0.swap(0, 1);
        rows0.swap(2, 3);
        result = result.min(swap_rows(3, count + 2, a, b, &rows0));

        let mut rows0 = rows.clone();
        rows0.swap(2, 3);
        result = result.min(swap_rows(3, count + 1, a, b, &rows0));
    }

    if i >= 3 {
        let mut rows0 = rows.clone();
        rows0.swap(0, 1);
        rows0.swap(1, 2);
        result = result.min(swap_rows(2, count + 2, a, b, &rows0));

        let mut rows0 = rows.clone();
        rows0.swap(1, 2);
        result = result.min(swap_rows(2, count + 1, a, b, &rows0));
    }

    if i >= 2 {
        let mut rows0 = rows.clone();
        rows0.swap(0, 1);
        result = result.min(swap_rows(1, count + 1, a, b, &rows0));
    }

    result = result.min(swap_rows(0, count, a, b, rows));
    result
}

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h],
        b: [[usize; w]; h],
    }

    let mut rows = vec![0; h];
    for i in 0..h {
        rows[i] = i;
    }

    let result = swap_rows(a.len(), 0, &a, &b, &rows);
    if result == usize::MAX {
        println!("-1");
    } else {
        println!("{result}");
    }
}
