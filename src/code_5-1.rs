use proconio::input;
use std::cmp::min;

const INF: i64 = 1 << 60;

fn main() {
    input! {
        n: usize,
        h: [i64; n],
    }

    let mut dp = vec![INF; n];

    dp[0] = 0;

    for i in 1..n {
        dp[i] = if i == 1 {
            (h[i] - h[i - 1]).abs()
        } else {
            min(
                dp[i - 1] + (h[i] - h[i - 1]).abs(),
                dp[i - 2] + (h[i] - h[i - 2]).abs(),
            )
        }
    }

    println!("{}", dp[n - 1]);
}
