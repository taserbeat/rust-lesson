pub fn run() {
    let result1 = division_option(5.0, 0.0);
    match result1 {
        Some(x) => println!("Result: {:.3}", x),
        None => println!("couldn't divide!"),
    }

    let result2 = division_result(2.0, 0.0);
    match result2 {
        Ok(x) => println!("Result: {:3.}", x),
        Err(e) => println!("{}", e),
    }

    let array1 = [0, 1];
    let result3 = sum(&array1);
    match result3 {
        Some(x) => println!("Sum Result: {}", x),
        None => println!("Out of index!"),
    }
}

/**  0で割ってしまったときにエラー処理も行える割り算関数 */
fn division_option(x: f64, y: f64) -> Option<f64> {
    if y == 0.0 {
        None
    } else {
        Some(x / y)
    }
}

fn division_result(x: f64, y: f64) -> Result<f64, String> {
    if y == 0.0 {
        Err(String::from("couldn't divide!"))
    } else {
        Ok(x / y)
    }
}

fn sum(array: &[i32]) -> Option<i32> {
    let item0 = array.get(0)?;
    let item1 = array.get(1)?;
    let item2 = array.get(2)?;

    Some(item0 + item1 + item2)
}
