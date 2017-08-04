/// 階乗を計算する。
fn factorial(n: usize) -> usize {
    // ifは式なので関数の最後に置くと値を返せる。
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn main() {
    println!("{}", factorial(10));
}
