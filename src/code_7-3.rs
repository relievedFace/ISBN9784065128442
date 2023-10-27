use proconio::input;

fn main() {
    input! {
        n: usize,
        mut ab: [(i64, i64); n],
    }

    let mut sum = 0;

    for (ai, bi) in ab.iter_mut().rev() {
        *ai += sum;
        let amari = *ai % *bi;
        let mut d = 0;
        if amari != 0 {
            d = *bi - amari;
        }
        sum += d;
    }

    println!("{}", sum);
}
