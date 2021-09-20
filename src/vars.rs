// varsモジュール

// サブモジュールであるsub_aをmain.rsから呼び出せるようにするにはpubを付ける
pub mod sub_a;
mod sub_b;

// 定数
const MAX_POINTS: u32 = 100_000;

// main.rsから呼び出すにはpubを付けることで公開可能
pub fn run() {
    println!("Here is vars module");
    sub_a::func_a();
    sub_b::func_b();

    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // 型推論
    // Rustは未使用の変数があると警告を出すが、_から始まる変数は未使用であることを明示的に宣言するので警告を出さない
    let _i1 = 3;
    let _f1 = 0.1;

    // const
    println!("Memory address of MAX_POINTS is: {:p}", &MAX_POINTS);

    // スタック
    // Rustでは、letで宣言した変数をスタックに格納する
    let i2: i64 = 1;
    let i3: i64 = 2;
    println!("Stack address of i2 is: {:p}", &i2);
    println!("Stack address of i3 is: {:p}", &i3);

    // シャドーイング
    // 一度宣言した変数を再宣言することで書き換えることをシャドーイングという
    let y = 5;
    println!("Stack address of y is: {:p}", &y);

    let y = y + 1;
    println!("Stack address of y is: {:p}", &y);

    let y = y * 2;
    println!("Stack address of y is: {:p}", &y);
    println!("The value of y is: {}", y);

    {
        let y = 0;
        println!("The value of y is: {}", y);
    }
    println!("The value of y is: {}", y);
}
