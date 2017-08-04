fn main() {
    // 型を混合したタプルが作れる。
    let a: (isize, f64, &str) = (1, 1.0, "abc");
    // `タプル.インデックス`でタプルの要素にアクセスできる。
    println!("{}, {}, {}", a.0, a.1, a.2); // => 1, 1, abc
}
