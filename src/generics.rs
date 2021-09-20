// 構造体
struct Point<T> {
    x: T,
    y: T,
}

struct PointAnother<T, U> {
    x: T,
    y: U,
}

// Implementation: 構造体にメソッドを持たせることができる
impl<T, U> PointAnother<T, U> {
    /** 2つのPointAnother型のxとyを組み合わせたものにする */
    fn mixup<V, W>(self, other: PointAnother<V, W>) -> PointAnother<T, W> {
        PointAnother {
            x: self.x,
            y: other.y,
        }
    }
}

pub fn run() {
    let number_list = vec![34, 50, 25, 100, 65];
    println!("The largest is: {}", largest(number_list));

    let char_list = vec!['a', 'd', 'c', 'b'];
    println!("The largest is {}", largest(char_list));

    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 1.0, y: 2.0 };
    println!("p1: {}, {}", p1.x, p1.y);
    println!("p2: {}, {}", p2.x, p2.y);

    let p3 = PointAnother { x: 5, y: 12.3 };
    let p4 = PointAnother { x: 'a', y: "hello" };

    let p5 = p3.mixup(p4);
    println!("p5: {}, {}", p5.x, p5.y);
}

// ジェネリクスで指定する型に制約を設けることを「トレイト境界」と呼ぶ
// トレイト境界はその型で操作や比較ができる内容を示すものである。(他言語のインターフェースのような概念)
fn largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largest = list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
