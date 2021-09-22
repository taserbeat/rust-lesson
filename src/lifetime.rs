pub fn run() {
    let st1 = String::from("x");
    let st2 = String::from("y");

    let res1 = get_longest(&st1, &st2);
    println!("res1: {}", res1);

    let st3 = String::from("x");
    let res2;
    {
        let st4 = String::from("y");
        res2 = get_longest(&st3, &st4);
        println!("res2: {}", res2);
    }
}

/**
### ジェネリックライフタイムアノテーション
引数として渡されるxとyは、それぞれの実引数のライフタイムが異なる場合がある。
以下のように<'a>というライフタイムのジェネックを付けることで、
どちらかのライフタイムのうち短い方を指定するという表現となる。
*/
fn get_longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// fn dummy1<'a>() -> &'a str {
//     let s = String::from("demo");
//     &s
// }

// fn dummy2<'a>() -> &'a i32 {
//     let x = 10;
//     &x
// }

fn dummy3() -> String {
    let s = String::from("demo");
    s
}
