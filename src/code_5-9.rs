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
        c: [[i64; n+1]; n+1],
    }

    let mut dp = vec![INF; n + 1];

    dp[0] = 0;

    for i in 0..=n {
        for j in 0..i {
            let prev = dp[j];

            chmin(&mut dp[i], &(prev + c[j][i]))
        }
    }

    println!("{}", dp[n]);
}
