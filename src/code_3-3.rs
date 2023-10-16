use proconio::input;

const INF: i32 = 20000000;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }

    let mut min_value = INF;

    for i in 0..n {
        if a[i] < min_value {
            min_value = a[i];
        }
    }

    println!("{}", min_value);
}
