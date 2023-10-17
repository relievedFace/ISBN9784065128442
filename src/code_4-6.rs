fn fibo(n: i32) -> i32 {
    println!("fibo({})を呼び出しました", n);

    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        let result = fibo(n - 1) + fibo(n - 2);

        println!("{} 項目 = {}", n, result);

        result
    }
}

fn main() {
    fibo(6);
}
