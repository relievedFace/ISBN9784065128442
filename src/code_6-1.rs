use std::cmp::Ordering;

// 本に記載されているコードは存在しない場合に-1を返しているが、
// RustではVecのインデックスにusize(0以上の値しか取らない)しか使用できないため、
// 返り値の方をOption<usize>に変更。
// 存在する場合、Some(目的の値keyの添え字)を返す。
// 存在しない場合、Noneを返す。
fn binary_search(key: i32, a: &Vec<i32>) -> Option<usize> {
    let mut left = 0;
    let mut right = a.len() - 1;

    // 本に記載されたコードの継続条件は`right >= left`だが
    // `Ordering::Greater => right = mid - 1,`がオーバーフローするため
    // `left < right`に変更した。
    // 本に記載されたコードはleft、rightの型がintのためオーバーフローしてない。
    while left < right {
        let mid = left + (right - left) / 2;

        // 本に記載されたコードはifで書かれているが、
        // matchに書き直した。
        match a[mid].cmp(&key) {
            Ordering::Less => left = mid + 1,
            Ordering::Equal => return Some(mid),
            Ordering::Greater => right = mid - 1,
        }
    }

    None
}

fn main() {
    let a = vec![3, 5, 8, 10, 14, 17, 21, 39];

    println!("{:?}", binary_search(10, &a)); // Some(3)
    println!("{:?}", binary_search(3, &a)); // Some(0)
    println!("{:?}", binary_search(39, &a)); // Some(7)
    println!("{:?}", binary_search(-100, &a)); // None
    println!("{:?}", binary_search(9, &a)); // None
    println!("{:?}", binary_search(100, &a)); // None
}
