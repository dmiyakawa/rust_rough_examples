fn main() {
    // 1つ目の`x`を束縛する。
    let x: i32 = 1;
    println!("{}", x); // => 1
    // 2つ目の`x`を束縛する。これは先のxとは別物。
    let x: &str = "abc";
    // 以後、`x`は`"abc"`を指すようになる。
    println!("{}", x); // => abc
}
