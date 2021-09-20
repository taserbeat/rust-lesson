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

    // Deep Copy (clone)
    let s3 = String::from("hello");
    let s4 = s3.clone();
    println!("Stack address of s3 is: {:p}", &s3);
    println!("Stack address of s4 is: {:p}", &s4);
    println!("Heap memory address of s3's hello is: {:?}", s3.as_ptr());
    println!("Heap memory address of s4's hello is: {:?}", s4.as_ptr());
    println!("{}, {}", s3, s4);

    // 関数の引数に渡した場合の所有権について
    let s5 = String::from("hello");
    println!("Stack address of s5 is: {:p}", &s5);
    println!("Heap memory address of s5's hello is: {:?}", s5.as_ptr());
    println!("Length of s5 is: {}", s5.len());
    println!("Capacity of s5 is: {}\n", s5.capacity());

    take_ownership(s5);
    /*
    take_ownership関数の引数に渡したs5は所有権も渡す。
    そのため、以降はs5を使うことができない。
    また、take_ownership関数では関数のスコープを抜けるときにHeap領域のデータもdrop(解放)される。
    */
    // println!("{}", s5);

    /*
    関数の戻り値には所有権も含まれる。
    以下では、s6が持っている所有権をtake_giveback_ownership関数に渡し、
    take_giveback_ownership関数からs7に所有権が譲渡されていく。
    */
    let s6 = String::from("hello");
    println!("Stack address of s6 is: {:p}", &s6);
    println!("Heap memory address of s6's data is: {:?}", s6.as_ptr());
    println!("Length of s6 is: {}\n", s6.len());

    let s7 = take_giveback_ownership(s6);
    println!("Stack address of s7 is: {:p}", &s7);
    println!("Heap memory address of s7's data is: {:?}", s7.as_ptr());
    println!("Length of s7 is: {}\n", s7.len());

    // 参照
    // 所有権を譲渡せずに、参照させることができる
    let s8 = String::from("hello");
    let len = calculate_length(&s8);
    println!("The Length of '{}' is {}.\n", s8, len);

    // 参照 (変更)
    let mut s9 = String::from("hello");
    add_world(&mut s9);
    println!("s9 (add_world): {}\n", s9);

    // 1つの変数からイミュータブルな参照を複数作ることができる
    let s10 = String::from("hello");
    let r1 = &s10;
    let r2 = &s10;
    println!("{}, {}, {}\n", s10, r1, r2);
}

fn take_ownership(s: String) {
    println!("---This is take_ownership function---");
    println!("Stack address of s is: {:p}", &s);
    println!("Heap memory address of s's data is: {:?}", s.as_ptr());
    println!("Length of s is: {}", s.len());
    println!("Capacity of s is: {}", s.capacity());

    println!("take_ownership: {}", s);
    println!("---End of take_ownership function---\n");
}

fn take_giveback_ownership(s: String) -> String {
    s
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn add_world(s: &mut String) {
    s.push_str("_world")
}
