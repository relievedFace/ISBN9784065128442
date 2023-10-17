fn gcd(m: i32, n: i32) -> i32 {
    if n == 0 {
        m
    } else {
        gcd(n, m % n)
    }
}

fn main() {
    println!("{}", gcd(51, 15));
    println!("{}", gcd(15, 51));
}
