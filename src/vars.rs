// varsモジュール

// サブモジュールであるsub_aをmain.rsから呼び出せるようにするにはpubを付ける
pub mod sub_a;
mod sub_b;

// main.rsから呼び出すにはpubを付けることで公開可能
pub fn run() {
    println!("Here is vars module");
    sub_a::func_a();
    sub_b::func_b();
}
