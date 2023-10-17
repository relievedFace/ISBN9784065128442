fn fibo(n: usize, memo: &mut Vec<i64>) -> i64 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else if memo[n] != -1 {
        memo[n]
    } else {
        memo[n] = fibo(n - 1, memo) + fibo(n - 2, memo);
        memo[n]
    }
}
fn main() {
    let mut memo = vec![-1; 50];

    fibo(49, &mut memo);

    for n in 2..50 {
        println!("{} 項目: {}", n, memo[n]);
    }
}
