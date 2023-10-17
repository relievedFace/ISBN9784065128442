fn func(n: i32) -> i32 {
    println!("func({})を呼び出しました", n);
    if n == 0 {
        0
    } else {
        let result = n + func(n - 1);
        println!("{}までの和 = {}", n, result);
        result
    }
}

fn main() {
    func(5);
}
