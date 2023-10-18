use proconio::input;

fn chmin<T>(a: &mut T, b: &T)
where
    T: Clone + Copy + Ord + PartialOrd,
{
    if *a > *b {
        *a = *b;
    }
}

const INF: i64 = 1 << 60;

fn main() {
    input! {
        n: usize,
        h: [i64; n],
    }

    let mut dp = vec![INF; n];

    dp[0] = 0;

    for i in 1..n {
        let prev1 = dp[i - 1];

        chmin(&mut dp[i], &(prev1 + (h[i] - h[i - 1]).abs()));

        if i > 1 {
            let prev2 = dp[i - 2];

            chmin(&mut dp[i], &(prev2 + (h[i] - h[i - 2]).abs()));
        }
    }

    println!("{}", dp[n - 1]);
}
