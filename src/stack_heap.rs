pub fn run() {
    println!("Here is stack_heap module");

    // Stackには合計で最大8MBまで確保できる
    // 9MBを確保しようとするとエラーになる
    // let 12: [u8; 9000000] = [1; 9000000];

    // Vector型
    let mut v1 = vec![1, 2, 3, 4];
    let v2 = vec![5, 6, 7, 8];
    let mut v3 = vec![9, 10];

    println!("Stack address of v1 is {:p}", &v1);
    println!("Stack address of v2 is {:p}", &v2);
    println!("Heap memory address of v1 is: {:?}", v1.as_ptr());
    println!("Length of v1 is: {}", v1.len());
    println!("Capacity of v1 is: {}", v1.capacity());

    v1.insert(1, 10);
    println!("v1 is: {:?}", v1);
    v1.remove(0);
    println!("v1 is: {:?}", v1);

    v1.append(&mut v3);
    println!("v1 is: {:?}", v1);
    println!("v3 is: {:?}", v3);
}
