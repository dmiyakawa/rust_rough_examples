// struct 名前; （Unit構造体の構文）
struct Dummy;

// struct 名前(型, ..); （タプル構造体の構文）
struct Point(f64, f64);

// struct 名前 {フィールド: 型, ..} （通常の構造体の構文）
struct Color {
    r: u8,
    g: u8,
    // 最後のフィールドの末尾にもカンマを付けられる
    b: u8,
}

fn main() {
    // Unit構造体は名前でそのまま初期化
    let dummy = Dummy;

    // タプル構造体は関数のように初期化
    // 実際、関数として扱うこともできる
    let point = Point(0.0, 0.1);

    // タプル構造体のフィールドへのアクセス
    let x = point.0;
    let y = point.1;

    println!("Point({}, {})", x, y);

    // 普通の構造体の初期化
    let black = Color { r: 0, g: 0, b: 0};

    // 普通の構造体のフィールドへのアクセス
    let r = black.r;
    let g = black.g;
    let b = black.b;

    println!("black = Color({}, {}, {})", r, g, b);
}
