use proconio::input;

const INF: i32 = 20000000;

fn main() {
    input! {
        n: usize,
        k: i32,
        a: [i32; n],
        b: [i32; n],
    }

    let mut min_value = INF;

    for i in 0..n {
        for j in 0..n {
            if a[i] + b[j] < k {
                continue;
            }

            if a[i] + b[j] < min_value {
                min_value = a[i] + b[j];
            }
        }
    }

    println!("{}", min_value);
}
