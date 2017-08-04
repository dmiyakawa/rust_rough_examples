// `trait トレイト名 {..}`でトレイトを定義
trait DuckLike {
    // トレイトを実装する型が実装すべきメソッドを定義
    fn quack(&self);

    // デフォルトメソッドを定義することもできる
    fn walk(&self) {
      println!("walking");
    }
}

// トレイトを実装するためだけのデータ型にはUnit構造体が便利
struct Duck;

// `impl トレイト名 for 型名 {..}`で定義可能
impl DuckLike for Duck {
    // トレイトで実装されていないメソッドを実装側で定義する
    fn quack(&self) {
        println!("quack");
    }
}

struct Tsuchinoko;

// 別の型にも実装できます。
impl DuckLike for Tsuchinoko {
    fn quack(&self) {
        // どうやらこのツチノコの正体はネコだったようです
        println!("mew");
    }

    // デフォルトメソッドを上書きすることもできる
    fn walk(&self) {
        println!("wriggling");
    }
}

// 既存の型にトレイトを実装することもできる
// モンキーパッチをしているような気分
impl DuckLike for i64 {
    fn quack(&self) {
        for _ in 0..*self {
            println!("quack");
        }
    }
}

fn main() {
    let duck = Duck;
    let tsuchinoko = Tsuchinoko;
    let i = 3;
    duck.quack(); // => quack
    tsuchinoko.quack(); // => mew
    i.quack(); // => quack; quack; quack
}
