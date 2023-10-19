use proconio::input;
use superslice::Ext;

const INF: i32 = 20000000;

fn main() {
    input! {
        n: usize,
        k: i32,
        a: [i32; n],
        mut b: [i32; n],
    }

    let mut min_value = INF;

    b.sort();

    // bの末尾にINFを追加することで、
    // lower_boundの結果でbにインデックスアクセスする場合の
    // 境界チェックが省略可能になる。
    // * bにINF以上の値が含まれないように、INFの値を設定する必要がある。
    b.push(INF);

    for &ai in &a {
        // 本に記載されたコードに準拠して変数名をiterにしているが、
        // iterの型はusizeのため注意する。
        let iter = b.lower_bound(&(k - ai));
        // bの末尾にINFを追加しているため、境界チェックを省略してもエラーにならない。
        let val = b[iter];

        if ai + val < min_value {
            min_value = ai + val
        }
    }

    println!("{}", min_value);
}
