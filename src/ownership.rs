pub fn run() {
    // 所有権について
    println!("This is ownership module");
    let s1 = String::from("hello");
    let s2 = s1; // s1からs2に所有権が移動する。移動後はs1を使うことができない。(ヒープ領域を使用するから)
    println!("{}", s2);

    let i1 = 1;
    let i2 = i1; // Copy traitが実装されているので、変数のデータが複製される。よって、i1もi2も使うことができる。(静的領域で完結するから)
    println!("{}, {}", i1, i2);
    println!("Stack address of i1 is: {:p}", &i1);
    println!("Stack address of i2 is: {:p}", &i2);

    let sl1 = "literal";
    let sl2 = sl1; // 文字列スライスも複製される。(静的領域で完結するから)
    println!("{}, {}", sl1, sl2);
    println!("Stack address of sl1 is: {:p}", &sl1);
    println!("Stack address of sl2 is: {:p}", &sl2);
}
